use std::{
    backtrace::Backtrace,
    fs, panic,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use stream_town_game::{load_player_settings, load_runtime_config, run};

fn main() {
    install_crash_reporter();
    let config = load_runtime_config().expect("game configuration must load and validate");
    let player_settings = load_player_settings().expect("player settings must load and validate");
    run(config, player_settings);
}

fn install_crash_reporter() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs());
        let directory = PathBuf::from(".stream-town").join("crashes");
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("unnamed");
        let report = format!(
            "Stream Town crash report\nunix_timestamp={timestamp}\nthread={thread_name}\n\n{panic_info}\n\nBacktrace:\n{}\n",
            Backtrace::force_capture()
        );
        if fs::create_dir_all(&directory).is_ok() {
            let path = directory.join(format!("crash-{timestamp}.log"));
            let _ = fs::write(path, report);
        }
        default_hook(panic_info);
    }));
}
