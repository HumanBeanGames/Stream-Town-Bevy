use super::*;

pub(super) struct BootstrapFlowPlugin;

impl Plugin for BootstrapFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_rendering,
                setup_accessibility,
                (spawn_loading_screen, begin_menu_loading)
                    .chain()
                    .after(setup_rendering),
                setup_world_audio_sources.after(setup_rendering),
                initialize_twitch_account_state,
                start_twitch_transport,
                apply_player_settings.after(setup_rendering),
                sync_authored_post_processing.after(apply_player_settings),
            ),
        )
        .add_systems(
            Update,
            (
                animate_loading_icon,
                poll_menu_loading,
                sync_boot_loading_screen,
            )
                .chain()
                .run_if(in_state(GameState::Boot)),
        );
    }
}
