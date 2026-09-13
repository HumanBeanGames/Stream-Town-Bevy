# Graph Report - Stream-Town-Bevy  (2026-09-14)

## Corpus Check
- 165 files · ~343,821 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 5194 nodes · 16440 edges · 174 communities (168 shown, 6 thin omitted)
- Extraction: 87% EXTRACTED · 13% INFERRED · 0% AMBIGUOUS · INFERRED: 2140 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `90305896`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- process_injected_commands
- ContentCatalog
- loading_state.rs
- embedded_content
- draw_world_preview
- ui/tests.rs
- loading.rs
- apply_player_settings
- spawn_main_menu
- CommunityEvent
- ShaderRef
- sync_stream_only_capture
- update_environment_presentation
- StableId
- setup_rendering
- stream_town_tools/src/main.rs
- RenderAssets
- record_gpu_readiness
- AnimationControllerDef
- content.rs
- update_credits_fireworks
- capture.rs
- move_agents
- capture_foliage_acceptance
- simulation.rs
- ground_loaded_surface_visuals
- GameConfig
- world_loading.rs
- ui/persistence.rs
- world_runtime.rs
- world_scene.rs
- menu_input
- Agent
- report_live_skin_deformation
- music_preview.rs
- select_grid_cell
- GpuStreamCaptureRing
- sync_authored_post_processing
- DirectBroadcastRuntime
- asset_editors.rs
- Transform
- accessibility_input
- update_hud
- src/navigation.rs
- rendering.rs
- bootstrap.rs
- LoadingWorkNode
- config.rs
- DirectBroadcastControl
- foundation.rs
- drive_converted_animations
- rig_and_equipment.rs
- profiling.rs
- cosmetics_and_curves.rs
- technology.rs
- input.rs
- menu_accessibility.rs
- RuntimeConfig
- ArchetypeDef
- timelapse.rs
- command.rs
- PreparedWorld
- sync_pooled_night_lights
- src/world.rs
- world_foundations.rs
- capture_screenshot
- PresentationCatalog
- twitch.rs
- ObjectiveDef
- embedded_presentation
- drive_tidal_music
- BroadcastController
- capture_city_timelapse
- combat.rs
- tools_ui
- .write
- update_enemy_encounters
- update_enemy_music_intensity
- GridPos
- loading_runtime.rs
- load_input
- GpuReadinessProbe
- sync_fish_god_presentation
- runtime_state.rs
- .start
- agents.rs
- bootstrap_runtime.rs
- GameState
- SimulationRuntime
- animation_audio.rs
- encoder.rs
- ResolvedMaterialHandle
- PlayerSettings
- TechTree
- spawn_resource_visual
- twitch_editor.rs
- generate_and_spawn_world
- Self
- BroadcastMetrics
- capture_stream_only_target
- field_editors.rs
- instantiate_building_materials
- tidal_plugin
- technology_graph.rs
- authorization.rs
- sync_building_placers
- update_stream_operator_chat
- sync_building_placement_overlays
- sync_boot_loading_screen
- spawn_runtime_building
- .default
- sync_active_pets
- run
- update_vote_panels
- xtask/src/main.rs
- targeting.rs
- tidal_music.rs
- InjectedCommands
- Option
- OperatorChatRuntime
- jump_start_snapshot_path
- objective_catalog_editor
- TwitchConfig
- duration_as_micros
- StreamOnlyCaptureInbox
- BootstrapFlowPlugin
- CreditsFlowPlugin
- GameplayFlowPlugin
- MainMenuFlowPlugin
- PresentationFlowPlugin
- SharedRuntimePlugin
- WorldLoadingFlowPlugin
- Stream Town Bevy
- What You Must Do When Invoked
- xtask/src/lib.rs
- Stream Town architecture
- Tree and Foliage Flicker Regression Checklist
- graphify reference: extra exports and benchmark
- VideoFrame
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
- GeneratedWorld
- emit_chimney_smoke
- runtime_console.rs
- direct_broadcast.rs
- MenuRuntime

## God Nodes (most connected - your core abstractions)
1. `StableId` - 477 edges
2. `WorldSimulation` - 269 edges
3. `ContentCatalog` - 252 edges
4. `GridPos` - 226 edges
5. `embedded_content()` - 148 edges
6. `GeneratedWorld` - 145 edges
7. `RenderAssets` - 144 edges
8. `ToolState` - 138 edges
9. `GameConfig` - 127 edges
10. `RuntimeConfig` - 114 edges

