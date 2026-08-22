use crate::TidalConfig;
use crate::audio::NativeAudioSender;
use crate::pattern::{ProgramCommand, TrackDefinition, parse_program};
use bevy::app::AppExit;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TidalBackendState {
    Starting,
    Ready,
    Error,
    Stopped,
}

#[derive(Clone, Debug)]
struct BackendStatusInner {
    state: TidalBackendState,
    detail: String,
}

/// Thread-safe status for the native Rust pattern scheduler.
#[derive(Resource, Clone, Debug)]
pub struct TidalBackendStatus(Arc<RwLock<BackendStatusInner>>);

impl Default for TidalBackendStatus {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(BackendStatusInner {
            state: TidalBackendState::Starting,
            detail: "Waiting to start the native pattern engine".to_owned(),
        })))
    }
}

impl TidalBackendStatus {
    pub fn state(&self) -> TidalBackendState {
        self.0
            .read()
            .map(|status| status.state.clone())
            .unwrap_or(TidalBackendState::Error)
    }

    pub fn detail(&self) -> String {
        self.0
            .read()
            .map(|status| status.detail.clone())
            .unwrap_or_else(|_| "Pattern-engine status lock was poisoned".to_owned())
    }

    pub fn is_ready(&self) -> bool {
        self.state() == TidalBackendState::Ready
    }

    fn set(&self, state: TidalBackendState, detail: impl Into<String>) {
        if let Ok(mut status) = self.0.write() {
            status.state = state;
            status.detail = detail.into();
        }
    }
}

enum BackendCommand {
    Apply(Vec<ProgramCommand>),
    Shutdown,
}

/// Controls the in-process pattern engine. Parsing happens synchronously, so a
/// syntax error is returned to the calling Bevy system instead of a child
/// process or log file.
#[derive(Resource, Clone)]
pub struct TidalController(mpsc::Sender<BackendCommand>);

impl TidalController {
    /// Parse and apply one or more `d1`/`d2`/etc. statements.
    pub fn send(&self, code: impl AsRef<str>) -> Result<(), String> {
        let commands = parse_program(code.as_ref())?;
        self.0
            .send(BackendCommand::Apply(commands))
            .map_err(|_| "The native pattern engine is not running".to_owned())
    }

    /// Convenience API for assigning a single expression to a numbered track.
    pub fn play(&self, track: u64, expression: impl AsRef<str>) -> Result<(), String> {
        self.send(format!("d{track} $ {}", expression.as_ref()))
    }

    /// Stop one numbered track.
    pub fn silence(&self, track: u64) -> Result<(), String> {
        self.send(format!("d{track} $ silence"))
    }

    /// Stop every active track.
    pub fn hush(&self) -> Result<(), String> {
        self.send("hush")
    }

    /// Change the global tempo without restarting the Bevy application.
    pub fn set_cycles_per_second(&self, cycles_per_second: f64) -> Result<(), String> {
        self.send(format!("setcps {cycles_per_second}"))
    }

    /// Assign a pattern using Tidal's `p \"name\"` form.
    pub fn play_named(&self, name: &str, expression: impl AsRef<str>) -> Result<(), String> {
        if name.contains('"') {
            return Err("A pattern name cannot contain a quote".to_owned());
        }
        self.send(format!("p \"{name}\" $ {}", expression.as_ref()))
    }

    /// Stop a pattern previously assigned with [`Self::play_named`].
    pub fn silence_named(&self, name: &str) -> Result<(), String> {
        if name.contains('"') {
            return Err("A pattern name cannot contain a quote".to_owned());
        }
        self.send(format!("p \"{name}\" $ silence"))
    }

    /// Play an expression for one cycle.
    pub fn once(&self, expression: impl AsRef<str>) -> Result<(), String> {
        self.send(format!("once $ {}", expression.as_ref()))
    }

    /// Stop patterns and immediately clear voices in the native mixer.
    pub fn panic(&self) -> Result<(), String> {
        self.send("panic")
    }
}

