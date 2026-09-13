use super::*;

pub(super) struct PresentationFlowPlugin;

impl Plugin for PresentationFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sync_primary_window_settings,
                apply_player_settings.run_if(resource_changed::<RuntimePlayerSettings>),
            ),
        )
        .add_systems(Update, ground_loaded_surface_visuals)
        .add_systems(
            Update,
            sync_authored_post_processing
                .after(apply_player_settings)
                .after(update_environment_presentation),
        );
    }
}
