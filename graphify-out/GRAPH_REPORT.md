# Graph Report - Stream-Town-Bevy  (2026-09-13)

## Corpus Check
- 139 files · ~355,351 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4782 nodes · 17074 edges · 203 communities (196 shown, 7 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 1455 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7f9ce274`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- control.rs
- ContentCatalog
- tests.rs
- embedded_content
- Ui
- stream_town_tools/src/main.rs
- MenuRuntime
- sync_building_health_overlays
- Commands
- rotated_footprint
- Handle
- sync_stream_only_capture
- update_environment_presentation
- save.rs
- setup_rendering
- ModelPreviewRuntime
- RenderAssets
- Vec
- AnimationControllerDef
- content.rs
- update_credits_fireworks
- stream_operator_chat_controls
- presentation/world.rs
- src/world.rs
- StableId
- generate_and_spawn_world
- GeneratedWorld
- Option
- presentation/animation.rs
- grid_to_world_on_surface
- menu.rs
- String
- Query
- Agent
- String
- sync_building_placers
- .new
- Option
- Option
- embedded_presentation
- rendering.rs
- accessibility_input
- update_hud
- NavGrid
- BuildingDef
- bootstrap.rs
- ResolvedMaterialHandle
- config.rs
- DirectBroadcastRuntime
- record_gpu_readiness
- building_placement_is_available
- instantiate_building_materials
- profiling.rs
- add_rotation_curve
- station_candidate
- WorldSnapshot
- update_vote_panels
- RuntimeConfig
- select_grid_cell
- timelapse.rs
- command.rs
- Mesh
- sync_pooled_night_lights
- generate_world_from_layers
- Option
- TerrainAppearanceConfig
- sync_equipment_nodes
- .validate
- twitch.rs
- tools_ui
- camera_zoom_and_commands
- animate_main_menu_clouds
- environment.rs
- capture_city_timelapse
- twitch_tab
- stream_town_domain/src/lib.rs
- sync_building_placement_overlays
- .default
- DirectBroadcastControl
- agent_simulation_due
- PostProcessProfileDef
- spawn_healing_effect
- roles_tab
- VideoSettings
- generate_world_with_content
- BroadcastOutput
- sync_accessibility_preferences
- stream_town_game/src/lib.rs
- String
- spawn_foliage_visual
- audio_acceptance_wavs
- OpenNode
- hash_world
- PlayerSettings
- .visit
- EnemyPathOpenNode
- crowd_separation_offsets
- placed_resource_prune_selection
- Self
- PresentationCatalog
- InterfaceSettings
- transformed_bounds_minimum_y
- SystemRandom
- preview_grid_point
- technology_graph.rs
- xtask/src/main.rs
- tidal_music.rs
- What You Must Do When Invoked
- xtask/src/lib.rs
- Stream Town architecture
- Q: There are still no animations.
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy.
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- Tree and Foliage Flicker Regression Checklist
- graphify reference: extra exports and benchmark
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- encode_broadcast_session
- Character Animation Regression Checklist
- Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system.
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- bevy-port/README.md
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- allocate_fifo_output
- extraction-spec.md
- Q: Don't touch the live app, this is just hypothetical. What causes food to go down? Are there upkeeps?
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Q: If there is more to do, keep going.
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Q: How should Tender and Forester planting cadence and a one-time seeded-tree reset be implemented without disturbing other town state?
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town authoring suite
- Stream Town Twitch command reference
- Q: Why does Tonyville's direct broadcast capture FPS decline below 29 while encoded FPS remains 30?
- Q: Implement utility-based forester planting, diagnose and fix stalled path and wall construction, halve guardhouse defender health, add complete live-parity Music tab controls, and redeploy Tonyville live.
- ToolState
- Q: Why did live capture FPS fall below 28 during the night transition?
- Q: What remaining CPU-side world synchronization explains Tonyville capture falling after building signature consolidation?
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Q: Well hang on. Maybe I am misunderstanding something. Is there a specific age 2 tech, AFTER the townhall research is completed? That was my understanding. We needed the next townhall upgrade research completed, AND town hall to have been upgraded to level 2, before the Age Up tech would appear as an option.
- Q: How do the six reported Tonyville rendering, worker, role, footprint, unstuck, and building-age defects connect to their runtime implementations?
- Q: Authenticate the Stream Town bot and broadcaster using the prepared remote device-auth helper, then redeploy Tonyville and take it online.
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: Which repeated building visual work can explain Tonyville capture decay while encoding remains at 30 FPS?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- Q: foliage generation third cell terrain texture path diagonal forester weighted distance random
- vcpkg.json
- Q: Did the Tonyville post-fix deployment sustain target capture and encoded frame rates during the 2026-09-08T04:20:24.548Z to 04:50:24.548Z monitoring window?
- Q: Which scale-sensitive CPU path can reduce Tonyville capture frame rate while encoder and mux remain healthy?
- FFmpeg runtime and relinking
- Q: What remaining CPU-side synchronization explains Tonyville capture falling below 29 FPS during the day-to-night transition?
- Q: Does Tonyville capture FPS loss come from sunrise or sunset transitions?
- Q: Did the 2026-09-11 01:37:22-02:07:22 UTC Tonyville FPS deficit coincide with a sunrise or sunset transition?
- Q: Was Tonyville dusk at 2026-09-11 02:10 UTC an isolated capture FPS stall?
- SimulationRuntime
- Q: Did Tonyville's 2026-09-11 02:18:53-02:48:53 UTC FPS deficit isolate to the dawn transition?
- Q: It's down again. Investigate.
- Q: The glitchyf lickering now appears on the models of the characters, rather than the terrain - though it seems to be ALL characters, regardless of location.
- agents.rs
- runtime_console.rs
- direct_broadcast.rs
- Res

## God Nodes (most connected - your core abstractions)
1. `StableId` - 476 edges
2. `WorldSimulation` - 267 edges
3. `ContentCatalog` - 250 edges
4. `GridPos` - 225 edges
5. `embedded_content()` - 146 edges
6. `RenderAssets` - 144 edges
7. `GeneratedWorld` - 143 edges
8. `ToolState` - 142 edges
9. `GameConfig` - 126 edges
10. `RuntimeConfig` - 114 edges

## Surprising Connections (you probably didn't know these)
- `town_hall_loss_carries_players_and_roles_into_a_real_world_reload()` --calls--> `SimulationRuntime`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `settings_pointer_controls_switch_tabs_and_adjust_values()` --calls--> `SettingsTabButton`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `standalone_seagull_resolves_the_authored_critter_material()` --calls--> `standalone_material_override()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `foliage_surface_height_matches_the_rendered_terrain_triangles()` --calls--> `terrain_surface_height_at_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs
- `foliage_habitat_is_checked_after_offset_at_the_visible_surface()` --calls--> `resolved_foliage_ground_position()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/tests.rs → bevy-port/crates/stream_town_game/src/lib.rs

## Import Cycles
- None detected.

## Communities (203 total, 7 thin omitted)

### Community 0 - "control.rs"
Cohesion: 0.05
Nodes (112): EnemyCampGenerationDef, CommunityEvent, StreamUserType, GridLocation, is_stream_player_actor(), PendingChatCommand, preset_player_name_color(), actor_material() (+104 more)

### Community 1 - "ContentCatalog"
Cohesion: 0.08
Nodes (84): ContentCatalog, ActorState, BuildingState, String, building_health_overlay_width_px(), action_animation_speed(), enemy_can_attack_building(), selected_building_footprint() (+76 more)

### Community 2 - "tests.rs"
Cohesion: 0.02
Nodes (38): build_enemy_navigation_field(), falling_fish_seed(), handle_twitch_event(), broadcaster_moderation_state_changes_do_not_overwrite_bot_connection_state(), citizen_auto_camera_translation_centres_the_follow_target_at_close_zoom(), connected_bot_dispatches_twitch_commands_without_a_chat_gate(), converted_playback_preserves_authored_loop_modes(), converted_state_crossfade_preserves_weights_and_finishes_at_destination() (+30 more)

### Community 3 - "embedded_content"
Cohesion: 0.03
Nodes (80): changing_seed_changes_world_hash(), generate_world(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), embedded_content(), archetype_id_by_source(), weighted_enemy_archetype(), authored_building_and_role_balance_is_explicit_and_complete() (+72 more)

### Community 4 - "Ui"
Cohesion: 0.10
Nodes (64): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+56 more)