## Surprising Connections (you probably didn't know these)
- `baked_menu_tree_variants_override_legacy_checkerboard_indices()` --calls--> `resource_visual_variant()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests/loading_visuals.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `paths_use_their_authored_health_and_catalog_level_bonus()` --calls--> `embedded_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests/animation_twitch.rs → bevy-port/crates/stream_town_game/src/core/bootstrap_runtime.rs
- `player_health_bar_uses_authored_damage_and_hide_contract()` --calls--> `embedded_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests/animation_twitch.rs → bevy-port/crates/stream_town_game/src/core/bootstrap_runtime.rs
- `community_events_apply_their_authored_role_and_invasion_rates()` --calls--> `embedded_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests/loading_runtime.rs → bevy-port/crates/stream_town_game/src/core/bootstrap_runtime.rs
- `fine_placement_border_allows_a_walkable_gap_but_rejects_physical_encroachment()` --calls--> `embedded_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests/loading_runtime.rs → bevy-port/crates/stream_town_game/src/core/bootstrap_runtime.rs

## Import Cycles
- None detected.

## Communities (174 total, 6 thin omitted)

### Community 0 - "process_injected_commands"
Cohesion: 0.07
Nodes (76): StreamUserType, CommandOrigin, PendingChatCommand, is_stream_player_actor(), preset_player_name_color(), simulation_player_count(), process_injected_commands(), AssetServer (+68 more)

### Community 1 - "ContentCatalog"
Cohesion: 0.06
Nodes (112): ContentCatalog, ActorState, BuildingState, RoleProgress, Default, String, StationTargetRuntime, actor_health_bar_hide_seconds() (+104 more)

### Community 2 - "loading_state.rs"
Cohesion: 0.04
Nodes (96): LoadWorldEntities, Transform, Without, ActorAnimationDriver, AnimatedCharacterShadowReceiver, AuthoredRotatingNode, AuthoredRotatingNodeProcessed, BuildingModelNode (+88 more)

### Community 3 - "embedded_content"
Cohesion: 0.03
Nodes (115): generate_world(), generate_world_with_content(), embedded_content(), path_route_is_complete(), complete_agent_goal(), Option, ensure_actor_station(), ensure_town_hall_state() (+107 more)

### Community 4 - "draw_world_preview"
Cohesion: 0.20
Nodes (10): WorldPreviewLayer, draw_world_preview(), preview_grid_point(), preview_lerp_color(), Color32, Option, Pos2, Rect (+2 more)

### Community 5 - "ui/tests.rs"
Cohesion: 0.07
Nodes (41): BuildingDraft, main(), Result, TechnologyDraft, building_draft(), default_catalog_path(), default_config_path(), default_presentation_path() (+33 more)

### Community 6 - "loading.rs"
Cohesion: 0.06
Nodes (63): BootDestination, MenuLoadingRuntime, Instant, UntypedHandle, WorldLoadingCoverRuntime, GoLiveConfirmationAction, advance_world_loading_cover(), apply_authored_main_menu_camera() (+55 more)

### Community 7 - "apply_player_settings"
Cohesion: 0.11
Nodes (27): AudioSink, apply_authored_ui_fonts(), apply_player_settings(), cleanup_state_entities(), drive_world_audio(), player_msaa(), Added, Entity (+19 more)

### Community 8 - "spawn_main_menu"
Cohesion: 0.08
Nodes (36): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+28 more)

### Community 9 - "CommunityEvent"
Cohesion: 0.15
Nodes (13): CommunityEvent, CommunityVoteOutcome, CommunityVoteProposal, CommunityVoteState, Option, TownEvent, community_vote_event(), community_event_description() (+5 more)

### Community 10 - "ShaderRef"
Cohesion: 0.07
Nodes (30): AccessibilityMotionDefaults, BuildingMaterialExtension, BuildingMaterialInstance, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension, CloudMaterialUniform (+22 more)

### Community 11 - "sync_stream_only_capture"
Cohesion: 0.09
Nodes (38): GameplayReady, select_ingest(), camera_targets_primary_window(), RenderTarget, apply_direct_broadcast_control(), begin_twitch_live_verification(), configure_direct_broadcast(), operator_window_close_requests_exit() (+30 more)

### Community 12 - "update_environment_presentation"
Cohesion: 0.09
Nodes (37): AmbientLight, Season, Weather, EnvironmentPalette, WeatherParticle, EnvironmentPresentation, blend_environment_palette(), building_snow_strength() (+29 more)

### Community 13 - "StableId"
Cohesion: 0.09
Nodes (24): Display, FromStr, StableId, complete_gameplay_scenario_round_trips(), EnemyCampState, FishGodState, RaidState, BTreeMap (+16 more)

### Community 14 - "setup_rendering"
Cohesion: 0.09
Nodes (51): BuildingMaterial, CritterMaterial, FlagMaterial, GrassMaterial, TerrainMaterial, TreeMaterial, WaterMaterial, setup_rendering() (+43 more)

### Community 15 - "stream_town_tools/src/main.rs"
Cohesion: 0.05
Nodes (74): AssetEditorSection, ModelPreviewCamera, ModelPreviewControls, ModelPreviewRuntime, ModelPreviewScene, MusicPreviewState, PreviewMaterialApplied, PreviewMaterialOverrides (+66 more)

### Community 16 - "RenderAssets"
Cohesion: 0.11
Nodes (51): HudMetric, HudMetricMaximum, RenderAssets, WorldAsset, SecretsAction, SecretsDynamicLabel, ChildSpawnerCommands, Commands (+43 more)

### Community 17 - "record_gpu_readiness"
Cohesion: 0.20
Nodes (10): record_gpu_readiness(), record_presented_render_frame(), GpuImage, GpuRenderAssets, Res, ErasedRenderAssets, PipelineCache, PreparedMaterial (+2 more)

### Community 18 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (34): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+26 more)

### Community 19 - "content.rs"
Cohesion: 0.10
Nodes (40): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, default_resource_generation_layers(), EnemyDef, EnemyModelSetDef (+32 more)

### Community 20 - "update_credits_fireworks"
Cohesion: 0.09
Nodes (45): AuthoredCreditsElement, CreditsFade, CreditsFireworkParticle, CreditsFireworkParticleKind, CreditsTimeline, LevelUpToast, Default, animation_property_value() (+37 more)

### Community 21 - "capture.rs"
Cohesion: 0.08
Nodes (44): SensitiveScreenActive, capture_direct_broadcast_frame(), format_minutes_seconds(), moderate_selected_operator_user(), operator_chat_scroll_rows(), operator_info_refresh_due(), operator_live_button_label(), receive_stream_only_captured_frames() (+36 more)

### Community 22 - "move_agents"
Cohesion: 0.07
Nodes (68): NavGrid, ActorKind, PathSurfaceRuntime, AgentGoal, BuildingPlacement, building_placement_visual_cell_is_available(), candidate_building_fine_cells(), confirmed_path_cells() (+60 more)

### Community 23 - "capture_foliage_acceptance"
Cohesion: 0.06
Nodes (56): ActivePetVisual, AutoCameraShot, CameraDamageRuntime, CameraFocusTarget, FoliageAcceptanceCapture, FoliageAcceptanceScreenshot, PathBuf, Transform (+48 more)

### Community 24 - "simulation.rs"
Cohesion: 0.07
Nodes (42): actor_appearance_hash(), actor_appearances_are_seeded_varied_and_persisted(), authored_trade_rates_clamp_to_stock_gold_and_capacity(), awakening_applies_to_role_experience_at_the_domain_boundary(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase(), capped_deposit_preserves_inventory_overflow(), capped_role_progression_discards_excess_at_the_requested_level() (+34 more)

### Community 25 - "ground_loaded_surface_visuals"
Cohesion: 0.15
Nodes (19): FoliageHabitat, PendingSurfaceGrounding, SurfaceFoliageHabitat, actor_material(), foliage_mesh_fits_habitat(), ground_loaded_surface_visuals(), ResolvedFoliageVisual, Aabb (+11 more)

### Community 26 - "GameConfig"
Cohesion: 0.25
Nodes (19): GameConfig, building_health_overlay_world_position(), grid_to_world(), grid_to_world_on_surface(), Transform, Vec3, shoreline_camera_transform(), shoreline_focus() (+11 more)

### Community 27 - "world_loading.rs"
Cohesion: 0.08
Nodes (48): MenuRevealRuntime, PresentedRenderFrames, AtomicU64, Self, WorldRevealRuntime, LoadingScreenEntity, LoadingUiCamera, UntypedHandle (+40 more)

### Community 28 - "ui/persistence.rs"
Cohesion: 0.11
Nodes (53): buildings_tab(), Option, TextureId, Ui, content_tab(), content_tab_contents(), gltf_material_bindings_editor(), Option (+45 more)

### Community 29 - "world_runtime.rs"
Cohesion: 0.07
Nodes (66): ActorHealthFill, ActorHealthOverlay, ActorNameOverlay, BuildingHealthFill, BuildingHealthOverlay, ResourceNode, RulerVoteAnnouncementRuntime, TemporaryWorldLabel (+58 more)

### Community 30 - "world_scene.rs"
Cohesion: 0.09
Nodes (54): FishSchoolParticle, MainMenuBuildingShadowRoot, MainMenuBuildingShadowVerified, MainMenuCloudPrism, MainMenuModelNodeProcessed, MainMenuRotatingDefinitions, MainMenuRotatingNode, MainMenuRotatingNodeProcessed (+46 more)

### Community 31 - "menu_input"
Cohesion: 0.07
Nodes (53): SettingsTab, SettingsConfirmModal, SettingsFeedbackText, SettingsRoot, SettingsRows, SettingsTabButton, SettingsUiCache, SettingsValueRow (+45 more)

### Community 32 - "Agent"
Cohesion: 0.10
Nodes (52): AnimatedBy, AnimationGraphHandle, AnimationTransitions, Agent, ConvertedAnimationApplied, ConvertedAnimationInstanceReady, ConvertedAnimationSpec, animate_agents() (+44 more)

### Community 33 - "report_live_skin_deformation"
Cohesion: 0.10
Nodes (41): animation_binding_diagnostics_enabled(), apply_material_overrides(), is_descendant_of(), named_character_slot_ancestor(), report_animation_binding_diagnostics(), report_animation_loop_diagnostics(), report_live_skin_deformation(), report_post_animation_stage() (+33 more)

### Community 34 - "music_preview.rs"
Cohesion: 0.18
Nodes (24): GltfMetadata, Vec, cached_gltf_metadata(), debug_fingerprint(), default_role_preview_animation(), discover_model_assets(), discover_texture_assets(), import_model_asset() (+16 more)

### Community 35 - "select_grid_cell"
Cohesion: 0.13
Nodes (19): SelectedActor, centre_screen_raycast_world_position(), pointer_is_over_button(), Assets, Button, ButtonInput, Camera, GlobalTransform (+11 more)

### Community 36 - "GpuStreamCaptureRing"
Cohesion: 0.19
Nodes (12): AtomicUsize, BroadcastStopDisposition, GpuStreamCaptureCounters, GpuStreamCaptureRing, GpuStreamCaptureShared, AtomicU64, Instant, Self (+4 more)

### Community 37 - "sync_authored_post_processing"
Cohesion: 0.22
Nodes (10): authored_color_grading(), color_grading_for_state(), motion_blur_supported(), State, sync_authored_post_processing(), PostProcessPresentation, daylight_signature(), scene_exposure_uses_approved_neutral_baselines_and_prism_clouds() (+2 more)

### Community 38 - "DirectBroadcastRuntime"
Cohesion: 0.10
Nodes (25): BroadcastVideoSink, DirectBroadcastPhase, DirectBroadcastProfile, DirectBroadcastRuntime, DirectBroadcastSnapshot, GpuStreamCaptureSlot, NativeGameAudioClip, NativeGameAudioMix (+17 more)

### Community 39 - "asset_editors.rs"
Cohesion: 0.15
Nodes (43): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+35 more)

### Community 40 - "Transform"
Cohesion: 0.18
Nodes (15): default_town_camera_transform(), in_game_sun_transform(), in_game_sun_transform_for_daylight(), pet_follow_step(), ping_pointer_scale(), ping_pointer_transform(), ray_sphere_distance(), Projection (+7 more)

### Community 41 - "accessibility_input"
Cohesion: 0.06
Nodes (73): AccessibilityFocusVisualQuery, AccessibleNode, CreditsSkipButton, AccessibilityRuntime, AccessibleButtonScope, GameMenuAction, GameMenuActionLabel, MainMenuAction (+65 more)

### Community 42 - "update_hud"
Cohesion: 0.11
Nodes (25): Hud, HudTechnologyProgressFill, SeasonMeter, active_event_text(), fit_hud_label(), format_rgb(), hud_play_time(), hud_season_meter_percent() (+17 more)

### Community 43 - "src/navigation.rs"
Cohesion: 0.14
Nodes (23): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavigationError, octile_distance_with_costs(), open_ground_paths_use_diagonal_steps() (+15 more)

### Community 44 - "rendering.rs"
Cohesion: 0.14
Nodes (30): CitizenDeathAnnouncementRuntime, announce_citizen_deaths(), announce_ruler_vote_result(), citizen_death_announcement(), community_vote_option_lines(), compact_technology_label(), content_label(), current_event_panel_state() (+22 more)

### Community 45 - "bootstrap.rs"
Cohesion: 0.16
Nodes (18): adaptive_music_preview_program(), configure_automatic_live(), is_transient_surface_configuration_error(), load_player_settings(), load_runtime_config(), player_settings_path(), AnyResult, PathBuf (+10 more)

### Community 46 - "LoadingWorkNode"
Cohesion: 0.16
Nodes (11): BuildingTimeCycleSignature, LoadingWork, LoadingWorkNode, round_robin_indices(), IntoIterator, Item, Iterator, Self (+3 more)

### Community 47 - "config.rs"
Cohesion: 0.15
Nodes (20): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig (+12 more)

### Community 48 - "DirectBroadcastControl"
Cohesion: 0.18
Nodes (7): AutomaticBroadcastStart, DirectBroadcastControl, exit_after_broadcast_stops(), request_automatic_broadcast_start(), AppExit, MessageWriter, Default

### Community 49 - "foundation.rs"
Cohesion: 0.06
Nodes (52): App, Plugin, StreamTownGamePlugin, AgentSimulationSet, automatic_resume_save_path(), automatic_resume_world_seed(), AutomaticResumeRuntime, CachedStationTargets (+44 more)

### Community 50 - "drive_converted_animations"
Cohesion: 0.10
Nodes (41): pseudo_noise(), ConvertedAnimationCrossfade, ConvertedAnimationDriver, ConvertedAnimationLayerDriver, ConvertedAnimationPlayback, Vec, advance_animation_crossfade(), animation_event_occurrences() (+33 more)

### Community 51 - "rig_and_equipment.rs"
Cohesion: 0.06
Nodes (66): ActorCustomization, AgentEquipmentPresentation, CosmeticNode, CosmeticNodeKind, CosmeticNodeProcessed, EnemyModelNodeProcessed, EquipmentNodeProcessed, PlayerRigAxisCorrected (+58 more)

### Community 52 - "profiling.rs"
Cohesion: 0.08
Nodes (63): append_profile_sample(), ArchetypeProfile, asset_count(), asset_counts(), begin_profiled_frame(), build_profile_sample(), collect_profile_sample(), diagnostic_profiles() (+55 more)

### Community 53 - "cosmetics_and_curves.rs"
Cohesion: 0.10
Nodes (52): AnimationTargetId, PlayerAnimatedRig, add_rotation_curve(), add_scale_curve(), add_translation_curve(), animated_player_renderer(), animation_target_for_track(), close_rotation_loop() (+44 more)

### Community 54 - "technology.rs"
Cohesion: 0.10
Nodes (40): TechVote, technology_vote_leader(), technology_vote_option_tally(), technology_vote_options(), technology_vote_requirements(), announce_technology_vote(), authored_technology_age(), building_construction_cost() (+32 more)

### Community 55 - "input.rs"
Cohesion: 0.08
Nodes (39): MenuPage, MenuOverlay, GameMenuRoot, TownDialogRoot, actor_detail_budget(), actor_scene_budget(), adjust_settings_menu(), animation_detail_budget() (+31 more)

### Community 56 - "menu_accessibility.rs"
Cohesion: 0.08
Nodes (14): AccessibilityActionRequest, SecretsCredentialState, SecretsStatusTone, bot_connection_status(), broadcast_connection_status(), String, accessibility_navigation_preserves_editable_text_focus(), broadcaster_control_failure_is_never_reported_as_a_bot_error() (+6 more)

### Community 57 - "RuntimeConfig"
Cohesion: 0.03
Nodes (42): RuntimeConfig, RuntimePlayerSettings, CameraCommandQueue, bandwidth_test_never_claims_to_be_publicly_live(), closing_the_operator_window_requests_a_graceful_game_exit(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once() (+34 more)

### Community 58 - "ArchetypeDef"
Cohesion: 0.27
Nodes (20): ArchetypeDef, ArchetypeKind, ArchetypeScene, pet_model(), archetype_by_source(), archetype_needs_self_shadow_suppression(), archetype_scene_for_age(), building_prefab_material_spec() (+12 more)

### Community 59 - "timelapse.rs"
Cohesion: 0.14
Nodes (18): CityTimelapsePlugin, CityTimelapseScreenshot, draw_timelapse_label(), next_timelapse_frame_index(), rebuild_timelapse_video(), App, On, Path (+10 more)

### Community 60 - "command.rs"
Cohesion: 0.16
Nodes (33): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, command_usage(), CommandParseError, content_id() (+25 more)

### Community 61 - "PreparedWorld"
Cohesion: 0.40
Nodes (4): PreparedWorld, AssetId, Mesh, Collider

### Community 62 - "sync_pooled_night_lights"
Cohesion: 0.10
Nodes (28): Default, TimeCycleConfig, NightPointLightPoolSlot, animate_weather_particles(), append_nearest_night_lights(), building_material_time_cycle(), building_night_light_profile(), color_from_rgb8() (+20 more)

### Community 63 - "src/world.rs"
Cohesion: 0.07
Nodes (73): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), averaged_terrain_corner_height(), cell_hash() (+65 more)

### Community 65 - "world_foundations.rs"
Cohesion: 0.07
Nodes (25): SeasonalTerrainPalette, TerrainAppearanceConfig, HashMap, traversal_score_for_rate(), traversal_wear_fraction(), TraversalWearCell, TraversalWearRuntime, record_completed_cell_traversal() (+17 more)

### Community 66 - "capture_screenshot"
Cohesion: 0.07
Nodes (38): WorldRenderStats, Default, RuntimeCaptureRequest, RuntimeConsoleRuntime, capture_screenshot(), report_frame_time_gate(), AppExit, ButtonInput (+30 more)

### Community 67 - "PresentationCatalog"
Cohesion: 0.07
Nodes (66): AnimationClipDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve (+58 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (64): bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_oauth_identity(), envelope_from_privmsg() (+56 more)

### Community 69 - "ObjectiveDef"
Cohesion: 0.19
Nodes (10): ObjectiveDef, common_gathered_resource(), is_gold_collect_objective(), objective_increment(), ObjectiveEvent, ObjectiveProgress, Vec, RulerVoteState (+2 more)

### Community 70 - "embedded_presentation"
Cohesion: 0.07
Nodes (27): embedded_presentation(), authored_building_and_role_balance_is_explicit_and_complete(), authored_research_ladders_grow_by_one_and_a_half(), baked_menu_tree_variants_override_legacy_checkerboard_indices(), building_bounds_material_preserves_unity_placement_contract(), character_material_preserves_authored_albedo_and_cosmetic_contract(), completed_gates_open_player_routes_but_remain_blocked_for_enemies(), construction_costs_scale_by_existing_copies_except_fixed_buildings() (+19 more)

### Community 71 - "drive_tidal_music"
Cohesion: 0.26
Nodes (15): AdaptiveMusicSignature, drive_tidal_music(), intensity_program_needs_update(), player_music_gain(), report_once(), NativeAudioRouting, NativeAudioStatus, Option (+7 more)

### Community 72 - "BroadcastController"
Cohesion: 0.10
Nodes (25): AudioFrame, AudioInput, BroadcastController, CadenceTick, capture_process_audio(), queue_audio_frame(), reconnect_wait_seconds(), redact_broadcast_target() (+17 more)

### Community 73 - "capture_city_timelapse"
Cohesion: 0.10
Nodes (25): Option, TimelapseInterval, BuildingPlacementGhostMesh, BuildingPlacementVisual, apply_confirmed_builds(), capture_city_timelapse(), CityTimelapseBuildConfirmed, CityTimelapseRuntime (+17 more)

### Community 74 - "combat.rs"
Cohesion: 0.15
Nodes (34): ActionPresentation, BuildingDamageEmitter, BuildingEffectKind, BuildingEffectParticle, CombatVisualKind, ProjectileSpawn, animate_building_effects(), animate_combat_effects() (+26 more)

### Community 75 - "tools_ui"
Cohesion: 0.08
Nodes (34): runtime_actions_sequence_after_latest_acknowledgement(), music_tab(), NativeAudioRouting, NativeAudioStatus, TidalBackendStatus, TidalController, Ui, poll_tool_job_events() (+26 more)

### Community 76 - ".write"
Cohesion: 0.33
Nodes (6): PlayerSettingsStore, Into, Path, PathBuf, Result, store_recovers_last_valid_backup()

### Community 77 - "update_enemy_encounters"
Cohesion: 0.13
Nodes (33): HealingEffectSample, animate_healing_effects(), f32_to_u16_saturating(), gradient_material(), healing_burst_effect(), healing_channel_effect(), healing_effect_duration(), healing_effect_sample() (+25 more)

### Community 78 - "update_enemy_music_intensity"
Cohesion: 0.16
Nodes (14): hud_building_count(), intensity_smoothing_has_a_five_second_time_constant(), point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, Res (+6 more)

### Community 79 - "GridPos"
Cohesion: 0.08
Nodes (69): GridPos, neighbour_candidates(), offset(), Fn, Option, seagull_hash(), RecentTreePlanting, RegenerationRoleRuntime (+61 more)

### Community 80 - "loading_runtime.rs"
Cohesion: 0.07
Nodes (15): Quat, transformed_bounds_minimum_y(), transformed_mesh_vertical_extent(), building_material_cycle_stays_phase_aligned_without_cpu_asset_updates(), community_events_apply_their_authored_role_and_invasion_rates(), fine_placement_border_allows_a_walkable_gap_but_rejects_physical_encroachment(), night_lights_are_prewarmed_reused_and_disabled_during_the_day(), returning_to_main_menu_recreates_the_cover_before_scene_construction() (+7 more)

### Community 81 - "load_input"
Cohesion: 0.12
Nodes (30): PlayerSettingsRuntime, SaveRuntime, SessionStats, WorldRuntime, MenuIoRequest, PathBuf, TownLoadChoice, TownPersistenceRequest (+22 more)

### Community 82 - "GpuReadinessProbe"
Cohesion: 0.09
Nodes (24): AgentRecoveryRuntime, GpuReadinessExpected, GpuReadinessProbe, GpuReadinessShared, GpuReadinessSnapshot, MainMenuHiddenModelNodes, RetreatingCitizens, AssetId (+16 more)

### Community 83 - "sync_fish_god_presentation"
Cohesion: 0.15
Nodes (29): FallingFish, FallingFishEmitter, FishGodAnimation, FishGodExitTimer, FishGodExitTriggerSent, FishGodPresentation, advance_falling_fish(), animate_falling_fish() (+21 more)

### Community 84 - "runtime_state.rs"
Cohesion: 0.09
Nodes (24): AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityHighContrastText, AgentCommand, AgentCommandQueue, BoundsMaterialExtension, BoundsMaterialUniform, BuildingCommandQueue (+16 more)

### Community 85 - ".start"
Cohesion: 0.09
Nodes (20): BroadcastEncoder, BroadcastOutput, Display, Drop, Encoder, Error, Formatter, NativeAudioRouting (+12 more)

### Community 86 - "agents.rs"
Cohesion: 0.12
Nodes (28): authored_rotating_node_names(), building_construction_stage(), building_model_node_names(), building_model_visibility_signature(), building_model_visibility_signature_from_structure(), building_node_visibility(), building_visual_structure_signature(), crowd_separation_offsets() (+20 more)

### Community 87 - "bootstrap_runtime.rs"
Cohesion: 0.14
Nodes (26): graphify, visual regression ledgers, deterministic_seagull_call_variant(), deterministic_seagull_call_wait(), deterministic_seagull_leg(), drive_seagull_flight(), AssetServer, Commands (+18 more)

### Community 88 - "GameState"
Cohesion: 0.12
Nodes (26): advance_agent_simulation_cadence(), advance_stream_only_cadence(), agent_simulation_due(), AgentSimulationCadence, AgentSimulationCadenceState, CrowdSeparationRuntime, GameState, LoadRenderParams (+18 more)

### Community 89 - "SimulationRuntime"
Cohesion: 0.17
Nodes (27): RuntimeContent, RuntimePresentation, SimulationRuntime, RuntimeBuilding, TowerShooter, Commands, Query, Res (+19 more)

### Community 90 - "animation_audio.rs"
Cohesion: 0.10
Nodes (22): authored_post_process_stack(), procedural_ambience_wav(), procedural_seagull_call_wav(), Assets, AudioSource, FnMut, ResMut, Vec (+14 more)

### Community 91 - "encoder.rs"
Cohesion: 0.13
Nodes (23): BroadcastEncoderPreference, broadcast_output_options(), BroadcastPrerequisites, configure_amf_quality(), configure_x264_quality(), copy_packed_video_frame(), encoder_candidates(), encoder_input_format() (+15 more)

### Community 92 - "ResolvedMaterialHandle"
Cohesion: 0.13
Nodes (21): ActiveMaterialHandles, ResolvedMaterialHandle, Assets, BoundsMaterial, BTreeSet, CharacterMaterial, CloudMaterial, CritterMaterial (+13 more)

### Community 93 - "PlayerSettings"
Cohesion: 0.15
Nodes (22): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, default_ui_scale_percent(), defaults_are_valid_and_round_trip(), InterfaceSettings, NameDisplayMode, PlayerSettings (+14 more)

### Community 94 - "TechTree"
Cohesion: 0.52
Nodes (4): ContentError, Result, TechTree, valid_asset_path()

### Community 95 - "spawn_resource_visual"
Cohesion: 0.15
Nodes (23): resolved_foliage_ground_position(), Option, terrain_surface_height_at_world(), centred_resource_visual_position(), foliage_visibility_distance(), generated_resource_world_position(), grounded_resource_visual_position(), locational_visual_offset() (+15 more)

### Community 96 - "twitch_editor.rs"
Cohesion: 0.21
Nodes (22): broadcast_encoder_label(), Duration, PathBuf, Result, Sender, String, Ui, save_and_apply_game_config() (+14 more)

### Community 97 - "generate_and_spawn_world"
Cohesion: 0.10
Nodes (21): FoliageNavigationLocation, FoliageRenderBatch, FoliageVisual, generate_and_spawn_world(), Assets, AssetServer, Commands, Mesh (+13 more)

### Community 99 - "BroadcastMetrics"
Cohesion: 0.19
Nodes (15): BroadcastMetrics, AtomicU64, BroadcastEncoder, discard_pending_audio(), encode_broadcast_session(), inspect_broadcast_prerequisites(), AtomicBool, Mutex (+7 more)

### Community 100 - "capture_stream_only_target"
Cohesion: 0.12
Nodes (21): capture_stream_only_target(), configure_stream_capture_ring(), copy_gpu_rows_into(), draw_centered_label(), label_glyph(), labeled_black_rgba_frame(), offline_rgba_frame(), recycle_stream_capture_pixels() (+13 more)

### Community 101 - "field_editors.rs"
Cohesion: 0.28
Nodes (21): archetype_runtime_editor(), default_navigation_footprint_thirds(), draw_building_visual(), draw_footprint_grid(), enemy_definition_editor(), enemy_model_set_editor(), enemy_run_animation_choice(), enemy_spawner_editor() (+13 more)

### Community 102 - "instantiate_building_materials"
Cohesion: 0.16
Nodes (18): BuildingMaterialInstanced, BuildingMaterialInstances, BTreeMap, building_damage_value(), instantiate_building_materials(), Assets, BuildingMaterial, ChildOf (+10 more)

### Community 103 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 104 - "technology_graph.rs"
Cohesion: 0.07
Nodes (54): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap, Default (+46 more)

### Community 105 - "authorization.rs"
Cohesion: 0.15
Nodes (14): AuthorizationEvent, BroadcastMetricsSnapshot, BroadcastTarget, build_ingest_url(), LiveVerificationTarget, PreparedBroadcast, Debug, Formatter (+6 more)

### Community 106 - "sync_building_placers"
Cohesion: 0.17
Nodes (18): FineNavigationRuntime, BuildingPlacementGhost, BuildingPlacementGhostHiddenNodes, apply_building_placement_ghosts(), passive_resource_rate_milli_per_second(), placement_ghost_material(), BoundsMaterial, BTreeMap (+10 more)

### Community 107 - "update_stream_operator_chat"
Cohesion: 0.11
Nodes (18): bounded_history_f32(), BackgroundColor, Commands, Entity, Handle, Image, Local, Node (+10 more)

### Community 108 - "sync_building_placement_overlays"
Cohesion: 0.15
Nodes (17): BuildingPlacementOwnerOverlay, CurrentEventFill, CurrentEventPanel, CurrentEventText, BuildingPlacers, announce_community_vote_results(), BackgroundColor, ImageNode (+9 more)

### Community 109 - "sync_boot_loading_screen"
Cohesion: 0.26
Nodes (16): LoadingProgressFill, LoadingStatusText, loading_display_percent(), loading_percent_text(), loading_progress_accessibility_node(), AccessibilityNode, LoadingPercentQuery, LoadingSubstatusQuery (+8 more)

### Community 110 - "spawn_runtime_building"
Cohesion: 0.34
Nodes (13): EnemyCampGenerationDef, GridLocation, enemy_camp_candidate(), generated_enemy_camp_hash(), rounded_milli_cells(), AssetServer, Commands, Option (+5 more)

### Community 111 - ".default"
Cohesion: 0.14
Nodes (12): AnimationConditionMode, rejects_dangling_avatar_mask_reference(), rejects_dangling_material_texture(), rejects_invalid_renderer_material_bindings(), Default, Error, Self, TextureTransform (+4 more)

### Community 112 - "sync_active_pets"
Cohesion: 0.30
Nodes (14): AgentAnimation, PingPointer, animate_ping_pointers(), apply_agent_commands(), apply_building_commands(), AssetServer, Commands, Entity (+6 more)

### Community 113 - "run"
Cohesion: 0.17
Nodes (13): DisplayMode, player_window_mode(), Entity, Option, WgpuSettings, run(), runtime_wgpu_settings(), startup_window_mode() (+5 more)

### Community 114 - "update_vote_panels"
Cohesion: 0.15
Nodes (13): RulerOptionsContainer, TechnologyVoteOptionRow, VoteFillKind, VotePanelKind, VoteTextKind, technology_vote_row_advance(), Children, ruler_vote_option_font_size() (+5 more)

### Community 115 - "xtask/src/main.rs"
Cohesion: 0.05
Nodes (96): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), load_native(), native_save_is_atomic_and_keeps_backup(), native_save_keeps_only_the_five_latest_backup_generations(), NativeSaveEnvelope, NativeSaveError (+88 more)

### Community 116 - "targeting.rs"
Cohesion: 0.27
Nodes (12): building_approaches(), building_perimeter_candidates(), enemy_navigation_signature(), enemy_route_buildings(), next_agent_goal_with_reservations(), reservation_available(), BTreeMap, Vec (+4 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.15
Nodes (28): AdaptiveMusicConfig, adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine() (+20 more)

### Community 118 - "InjectedCommands"
Cohesion: 0.20
Nodes (11): CommandAcknowledgementRuntime, InjectedCommands, building_placement_remains_active(), CommandResponseRuntime, CommandSaveRuntime, expire_inactive_building_placements(), Res, ResMut (+3 more)

### Community 119 - "Option"
Cohesion: 0.18
Nodes (12): BuildingMaterialUpdateRuntime, CameraRequest, CommandWorldViewRuntime, Camera, GlobalTransform, Instant, Option, SpatialQuery (+4 more)

### Community 120 - "OperatorChatRuntime"
Cohesion: 0.29
Nodes (5): OperatorChatBadges, OperatorChatLine, OperatorChatRuntime, Into, String

### Community 121 - "jump_start_snapshot_path"
Cohesion: 0.31
Nodes (10): is_jump_start_path(), jump_start_snapshot_path(), jump_start_working_path(), Path, PathBuf, String, safe_town_filename(), town_save_entry() (+2 more)

### Community 122 - "objective_catalog_editor"
Cohesion: 0.36
Nodes (8): ObjectiveKind, objective_catalog_editor(), objective_kind_choice(), role_i32(), role_u16(), role_u32(), String, Ui

### Community 123 - "TwitchConfig"
Cohesion: 0.33
Nodes (7): BTreeSet, Option, String, shipping_fish_god_reward_id(), TwitchConfig, WindowConfig, secrets_restart_requirements()

### Community 124 - "duration_as_micros"
Cohesion: 0.38
Nodes (4): Duration, twitch_live_request_timeout(), duration_as_micros(), Duration

### Community 126 - "StreamOnlyCaptureInbox"
Cohesion: 0.33
Nodes (5): DirectTwitchBroadcastPlugin, App, Plugin, Receiver, StreamOnlyCaptureInbox

### Community 127 - "BootstrapFlowPlugin"
Cohesion: 0.40
Nodes (3): BootstrapFlowPlugin, App, Plugin

### Community 128 - "CreditsFlowPlugin"
Cohesion: 0.40
Nodes (3): CreditsFlowPlugin, App, Plugin

### Community 129 - "GameplayFlowPlugin"
Cohesion: 0.40
Nodes (3): GameplayFlowPlugin, App, Plugin

### Community 130 - "MainMenuFlowPlugin"
Cohesion: 0.40
Nodes (3): MainMenuFlowPlugin, App, Plugin

### Community 131 - "PresentationFlowPlugin"
Cohesion: 0.40
Nodes (3): PresentationFlowPlugin, App, Plugin

### Community 132 - "SharedRuntimePlugin"
Cohesion: 0.40
Nodes (3): App, Plugin, SharedRuntimePlugin

### Community 133 - "WorldLoadingFlowPlugin"
Cohesion: 0.40
Nodes (3): App, Plugin, WorldLoadingFlowPlugin

### Community 134 - "Stream Town Bevy"
Cohesion: 0.50
Nodes (4): Binaries, Commands, Runtime architecture, Stream Town Bevy

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

### Community 210 - "VideoFrame"
Cohesion: 0.13
Nodes (25): LiveVerification, LiveVerificationEvent, publish_latest_video(), Arc, AtomicBool, Drop, Error, Mutex (+17 more)

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 229 - "bevy-port/README.md"
Cohesion: 0.27
Nodes (4): Audio provenance, Development, License and media, Stream Town

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
Cohesion: 0.11
Nodes (66): AuthoringSnapshot, RoleDraft, Arc, Mutex, Receiver, ToolState, apply_building_draft(), apply_enemy_camp_generation_draft() (+58 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 336 - "GeneratedWorld"
Cohesion: 0.07
Nodes (83): DirtyRegion, GeneratedWorld, building_blocks_navigation(), building_placement_is_available(), is_path_building(), remove_selected_building(), enemy_navigation_can_reach(), nearest_reachable_building_to_town_hall() (+75 more)

### Community 495 - "emit_chimney_smoke"
Cohesion: 0.12
Nodes (34): AgentLocomotion, ChimneySmokeEmitters, ChimneySmokeParticle, animate_chimney_smoke_particles(), apply_authored_local_rotation(), apply_crowd_separation(), chimney_alpha_step(), chimney_emission_count() (+26 more)

### Community 938 - "runtime_console.rs"
Cohesion: 0.09
Nodes (28): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError (+20 more)

### Community 1189 - "direct_broadcast.rs"
Cohesion: 0.09
Nodes (30): append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), average_milliseconds(), direct_broadcast_log_path(), micros_to_milliseconds(), OperatorChatBadgeKind, report_stream_health(), rate_per_second() (+22 more)

### Community 2375 - "MenuRuntime"
Cohesion: 0.05
Nodes (95): BroadcastConfig, MenuRuntime, Arc, Default, Mutex, Receiver, SecretsAuthorizationKind, SecretsRuntime (+87 more)

## Knowledge Gaps
- **142 isolated node(s):** `stream_town_ffmpeg_bridge`, `GameplaySimulationSet`, `AgentSimulationSet`, `NightLightSyncSet`, `SensitiveScreenUpdateSet` (+137 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StableId` connect `StableId` to `process_injected_commands`, `ContentCatalog`, `loading_state.rs`, `embedded_content`, `draw_world_preview`, `ui/tests.rs`, `loading.rs`, `spawn_main_menu`, `CommunityEvent`, `stream_town_tools/src/main.rs`, `RenderAssets`, `AnimationControllerDef`, `content.rs`, `update_credits_fireworks`, `move_agents`, `capture_foliage_acceptance`, `simulation.rs`, `ground_loaded_surface_visuals`, `ui/persistence.rs`, `world_runtime.rs`, `Agent`, `music_preview.rs`, `select_grid_cell`, `asset_editors.rs`, `runtime_console.rs`, `ToolState`, `rendering.rs`, `config.rs`, `foundation.rs`, `drive_converted_animations`, `rig_and_equipment.rs`, `technology.rs`, `input.rs`, `ArchetypeDef`, `command.rs`, `sync_pooled_night_lights`, `src/world.rs`, `world_foundations.rs`, `PresentationCatalog`, `twitch.rs`, `ObjectiveDef`, `capture_city_timelapse`, `combat.rs`, `update_enemy_encounters`, `GridPos`, `GeneratedWorld`, `load_input`, `GpuReadinessProbe`, `sync_fish_god_presentation`, `runtime_state.rs`, `agents.rs`, `SimulationRuntime`, `TechTree`, `spawn_resource_visual`, `generate_and_spawn_world`, `field_editors.rs`, `instantiate_building_materials`, `technology_graph.rs`, `sync_building_placers`, `sync_building_placement_overlays`, `spawn_runtime_building`, `emit_chimney_smoke`, `sync_active_pets`, `xtask/src/main.rs`, `targeting.rs`, `Option`, `objective_catalog_editor`?**
  _High betweenness centrality (0.286) - this node is a cross-community bridge._