pub(crate) struct TidalBackendPlugin;

impl Plugin for TidalBackendPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::channel();
        app.insert_resource(TidalController(tx))
            .init_resource::<TidalBackendStatus>()
            .add_systems(Startup, start_tidal_backend)
            .add_systems(Last, stop_tidal_backend);
        app.world_mut().insert_non_send(BackendReceiver(Some(rx)));
    }
}

struct BackendReceiver(Option<mpsc::Receiver<BackendCommand>>);

#[derive(Resource)]
struct BackendThread(Mutex<Option<thread::JoinHandle<()>>>);

fn start_tidal_backend(
    mut commands: Commands,
    config: Res<TidalConfig>,
    audio: Res<NativeAudioSender>,
    status: Res<TidalBackendStatus>,
    mut receiver: NonSendMut<BackendReceiver>,
) {
    let Some(rx) = receiver.0.take() else {
        status.set(
            TidalBackendState::Error,
            "Native pattern engine was already started",
        );
        return;
    };
    let config = config.clone();
    let audio = audio.clone();
    let status = status.clone();
    let handle = thread::Builder::new()
        .name("bevy-tidal-patterns".to_owned())
        .spawn(move || run_backend(config, audio, status, rx))
        .expect("failed to spawn the native pattern-engine thread");
    commands.insert_resource(BackendThread(Mutex::new(Some(handle))));
}

fn stop_tidal_backend(
    mut exits: MessageReader<AppExit>,
    controller: Option<Res<TidalController>>,
    backend_thread: Option<Res<BackendThread>>,
) {
    if exits.read().next().is_some()
        && let Some(controller) = controller
    {
        let _ = controller.0.send(BackendCommand::Shutdown);
        if let Some(backend_thread) = backend_thread
            && let Ok(mut handle) = backend_thread.0.lock()
            && let Some(handle) = handle.take()
        {
            let _ = handle.join();
        }
    }
}

struct ActiveTrack {
    definition: TrackDefinition,
    origin: Instant,
    next_cycles: Vec<u64>,
    one_shot: bool,
}

impl ActiveTrack {
    fn new(definition: TrackDefinition, origin: Instant, one_shot: bool) -> Self {
        Self {
            next_cycles: vec![0; definition.layer_count()],
            definition,
            origin,
            one_shot,
        }
    }
}

fn run_backend(
    config: TidalConfig,
    audio: NativeAudioSender,
    status: TidalBackendStatus,
    rx: mpsc::Receiver<BackendCommand>,
) {
    if !config.cycles_per_second.is_finite() || config.cycles_per_second <= 0.0 {
        status.set(
            TidalBackendState::Error,
            "cycles_per_second must be positive and finite",
        );
        return;
    }
    let mut cycles_per_second = config.cycles_per_second;
    let lookahead = config.scheduler_lookahead.max(Duration::from_millis(10));
    status.set(
        TidalBackendState::Ready,
        "Native Rust pattern engine ready — no external runtime",
    );
    let mut tracks = HashMap::<u64, ActiveTrack>::new();

    loop {
        match rx.recv_timeout(Duration::from_millis(5)) {
            Ok(BackendCommand::Apply(commands)) => {
                let panic =
                    apply_commands(commands, &mut tracks, &mut cycles_per_second, lookahead);
                if panic {
                    let _ = audio.schedule(
                        Instant::now(),
                        crate::audio::TidalEvent {
                            sound: "__panic__".to_owned(),
                            ..Default::default()
                        },
                    );
                }
                status.set(
                    TidalBackendState::Ready,
                    format!(
                        "Native Rust pattern engine: {} active track(s)",
                        tracks.len()
                    ),
                );
            }
            Ok(BackendCommand::Shutdown) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }

        let base_cycle_seconds = 1.0 / cycles_per_second;
        if let Err(error) = schedule_tracks(&mut tracks, &audio, base_cycle_seconds, lookahead) {
            status.set(TidalBackendState::Error, error);
            return;
        }
    }

    tracks.clear();
    if status.state() != TidalBackendState::Error {
        status.set(TidalBackendState::Stopped, "Native pattern engine stopped");
    }
}