### Community 5 - "stream_town_tools/src/main.rs"
Cohesion: 0.07
Nodes (59): animation_property_curves_editor(), apply_building_draft(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authoring_config_save_bar(), authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record() (+51 more)

### Community 6 - "MenuRuntime"
Cohesion: 0.09
Nodes (61): GoLiveConfirmationAction, MenuIoRequest, MenuPage, MenuRuntime, PendingTownStart, Receiver, RuntimePlayerSettings, SaveRuntime (+53 more)

### Community 7 - "sync_building_health_overlays"
Cohesion: 0.08
Nodes (59): AudioSink, ActorHealthOverlay, ActorNameOverlay, apply_player_settings(), building_health_fraction(), BuildingHealthOverlay, BuildingPresentation, CitizenDeathAnnouncementRuntime (+51 more)

### Community 8 - "Commands"
Cohesion: 0.08
Nodes (59): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+51 more)

### Community 9 - "rotated_footprint"
Cohesion: 0.07
Nodes (58): DirtyRegion, ActionPresentation, remove_paths_covered_by_regions(), remove_selected_building(), Result, building_approach(), building_base_max_health(), complete_agent_goal() (+50 more)

### Community 10 - "Handle"
Cohesion: 0.06
Nodes (35): AccessibilityMotionDefaults, BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CachedRoleActionAudio, CharacterMaterialExtension, CharacterMaterialUniform (+27 more)

### Community 11 - "sync_stream_only_capture"
Cohesion: 0.06
Nodes (47): bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), camera_targets_primary_window(), capture_direct_broadcast_frame(), NativeGameAudioRouting, pcm16_wav_clip(), pcm16_wav_data(), poll_direct_broadcast_authorization() (+39 more)

### Community 12 - "update_environment_presentation"
Cohesion: 0.09
Nodes (29): AmbientLight, deterministic_weather(), Season, Weather, EnvironmentPalette, WeatherParticle, blend_environment_palette(), environment_palette() (+21 more)

### Community 13 - "save.rs"
Cohesion: 0.22
Nodes (20): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspects_legacy_binary_header_without_modifying_source(), native_save_is_atomic_and_keeps_backup(), native_save_keeps_only_the_five_latest_backup_generations(), NativeSaveStore, retained_terrain_mesh_rejects_malformed_geometry() (+12 more)

### Community 14 - "setup_rendering"
Cohesion: 0.09
Nodes (52): setup_rendering(), water_color_tint(), bounds_material(), building_material(), character_material(), character_material_from_standard(), cloud_material(), critter_material() (+44 more)

### Community 15 - "ModelPreviewRuntime"
Cohesion: 0.08
Nodes (54): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+46 more)

### Community 16 - "RenderAssets"
Cohesion: 0.13
Nodes (52): HudMetric, HudMetricMaximum, RenderAssets, WorldAsset, SecretsAction, SecretsConnectionKind, SecretsConnectionText, SecretsDynamicLabel (+44 more)

### Community 17 - "Vec"
Cohesion: 0.09
Nodes (32): append_terrain_quad(), append_terrain_skirt(), EnemyClusterNode, EnemyNavigationField, EnemyNavigationRuntime, EnemyNavigationTask, generated_terrain_chunk_mesh(), generated_terrain_chunks() (+24 more)

### Community 18 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+18 more)

### Community 19 - "content.rs"
Cohesion: 0.09
Nodes (43): ArchetypeBounds, ArchetypeDef, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingModelDef, default_resource_generation_layers(), EnemyDef (+35 more)

### Community 20 - "update_credits_fireworks"
Cohesion: 0.09
Nodes (47): AuthoredCreditsElement, CreditsFade, CreditsFireworkParticle, CreditsFireworkParticleKind, CreditsTimeline, LevelUpPresentation, LevelUpToast, pseudo_noise() (+39 more)