- **Why does `RuntimeConfig` connect `RuntimeConfig` to `process_injected_commands`, `ContentCatalog`, `embedded_content`, `spawn_main_menu`, `sync_stream_only_capture`, `update_environment_presentation`, `setup_rendering`, `RenderAssets`, `capture.rs`, `move_agents`, `capture_foliage_acceptance`, `ground_loaded_surface_visuals`, `GameConfig`, `world_loading.rs`, `world_runtime.rs`, `menu_input`, `Agent`, `select_grid_cell`, `sync_authored_post_processing`, `update_hud`, `foundation.rs`, `menu_accessibility.rs`, `sync_pooled_night_lights`, `world_foundations.rs`, `capture_screenshot`, `MenuRuntime`, `drive_tidal_music`, `combat.rs`, `update_enemy_encounters`, `update_enemy_music_intensity`, `GeneratedWorld`, `load_input`, `loading_runtime.rs`, `sync_fish_god_presentation`, `agents.rs`, `GameState`, `SimulationRuntime`, `generate_and_spawn_world`, `instantiate_building_materials`, `sync_building_placers`, `sync_building_placement_overlays`, `emit_chimney_smoke`, `sync_active_pets`, `run`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._
- **Why does `ToolState` connect `ToolState` to `ContentCatalog`, `draw_world_preview`, `ui/tests.rs`, `StableId`, `stream_town_tools/src/main.rs`, `content.rs`, `GameConfig`, `ui/persistence.rs`, `music_preview.rs`, `asset_editors.rs`, `runtime_console.rs`, `ArchetypeDef`, `PresentationCatalog`, `twitch.rs`, `ObjectiveDef`, `tools_ui`, `GeneratedWorld`, `PlayerSettings`, `twitch_editor.rs`, `technology_graph.rs`, `spawn_runtime_building`, `objective_catalog_editor`?**
  _High betweenness centrality (0.084) - this node is a cross-community bridge._
- **Are the 146 inferred relationships involving `embedded_content()` (e.g. with `.build()` and `run()`) actually correct?**
  _`embedded_content()` has 146 INFERRED edges - model-reasoned connections that need verification._
- **What connects `stream_town_ffmpeg_bridge`, `GameplaySimulationSet`, `AgentSimulationSet` to the rest of the system?**
  _142 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `process_injected_commands` be split into smaller, more focused modules?**
  _Cohesion score 0.06751054852320675 - nodes in this community are weakly interconnected._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.06349945660611707 - nodes in this community are weakly interconnected._