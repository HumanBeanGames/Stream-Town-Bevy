use stream_town_domain::GameConfig;
use stream_town_game::run;

fn main() {
    let config = ron::from_str::<GameConfig>(include_str!("../../../assets/config/game.ron"))
        .expect("embedded game configuration must be valid RON");
    config
        .validate()
        .expect("embedded game configuration must pass validation");
    run(config);
}