### Community 21 - "stream_operator_chat_controls"
Cohesion: 0.06
Nodes (43): bounded_history_f32(), CadenceTick, moderate_selected_operator_user(), operator_chat_scroll_rows(), operator_info_refresh_due(), operator_window_close_requests_exit(), Changed, Instant (+35 more)

### Community 22 - "presentation/world.rs"
Cohesion: 0.05
Nodes (128): GridPos, ActorKind, AgentCommand, AgentGoal, EnemyRouteBuilding, PathSurfaceRuntime, RecentTreePlanting, RegenerationRoleRuntime (+120 more)

### Community 23 - "src/world.rs"
Cohesion: 0.11
Nodes (43): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_world_to_grid(), avalanche_instance_hash(), averaged_terrain_corner_height(), cell_hash(), final_surface_filtered_generation_remains_deterministic(), final_surface_height_overrides_a_raw_land_cell_at_the_waterline() (+35 more)

### Community 24 - "StableId"
Cohesion: 0.04
Nodes (90): ObjectiveDef, Display, FromStr, StableId, actor_appearance_hash(), actor_appearances_are_seeded_varied_and_persisted(), authored_trade_rates_clamp_to_stock_gold_and_capacity(), awakening_applies_to_role_experience_at_the_domain_boundary() (+82 more)

### Community 25 - "generate_and_spawn_world"
Cohesion: 0.09
Nodes (68): run(), BuildingDamageEmitter, BuildingEffectKind, BuildingEffectParticle, CombatProjectile, CombatVisualKind, FallingFishEmitter, FishGodAnimation (+60 more)

### Community 26 - "GeneratedWorld"
Cohesion: 0.11
Nodes (44): GameConfig, GameplayConfig, BTreeMap, FoliageHabitat, GeneratedFoliage, GeneratedResource, GeneratedWorld, building_health_overlay_world_position() (+36 more)

### Community 27 - "Option"
Cohesion: 0.09
Nodes (42): AutomaticResumeRuntime, BootDestination, MenuLoadingRuntime, MenuRevealRuntime, Default, Instant, UntypedHandle, WorldLoadingCoverRuntime (+34 more)

### Community 28 - "presentation/animation.rs"
Cohesion: 0.11
Nodes (39): AnimationBlendSelection, AnimationTransitionPlayback, EnemyModelSetDef, ConvertedAnimationCrossfade, ConvertedAnimationLayerDriver, ConvertedAnimationPlayback, GateAnimationDriver, AnimationNodeIndex (+31 more)

### Community 29 - "grid_to_world_on_surface"
Cohesion: 0.09
Nodes (35): AuthoredRotatingNode, automatic_resume_world_seed(), ChimneySmokeEmitterRuntime, CombatImpactParticle, CombatTrailSegment, FallingFish, FoliageAcceptanceCapture, grid_to_world() (+27 more)

### Community 30 - "menu.rs"
Cohesion: 0.05
Nodes (70): AccessibleNode, MainMenuHiddenModelNodes, SecretsCredentialState, SecretsField, SecretsStatusTone, SettingsTab, TownSaveEntry, accessibility_settings_selection() (+62 more)

### Community 31 - "String"
Cohesion: 0.08
Nodes (29): AccessibilityRuntime, ActorAnimationDriver, AnimatedCharacterShadowReceiver, BuildingModelNode, CachedConvertedAnimation, CachedGateAnimation, CommandFeedback, ConvertedAnimationCache (+21 more)

### Community 32 - "Query"
Cohesion: 0.12
Nodes (49): AnimationGraphHandle, ConvertedAnimationApplied, ConvertedAnimationInstanceReady, ConvertedAnimationSpec, CosmeticNodeProcessed, EnemyModelNodeProcessed, EquipmentNodeProcessed, GateAnimationBinding (+41 more)

### Community 33 - "Agent"
Cohesion: 0.09
Nodes (35): AnimatedBy, Agent, ConvertedAnimationDriver, FoliageAcceptanceScreenshot, PendingSurfaceGrounding, ResourceNode, agent_action_animation(), agent_is_moving() (+27 more)

### Community 34 - "String"
Cohesion: 0.13
Nodes (35): ability_choices(), action_animation_choices(), building_model_node_choices(), buildings_tab(), cached_gltf_metadata(), discover_model_assets(), discover_texture_assets(), discovered_model_assets_are_project_relative_glbs() (+27 more)

### Community 35 - "sync_building_placers"
Cohesion: 0.14
Nodes (34): ActivePetVisual, BuildingPlacementGhost, BuildingPlacementGhostHiddenNodes, BuildingPlacementGhostMesh, BuildingPlacementGhostNodeProcessed, PingPointer, animate_ping_pointers(), announce_ruler_vote_result() (+26 more)

