use bevy::app::AppExit;
use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_tidal::{
    NativeAudioState, NativeAudioStatus, TidalBackendState, TidalBackendStatus, TidalController,
    TidalEvent, TidalPlugin,
};
use std::fs;
use std::num::NonZeroU8;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const TIDAL_BUFFER_FILE: &str = "tidal_console.tidal";
const INTEGRATION_TIMEOUT: Duration = Duration::from_secs(15);
const INTEGRATION_PLAY_TIME: Duration = Duration::from_secs(5);

fn main() {
    let test_file = requested_test_file();
    let integration = test_file.map(|path| {
        let code = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("Could not read test file {}: {error}", path.display()));
        IntegrationRun {
            path,
            code,
            started: Instant::now(),
            sent_at: None,
        }
    });

    if let Some(integration) = integration {
        run_headless_integration(integration);
    } else {
        run_console();
    }
}

fn run_headless_integration(integration: IntegrationRun) {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .add_plugins(LogPlugin::default())
        .add_plugins(TidalPlugin::default())
        .insert_resource(integration)
        .add_systems(Update, run_integration_test)
        .run();
}

fn run_console() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Tidal".to_owned(),
                    resolution: WindowResolution::new(640, 920),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .build()
            .disable::<bevy::audio::AudioPlugin>(),
    )
    .add_plugins(EguiPlugin::default())
    .add_plugins(TidalPlugin::default())
    .add_systems(Startup, (setup, setup_ui))
    .add_systems(Update, log_tidal_events)
    .add_systems(EguiPrimaryContextPass, ui_panel)
    .add_systems(Last, save_tidal_buffer_on_exit);

    app.run();
}

fn requested_test_file() -> Option<PathBuf> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for (index, argument) in args.iter().enumerate() {
        if matches!(argument.as_str(), "--test-file" | "--test") {
            return args.get(index + 1).map(PathBuf::from);
        }
        if argument == "--autotest" {
            return Some(PathBuf::from("test1.tidal"));
        }
    }
    None
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn log_tidal_events(mut reader: MessageReader<TidalEvent>) {
    for event in reader.read() {
        debug!(target: "tidal", "Native event: {event:?}");
    }
}

#[derive(Resource, Default)]
struct UiState {
    input: String,
    status: String,
    save_as: String,
}

#[derive(Resource, Default)]
struct UiLogOnce(bool);

fn setup_ui(mut commands: Commands) {
    let input = fs::read_to_string(TIDAL_BUFFER_FILE).unwrap_or_default();
    commands.insert_resource(UiState {
        input,
        status: "Ready".to_owned(),
        save_as: TIDAL_BUFFER_FILE.to_owned(),
    });
    commands.insert_resource(UiLogOnce::default());
}

fn ui_panel(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    controller: Res<TidalController>,
    mut once: ResMut<UiLogOnce>,
    backend: Res<TidalBackendStatus>,
    audio: Res<NativeAudioStatus>,
) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };
    if !once.0 {
        info!(target: "tidal", "Showing zero-setup native Bevy Tidal console");
        context.set_zoom_factor(2.0);
        once.0 = true;
    }

    let screen_rect = context.content_rect();
    let window_size = egui::vec2(560.0, 900.0);
    let center = screen_rect.center();
    let position = egui::pos2(
        (center.x - window_size.x * 0.5).max(0.0),
        (center.y - window_size.y * 0.5).max(0.0),
    );

    egui::Window::new("Tidal Console")
        .default_pos(position)
        .default_size(window_size)
        .resizable(false)
        .collapsible(false)
        .show(context, |ui| {
            ui.label(format!("Pattern engine: {}", backend.detail()));
            ui.label(format!("Audio engine: {}", audio.detail()));
            ui.label(&state.status);
            ui.separator();

            let available_width = ui.available_width();
            let scroll_height = 700.0;
            let response = egui::ScrollArea::vertical()
                .max_height(scroll_height)
                .show(ui, |ui| {
                    ui.set_width(available_width);
                    ui.add_sized(
                        egui::vec2(available_width, scroll_height),
                        egui::TextEdit::multiline(&mut state.input),
                    )
                })
                .inner;

            let (enter_pressed, control_down) =
                ui.input(|input| (input.key_pressed(egui::Key::Enter), input.modifiers.ctrl));
            let shortcut = response.has_focus() && enter_pressed && control_down;
            if shortcut {
                while state.input.ends_with(['\n', '\r']) {
                    state.input.pop();
                }
            }
            if ui.button("Send").clicked() || shortcut {
                if !backend.is_ready() {
                    state.status = "The native pattern engine is not ready yet".to_owned();
                } else if !audio.is_ready() {
                    state.status = "The native audio engine is not ready yet".to_owned();
                } else {
                    let code = state.input.trim();
                    match controller.send(code) {
                        Ok(()) => state.status = "Pattern applied by the native engine".to_owned(),
                        Err(error) => state.status = error,
                    }
                }
            }

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut state.save_as);
                if ui.button("Save").clicked() {
                    save_buffer(&mut state);
                }
                if ui.button("Load").clicked() {
                    load_buffer(&mut state);
                }
                if ui.button("Hush").clicked() {
                    match controller.hush() {
                        Ok(()) => state.status = "Sent: hush".to_owned(),
                        Err(error) => state.status = error,
                    }
                }
            });
        });
}

