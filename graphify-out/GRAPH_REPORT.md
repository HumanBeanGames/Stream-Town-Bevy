# Graph Report - Stream-Town-Bevy  (2026-09-13)

## Corpus Check
- 81 files · ~347,035 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4507 nodes · 16892 edges · 145 communities (139 shown, 6 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 1470 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1395a18d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- StableId
- ContentCatalog
- tests.rs
- embedded_content
- Ui
- .default
- MenuRuntime
- sync_building_health_overlays
- spawn_main_menu
- WorldSimulation
- Handle
- sync_stream_only_capture
- update_environment_presentation
- SimulationError
- setup_rendering
- ModelPreviewRuntime
- update_secrets_ui
- .from
- AnimationControllerDef
- content.rs
- update_credits_fireworks
- stream_operator_chat_controls
- GridPos
- src/world.rs
- simulation.rs
- RenderAssets
- GameConfig
- Option
- drive_converted_animations
- Vec3
- menu.rs
- String
- Query
- capture_foliage_acceptance
- roles_tab
- update_vote_panels
- .new
- Option
- Option
- embedded_presentation
- town_camera_projection
- GameState
- update_hud
- navigation.rs
- rendering.rs
- bootstrap.rs
- ResolvedMaterialHandle
- config.rs
- DirectBroadcastRuntime
- RuntimeContent
- generate_world
- instantiate_building_materials
- profiling.rs
- Transform
- TechnologyGraphLayout
- .tick
- advance_world_loading_cover
- enter_headless_world
- ArchetypeDef
- timelapse.rs
- command.rs
- Mesh
- sync_pooled_night_lights
- generate_world_from_layers
- presentation/animation.rs
- TraversalWearRuntime
- Agent
- presentation.rs
- twitch.rs
- ObjectiveDef
- camera_zoom_and_commands
- drive_tidal_music
- environment.rs
- capture_city_timelapse
- stream_town_tools/src/main.rs
- stream_town_domain/src/lib.rs
- .write
- .join_player
- update_enemy_music_intensity
- .walkable_neighbours_with
- PostProcessProfileDef
- healing_burst_effect
- navigation_surface_height_at_world
- VideoSettings
- save_secrets_fields
- BroadcastOutput
- sync_accessibility_preferences
- stream_town_game/src/lib.rs
- node_detail_lines
- stream_town_render_error_handler
- audio_acceptance_wavs
- OpenNode
- WorldDiagnosticRuntime
- settings.rs
- .visit
- EnemyPathOpenNode
- handle_twitch_event
- complete_gameplay_scenario_round_trips
- TimeCycleConfig
- PresentationCatalog
- CityTimelapsePlugin
- transformed_mesh_vertical_extent
- .new
- tidal_plugin
- technology_graph.rs
- xtask/src/main.rs
- tidal_music.rs
- What You Must Do When Invoked
- xtask/src/lib.rs
- Stream Town architecture
- Tree and Foliage Flicker Regression Checklist
- graphify reference: extra exports and benchmark
- encode_broadcast_session
- Character Animation Regression Checklist
- graphify reference: query, path, explain
- bevy-port/README.md
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- allocate_fifo_output
- extraction-spec.md
- stream_town_domain
- music/README.md
- Stream Town authoring suite
- Stream Town Twitch command reference
- ToolState
- vcpkg.json
- FFmpeg runtime and relinking
- RuntimeConfig
- agents.rs
- runtime_console.rs
- direct_broadcast.rs
- Res

## God Nodes (most connected - your core abstractions)
1. `StableId` - 477 edges
2. `WorldSimulation` - 269 edges
3. `ContentCatalog` - 252 edges
4. `GridPos` - 226 edges
5. `embedded_content()` - 148 edges
6. `GeneratedWorld` - 145 edges
7. `RenderAssets` - 144 edges
8. `ToolState` - 142 edges
9. `GameConfig` - 127 edges
10. `RuntimeConfig` - 114 edges

## Surprising Connections (you probably didn't know these)
- `world_loading_transition_reuses_the_already_rendered_cover_entities()` --calls--> `RuntimeConfig`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `town_hall_loss_carries_players_and_roles_into_a_real_world_reload()` --calls--> `SimulationRuntime`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `settings_pointer_controls_switch_tabs_and_adjust_values()` --calls--> `SettingsTabButton`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `production_resource_glbs_expose_unity_masks_as_color_zero()` --calls--> `locate_asset_root()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `foliage_surface_height_matches_the_rendered_terrain_triangles()` --calls--> `terrain_surface_height_at_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs

## Import Cycles
- None detected.

## Communities (145 total, 6 thin omitted)

### Community 0 - "StableId"
Cohesion: 0.04
Nodes (158): BuildingDef, EnemyCampGenerationDef, PassiveResourceContribution, RoleSlotContribution, StorageContribution, TargetingScoreDef, Display, FromStr (+150 more)

### Community 1 - "ContentCatalog"
Cohesion: 0.07
Nodes (109): ContentCatalog, ActorState, BuildingState, RoleProgress, Default, String, ActionPresentation, actor_health_bar_hide_seconds() (+101 more)

### Community 2 - "tests.rs"
Cohesion: 0.02
Nodes (40): building_material_cycle_stays_phase_aligned_without_cpu_asset_updates(), camera_smoothing_is_frame_rate_independent(), citizen_auto_camera_translation_centres_the_follow_target_at_close_zoom(), combat_camera_holds_a_target_for_five_seconds_before_redirecting(), converted_playback_preserves_authored_loop_modes(), enemy_flow_field_routes_each_component_to_its_nearest_town_target(), failed_regeneration_target_search_is_backed_off(), falling_fish_sampling_is_repeatable_and_sequence_sensitive() (+32 more)

### Community 3 - "embedded_content"
Cohesion: 0.04
Nodes (57): different_town_seeds_produce_different_resource_and_foliage_layouts(), generate_world_with_content(), embedded_content(), archetype_id_by_source(), weighted_enemy_archetype(), authored_building_and_role_balance_is_explicit_and_complete(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats() (+49 more)

### Community 4 - "Ui"
Cohesion: 0.10
Nodes (67): ArchetypeKind, animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice() (+59 more)

### Community 5 - ".default"
Cohesion: 0.07
Nodes (45): authoring_apply_preserves_the_complete_local_twitch_setup(), authoring_config_save_bar(), authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_presentation_path(), default_technology_layout_path() (+37 more)

### Community 6 - "MenuRuntime"
Cohesion: 0.09
Nodes (60): PlayerSettings, MenuIoRequest, MenuPage, MenuRuntime, PendingTownStart, Receiver, RuntimePlayerSettings, SaveRuntime (+52 more)

### Community 7 - "sync_building_health_overlays"
Cohesion: 0.05
Nodes (74): AudioSink, ActorHealthFill, ActorNameOverlay, advance_agent_simulation_cadence(), advance_stream_only_cadence(), agent_simulation_due(), AgentSimulationCadence, AgentSimulationCadenceState (+66 more)

### Community 8 - "spawn_main_menu"
Cohesion: 0.09
Nodes (36): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+28 more)

### Community 9 - "WorldSimulation"
Cohesion: 0.09
Nodes (26): CommunityEvent, CommunityVoteOutcome, CommunityVoteProposal, CommunityVoteState, EnemyCampState, RaidState, BTreeSet, Option (+18 more)

### Community 10 - "Handle"
Cohesion: 0.06
Nodes (36): AccessibilityMotionDefaults, BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CachedRoleActionAudio, CharacterMaterialExtension, CharacterMaterialUniform (+28 more)

### Community 11 - "sync_stream_only_capture"
Cohesion: 0.08
Nodes (34): camera_targets_primary_window(), NativeGameAudioRouting, pcm16_wav_clip(), pcm16_wav_data(), prepared_broadcast_can_start(), Assets, Commands, Entity (+26 more)

### Community 12 - "update_environment_presentation"
Cohesion: 0.09
Nodes (31): AmbientLight, SeasonalTerrainPalette, Season, Weather, EnvironmentPalette, EnvironmentPresentation, WeatherParticle, blend_environment_palette() (+23 more)

### Community 13 - "SimulationError"
Cohesion: 0.16
Nodes (5): BTreeMap, Result, SimulationError, unattended_respawn_halves_every_role_level_but_role_revive_does_not(), validate_trade_resource()

### Community 14 - "setup_rendering"
Cohesion: 0.10
Nodes (50): setup_rendering(), water_color_tint(), bounds_material(), building_material(), character_material(), character_material_from_standard(), cloud_material(), critter_material() (+42 more)

### Community 15 - "ModelPreviewRuntime"
Cohesion: 0.08
Nodes (52): apply_preview_material_overrides(), apply_preview_node_visibility(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene, player_preview_material_overrides() (+44 more)

### Community 16 - "update_secrets_ui"
Cohesion: 0.11
Nodes (51): HudMetric, HudMetricMaximum, SecretsAction, SecretsConnectionKind, SecretsConnectionText, SecretsDynamicLabel, SettingsUiCache, SettingsValueRow (+43 more)

### Community 17 - ".from"
Cohesion: 0.06
Nodes (40): append_terrain_skirt(), apply_player_settings(), generated_terrain_chunk_mesh(), generated_terrain_chunks(), generated_terrain_mesh(), generated_terrain_uvs(), generated_water_mesh(), GeneratedTerrainChunk (+32 more)

### Community 18 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (29): AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires() (+21 more)

### Community 19 - "content.rs"
Cohesion: 0.10
Nodes (35): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingModelDef, default_resource_generation_layers(), EnemyDef, EnemyModelSetDef, EnemyRunAnimation (+27 more)

### Community 20 - "update_credits_fireworks"
Cohesion: 0.09
Nodes (48): AuthoredCreditsElement, CreditsFade, CreditsFireworkParticle, CreditsFireworkParticleKind, CreditsTimeline, LevelUpPresentation, LevelUpToast, pseudo_noise() (+40 more)

### Community 21 - "stream_operator_chat_controls"
Cohesion: 0.06
Nodes (43): bounded_history_f32(), CadenceTick, moderate_selected_operator_user(), operator_chat_scroll_rows(), operator_info_refresh_due(), operator_window_close_requests_exit(), Changed, Instant (+35 more)

### Community 22 - "GridPos"
Cohesion: 0.07
Nodes (95): GridPos, ActorKind, GeneratedResource, GeneratedWorld, PathSurfaceRuntime, RecentTreePlanting, RegenerationRoleRuntime, RegenerationWorkerState (+87 more)

### Community 23 - "src/world.rs"
Cohesion: 0.16
Nodes (23): avalanche_instance_hash(), cell_hash(), fnv_mix(), foliage_candidate_jitter(), foliage_visual_variant(), foliage_visual_yaw_milliradians(), hash_world(), legacy_resource_navigation() (+15 more)

### Community 24 - "simulation.rs"
Cohesion: 0.09
Nodes (25): actor_appearances_are_seeded_varied_and_persisted(), authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase(), capped_deposit_preserves_inventory_overflow(), capped_role_progression_discards_excess_at_the_requested_level(), default_community_event_fields_preserve_legacy_save_checksums(), default_ruler_vote_cooldown() (+17 more)

### Community 25 - "RenderAssets"
Cohesion: 0.08
Nodes (83): FoliageHabitat, BuildingEffectKind, BuildingEffectParticle, CombatProjectile, CombatVisualKind, FallingFish, FishGodExitTimer, GridLocation (+75 more)

### Community 26 - "GameConfig"
Cohesion: 0.07
Nodes (71): GameConfig, GameplayConfig, BTreeMap, RoleDef, StationDef, GeneratedFoliage, building_health_overlay_world_position(), grid_to_world() (+63 more)

### Community 27 - "Option"
Cohesion: 0.07
Nodes (48): BootDestination, GpuReadinessProbe, MenuLoadingRuntime, MenuRevealRuntime, PresentedRenderFrames, Arc, Default, Instant (+40 more)

### Community 28 - "drive_converted_animations"
Cohesion: 0.13
Nodes (27): AnimationBlendSelection, ConvertedAnimationCrossfade, ConvertedAnimationLayerDriver, ConvertedAnimationPlayback, FishGodAnimation, advance_animation_crossfade(), animation_event_occurrences(), animation_nodes_for_selection() (+19 more)

### Community 29 - "Vec3"
Cohesion: 0.08
Nodes (37): automatic_resume_world_seed(), ChimneySmokeEmitterRuntime, cleanup_credits(), cleanup_loading_runtime(), cleanup_menu_overlay(), clear_loading_runtime(), CombatImpactParticle, CombatTrailSegment (+29 more)

### Community 30 - "menu.rs"
Cohesion: 0.06
Nodes (62): AccessibleNode, FishSchoolParticle, MainMenuCloudPrism, SecretsCredentialState, SecretsStatusTone, accessibility_settings_selection(), actor_detail_budget(), actor_scene_budget() (+54 more)

### Community 31 - "String"
Cohesion: 0.08
Nodes (31): AnimatedCharacterShadowReceiver, CachedConvertedAnimation, CachedGateAnimation, CachedStationTargets, ConvertedAnimationCache, ConvertedAnimationLayerTemplate, GateAnimationCache, GateAnimationDriver (+23 more)

### Community 32 - "Query"
Cohesion: 0.14
Nodes (46): AnimatedBy, AnimationGraphHandle, ActivePetVisual, ConvertedAnimationApplied, ConvertedAnimationInstanceReady, ConvertedAnimationSpec, PlayerAnimatedRig, animated_player_renderer() (+38 more)

### Community 33 - "capture_foliage_acceptance"
Cohesion: 0.13
Nodes (27): ConvertedAnimationDriver, TownCamera, animation_binding_diagnostics_enabled(), capture_foliage_acceptance(), debug_player_model_bounds(), foliage_capture_camera(), quantized_render_transform(), report_animation_binding_diagnostics() (+19 more)

### Community 34 - "roles_tab"
Cohesion: 0.13
Nodes (33): ability_choices(), action_animation_choices(), building_model_node_choices(), buildings_tab(), cached_gltf_metadata(), character_model_choices_include_converted_hierarchy_nodes(), default_navigation_footprint_thirds(), delete_selected_role() (+25 more)

### Community 35 - "update_vote_panels"
Cohesion: 0.05
Nodes (88): BuildingPlacementGhost, BuildingPlacementGhostHiddenNodes, BuildingPlacers, CurrentEventFill, CurrentEventPanel, CurrentEventText, FineNavigationRuntime, PingPointer (+80 more)

### Community 36 - ".new"
Cohesion: 0.13
Nodes (22): apply_direct_broadcast_control(), closing_the_operator_window_requests_a_graceful_game_exit(), configure_direct_broadcast(), direct_broadcast_stays_offline_until_operator_requests_it(), DirectTwitchBroadcastPlugin, ending_stream_returns_the_operator_to_main_menu_after_shutdown(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting() (+14 more)

### Community 37 - "Option"
Cohesion: 0.10
Nodes (28): append_terrain_quad(), automatic_resume_save_path(), AutomaticResumeRuntime, CameraRequest, CharacterBaseMaterialCache, EnemyClusterNode, EnemyNavigationField, EnemyNavigationRuntime (+20 more)

### Community 38 - "Option"
Cohesion: 0.07
Nodes (53): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastVideoSink, capture_process_audio(), capture_stream_only_target(), configure_stream_capture_ring() (+45 more)

### Community 39 - "embedded_presentation"
Cohesion: 0.08
Nodes (31): embedded_presentation(), archetype_by_source(), converted_animation_spec(), default_archetype_scene(), runtime_archetype_scene(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), building_bounds_material_preserves_unity_placement_contract(), converted_crossfade_uses_fixed_or_normalized_authored_duration() (+23 more)

### Community 40 - "town_camera_projection"
Cohesion: 0.67
Nodes (3): Projection, town_camera_projection(), town_camera_keeps_the_unity_angle_with_the_wider_upper_third_frame()

### Community 41 - "GameState"
Cohesion: 0.11
Nodes (32): AccessibilityFocusVisualQuery, AccessibilityRuntime, AccessibleButtonScope, GameMenuAction, GameMenuActionLabel, GameState, MainMenuAction, accessibility_button_enabled() (+24 more)

### Community 42 - "update_hud"
Cohesion: 0.11
Nodes (25): Hud, HudTechnologyProgressFill, SeasonMeter, active_event_text(), fit_hud_label(), format_rgb(), hud_play_time(), hud_season_meter_percent() (+17 more)

### Community 43 - "navigation.rs"
Cohesion: 0.20
Nodes (18): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavigationError, octile_distance_with_costs(), open_ground_paths_use_diagonal_steps() (+10 more)

### Community 44 - "rendering.rs"
Cohesion: 0.05
Nodes (97): DirtyRegion, NavGrid, TechVote, BuildingPlacement, EnemyRouteBuilding, building_placement_is_available(), building_placement_overlay_text(), building_placement_overlay_world_position() (+89 more)

### Community 45 - "bootstrap.rs"
Cohesion: 0.14
Nodes (21): DisplayMode, adaptive_music_preview_program(), configure_automatic_live(), load_player_settings(), load_runtime_config(), player_settings_path(), player_window_mode(), AnyResult (+13 more)

### Community 46 - "ResolvedMaterialHandle"
Cohesion: 0.11
Nodes (31): ActiveMaterialHandles, BuildingMaterialInstance, BuildingMaterialUpdateRuntime, BuildingTimeCycleSignature, CharacterBaseMaterialVariant, CitizenDeathAnnouncementRuntime, cleanup_world(), CoreRenderAssets (+23 more)

### Community 47 - "config.rs"
Cohesion: 0.14
Nodes (22): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Option (+14 more)

### Community 48 - "DirectBroadcastRuntime"
Cohesion: 0.06
Nodes (40): AuthorizationEvent, AutomaticBroadcastStart, bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastMetricsSnapshot, BroadcastPrerequisites, BroadcastStopDisposition, BroadcastTarget (+32 more)

### Community 49 - "RuntimeContent"
Cohesion: 0.12
Nodes (31): App, Plugin, StreamTownGamePlugin, run(), locate_asset_root(), RuntimeAssetRoot, RuntimeContent, RuntimePresentation (+23 more)

### Community 50 - "generate_world"
Cohesion: 0.07
Nodes (29): changing_seed_changes_world_hash(), generate_world(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), builder_selection_uses_an_alternate_approach_after_path_rejection(), builders_can_work_from_every_planned_corner_approach(), builders_reserve_distinct_unoccupied_construction_approaches(), building_direction_commands_follow_the_visible_town_axes() (+21 more)

### Community 51 - "instantiate_building_materials"
Cohesion: 0.14
Nodes (25): ActorCustomization, BuildingMaterialInstances, CosmeticNode, CosmeticNodeKind, CosmeticRenderer, building_damage_value(), cosmetic_color(), cosmetic_node() (+17 more)

### Community 52 - "profiling.rs"
Cohesion: 0.08
Nodes (63): append_profile_sample(), ArchetypeProfile, asset_count(), asset_counts(), begin_profiled_frame(), build_profile_sample(), collect_profile_sample(), diagnostic_profiles() (+55 more)

### Community 53 - "Transform"
Cohesion: 0.18
Nodes (25): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), animation_target_for_track(), close_rotation_loop(), close_scale_loop() (+17 more)

### Community 54 - "TechnologyGraphLayout"
Cohesion: 0.19
Nodes (17): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+9 more)

### Community 55 - ".tick"
Cohesion: 0.17
Nodes (8): community_event_request_cooldown_and_strict_majority_are_enforced(), community_event_requests_queue_behind_ruler_votes_and_apply_majority_results(), healing_and_food_revives_preserve_health_invariants(), numbered_technology_ballot_waits_for_a_vote_and_resolves_the_winner(), ruler_and_technology_ballots_run_and_accept_votes_concurrently(), ruler_vote_rejects_duplicates_and_invalid_candidates(), stable(), trade_spending_follows_gold_votes_and_active_objectives()

### Community 56 - "advance_world_loading_cover"
Cohesion: 0.25
Nodes (18): AccessibilityActionRequest, AccessibilityNode, LoadingProgressFill, LoadingStatusText, advance_world_loading_cover(), loading_cover_ready(), loading_display_percent(), loading_percent_text() (+10 more)

### Community 57 - "enter_headless_world"
Cohesion: 0.25
Nodes (8): town_resource_amount(), enter_headless_world(), headless_launch_through_credits_round_trip_covers_shipping_states(), headless_new_town_matches_shipping_starting_roster(), native_load_preflight_preserves_live_town_on_invalid_building(), native_load_restores_saved_world_seed_after_runtime_config_changes(), App, starting_logger_and_miner_recover_from_bad_targets_and_gather_resources()

### Community 58 - "ArchetypeDef"
Cohesion: 0.23
Nodes (16): ArchetypeDef, ArchetypeScene, HealthDef, Option, menu_scene_for_source_model(), pet_model(), archetype_needs_self_shadow_suppression(), archetype_scene_for_age() (+8 more)

### Community 59 - "timelapse.rs"
Cohesion: 0.14
Nodes (20): apply_confirmed_builds(), CityTimelapseBuildConfirmed, CityTimelapseScreenshot, draw_timelapse_label(), next_timelapse_frame_index(), rebuild_timelapse_video(), MessageReader, On (+12 more)

### Community 60 - "command.rs"
Cohesion: 0.16
Nodes (33): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, command_usage(), CommandParseError, content_id() (+25 more)

### Community 61 - "Mesh"
Cohesion: 0.10
Nodes (19): AssetId, GpuReadinessExpected, GpuReadinessShared, GpuReadinessSnapshot, MainMenuHiddenModelNodes, PreparedWorld, round_robin_indices(), BTreeSet (+11 more)

### Community 62 - "sync_pooled_night_lights"
Cohesion: 0.17
Nodes (16): NightPointLightPoolSlot, animate_weather_particles(), building_night_light_profile(), night_light_transition_delay(), Commands, Entity, GlobalTransform, Option (+8 more)

### Community 63 - "generate_world_from_layers"
Cohesion: 0.17
Nodes (26): WorldGenConfig, authored_grid_centre(), authored_world_to_grid(), foliage_horizontal_hash(), FoliageCandidate, generate_authored_resources(), generate_candidate_mask(), generate_foliage() (+18 more)

### Community 64 - "presentation/animation.rs"
Cohesion: 0.15
Nodes (32): AnimationClipDef, add_animation_composition(), add_animation_layer_branch(), animation_root_name(), build_converted_animation(), converted_clip_request(), ConvertedClipRequest, enemy_animation_contract() (+24 more)

### Community 65 - "TraversalWearRuntime"
Cohesion: 0.15
Nodes (16): TerrainAppearanceConfig, CommandSaveRuntime, HashMap, traversal_score_for_rate(), traversal_wear_fraction(), TraversalWearCell, TraversalWearRuntime, record_completed_cell_traversal() (+8 more)

### Community 66 - "Agent"
Cohesion: 0.19
Nodes (15): ActorAnimationDriver, Agent, AgentAnimation, MovementAnimationState, TransientCarryVisibility, actor_carries_role_resource(), agent_action_animation(), agent_is_moving() (+7 more)

### Community 67 - "presentation.rs"
Cohesion: 0.07
Nodes (40): AnimationConditionMode, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationTangent, AnimationTransformTrack (+32 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (66): BTreeSet, TwitchConfig, SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault (+58 more)

### Community 69 - "ObjectiveDef"
Cohesion: 0.16
Nodes (13): ObjectiveDef, ObjectiveKind, common_gathered_resource(), is_gold_collect_objective(), objective_increment(), ObjectiveEvent, ObjectiveProgress, Vec (+5 more)

### Community 70 - "camera_zoom_and_commands"
Cohesion: 0.11
Nodes (22): AnimationTransitions, AutoCameraShot, CameraDamageRuntime, CameraFocusTarget, TemporaryCameraFocus, TownCameraControllerRuntime, auto_camera_citizen_translation(), auto_camera_focus_translation() (+14 more)

### Community 71 - "drive_tidal_music"
Cohesion: 0.26
Nodes (15): AdaptiveMusicSignature, drive_tidal_music(), intensity_program_needs_update(), player_music_gain(), report_once(), NativeAudioRouting, NativeAudioStatus, Option (+7 more)

### Community 72 - "environment.rs"
Cohesion: 0.17
Nodes (14): append_nearest_night_lights(), color_from_rgb8(), NightLightSpec, perceptually_normalized_light_color(), player_night_light_level_multiplier(), Color, Vec, Vec3 (+6 more)

### Community 73 - "capture_city_timelapse"
Cohesion: 0.12
Nodes (20): TimelapseInterval, BuildingPlacementGhostMesh, BuildingPlacementOwnerOverlay, BuildingPlacementVisual, capture_city_timelapse(), CityTimelapseRuntime, Commands, Default (+12 more)

### Community 74 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (63): apply_building_draft(), AssetEditorSection, broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, canonical_preview_node_name(), debug_fingerprint() (+55 more)

### Community 75 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 76 - ".write"
Cohesion: 0.29
Nodes (7): PlayerSettingsStore, PlayerSettingsStoreError, Into, Path, PathBuf, Result, SpannedError

### Community 77 - ".join_player"
Cohesion: 0.21
Nodes (7): actor_appearance_hash(), awakening_applies_to_role_experience_at_the_domain_boundary(), deterministic_weather(), legacy_simulation_calendar_upgrades_without_advancing_timers(), Self, schema_five_randomizes_legacy_appearances_but_preserves_player_colors(), zero_skin_color_preserves_pre_schema_five_canonical_ron()

### Community 78 - "update_enemy_music_intensity"
Cohesion: 0.18
Nodes (13): intensity_smoothing_has_a_five_second_time_constant(), point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, ResMut, Time (+5 more)

### Community 79 - ".walkable_neighbours_with"
Cohesion: 0.33
Nodes (4): neighbour_candidates(), offset(), Fn, Option

### Community 80 - "PostProcessProfileDef"
Cohesion: 0.20
Nodes (12): PostProcessBloomDef, PostProcessColorAdjustmentsDef, PostProcessMotionBlurDef, PostProcessProfileDef, PostProcessTonemapping, PostProcessVignetteDef, authored_color_grading(), authored_post_process_stack() (+4 more)

### Community 81 - "healing_burst_effect"
Cohesion: 0.44
Nodes (9): HealingEffectKind, HealingEffectSample, HealingRingEffect, healing_burst_effect(), healing_channel_effect(), healing_effect_duration(), healing_effect_sample(), healing_effect_curves_preserve_authored_lifetimes_and_channel_keys() (+1 more)

### Community 82 - "navigation_surface_height_at_world"
Cohesion: 0.29
Nodes (10): authored_foliage_is_deterministic_and_respects_habitat_and_resources(), averaged_terrain_corner_height(), foliage_third_cell(), navigation_corner_height_metres(), navigation_surface_height_at_world(), Fn, Option, terrain_surface_height_from_centimetres() (+2 more)

### Community 83 - "VideoSettings"
Cohesion: 0.50
Nodes (3): PostProcessAntiAliasing, Option, VideoSettings

### Community 84 - "save_secrets_fields"
Cohesion: 0.32
Nodes (8): SecretsField, open_twitch_verification_uri(), open_twitch_verification_uri_with(), AnyResult, FnOnce, save_secrets_fields(), twitch_verification_opens_only_the_https_url_returned_by_twitch(), EditableText

### Community 85 - "BroadcastOutput"
Cohesion: 0.29
Nodes (5): BroadcastOutput, Output, Deref, DerefMut, Target

### Community 86 - "sync_accessibility_preferences"
Cohesion: 0.24
Nodes (10): reduced_grass_wind(), reduced_tree_wind(), reduced_water_wind(), GrassMaterial, Local, TreeMaterial, Vec4, WaterMaterial (+2 more)

### Community 87 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (98): graphify, visual regression ledgers, AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityHighContrastText, actor_health_fill_color(), actor_name_color(), ActorHealthOverlay (+90 more)

### Community 88 - "node_detail_lines"
Cohesion: 0.33
Nodes (6): node_detail_lines(), Item, Iterator, String, Vec, summarized_pairs()

### Community 89 - "stream_town_render_error_handler"
Cohesion: 0.50
Nodes (5): is_transient_surface_configuration_error(), World, stream_town_render_error_handler(), RenderError, RenderErrorPolicy

### Community 90 - "audio_acceptance_wavs"
Cohesion: 0.32
Nodes (8): audio_acceptance_manifest(), audio_acceptance_record(), audio_acceptance_wavs(), procedural_audio_matches_curated_acceptance_baseline(), BTreeMap, String, Value, Vec

### Community 91 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Ord, Ordering, PartialOrd, Self

### Community 92 - "WorldDiagnosticRuntime"
Cohesion: 0.50
Nodes (3): WorldDiagnosticMode, WorldDiagnosticOverlay, WorldDiagnosticRuntime

### Community 93 - "settings.rs"
Cohesion: 0.14
Nodes (19): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, default_ui_scale_percent(), defaults_are_valid_and_round_trip(), InterfaceSettings, NameDisplayMode, PlayerSettingsValidationError (+11 more)

### Community 94 - ".visit"
Cohesion: 0.60
Nodes (3): ContentError, Result, valid_asset_path()

### Community 95 - "EnemyPathOpenNode"
Cohesion: 0.47
Nodes (4): EnemyPathOpenNode, Ord, Ordering, PartialOrd

### Community 96 - "handle_twitch_event"
Cohesion: 0.40
Nodes (5): handle_twitch_event(), broadcaster_moderation_state_changes_do_not_overwrite_bot_connection_state(), connected_bot_dispatches_twitch_commands_without_a_chat_gate(), fish_god_channel_reward_dispatches_praise_without_command_text(), unconfigured_or_different_channel_reward_does_not_dispatch_praise()

### Community 98 - "TimeCycleConfig"
Cohesion: 0.22
Nodes (5): Default, Self, TimeCycleConfig, night_light_is_active(), night_light_sources_are_inactive()

### Community 99 - "PresentationCatalog"
Cohesion: 0.12
Nodes (30): AnimationEventDef, AnimationMotionDef, AnimationObjectReference, AnimationStateDef, AnimationStateMachineDef, AvatarMaskDef, ChimneySmokeDef, cubic_bezier() (+22 more)

### Community 100 - "CityTimelapsePlugin"
Cohesion: 0.50
Nodes (3): CityTimelapsePlugin, App, Plugin

### Community 101 - "transformed_mesh_vertical_extent"
Cohesion: 0.40
Nodes (6): Quat, transformed_bounds_minimum_y(), transformed_mesh_vertical_extent(), surface_grounding_places_rotated_bounds_on_the_surface(), surface_grounding_uses_real_vertices_instead_of_empty_aabb_corners(), surface_visual_remains_hidden_until_grounding_is_complete()

### Community 102 - ".new"
Cohesion: 0.21
Nodes (10): algorithmic_generation_matches_unity_validation_fingerprints(), final_surface_filtered_generation_remains_deterministic(), final_surface_height_overrides_a_raw_land_cell_at_the_waterline(), observed_generation_reports_every_real_stage_without_changing_output(), positive_noise_offset(), Self, SystemRandom, town_layer_noise_offset() (+2 more)

### Community 103 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 104 - "technology_graph.rs"
Cohesion: 0.15
Nodes (32): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+24 more)

### Community 115 - "xtask/src/main.rs"
Cohesion: 0.05
Nodes (100): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+92 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.15
Nodes (28): AdaptiveMusicConfig, adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine() (+20 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 157 - "Stream Town architecture"
Cohesion: 0.33
Nodes (5): Data flow, Design rules, Stream Town architecture, Verification, Workspace boundaries

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Current attempts, Do not retry unchanged, Next narrow diagnostic pass, Tree and Foliage Flicker Regression Checklist, What did not fix the flicker, What did work

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 210 - "encode_broadcast_session"
Cohesion: 0.07
Nodes (41): AtomicUsize, BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, BroadcastMetrics, configured_1080p30_x264_encoder_sustains_realtime_output(), configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame() (+33 more)

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 229 - "bevy-port/README.md"
Cohesion: 0.19
Nodes (7): Audio provenance, Binaries, Commands, Stream Town Bevy, Development, License and media, Stream Town

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 238 - "Twitch setup"
Cohesion: 0.20
Nodes (10): 1. Secure the old credentials, 2. Register the Twitch application, 3. Configure and authorize `HumanBeanBot`, 4. Prepare the channel, 5. Bind the Fish God Channel Points reward, 6. Authorize direct broadcasting, 7. Choose broadcast quality and test bandwidth, 8. Go live without OBS (+2 more)

### Community 240 - "Accessibility"
Cohesion: 0.29
Nodes (6): Accessibility, Automated verification, Keyboard operation, Persisted preferences, Screen-reader contract, Windows Narrator acceptance

### Community 251 - "allocate_fifo_output"
Cohesion: 0.50
Nodes (3): allocate_fifo_output(), Output, Result

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_ffmpeg_bridge, stream_town_game, stream_town_tools, xtask

### Community 283 - "Stream Town authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town authoring suite, Workflows

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.12
Nodes (17): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, City timelapse output, Getting started, Information, Moderator and game-master commands (+9 more)

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (74): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot() (+66 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 336 - "RuntimeConfig"
Cohesion: 0.07
Nodes (67): AgentCommandQueue, BuildingCommandQueue, CameraCommandQueue, FoliageVisual, InjectedCommands, VecDeque, RuntimeCaptureRequest, RuntimeCommandQueues (+59 more)

### Community 495 - "agents.rs"
Cohesion: 0.07
Nodes (73): AgentLocomotion, AuthoredRotatingNodeProcessed, BuildingModelNodeProcessed, ChimneySmokeEmitters, ChimneySmokeParticle, CrowdSeparationRuntime, RuntimeBuilding, animate_chimney_smoke_particles() (+65 more)

### Community 938 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 1189 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (51): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), broadcast_output_options(), build_ingest_url(), configure_amf_quality() (+43 more)

### Community 2375 - "Res"
Cohesion: 0.11
Nodes (47): cleanup_loading_screen(), CreditsSkipButton, GoLiveConfirmationAction, LoadingScreenEntity, LoadingUiCamera, SettingsAction, SettingsTabButton, SettingsValueButton (+39 more)

## Knowledge Gaps
- **141 isolated node(s):** `stream_town_ffmpeg_bridge`, `StreamOperatorCamera`, `StreamOperatorLiveButton`, `StreamOperatorRestartButton`, `StreamOperatorChatInputText` (+136 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StableId` connect `StableId` to `ContentCatalog`, `embedded_content`, `Ui`, `.default`, `sync_building_health_overlays`, `spawn_main_menu`, `WorldSimulation`, `SimulationError`, `AnimationControllerDef`, `content.rs`, `update_credits_fireworks`, `GridPos`, `src/world.rs`, `simulation.rs`, `RenderAssets`, `GameConfig`, `Option`, `drive_converted_animations`, `Vec3`, `String`, `Query`, `capture_foliage_acceptance`, `roles_tab`, `update_vote_panels`, `Option`, `runtime_console.rs`, `ToolState`, `rendering.rs`, `ResolvedMaterialHandle`, `config.rs`, `RuntimeContent`, `instantiate_building_materials`, `TechnologyGraphLayout`, `.tick`, `ArchetypeDef`, `command.rs`, `sync_pooled_night_lights`, `presentation/animation.rs`, `Agent`, `presentation.rs`, `twitch.rs`, `ObjectiveDef`, `camera_zoom_and_commands`, `capture_city_timelapse`, `stream_town_tools/src/main.rs`, `stream_town_domain/src/lib.rs`, `.join_player`, `RuntimeConfig`, `healing_burst_effect`, `stream_town_game/src/lib.rs`, `.visit`, `complete_gameplay_scenario_round_trips`, `PresentationCatalog`, `.new`, `technology_graph.rs`, `agents.rs`, `xtask/src/main.rs`?**
  _High betweenness centrality (0.218) - this node is a cross-community bridge._
- **Why does `RuntimeConfig` connect `RuntimeConfig` to `StableId`, `ContentCatalog`, `tests.rs`, `MenuRuntime`, `sync_building_health_overlays`, `sync_stream_only_capture`, `update_environment_presentation`, `setup_rendering`, `update_secrets_ui`, `stream_operator_chat_controls`, `RenderAssets`, `GameConfig`, `Query`, `capture_foliage_acceptance`, `update_vote_panels`, `.new`, `Option`, `update_hud`, `rendering.rs`, `DirectBroadcastRuntime`, `RuntimeContent`, `instantiate_building_materials`, `advance_world_loading_cover`, `enter_headless_world`, `sync_pooled_night_lights`, `camera_zoom_and_commands`, `Res`, `drive_tidal_music`, `update_enemy_music_intensity`, `save_secrets_fields`, `stream_town_game/src/lib.rs`, `agents.rs`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._
- **Why does `RenderAssets` connect `RenderAssets` to `StableId`, `ContentCatalog`, `sync_building_health_overlays`, `spawn_main_menu`, `Handle`, `update_environment_presentation`, `update_secrets_ui`, `update_credits_fireworks`, `GameConfig`, `Option`, `Vec3`, `menu.rs`, `String`, `Query`, `update_vote_panels`, `Option`, `GameState`, `rendering.rs`, `ResolvedMaterialHandle`, `RuntimeContent`, `advance_world_loading_cover`, `ArchetypeDef`, `Mesh`, `Agent`, `Res`, `environment.rs`, `RuntimeConfig`, `sync_accessibility_preferences`, `stream_town_game/src/lib.rs`, `agents.rs`?**
  _High betweenness centrality (0.060) - this node is a cross-community bridge._
- **Are the 146 inferred relationships involving `embedded_content()` (e.g. with `.build()` and `run()`) actually correct?**
  _`embedded_content()` has 146 INFERRED edges - model-reasoned connections that need verification._
- **What connects `stream_town_ffmpeg_bridge`, `StreamOperatorCamera`, `StreamOperatorLiveButton` to the rest of the system?**
  _141 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `StableId` be split into smaller, more focused modules?**
  _Cohesion score 0.03762010347376201 - nodes in this community are weakly interconnected._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.06538782318598832 - nodes in this community are weakly interconnected._