### Community 36 - ".new"
Cohesion: 0.13
Nodes (22): apply_direct_broadcast_control(), closing_the_operator_window_requests_a_graceful_game_exit(), configure_direct_broadcast(), direct_broadcast_stays_offline_until_operator_requests_it(), DirectTwitchBroadcastPlugin, ending_stream_returns_the_operator_to_main_menu_after_shutdown(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting() (+14 more)

### Community 37 - "Option"
Cohesion: 0.08
Nodes (26): App, Plugin, StreamTownGamePlugin, actor_health_bar_hide_seconds(), apply_authored_ui_fonts(), automatic_resume_save_path(), EnvironmentPresentation, locate_asset_root() (+18 more)

### Community 38 - "Option"
Cohesion: 0.07
Nodes (53): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastVideoSink, capture_process_audio(), capture_stream_only_target(), configure_stream_capture_ring() (+45 more)

### Community 39 - "embedded_presentation"
Cohesion: 0.08
Nodes (31): ArchetypeKind, embedded_presentation(), archetype_by_source(), converted_animation_spec(), default_archetype_scene(), runtime_archetype_scene(), building_bounds_material_preserves_unity_placement_contract(), converted_crossfade_uses_fixed_or_normalized_authored_duration() (+23 more)

### Community 40 - "rendering.rs"
Cohesion: 0.12
Nodes (30): announce_citizen_deaths(), announce_community_vote_results(), citizen_death_announcement(), community_vote_event(), community_vote_option_lines(), compact_technology_label(), content_label(), current_event_panel_state() (+22 more)

### Community 41 - "accessibility_input"
Cohesion: 0.12
Nodes (30): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AccessibleButtonScope, CreditsSkipButton, GameMenuAction, GameMenuActionLabel, MainMenuAction, SettingsAction (+22 more)

### Community 42 - "update_hud"
Cohesion: 0.11
Nodes (25): Hud, HudTechnologyProgressFill, SeasonMeter, active_event_text(), fit_hud_label(), format_rgb(), hud_play_time(), hud_season_meter_percent() (+17 more)

### Community 43 - "NavGrid"
Cohesion: 0.14
Nodes (23): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavGrid, NavigationError, neighbour_candidates() (+15 more)

### Community 44 - "BuildingDef"
Cohesion: 0.11
Nodes (30): BuildingDef, PassiveResourceContribution, RoleSlotContribution, StorageContribution, StorageModelDef, TargetingScoreDef, building_blocks_navigation(), passive_resource_rate_milli_per_second() (+22 more)

### Community 45 - "bootstrap.rs"
Cohesion: 0.15
Nodes (19): adaptive_music_preview_program(), is_transient_surface_configuration_error(), load_player_settings(), load_runtime_config(), player_settings_path(), AnyResult, PathBuf, Result (+11 more)

### Community 46 - "ResolvedMaterialHandle"
Cohesion: 0.10
Nodes (28): ActiveMaterialHandles, BuildingMaterialInstance, BuildingTimeCycleSignature, CharacterBaseMaterialCache, CharacterBaseMaterialVariant, CosmeticRenderer, ResolvedMaterialHandle, Assets (+20 more)

### Community 47 - "config.rs"
Cohesion: 0.14
Nodes (22): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Option (+14 more)

### Community 48 - "DirectBroadcastRuntime"
Cohesion: 0.13
Nodes (21): AuthorizationEvent, average_milliseconds(), BroadcastMetricsSnapshot, BroadcastPrerequisites, BroadcastTarget, DirectBroadcastPhase, DirectBroadcastRuntime, DirectBroadcastSnapshot (+13 more)

### Community 49 - "record_gpu_readiness"
Cohesion: 0.11
Nodes (16): LoadingWork, LoadingWorkNode, record_gpu_readiness(), round_robin_indices(), GpuImage, GpuRenderAssets, Item, Iterator (+8 more)

### Community 50 - "building_placement_is_available"
Cohesion: 0.18
Nodes (25): building_placement_remains_active(), BuildingPlacement, BuildingPlacers, expire_inactive_building_placements(), building_placement_is_available(), building_placement_overlay_world_position(), building_placement_visual_cell_is_available(), candidate_building_fine_cells() (+17 more)

### Community 51 - "instantiate_building_materials"
Cohesion: 0.14
Nodes (25): BuildingMaterialInstanced, BuildingMaterialInstances, BuildingMaterialUpdateRuntime, MaterialOverrideApplied, animate_agents(), apply_material_overrides(), building_damage_value(), instantiate_building_materials() (+17 more)

### Community 52 - "profiling.rs"
Cohesion: 0.08
Nodes (63): append_profile_sample(), ArchetypeProfile, asset_count(), asset_counts(), begin_profiled_frame(), build_profile_sample(), collect_profile_sample(), diagnostic_profiles() (+55 more)

### Community 53 - "add_rotation_curve"
Cohesion: 0.15
Nodes (24): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), close_rotation_loop(), close_scale_loop(), close_translation_loop() (+16 more)

### Community 54 - "station_candidate"
Cohesion: 0.17
Nodes (23): RoleDef, StationDef, CachedStationTargets, VecDeque, StationResourceTargetIndex, StationTargetRuntime, active_station_ids(), assigned_station() (+15 more)

