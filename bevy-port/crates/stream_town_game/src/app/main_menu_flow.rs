use super::*;

pub(super) struct MainMenuFlowPlugin;

impl Plugin for MainMenuFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (
                spawn_loading_screen,
                ensure_main_menu_loading,
                spawn_main_menu,
                spawn_menu_overlay,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    main_menu_input.in_set(AccessibilityActionDispatch),
                    main_menu_buttons.in_set(AccessibilityActionDispatch),
                    update_main_menu_buttons,
                    town_dialog_buttons.in_set(AccessibilityActionDispatch),
                    update_town_dialog_ui,
                    scroll_town_load_list,
                )
                    .chain(),
                menu_input,
                (
                    secrets_buttons.in_set(AccessibilityActionDispatch),
                    poll_secrets_authorization,
                    update_secrets_ui,
                    sync_sensitive_screen_active.in_set(SensitiveScreenUpdateSet),
                )
                    .chain(),
                settings_tab_buttons.in_set(AccessibilityActionDispatch),
                settings_value_buttons.in_set(AccessibilityActionDispatch),
                settings_action_buttons.in_set(AccessibilityActionDispatch),
                (rebuild_settings_rows, update_settings_value_rows).chain(),
                update_settings_controls,
                update_menu_overlay,
                (
                    spawn_loading_screen,
                    begin_world_loading_cover,
                    advance_world_loading_cover,
                )
                    .chain()
                    .run_if(world_loading_cover_requested),
                (
                    animate_loading_icon,
                    spawn_main_menu_incrementally,
                    finish_menu_reveal,
                )
                    .chain(),
                sync_boot_loading_screen,
                animate_fish_school,
                animate_main_menu_clouds,
                hide_main_menu_inactive_model_nodes,
                enforce_main_menu_building_shadow_casters,
                tag_main_menu_rotating_nodes,
                rotate_main_menu_nodes
                    .after(tag_main_menu_rotating_nodes)
                    .after(hide_main_menu_inactive_model_nodes),
            )
                .chain()
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                go_live_confirmation_buttons.in_set(AccessibilityActionDispatch),
                update_go_live_confirmation_ui,
            )
                .chain()
                .after(main_menu_buttons)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            smoke_start_new_game_after_menu_reveal
                .after(finish_menu_reveal)
                .before(go_live_confirmation_buttons)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), cleanup_state_entities);
    }
}
