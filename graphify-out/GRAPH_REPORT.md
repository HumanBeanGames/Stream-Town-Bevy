# Graph Report - Stream-Town-Bevy  (2026-09-05)

## Corpus Check
- 672 files · ~1,854,428 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9839 nodes · 30521 edges · 327 communities (303 shown, 24 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1071 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a8fb26e6`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .new
- BuildingProcessor
- stream_town_migrate/src/content.rs
- SeasonProcessor
- .count
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- DebugProcessor
- BottomBarInterface
- STSM_StateAction
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- TownGoalProcessor
- OperatorChatRuntime
- Processors
- command.rs
- MonoBehaviour
- navigation.rs
- PlayerRoleData
- UnityGraphics
- simulation.rs
- Node_SO
- stream_town_migrate/src/presentation.rs
- Age
- SaveFileData
- Ui
- Res
- WorldGenProcessor
- PathBuf
- Vec4
- GenerationSettings
- World.Generation.Settings
- PlayerSettings
- SettingsData
- direct_broadcast.rs
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- SimulationError
- runtime_console.rs
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- encode_broadcast_session
- ResourceDataSaveData
- CommonEnums.cs
- AudioHandler
- StreamTownSessionBridge
- VfxAnimationController
- TwitchClientProcessor
- .new
- .SerializeComponent
- GameEvent
- BevyMigrationExporter
- ScriptableObject
- TechTreeEditorWindow
- Access_Dropdown
- stream_town_domain/src/content.rs
- CameraController
- Result
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- twitch.rs
- GamestateJukebox
- xtask/src/main.rs
- models.rs
- Tiler
- PlayerProcessor
- Utils
- roles_tab
- Targetable
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- WorldGenSaveData
- Access_Text
- Option
- GridNode
- legacy.rs
- CharacterModelHandler
- FoliageProcessor
- SnapToGridMouseMovement
- RaidEvent
- .Log
- BinaryReader
- convert_fbx_to_glb.py
- content_tab_contents
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- ObjectiveDef
- StateMachine
- GameEventProcessor
- ObjectSelectionProcessor.Editor.cs
- TwitchUser
- ResourceProcessor
- LoadingManager
- VFXArrowPointer
- Coordinator
- CustomLogHandler
- LevelHandler
- .Draw
- UpdateGraphBounds
- GlobalAudioController
- ScriptablesEditor
- world.rs
- WorldInstanceDeterminism
- tidal_music.rs
- .tick
- VfxSeagullSpawner
- UserInterface_TownVote
- TechTreeNode
- IProcessor.cs
- .randomized
- AIPath
- RoleSlot
- GateController
- stream_operator_chat_controls
- .new
- .StartupSequence
- DirectBroadcastRuntime
- UserInterface_RulerVote
- DayAndNightProcessor
- matching_role_animation_state
- ResourceHolder
- GridProcessor
- TargetSensor
- sync_stream_only_capture
- EnemyWeaponModel
- AnimationHandler
- What You Must Do When Invoked
- RuntimeData Template
- IRuntimeDataScriptable
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldSaveData
- xtask/src/lib.rs
- BuildingDamageMaterialHandler
- CommandDictionary
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- StableId
- SelectedPlayerGroup
- UnitHealthBar
- update_enemy_music_intensity
- Stream Town Reloaded - Architecture Documentation
- .InitializeAndActivateProcessorsAsync
- UserInterface_ObjectSelection
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- stream_town_tools/src/main.rs
- SelectedObject
- TransformSaveData
- UserInterface_DisplayUsernames
- PlayerRole
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- SimpleMusicController
- EditorHelpers
- FoliageGenerationSettings
- RenderAssets
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- SelectedEnemy
- ProjectCamera
- List
- VoteEvent
- BTreeMap
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- GUIDProcessor
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- EditorUtils
- UIElementWrapper
- WorldGenerationReferenceExporter
- PlayerInputProcessor
- RoleHandler
- UserInterface_BuildingHealthBar
- stream_town_migrate/src/menu_scene.rs
- Access_Toggle
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- CreditsProcessor
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- SelectedBuilding
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- STSM_HelperBase
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- drive_tidal_music
- Common Patterns
- StationProcessor
- Requirement
- .SetTargetType
- UI_TechOption
- TimeCycleConfig
- Access_GOList
- Key Rules
- BuildingModelHandler
- RuntimeData Template
- Character Animation Regression Checklist
- .walkable_neighbours_with
- ScriptKeywordProcessor
- FPSDisplay
- record_gpu_readiness
- Processor Template
- Common Patterns
- Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system.
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- bevy-port/README.md
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- EnemyModelHandler
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SelectableObject
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- parse_model_clip_events
- extraction-spec.md
- Season
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- DataStructures
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- KeepKingVote
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- TL_API
- Q: If there is more to do, keep going.
- EventProcessor
- Editor
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Easings
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- HealthModifier
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- UnitTravelToPosition
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch command reference
- InjectedCommands
- capture_stream_only_target
- DontDestroyOnLoad
- .DrawDataFieldAndLabel
- RotationHandler
- UIGameObjectAccessor
- PlayerSaveData
- CreateProjectScopeProcessors.cs
- RandomEnabler
- .ExportModification
- PlayerDeathHandler
- Access_TextInput
- ToolState
- BuildPlacerData
- NewKingVote
- OpenNode
- SeasonDataSettings
- StringUtils
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- complete_gameplay_scenario_round_trips
- PlayerInputRuntimeData
- tidal_plugin
- StatusBar
- SimpleScreenShot
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- stream_town_domain/src/lib.rs
- parse_transform_tracks
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- Projectile
- .RefreshSceneBindingsAndTryGenerate
- ScriptableObjectAssetData
- vcpkg.json
- ObjectiveSaveData
- Autosave
- SharedTypes.cs
- preview_grid_point
- FFmpeg runtime and relinking
- StreamTown.Migration

## God Nodes (most connected - your core abstractions)
1. `StableId` - 491 edges
2. `WorldSimulation` - 228 edges
3. `ContentCatalog` - 224 edges
4. `GridPos` - 189 edges
5. `Utils` - 159 edges
6. `Processors` - 156 edges
7. `ScriptablesProcessorInfrastructure` - 150 edges
8. `RenderAssets` - 144 edges
9. `Player` - 142 edges
10. `ToolState` - 141 edges

## Surprising Connections (you probably didn't know these)
- `handle_twitch_event()` --calls--> `parse_chat_commands()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `process_runtime_console()` --calls--> `parse_chat_commands()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `generate_world()` --calls--> `default_resource_generation_layers()`  [INFERRED]
  bevy-port/crates/stream_town_domain/src/world.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs

## Import Cycles
- None detected.

## Communities (327 total, 24 thin omitted)

### Community 0 - ".new"
Cohesion: 0.02
Nodes (254): AccessibilityActionRequest, generate_world(), generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean() (+246 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.03
Nodes (48): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, bool (+40 more)

### Community 2 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (131): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+123 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.13
Nodes (4): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 4 - ".count"
Cohesion: 0.03
Nodes (116): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), append_terrain_quad(), append_terrain_skirt(), asset_root_collection_ready() (+108 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (419): AnimationClip, AnimationTargetId, Default, SeasonalTerrainPalette, TerrainAppearanceConfig, accessibility_settings_selection(), accessibility_should_clear_focus(), AccessibilityActionDispatch (+411 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (27): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+19 more)

### Community 7 - "DebugProcessor"
Cohesion: 0.05
Nodes (21): AttackUnit, Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory (+13 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "STSM_StateAction"
Cohesion: 0.15
Nodes (6): int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (11): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+3 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.14
Nodes (22): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Option (+14 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (13): Action, float, Enemy, int, STSM_Helper_Attack, STSM_Action_Heal, Action, bool (+5 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "OperatorChatRuntime"
Cohesion: 0.18
Nodes (10): AccessibleNode, moderate_selected_operator_user(), CommandAcknowledgementRuntime, operator_chat_scroll_stays_anchored_when_new_messages_arrive(), OperatorChatBadges, OperatorChatLine, OperatorChatRuntime, poll_twitch_transport() (+2 more)

### Community 18 - "Processors"
Cohesion: 0.04
Nodes (13): GridSystem.Utils, TownGoal.Data, Processors, StreamTown.EditorTools, TownGoal, Reflex.Core, Data.Containers, MetaData (+5 more)

### Community 19 - "command.rs"
Cohesion: 0.15
Nodes (35): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+27 more)

### Community 20 - "MonoBehaviour"
Cohesion: 0.01
Nodes (133): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+125 more)

### Community 21 - "navigation.rs"
Cohesion: 0.20
Nodes (18): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavigationError, octile_distance_with_costs(), open_ground_paths_use_diagonal_steps() (+10 more)

### Community 22 - "PlayerRoleData"
Cohesion: 0.09
Nodes (11): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+3 more)

### Community 23 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 24 - "simulation.rs"
Cohesion: 0.10
Nodes (21): actor_appearances_are_seeded_varied_and_persisted(), authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+13 more)

### Community 25 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 26 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (75): AnimationParameterDef, animation_state_id(), animation_state_machine_id(), animation_take_name(), animator_component(), animator_reference_path(), array_index(), avatar_mask_id() (+67 more)

### Community 27 - "Age"
Cohesion: 0.07
Nodes (24): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+16 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "Ui"
Cohesion: 0.10
Nodes (62): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+54 more)

### Community 30 - "Res"
Cohesion: 0.02
Nodes (395): AccessibilityFocusVisualQuery, Added, AmbientLight, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, accessibility_button_enabled() (+387 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "PathBuf"
Cohesion: 0.07
Nodes (35): AnyResult, automatic_resume_save_path(), AutomaticResumeRuntime, confirm_gameplay_ready(), is_jump_start_path(), jump_start_saves_are_identifiable_and_never_reuse_an_existing_path(), jump_start_snapshot_path(), jump_start_working_path() (+27 more)

### Community 33 - "Vec4"
Cohesion: 0.06
Nodes (33): AccessibilityMotionDefaults, BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension (+25 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "World.Generation.Settings"
Cohesion: 0.04
Nodes (35): Transform, PlayerSpawnPoint, float, GameObject, SimpleDisableAfterTime, CampGenerationSettings, List, CampGenerationSettingsContainer (+27 more)

### Community 36 - "PlayerSettings"
Cohesion: 0.07
Nodes (46): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+38 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "direct_broadcast.rs"
Cohesion: 0.06
Nodes (41): AtomicUsize, amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), configure_amf_quality() (+33 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.05
Nodes (19): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+11 more)

### Community 42 - "SimulationError"
Cohesion: 0.19
Nodes (4): Result, SimulationError, unattended_respawn_halves_every_role_level_but_role_revive_does_not(), validate_trade_resource()

### Community 43 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "UIProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Container, ContainerBuilder, TimeProcessor (+8 more)

### Community 47 - "encode_broadcast_session"
Cohesion: 0.08
Nodes (37): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, BroadcastMetrics, BroadcastTarget, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), discard_pending_audio() (+29 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (14): List, UnityEvent, StationSensor, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+6 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - ".new"
Cohesion: 0.08
Nodes (32): BroadcastMetricsSnapshot, closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), DirectTwitchBroadcastPlugin, ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once() (+24 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 58 - "ScriptableObject"
Cohesion: 0.02
Nodes (90): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+82 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Access_Dropdown"
Cohesion: 0.09
Nodes (12): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, ContainerBuilder, TMP_Dropdown (+4 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.06
Nodes (62): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingModelDef, default_resource_generation_layers() (+54 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Result"
Cohesion: 0.10
Nodes (54): AnimationFloatKeyframe, AnimationPropertyCurve, AnimationTangent, convert_chimney_smoke(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id() (+46 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.14
Nodes (11): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, ContainerBuilder, ColorAdjustments (+3 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (66): BTreeSet, TwitchConfig, secrets_restart_requirements(), bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault (+58 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "xtask/src/main.rs"
Cohesion: 0.06
Nodes (91): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+83 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "PlayerProcessor"
Cohesion: 0.07
Nodes (15): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+7 more)

### Community 74 - "Utils"
Cohesion: 0.04
Nodes (38): BuildCostModifier, InputButton, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Utils, Pets.Enumerations (+30 more)

### Community 75 - "roles_tab"
Cohesion: 0.15
Nodes (26): ability_choices(), action_animation_choices(), building_model_node_choices(), buildings_tab(), cached_gltf_metadata(), default_navigation_footprint_thirds(), delete_selected_building(), duplicate_selected_building() (+18 more)

### Community 76 - "Targetable"
Cohesion: 0.05
Nodes (22): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+14 more)

### Community 77 - "Objective"
Cohesion: 0.08
Nodes (13): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+5 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.05
Nodes (72): ContentError, Result, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id() (+64 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 83 - "WorldGenSaveData"
Cohesion: 0.09
Nodes (15): Mesh, Vector3, bool, int, MeshSaveData, List, SaveGameData, float (+7 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Option"
Cohesion: 0.09
Nodes (44): AtomicBool, AudioFrame, AudioInput, BroadcastController, capture_process_audio(), GpuStreamCaptureRing, GpuStreamCaptureShared, GpuStreamCaptureSlot (+36 more)

### Community 86 - "GridNode"
Cohesion: 0.10
Nodes (12): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+4 more)

### Community 87 - "legacy.rs"
Cohesion: 0.07
Nodes (86): ActorCustomization, StreamUserType, should_show_actor_name(), absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer() (+78 more)

### Community 88 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - ".Log"
Cohesion: 0.05
Nodes (37): Container, ContainerBuilder, GameStateProcessor, Action, bool, BoxCollider, CancellationToken, Container (+29 more)

### Community 93 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "content_tab_contents"
Cohesion: 0.25
Nodes (11): add_archetype_scene(), archetype_kind_choice(), content_tab_contents(), create_model_archetype(), delete_model_archetype(), draw_footprint_grid(), footprint_editor(), model_archetype_and_variant_lifecycle_remains_valid() (+3 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (48): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+40 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (19): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+11 more)

### Community 98 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (67): AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef, AnimationTransformTrack, AnimationVec3Keyframe (+59 more)

### Community 100 - "ObjectiveDef"
Cohesion: 0.15
Nodes (13): ObjectiveDef, ObjectiveKind, common_gathered_resource(), is_gold_collect_objective(), objective_increment(), ObjectiveEvent, ObjectiveProgress, BTreeMap (+5 more)

### Community 101 - "StateMachine"
Cohesion: 0.12
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 104 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.08
Nodes (15): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+7 more)

### Community 107 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 108 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - ".Draw"
Cohesion: 0.08
Nodes (25): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Port, VisualElement, Button, EnumField (+17 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "world.rs"
Cohesion: 0.06
Nodes (68): WorldGenConfig, ResourceGenerationHabitat, ResourceGenerationLayerDef, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash() (+60 more)

### Community 116 - "WorldInstanceDeterminism"
Cohesion: 0.30
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 117 - "tidal_music.rs"
Cohesion: 0.15
Nodes (26): AdaptiveMusicConfig, adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine() (+18 more)

### Community 118 - ".tick"
Cohesion: 0.25
Nodes (5): numbered_technology_ballot_waits_for_a_vote_and_resolves_the_winner(), ruler_and_technology_ballots_run_and_accept_votes_concurrently(), ruler_vote_rejects_duplicates_and_invalid_candidates(), scheduled_ruler_elections_pause_resolve_and_restore_roles(), trade_spending_follows_gold_votes_and_active_objectives()

### Community 119 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 120 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 121 - "TechTreeNode"
Cohesion: 0.05
Nodes (33): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+25 more)

### Community 122 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 123 - ".randomized"
Cohesion: 0.19
Nodes (8): actor_appearance_hash(), deterministic_weather(), legacy_simulation_calendar_upgrades_without_advancing_timers(), Self, schema_five_randomizes_legacy_appearances_but_preserves_player_colors(), Season, Weather, zero_skin_color_preserves_pre_schema_five_canonical_ron()

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 126 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "stream_operator_chat_controls"
Cohesion: 0.07
Nodes (37): bounded_history_f32(), operator_chat_scroll_rows(), operator_window_close_requests_exit(), Changed, Interaction, MessageReader, MouseWheel, Node (+29 more)

### Community 129 - ".new"
Cohesion: 0.19
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 130 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 131 - "DirectBroadcastRuntime"
Cohesion: 0.07
Nodes (46): apply_direct_broadcast_control(), AuthorizationEvent, AutomaticBroadcastStart, bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastPrerequisites, BroadcastStopDisposition, CadenceTick (+38 more)

### Community 132 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 133 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 134 - "matching_role_animation_state"
Cohesion: 0.24
Nodes (11): debug_fingerprint(), default_role_preview_animation(), matching_role_animation_state(), player_animation_controller(), role_preview_animation_request(), role_preview_uses_shipping_rig_animation_and_composition_rules(), role_preview_visible_nodes(), Debug (+3 more)

### Community 135 - "ResourceHolder"
Cohesion: 0.09
Nodes (17): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent (+9 more)

### Community 136 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 137 - "TargetSensor"
Cohesion: 0.08
Nodes (9): ProjectileShooter, float, int, string, SensorBase, bool, float, UnityEvent (+1 more)

### Community 138 - "sync_stream_only_capture"
Cohesion: 0.10
Nodes (22): camera_targets_primary_window(), NativeGameAudioRouting, pcm16_wav_clip(), pcm16_wav_data(), Assets, Commands, Entity, Handle (+14 more)

### Community 139 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

### Community 140 - "AnimationHandler"
Cohesion: 0.17
Nodes (7): AnimationHandler, Animator, bool, Dictionary, float, int, AnimationName

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "IRuntimeDataScriptable"
Cohesion: 0.04
Nodes (36): float, int, Material, AllSeasonSettings, Container, ContainerBuilder, TargetProcessor, Container (+28 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 147 - "WorldSaveData"
Cohesion: 0.17
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 150 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "StableId"
Cohesion: 0.02
Nodes (373): GameConfig, GameplayConfig, BTreeMap, BuildingDef, ContentCatalog, EnemyCampGenerationDef, PassiveResourceContribution, RoleDef (+365 more)

### Community 154 - "SelectedPlayerGroup"
Cohesion: 0.17
Nodes (3): List, List, SelectedPlayerGroup

### Community 155 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 156 - "update_enemy_music_intensity"
Cohesion: 0.18
Nodes (13): intensity_smoothing_has_a_five_second_time_constant(), point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, Res, Time (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.17
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 159 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "stream_town_tools/src/main.rs"
Cohesion: 0.05
Nodes (82): apply_building_draft(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authoring_config_save_bar(), authority_tab(), broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record() (+74 more)

### Community 163 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

### Community 164 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "PlayerRole"
Cohesion: 0.05
Nodes (30): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+22 more)

### Community 167 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 170 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 171 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 172 - "FoliageGenerationSettings"
Cohesion: 0.20
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 173 - "RenderAssets"
Cohesion: 0.03
Nodes (194): MainMenuModelInstance, MainMenuSceneReference, Option, String, PresentationCatalog, actor_material(), advance_falling_fish(), animate_chimney_smoke_particles() (+186 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 176 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 177 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 178 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 179 - "BTreeMap"
Cohesion: 0.10
Nodes (52): AnimationClipDef, MaterialDef, PrefabPresentationBinding, assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks(), convert_clips() (+44 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 185 - "UIElementWrapper"
Cohesion: 0.25
Nodes (5): GameObject, List, PresetButtons, ContainerBuilder, UIElementWrapper

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 189 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 190 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (42): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuResourceVisual, Vec, adjacent_farm_tiles_share_one_flat_plateau(), adjacent_foundations_sample_the_unmodified_generated_surface() (+34 more)

### Community 192 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Current attempts, Do not retry unchanged, Next narrow diagnostic pass, Tree and Foliage Flicker Regression Checklist, What did not fix the flicker, What did work

### Community 200 - "Bevy Migration Status"
Cohesion: 0.40
Nodes (5): Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity, Validation

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "drive_tidal_music"
Cohesion: 0.26
Nodes (15): AdaptiveMusicSignature, drive_tidal_music(), intensity_program_needs_update(), player_music_gain(), report_once(), NativeAudioRouting, Option, ResMut (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "StationProcessor"
Cohesion: 0.06
Nodes (24): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, Container, ContainerBuilder (+16 more)

### Community 209 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 210 - ".SetTargetType"
Cohesion: 0.21
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 211 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 212 - "TimeCycleConfig"
Cohesion: 0.29
Nodes (3): Self, TimeCycleConfig, night_light_is_active()

### Community 213 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - ".walkable_neighbours_with"
Cohesion: 0.39
Nodes (4): neighbour_candidates(), offset(), Option, Fn

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), GpuImage, GpuRenderAssets, ErasedRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system., Source Nodes

### Community 225 - "Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?, Source Nodes

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed, Source Nodes

### Community 229 - "bevy-port/README.md"
Cohesion: 0.15
Nodes (6): Audio provenance, Binaries, Commands, Stream Town Bevy, Original project notes, Stream Town: Bevy Migration

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

### Community 234 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

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

### Community 243 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

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

### Community 251 - "parse_model_clip_events"
Cohesion: 0.31
Nodes (9): AnimationEventDef, AnimationObjectReference, inline_mapping_value(), parse_animation_events(), parse_model_clip_events(), parse_object_reference(), parses_normalized_animation_events_from_model_importer_clips(), parses_property_curves_and_animation_events_without_unity_types() (+1 more)

### Community 253 - "Season"
Cohesion: 0.32
Nodes (5): bool, float, int, SeasonRuntimeData, Season

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "DataStructures"
Cohesion: 0.25
Nodes (4): int, ChangeTimeStamp, DataStructures, DateTime

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

### Community 267 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 268 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

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

### Community 276 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

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

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.07
Nodes (55): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+47 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.12
Nodes (16): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+8 more)

### Community 287 - "InjectedCommands"
Cohesion: 0.25
Nodes (8): AgentCommandQueue, BuildingCommandQueue, BuildingRuntimeCommand, CameraCommandQueue, inject_environment_commands(), InjectedCommands, queued_camera_command_executes_without_legacy_idle_gate(), RuntimeCommandQueues

### Community 288 - "capture_stream_only_target"
Cohesion: 0.29
Nodes (8): capture_stream_only_target(), configure_stream_capture_ring(), copy_gpu_rows_into(), GpuImage, GpuRenderAssets, stream_readback_due(), RenderDevice, RenderQueue

### Community 290 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 291 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 292 - "UIGameObjectAccessor"
Cohesion: 0.29
Nodes (3): bool, UIGameObjectAccessor, ConnectionTab

### Community 293 - "PlayerSaveData"
Cohesion: 0.09
Nodes (18): Component, Dictionary, Transform, bool, int, List, string, InventoryEntrySaveData (+10 more)

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 297 - "PlayerDeathHandler"
Cohesion: 0.29
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 298 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (78): apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot (+70 more)

### Community 300 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 301 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 302 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Ord, Ordering, PartialOrd, Self

### Community 303 - "SeasonDataSettings"
Cohesion: 0.29
Nodes (6): Color, float, int, VisualEffect, SeasonDataSettings, Gradient

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (11): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+3 more)

### Community 307 - "complete_gameplay_scenario_round_trips"
Cohesion: 0.33
Nodes (4): capped_role_progression_discards_excess_at_the_requested_level(), complete_gameplay_scenario_round_trips(), role_progression_matches_unity_curve_and_carries_excess(), technology_vote_starts_persistent_goal_and_unlocks_after_all_objectives()

### Community 308 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 309 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 310 - "StatusBar"
Cohesion: 0.33
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 312 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "stream_town_domain/src/lib.rs"
Cohesion: 0.40
Nodes (5): is_transient_surface_configuration_error(), stream_town_render_error_handler(), RenderError, RenderErrorPolicy, World

### Community 315 - "parse_transform_tracks"
Cohesion: 0.33
Nodes (6): append_vec3_keys(), parse_inline_array(), parse_transform_tracks(), parses_unity_transform_curves_without_editor_types(), Item, Iterator

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 319 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 321 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 324 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

## Knowledge Gaps
- **395 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+390 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=3.403392777) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=2.430388761) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.22348788) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=1.859553563)
- `WorldSnapshot` (3× useful, score=1.77633782)
- `SkinnedMesh` (2× useful, score=1.499416607)
- `drive_tidal_music()` (2× useful, score=1.476128636)
- `WorldSimulation` (2× useful, score=1.289448861)
- `load_input()` (2× useful, score=1.18405319) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.161931164)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StableId` connect `StableId` to `.new`, `.new`, `stream_town_migrate/src/content.rs`, `.count`, `stream_town_game/src/lib.rs`, `matching_role_animation_state`, `config.rs`, `command.rs`, `simulation.rs`, `stream_town_migrate/src/presentation.rs`, `Ui`, `Res`, `InjectedCommands`, `stream_town_tools/src/main.rs`, `SimulationError`, `runtime_console.rs`, `AnimationControllerDef`, `RenderAssets`, `ToolState`, `BTreeMap`, `complete_gameplay_scenario_round_trips`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `Result`, `twitch.rs`, `xtask/src/main.rs`, `roles_tab`, `technology_graph.rs`, `legacy.rs`, `content_tab_contents`, `stream_town_domain/src/presentation.rs`, `ObjectiveDef`, `world.rs`, `.tick`, `.randomized`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `Utils` connect `Utils` to `DataStructures`, `DayAndNightProcessor`, `IRuntimeDataScriptable`, `Easings`, `Processors`, `MonoBehaviour`, `Age`, `.DrawDataFieldAndLabel`, `World.Generation.Settings`, `GenerationSettings`, `RandomEnabler`, `UIProcessor`, `StringUtils`, `CommonEnums.cs`, `SimpleScreenShot`, `ScriptableObject`, `BuildingPlacer`, `StationProcessor`, `SnapToGridMouseMovement`, `FPSDisplay`, `Resource`, `.Draw`, `UpdateGraphBounds`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `DayAndNightProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `MonoBehaviour`, `PlayerRoleData`, `SelectedPlayerGroup`, `WorldGenProcessor`, `PlayerRole`, `TechTreeProcessor`, `NewKingVote`, `UIProcessor`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `GUIDProcessor`, `RoleHandler`, `Utils`, `BuildingPlacer`, `RaidEvent`, `.Log`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `UserInterface_TownVote`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _395 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `.new` be split into smaller, more focused modules?**
  _Cohesion score 0.017722519455252918 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.029161290322580646 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06843559977888336 - nodes in this community are weakly interconnected._