### Community 55 - "WorldSnapshot"
Cohesion: 0.18
Nodes (20): inspect_legacy_save(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native(), NativeSaveEnvelope, NativeSaveError, BTreeMap (+12 more)

### Community 56 - "update_vote_panels"
Cohesion: 0.12
Nodes (21): TechVote, RulerOptionsContainer, TechnologyVoteOptionRow, VoteFillKind, VotePanelKind, VoteTextKind, technology_vote_row_advance(), Children (+13 more)

### Community 57 - "RuntimeConfig"
Cohesion: 0.14
Nodes (20): RuntimeConfig, town_resource_amount(), accessibility_navigation_preserves_editable_text_focus(), combat_camera_holds_a_target_for_five_seconds_before_redirecting(), enter_headless_world(), game_menu_mouse_actions_use_authoritative_requests_and_settings(), headless_launch_through_credits_round_trip_covers_shipping_states(), headless_new_town_matches_shipping_starting_roster() (+12 more)

### Community 58 - "select_grid_cell"
Cohesion: 0.12
Nodes (20): SelectedActor, centre_screen_raycast_world_position(), pointer_is_over_button(), Assets, Button, ButtonInput, Camera, GlobalTransform (+12 more)

### Community 59 - "timelapse.rs"
Cohesion: 0.14
Nodes (18): CityTimelapsePlugin, CityTimelapseScreenshot, draw_timelapse_label(), next_timelapse_frame_index(), rebuild_timelapse_video(), App, On, Path (+10 more)

### Community 60 - "command.rs"
Cohesion: 0.15
Nodes (34): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, command_usage(), CommandParseError, content_id() (+26 more)

### Community 61 - "Mesh"
Cohesion: 0.17
Nodes (11): AssetId, CoreRenderAssets, GpuReadinessExpected, GpuReadinessShared, GpuReadinessSnapshot, PreparedWorld, BTreeSet, Image (+3 more)

### Community 62 - "sync_pooled_night_lights"
Cohesion: 0.15
Nodes (19): TimeCycleConfig, NightPointLightPoolSlot, animate_weather_particles(), building_night_light_profile(), night_light_is_active(), night_light_sources_are_inactive(), night_light_transition_delay(), Commands (+11 more)

### Community 63 - "generate_world_from_layers"
Cohesion: 0.18
Nodes (19): WorldGenConfig, ResourceGenerationLayerDef, authored_grid_centre(), foliage_horizontal_hash(), generate_shoreline_resources(), generate_world_from_layers(), generate_world_with_content_observed(), horizontal_hash() (+11 more)

### Community 64 - "Option"
Cohesion: 0.24
Nodes (19): add_animation_composition(), add_animation_layer_branch(), animation_nodes_for_selection(), animation_target_for_track(), build_converted_animation(), converted_clip_request(), ConvertedClipRequest, gate_animation_contract() (+11 more)

### Community 65 - "TerrainAppearanceConfig"
Cohesion: 0.16
Nodes (15): Default, SeasonalTerrainPalette, TerrainAppearanceConfig, traversal_score_for_rate(), traversal_wear_fraction(), TraversalWearCell, seasonal_terrain_palette(), foliage_should_be_hidden() (+7 more)

### Community 66 - "sync_equipment_nodes"
Cohesion: 0.14
Nodes (17): ActorCustomization, AgentAnimation, AgentEquipmentPresentation, CosmeticNode, CosmeticNodeKind, MovementAnimationState, TransientCarryVisibility, actor_carries_role_resource() (+9 more)

### Community 67 - ".validate"
Cohesion: 0.19
Nodes (15): finite_positive(), is_glb_path(), ordered_positive_range(), PresentationError, Item, Iterator, Result, valid_curve_times() (+7 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (66): BTreeSet, TwitchConfig, SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault (+58 more)

### Community 69 - "tools_ui"
Cohesion: 0.16
Nodes (17): debug_fingerprint(), default_role_preview_animation(), matching_role_animation_state(), music_tab(), player_animation_controller(), poll_tool_job_events(), poll_twitch_tool_events(), role_preview_animation_request() (+9 more)

### Community 70 - "camera_zoom_and_commands"
Cohesion: 0.18
Nodes (16): AnimationTransitions, auto_camera_citizen_translation(), auto_camera_focus_translation(), camera_ground_focus(), camera_zoom_and_commands(), constrain_town_camera_position(), drive_gate_animations(), frame_independent_lerp_factor() (+8 more)

### Community 71 - "animate_main_menu_clouds"
Cohesion: 0.17
Nodes (16): FishSchoolParticle, MainMenuCloudPrism, animate_fish_school(), animate_loading_icon(), animate_main_menu_clouds(), apply_loading_icon_rotation(), fish_school_noise_axis(), fish_school_transform() (+8 more)

### Community 72 - "environment.rs"
Cohesion: 0.17
Nodes (14): append_nearest_night_lights(), color_from_rgb8(), NightLightSpec, perceptually_normalized_light_color(), player_night_light_level_multiplier(), Color, Vec, Vec3 (+6 more)

### Community 73 - "capture_city_timelapse"
Cohesion: 0.11
Nodes (24): TimelapseInterval, BuildingPlacementOwnerOverlay, BuildingPlacementVisual, apply_confirmed_builds(), capture_city_timelapse(), CityTimelapseBuildConfirmed, CityTimelapseRuntime, Commands (+16 more)

### Community 74 - "twitch_tab"
Cohesion: 0.20
Nodes (16): broadcast_encoder_label(), Duration, Sender, start_twitch_authorization(), start_twitch_broadcast_authorization(), start_twitch_broadcast_clear(), start_twitch_broadcast_diagnostic(), start_twitch_clear() (+8 more)

### Community 75 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 76 - "sync_building_placement_overlays"
Cohesion: 0.17
Nodes (15): CurrentEventFill, CurrentEventPanel, CurrentEventText, building_placement_overlay_text(), BackgroundColor, ImageNode, Node, Text (+7 more)

### Community 77 - ".default"
Cohesion: 0.14
Nodes (12): AnimationConditionMode, rejects_dangling_avatar_mask_reference(), rejects_dangling_material_texture(), rejects_invalid_renderer_material_bindings(), Default, Error, Self, TextureTransform (+4 more)

### Community 78 - "DirectBroadcastControl"
Cohesion: 0.16
Nodes (10): AutomaticBroadcastStart, BroadcastStopDisposition, DirectBroadcastControl, exit_after_broadcast_stops(), request_automatic_broadcast_start(), AppExit, Default, MessageWriter (+2 more)

### Community 79 - "agent_simulation_due"
Cohesion: 0.32
Nodes (11): advance_agent_simulation_cadence(), advance_stream_only_cadence(), agent_simulation_due(), AgentSimulationCadence, AgentSimulationCadenceState, night_light_sync_due(), AtomicU64, Duration (+3 more)

### Community 80 - "PostProcessProfileDef"
Cohesion: 0.20
Nodes (12): PostProcessBloomDef, PostProcessColorAdjustmentsDef, PostProcessMotionBlurDef, PostProcessProfileDef, PostProcessTonemapping, PostProcessVignetteDef, authored_color_grading(), authored_post_process_stack() (+4 more)

### Community 81 - "spawn_healing_effect"
Cohesion: 0.33
Nodes (12): HealingEffectKind, HealingEffectSample, HealingMoteEffect, HealingRingEffect, f32_to_u16_saturating(), healing_burst_effect(), healing_channel_effect(), healing_effect_duration() (+4 more)

### Community 82 - "roles_tab"
Cohesion: 0.32
Nodes (12): apply_role_draft(), delete_selected_role(), duplicate_selected_role(), legacy_roles_tab(), refresh_role_draft(), role_draft(), role_editor_applies_every_reference_family_without_partial_mutation(), role_i32() (+4 more)

### Community 83 - "VideoSettings"
Cohesion: 0.22
Nodes (9): DisplayMode, PostProcessAntiAliasing, Option, VideoSettings, player_window_mode(), Entity, Option, startup_window_mode() (+1 more)

### Community 84 - "generate_world_with_content"
Cohesion: 0.20
Nodes (10): different_town_seeds_produce_different_resource_and_foliage_layouts(), generate_world_with_content(), automatic_resume_seed_is_loaded_before_world_generation(), enemy_attacks_can_kill_citizens(), fish_school_uses_authored_spawn_volume_noise_and_velocity_alignment(), placement_ghost_uses_runtime_building_alignment_and_validity_colour(), placement_visual_switches_typed_bounds_material_for_collision_state(), prior_native_resource_stock_scales_once_for_long_lived_land_nodes() (+2 more)

### Community 85 - "BroadcastOutput"
Cohesion: 0.29
Nodes (5): BroadcastOutput, Output, Deref, DerefMut, Target

### Community 86 - "sync_accessibility_preferences"
Cohesion: 0.28
Nodes (9): reduced_grass_wind(), reduced_tree_wind(), reduced_water_wind(), GrassMaterial, TreeMaterial, Vec4, WaterMaterial, sync_accessibility_preferences() (+1 more)

### Community 87 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (110): graphify, visual regression ledgers, AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityHighContrastText, actor_health_fill_color(), actor_name_color(), ActorHealthFill (+102 more)

### Community 88 - "String"
Cohesion: 0.28
Nodes (9): AudioBaselineManifest, Cli, Command, reset_role_is_available(), BTreeMap, PathBuf, String, VisualBaselineManifest (+1 more)

### Community 89 - "spawn_foliage_visual"
Cohesion: 0.29
Nodes (8): FoliageNavigationLocation, FoliageRenderBatch, SurfaceFoliageHabitat, FoliageBatchKey, insert_world_material(), EntityCommands, spawn_foliage_visual(), runtime_foliage_waits_for_world_before_habitat_classification()

### Community 90 - "audio_acceptance_wavs"
Cohesion: 0.32
Nodes (8): audio_acceptance_manifest(), audio_acceptance_record(), audio_acceptance_wavs(), procedural_audio_matches_curated_acceptance_baseline(), BTreeMap, String, Value, Vec

### Community 91 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Ord, Ordering, PartialOrd, Self

### Community 92 - "hash_world"
Cohesion: 0.52
Nodes (7): hash_world(), legacy_resource_navigation(), legacy_v1_world_hash(), legacy_v2_world_hash(), legacy_v3_world_hash(), legacy_variable_resource_amounts(), String

### Community 93 - "PlayerSettings"
Cohesion: 0.14
Nodes (23): AudioMixSettings, CameraSettings, default_ui_scale_percent(), defaults_are_valid_and_round_trip(), PlayerSettings, PlayerSettingsStore, PlayerSettingsStoreError, PlayerSettingsValidationError (+15 more)

### Community 94 - ".visit"
Cohesion: 0.60
Nodes (3): ContentError, Result, valid_asset_path()

### Community 95 - "EnemyPathOpenNode"
Cohesion: 0.47
Nodes (4): EnemyPathOpenNode, Ord, Ordering, PartialOrd

### Community 96 - "crowd_separation_offsets"
Cohesion: 0.47
Nodes (6): crowd_separation_offsets(), crowd_spatial_bucket(), deterministic_overlap_direction(), Vec, Vec2, crowd_separation_is_deterministic_bounded_and_balanced()

### Community 97 - "placed_resource_prune_selection"
Cohesion: 0.40
Nodes (4): placed_resource_prune_is_exact_deterministic_and_preserves_generated_nodes(), placed_resource_prune_selection(), PlacedResourceKind, PlacedResourcePruneCount

### Community 99 - "PresentationCatalog"
Cohesion: 0.09
Nodes (46): AnimationClipDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationParameterDef (+38 more)

### Community 100 - "InterfaceSettings"
Cohesion: 0.40
Nodes (5): BuildingHealthDisplayMode, InterfaceSettings, NameDisplayMode, should_show_actor_name(), should_show_building_health()

### Community 101 - "transformed_bounds_minimum_y"
Cohesion: 0.40
Nodes (5): Quat, transformed_bounds_minimum_y(), surface_grounding_places_rotated_bounds_on_the_surface(), surface_grounding_uses_real_vertices_instead_of_empty_aabb_corners(), surface_visual_remains_hidden_until_grounding_is_complete()

### Community 103 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

### Community 104 - "technology_graph.rs"
Cohesion: 0.08
Nodes (55): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+47 more)

### Community 115 - "xtask/src/main.rs"
Cohesion: 0.12
Nodes (47): adaptive_wall_or_gate_cells(), archive_purge_backup_history(), exact_building_cells(), fine_navigation_town_reset_preserves_only_requested_progress(), glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes() (+39 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.07
Nodes (59): AdaptiveMusicConfig, adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), AdaptiveMusicSignature, authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), drive_tidal_music() (+51 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 157 - "Stream Town architecture"
Cohesion: 0.33
Nodes (5): Data flow, Design rules, Stream Town architecture, Verification, Workspace boundaries

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy."
Cohesion: 0.50
Nodes (3): Answer, Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy., Source Nodes

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Current attempts, Do not retry unchanged, Next narrow diagnostic pass, Tree and Foliage Flicker Regression Checklist, What did not fix the flicker, What did work

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 210 - "encode_broadcast_session"
Cohesion: 0.07
Nodes (41): AtomicUsize, BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, BroadcastMetrics, configured_1080p30_x264_encoder_sustains_realtime_output(), configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame() (+33 more)

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 224 - "Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system., Source Nodes

### Community 225 - "Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?, Source Nodes

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 228 - "Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed, Source Nodes

### Community 229 - "bevy-port/README.md"
Cohesion: 0.19
Nodes (7): Audio provenance, Binaries, Commands, Stream Town Bevy, Development, License and media, Stream Town

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "Q: shader material giraffe pet skinning prefab reachable shipping presentation"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: shader material giraffe pet skinning prefab reachable shipping presentation, Source Nodes

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "Q: Why are we vendoring Bevy Tidal and not just using the library that exists??"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why are we vendoring Bevy Tidal and not just using the library that exists??, Source Nodes

### Community 238 - "Twitch setup"
Cohesion: 0.20
Nodes (10): 1. Secure the old credentials, 2. Register the Twitch application, 3. Configure and authorize `HumanBeanBot`, 4. Prepare the channel, 5. Bind the Fish God Channel Points reward, 6. Authorize direct broadcasting, 7. Choose broadcast quality and test bandwidth, 8. Go live without OBS (+2 more)

### Community 240 - "Accessibility"
Cohesion: 0.29
Nodes (6): Accessibility, Automated verification, Keyboard operation, Persisted preferences, Screen-reader contract, Windows Narrator acceptance

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Q: How are shipping visual and audio parity implemented in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How are shipping visual and audio parity implemented in the Bevy migration?, Source Nodes

### Community 248 - "Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?"
Cohesion: 0.50
Nodes (3): Answer, Outcome, Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "allocate_fifo_output"
Cohesion: 0.50
Nodes (3): allocate_fifo_output(), Output, Result

### Community 253 - "Q: Don't touch the live app, this is just hypothetical. What causes food to go down? Are there upkeeps?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Don't touch the live app, this is just hypothetical. What causes food to go down? Are there upkeeps?, Source Nodes

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_ffmpeg_bridge, stream_town_game, stream_town_tools, xtask

### Community 270 - "Q: The Bevy Tidal repo is now public, so fix the integration."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The Bevy Tidal repo is now public, so fix the integration., Source Nodes

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "Q: How should Tender and Forester planting cadence and a one-time seeded-tree reset be implemented without disturbing other town state?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How should Tender and Forester planting cadence and a one-time seeded-tree reset be implemented without disturbing other town state?, Source Nodes

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town authoring suite, Workflows

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.12
Nodes (17): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, City timelapse output, Getting started, Information, Moderator and game-master commands (+9 more)

### Community 290 - "Q: Why does Tonyville's direct broadcast capture FPS decline below 29 while encoded FPS remains 30?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why does Tonyville's direct broadcast capture FPS decline below 29 while encoded FPS remains 30?, Source Nodes

### Community 292 - "Q: Implement utility-based forester planting, diagnose and fix stalled path and wall construction, halve guardhouse defender health, add complete live-parity Music tab controls, and redeploy Tonyville live."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Implement utility-based forester planting, diagnose and fix stalled path and wall construction, halve guardhouse defender health, add complete live-parity Music tab controls, and redeploy Tonyville live., Source Nodes

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (73): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot (+65 more)

### Community 303 - "Q: Why did live capture FPS fall below 28 during the night transition?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did live capture FPS fall below 28 during the night transition?, Source Nodes

### Community 304 - "Q: What remaining CPU-side world synchronization explains Tonyville capture falling after building signature consolidation?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What remaining CPU-side world synchronization explains Tonyville capture falling after building signature consolidation?, Source Nodes

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 308 - "Q: Well hang on. Maybe I am misunderstanding something. Is there a specific age 2 tech, AFTER the townhall research is completed? That was my understanding. We needed the next townhall upgrade research completed, AND town hall to have been upgraded to level 2, before the Age Up tech would appear as an option."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Well hang on. Maybe I am misunderstanding something. Is there a specific age 2 tech, AFTER the townhall research is completed? That was my understanding. We needed the next townhall upgrade research completed, AND town hall to have been upgraded to level 2, before the Age Up tech would appear as an option., Source Nodes

### Community 309 - "Q: How do the six reported Tonyville rendering, worker, role, footprint, unstuck, and building-age defects connect to their runtime implementations?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do the six reported Tonyville rendering, worker, role, footprint, unstuck, and building-age defects connect to their runtime implementations?, Source Nodes

### Community 310 - "Q: Authenticate the Stream Town bot and broadcaster using the prepared remote device-auth helper, then redeploy Tonyville and take it online."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Authenticate the Stream Town bot and broadcaster using the prepared remote device-auth helper, then redeploy Tonyville and take it online., Source Nodes

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "Q: Which repeated building visual work can explain Tonyville capture decay while encoding remains at 30 FPS?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which repeated building visual work can explain Tonyville capture decay while encoding remains at 30 FPS?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "Q: foliage generation third cell terrain texture path diagonal forester weighted distance random"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: foliage generation third cell terrain texture path diagonal forester weighted distance random, Source Nodes

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 321 - "Q: Did the Tonyville post-fix deployment sustain target capture and encoded frame rates during the 2026-09-08T04:20:24.548Z to 04:50:24.548Z monitoring window?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Did the Tonyville post-fix deployment sustain target capture and encoded frame rates during the 2026-09-08T04:20:24.548Z to 04:50:24.548Z monitoring window?, Source Nodes

### Community 330 - "Q: Which scale-sensitive CPU path can reduce Tonyville capture frame rate while encoder and mux remain healthy?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which scale-sensitive CPU path can reduce Tonyville capture frame rate while encoder and mux remain healthy?, Source Nodes

### Community 332 - "Q: What remaining CPU-side synchronization explains Tonyville capture falling below 29 FPS during the day-to-night transition?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What remaining CPU-side synchronization explains Tonyville capture falling below 29 FPS during the day-to-night transition?, Source Nodes

### Community 333 - "Q: Does Tonyville capture FPS loss come from sunrise or sunset transitions?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Does Tonyville capture FPS loss come from sunrise or sunset transitions?, Source Nodes

### Community 334 - "Q: Did the 2026-09-11 01:37:22-02:07:22 UTC Tonyville FPS deficit coincide with a sunrise or sunset transition?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Did the 2026-09-11 01:37:22-02:07:22 UTC Tonyville FPS deficit coincide with a sunrise or sunset transition?, Source Nodes

### Community 335 - "Q: Was Tonyville dusk at 2026-09-11 02:10 UTC an isolated capture FPS stall?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Was Tonyville dusk at 2026-09-11 02:10 UTC an isolated capture FPS stall?, Source Nodes

### Community 336 - "SimulationRuntime"
Cohesion: 0.08
Nodes (63): FineNavigationRuntime, FoliageVisual, InjectedCommands, MovementStats, RuntimeCaptureRequest, RuntimeConsoleRuntime, RuntimeContent, SessionStats (+55 more)

### Community 337 - "Q: Did Tonyville's 2026-09-11 02:18:53-02:48:53 UTC FPS deficit isolate to the dawn transition?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Did Tonyville's 2026-09-11 02:18:53-02:48:53 UTC FPS deficit isolate to the dawn transition?, Source Nodes

### Community 338 - "Q: It's down again. Investigate."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: It's down again. Investigate., Source Nodes

### Community 357 - "Q: The glitchyf lickering now appears on the models of the characters, rather than the terrain - though it seems to be ALL characters, regardless of location."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The glitchyf lickering now appears on the models of the characters, rather than the terrain - though it seems to be ALL characters, regardless of location., Source Nodes

### Community 495 - "agents.rs"
Cohesion: 0.08
Nodes (67): AgentLocomotion, AuthoredRotatingNodeProcessed, BuildingModelNodeProcessed, ChimneySmokeEmitters, ChimneySmokeParticle, CrowdSeparationRuntime, RuntimeBuilding, animate_chimney_smoke_particles() (+59 more)

### Community 938 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 1189 - "direct_broadcast.rs"
Cohesion: 0.05
Nodes (47): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), bandwidth_test_url_is_constructed_without_logging_the_key(), broadcast_output_options(), build_ingest_url(), configure_amf_quality(), configure_x264_quality() (+39 more)

