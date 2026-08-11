use stream_town_game::{load_runtime_config, run};

fn main() {
    let config = load_runtime_config().expect("game configuration must load and validate");
    run(config);
}