fn save_buffer(state: &mut UiState) {
    let Some(filename) = normalized_tidal_filename(&state.save_as) else {
        state.status = "Save filename is empty".to_owned();
        return;
    };
    state.save_as = filename.clone();
    match fs::write(&filename, &state.input) {
        Ok(()) => state.status = format!("Saved to {filename}"),
        Err(error) => state.status = format!("Save failed: {error}"),
    }
}

fn load_buffer(state: &mut UiState) {
    let Some(filename) = normalized_tidal_filename(&state.save_as) else {
        state.status = "Load filename is empty".to_owned();
        return;
    };
    state.save_as = filename.clone();
    match fs::read_to_string(&filename) {
        Ok(contents) => {
            state.input = contents;
            state.status = format!("Loaded {filename}");
        }
        Err(error) => state.status = format!("Load failed: {error}"),
    }
}

fn normalized_tidal_filename(raw: &str) -> Option<String> {
    let mut filename = raw.trim().to_owned();
    if filename.is_empty() {
        return None;
    }
    if !filename.to_ascii_lowercase().ends_with(".tidal") {
        filename.push_str(".tidal");
    }
    Some(filename)
}

#[derive(Resource)]
struct IntegrationRun {
    path: PathBuf,
    code: String,
    started: Instant,
    sent_at: Option<Instant>,
}

fn run_integration_test(
    integration: Option<ResMut<IntegrationRun>>,
    controller: Res<TidalController>,
    backend: Res<TidalBackendStatus>,
    audio: Res<NativeAudioStatus>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(mut integration) = integration else {
        return;
    };
    if backend.state() == TidalBackendState::Error {
        error!("Integration test failed: {}", backend.detail());
        exit.write(AppExit::Error(NonZeroU8::new(1).expect("one is non-zero")));
        return;
    }
    if audio.state() == NativeAudioState::Error {
        error!("Integration test failed: {}", audio.detail());
        exit.write(AppExit::Error(NonZeroU8::new(1).expect("one is non-zero")));
        return;
    }
    if integration.started.elapsed() > INTEGRATION_TIMEOUT {
        error!("Integration test timed out before the full audio stack became ready");
        exit.write(AppExit::Error(NonZeroU8::new(2).expect("two is non-zero")));
        return;
    }
    if integration.sent_at.is_none() && backend.is_ready() && audio.is_ready() {
        match controller.send(integration.code.trim()) {
            Ok(()) => {
                info!(
                    "Running integration pattern from {} through native audio",
                    integration.path.display()
                );
                integration.sent_at = Some(Instant::now());
            }
            Err(error) => {
                error!("Integration test could not send its pattern: {error}");
                exit.write(AppExit::Error(
                    NonZeroU8::new(3).expect("three is non-zero"),
                ));
            }
        }
    }
    if integration
        .sent_at
        .is_some_and(|sent_at| sent_at.elapsed() >= INTEGRATION_PLAY_TIME)
    {
        let _ = controller.hush();
        if audio.events_received() == 0 {
            error!("Integration test sent its pattern, but native audio received no events");
            exit.write(AppExit::Error(NonZeroU8::new(4).expect("four is non-zero")));
        } else {
            info!(
                "Integration pattern completed successfully with {} native audio events",
                audio.events_received()
            );
            exit.write(AppExit::Success);
        }
    }
}

fn save_tidal_buffer_on_exit(mut exit_events: MessageReader<AppExit>, state: Option<Res<UiState>>) {
    if exit_events.read().next().is_some()
        && let Some(state) = state
    {
        let _ = fs::write(TIDAL_BUFFER_FILE, &state.input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tidal_extension_is_added_once() {
        assert_eq!(
            normalized_tidal_filename("test"),
            Some("test.tidal".to_owned())
        );
        assert_eq!(
            normalized_tidal_filename("test.TIDAL"),
            Some("test.TIDAL".to_owned())
        );
    }
}