### Community 2375 - "Res"
Cohesion: 0.08
Nodes (61): AccessibilityNode, GameState, LoadingProgressFill, LoadingScreenEntity, LoadingStatusText, LoadingUiCamera, MainMenuRotatingDefinitions, SettingsValueRow (+53 more)

## Knowledge Gaps
- **307 isolated node(s):** `stream_town_ffmpeg_bridge`, `StreamOperatorCamera`, `StreamOperatorLiveButton`, `StreamOperatorRestartButton`, `StreamOperatorChatInputText` (+302 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **7 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BroadcastMetrics` (8× useful, score=7.295753427)
- `ConvertedAnimationDriver` (5× useful, score=2.829086046) _(code changed — re-verify)_
- `update_environment_presentation()` (4× useful, score=3.822255583) _(code changed — re-verify)_
- `capture_stream_only_target()` (4× useful, score=3.760823498)
- `RenderAssets` (4× useful, score=2.02027194) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=1.848284623) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=1.545762532)
- `WorldSnapshot` (3× useful, score=1.47658906)
- `TraversalWearRuntime` (2× useful, score=1.911683571) _(code changed — re-verify)_
- `sync_building_health_overlays()` (2× useful, score=1.862435654) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StableId` connect `StableId` to `control.rs`, `ContentCatalog`, `embedded_content`, `Ui`, `stream_town_tools/src/main.rs`, `sync_building_health_overlays`, `Commands`, `rotated_footprint`, `save.rs`, `RenderAssets`, `Vec`, `AnimationControllerDef`, `content.rs`, `update_credits_fireworks`, `presentation/world.rs`, `src/world.rs`, `generate_and_spawn_world`, `GeneratedWorld`, `Option`, `presentation/animation.rs`, `grid_to_world_on_surface`, `menu.rs`, `String`, `Query`, `Agent`, `String`, `sync_building_placers`, `Option`, `rendering.rs`, `runtime_console.rs`, `ToolState`, `BuildingDef`, `config.rs`, `building_placement_is_available`, `instantiate_building_materials`, `add_rotation_curve`, `station_candidate`, `WorldSnapshot`, `update_vote_panels`, `select_grid_cell`, `command.rs`, `sync_pooled_night_lights`, `generate_world_from_layers`, `Option`, `.validate`, `twitch.rs`, `tools_ui`, `capture_city_timelapse`, `stream_town_domain/src/lib.rs`, `sync_building_placement_overlays`, `SimulationRuntime`, `spawn_healing_effect`, `roles_tab`, `stream_town_game/src/lib.rs`, `String`, `spawn_foliage_visual`, `.visit`, `crowd_separation_offsets`, `placed_resource_prune_selection`, `PresentationCatalog`, `technology_graph.rs`, `agents.rs`, `xtask/src/main.rs`?**
  _High betweenness centrality (0.178) - this node is a cross-community bridge._
- **Why does `RuntimeConfig` connect `RuntimeConfig` to `control.rs`, `MenuRuntime`, `sync_building_health_overlays`, `Commands`, `sync_stream_only_capture`, `update_environment_presentation`, `setup_rendering`, `RenderAssets`, `stream_operator_chat_controls`, `presentation/world.rs`, `generate_and_spawn_world`, `GeneratedWorld`, `menu.rs`, `Query`, `Agent`, `sync_building_placers`, `.new`, `Option`, `accessibility_input`, `update_hud`, `BuildingDef`, `instantiate_building_materials`, `station_candidate`, `select_grid_cell`, `sync_pooled_night_lights`, `camera_zoom_and_commands`, `Res`, `sync_building_placement_overlays`, `agent_simulation_due`, `SimulationRuntime`, `stream_town_game/src/lib.rs`, `spawn_foliage_visual`, `agents.rs`, `tidal_music.rs`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **Why does `ContentCatalog` connect `ContentCatalog` to `control.rs`, `embedded_content`, `Ui`, `stream_town_tools/src/main.rs`, `Commands`, `rotated_footprint`, `content.rs`, `presentation/world.rs`, `StableId`, `GeneratedWorld`, `Option`, `presentation/animation.rs`, `grid_to_world_on_surface`, `menu.rs`, `Agent`, `String`, `Option`, `embedded_presentation`, `rendering.rs`, `update_hud`, `ToolState`, `BuildingDef`, `building_placement_is_available`, `station_candidate`, `update_vote_panels`, `generate_world_from_layers`, `sync_equipment_nodes`, `tools_ui`, `animate_main_menu_clouds`, `SimulationRuntime`, `roles_tab`, `generate_world_with_content`, `String`, `.visit`, `technology_graph.rs`, `agents.rs`, `xtask/src/main.rs`?**
  _High betweenness centrality (0.056) - this node is a cross-community bridge._
- **Are the 144 inferred relationships involving `embedded_content()` (e.g. with `.build()` and `run()`) actually correct?**
  _`embedded_content()` has 144 INFERRED edges - model-reasoned connections that need verification._
- **What connects `stream_town_ffmpeg_bridge`, `StreamOperatorCamera`, `StreamOperatorLiveButton` to the rest of the system?**
  _307 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `control.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.04735289551311908 - nodes in this community are weakly interconnected._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.07544463568559955 - nodes in this community are weakly interconnected._