use super::*;

pub(super) struct WorldLoadingFlowPlugin;

impl Plugin for WorldLoadingFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::WorldLoading),
            (
                cleanup_loading_runtime,
                restore_town_camera_for_world,
                spawn_loading_screen,
                begin_world_loading,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                animate_loading_icon,
                poll_world_loading,
                generate_and_spawn_world.after(poll_world_loading),
                sync_gpu_world_expectations.after(apply_material_overrides),
                advance_loading_phase,
                sync_loading_screen_status,
            )
                .chain()
                .run_if(in_state(GameState::WorldLoading)),
        );
    }
}