fn apply_commands(
    commands: Vec<ProgramCommand>,
    tracks: &mut HashMap<u64, ActiveTrack>,
    cycles_per_second: &mut f64,
    lookahead: Duration,
) -> bool {
    let mut panic = false;
    for command in commands {
        match command {
            ProgramCommand::Set { track, definition } => {
                tracks.insert(
                    track,
                    ActiveTrack::new(definition, Instant::now() + lookahead, false),
                );
            }
            ProgramCommand::Silence(track) => {
                tracks.remove(&track);
            }
            ProgramCommand::Hush => tracks.clear(),
            ProgramCommand::SetCps(value) => {
                *cycles_per_second = value;
                let origin = Instant::now() + lookahead;
                for track in tracks.values_mut() {
                    track.origin = origin;
                    track.next_cycles.fill(0);
                }
            }
            ProgramCommand::Once(definition) => {
                tracks.insert(
                    u64::MAX,
                    ActiveTrack::new(definition, Instant::now() + lookahead, true),
                );
            }
            ProgramCommand::Panic => {
                tracks.clear();
                panic = true;
            }
        }
    }
    panic
}

fn schedule_tracks(
    tracks: &mut HashMap<u64, ActiveTrack>,
    audio: &NativeAudioSender,
    base_cycle_seconds: f64,
    lookahead: Duration,
) -> Result<(), String> {
    let horizon = Instant::now() + lookahead;
    let mut completed = Vec::new();
    for (&track_number, track) in tracks.iter_mut() {
        for layer_index in 0..track.definition.layer_count() {
            let period = track
                .definition
                .layer_period(layer_index, base_cycle_seconds);
            let mut cycles_scheduled = 0;
            loop {
                let cycle = track.next_cycles[layer_index];
                if track.one_shot && cycle > 0 {
                    break;
                }
                let cycle_start = track.origin + Duration::from_secs_f64(period * cycle as f64);
                if cycle_start > horizon {
                    break;
                }
                for (phase, event) in
                    track
                        .definition
                        .events_for_layer(layer_index, track_number, cycle, period)
                {
                    let at = cycle_start + Duration::from_secs_f64(period * phase);
                    audio.schedule(at, event)?;
                }
                track.next_cycles[layer_index] = cycle.saturating_add(1);
                cycles_scheduled += 1;
                if cycles_scheduled >= 1_024 {
                    return Err("Pattern rate is too high for the native scheduler".to_owned());
                }
            }
        }
        if track.one_shot && track.next_cycles.iter().all(|cycle| *cycle > 0) {
            completed.push(track_number);
        }
    }
    for track in completed {
        tracks.remove(&track);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controller_reports_parse_errors_synchronously() {
        let (tx, _rx) = mpsc::channel();
        let controller = TidalController(tx);
        let error = controller
            .send("not a pattern")
            .expect_err("syntax should fail");
        assert!(error.contains("Expected a d1/d2"));
    }

    #[test]
    fn applying_hush_removes_every_track() {
        let ProgramCommand::Set { definition, .. } = parse_program("d1 $ sound \"bd\"")
            .expect("pattern should parse")
            .remove(0)
        else {
            panic!("expected a set command");
        };
        let mut tracks = HashMap::new();
        let mut cycles_per_second = 0.5;
        apply_commands(
            vec![ProgramCommand::Set {
                track: 1,
                definition,
            }],
            &mut tracks,
            &mut cycles_per_second,
            Duration::from_millis(10),
        );
        assert_eq!(tracks.len(), 1);
        apply_commands(
            vec![ProgramCommand::Hush],
            &mut tracks,
            &mut cycles_per_second,
            Duration::from_millis(10),
        );
        assert!(tracks.is_empty());
    }
}
