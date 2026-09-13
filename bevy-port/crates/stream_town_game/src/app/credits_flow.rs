use super::*;

pub(super) struct CreditsFlowPlugin;

impl Plugin for CreditsFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Credits),
            (cleanup_loading_screen, spawn_credits).chain(),
        )
        .add_systems(
            Update,
            (
                drive_credits_animation,
                update_credits_fireworks,
                credits_skip_button.in_set(AccessibilityActionDispatch),
                credits_input,
            )
                .chain()
                .run_if(in_state(GameState::Credits)),
        )
        .add_systems(
            OnExit(GameState::Credits),
            (cleanup_state_entities, cleanup_credits),
        );
    }
}
