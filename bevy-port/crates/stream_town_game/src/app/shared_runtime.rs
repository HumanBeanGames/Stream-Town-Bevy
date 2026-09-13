use super::*;

pub(super) struct SharedRuntimePlugin;

impl Plugin for SharedRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            upgrade_actor_placeholders
                .after(generate_and_spawn_world)
                .before(sync_gpu_world_expectations)
                .run_if(in_state(GameState::WorldLoading).or_else(in_state(GameState::InGame)))
                .run_if(resource_exists::<WorldRuntime>),
        )
        .add_systems(
            Update,
            (
                poll_twitch_transport,
                twitch_connection_input,
                process_runtime_console,
                apply_town_persistence_requests
                    .after(process_runtime_console)
                    .before(save_input)
                    .before(load_input),
                publish_runtime_console_status.after(process_runtime_console),
                capture_screenshot,
                report_frame_time_gate,
                apply_authored_ui_fonts,
            ),
        )
        .add_systems(PostUpdate, sync_cursor_visibility)
        .add_systems(
            Update,
            (
                (tag_accessible_buttons, tag_accessible_text).chain(),
                enhance_accessible_buttons.after(tag_accessible_buttons),
                prune_decorative_accessibility_nodes,
                accessibility_input
                    .after(tag_accessible_buttons)
                    .before(AccessibilityActionDispatch),
                sync_accessibility_focus_visuals.after(accessibility_input),
                sync_accessibility_contrast,
                sync_accessibility_preferences,
                announce_accessibility_state,
            ),
        )
        .add_systems(
            Update,
            apply_material_overrides.run_if(in_rendered_game_state),
        );
    }
}
