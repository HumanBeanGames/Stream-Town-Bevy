use stream_town_game::{load_player_settings, load_runtime_config, run};

fn main() {
    let config = load_runtime_config().expect("game configuration must load and validate");
    let player_settings = load_player_settings().expect("player settings must load and validate");
    run(config, player_settings);
}
