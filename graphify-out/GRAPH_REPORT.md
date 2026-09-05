# Graph Report - Stream-Town-Bevy  (2026-09-05)

## Corpus Check
- 677 files · ~1,856,495 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9871 nodes · 30857 edges · 316 communities (298 shown, 18 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 1072 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4850ecb3`
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
- STSM_GoToLocation
- BottomBarInterface
- Res
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- TownGoalProcessor
- ContentCatalog
- MonoBehaviour
- command.rs
- RenderAssets
- NavGrid
- String
- UnityGraphics
- StableId
- Node_SO
- GameConfig
- TechTree.Elements
- SaveFileData
- Ui
- ResMut
- WorldGenProcessor
- Option
- OperatorChatRuntime
- GenerationSettings
- DayAndNightProcessor
- PlayerSettings
- SettingsData
- direct_broadcast.rs
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- build_converted_animation
- stream_operator_chat_controls
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- encode_broadcast_session
- ResourceDataSaveData
- runtime_console.rs
- AudioHandler
- StreamTownSessionBridge
- TimeProcessor
- TwitchClientProcessor
- .new
- .SerializeComponent
- stream_town_domain/src/content.rs
- BevyMigrationExporter
- IInstaller
- TechTreeEditorWindow
- PlayerProcessor
- World.Generation
- CameraController
- stream_town_migrate/src/presentation.rs
- Access_Slider
- GraphicsProcessor
- SelectedBuilding
- Pet
- twitch.rs
- GamestateJukebox
- xtask/src/main.rs
- models.rs
- Tiler
- StationSensor
- Utils
- String
- Targetable
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Instant
- TerrainAppearanceConfig
- Access_Text
- Option
- GridProcessor
- legacy.rs
- CharacterModelHandler
- FoliageProcessor
- SnapToGridMouseMovement
- GameEvent
- ObjectPoolingProcessor
- .Log
- convert_fbx_to_glb.py
- RoleHandler
- Resource
- SaveProcessor
- UserInterface_GameMenu
- PresentationCatalog
- ResourceRuntimeData
- StateMachine
- GameEventProcessor
- ObjectSelectionProcessor.Editor.cs
- TwitchUser
- ResourceProcessor
- LoadingManager
- TargetProcessor
- SelectedPlayer
- CustomLogHandler
- BinaryReader
- .CreateEnumField
- UpdateGraphBounds
- .SetTargetType
- ScriptablesEditor
- world.rs
- UserInterface_RulerVote
- tidal_music.rs
- UserInterface_BuildingHealthBar
- VfxSeagullSpawner
- UserInterface_TownVote
- TechTreeGraphView
- stream_town_tools/src/main.rs
- .RenderResourceType
- AIPath
- twitch_tab
- GateController
- .from
- .new
- Coordinator
- DirectBroadcastRuntime
- SelectedObject
- EnemySpawner
- SelectableObject
- EditorUtils
- TechnologyGraphLayout
- TargetSensor
- sync_stream_only_capture
- Station
- IRuntimeDataScriptable
- What You Must Do When Invoked
- RuntimeData Template
- GlobalAudioController
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldSaveData
- xtask/src/lib.rs
- .DrawDataFieldAndLabel
- WorldGenRuntimeData
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- UserInterface.MainMenu
- SerializableDictionary
- UnitHealthBar
- update_enemy_music_intensity
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- TechNodeData
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- SelectedEnemy
- UserInterface_ObjectSelection
- TransformSaveData
- UserInterface_DisplayUsernames
- RoleProcessor
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- .Draw
- EditorHelpers
- EnemyModelHandler
- Vec4
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- GUIDComponent
- Access_Dropdown
- List
- VoteEvent
- String
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy.
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Goal
- Access_GOList
- WorldGenerationReferenceExporter
- PlayerInputProcessor
- CommandDictionary
- UserInterface_Roles
- stream_town_migrate/src/menu_scene.rs
- Access_Toggle
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- VfxAnimationController
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- CommonEnums.cs
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- TargetMask
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- drive_tidal_music
- Common Patterns
- LabelDisplayProcessor
- Requirement
- TidalPcmMix
- RoleDataContainer
- stream_town_domain/src/lib.rs
- HealthModifier
- Key Rules
- BuildingModelHandler
- RuntimeData Template
- Character Animation Regression Checklist
- StringUtils
- ScriptKeywordProcessor
- FPSDisplay
- CreateProjectScopeProcessors.cs
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
- AnimationHandler
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- .write
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- allocate_fifo_output
- extraction-spec.md
- PlacementProbeHandler
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- SaveState
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- ScriptableObjectAssetData
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- draw_world_preview
- Q: If there is more to do, keep going.
- RotationHandler
- Editor
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Easings
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- ResourceHolder
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- WeatherProcessor
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch command reference
- SimpleMusicController
- UIElementWrapper
- AdaptiveMusicConfig
- DataStructures
- TechTreeNode
- .RestoreObjectiveProgress
- PlayerSaveData
- ScriptablesProcessorInfrastructure
- record_gpu_readiness
- .ExportModification
- DontDestroyOnLoad
- RandomEnabler
- ToolState
- Access_TextInput
- healing_burst_effect
- color_grading_for_state
- Q: Why did live capture FPS fall below 28 during the night transition?
- BuildCostModifier.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- GpuStreamCaptureCounters
- tidal_plugin
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- FoliageGenerationSettings
- vcpkg.json
- FFmpeg runtime and relinking
- StreamTown.Migration

## God Nodes (most connected - your core abstractions)
1. `StableId` - 492 edges
2. `WorldSimulation` - 228 edges
3. `ContentCatalog` - 225 edges
4. `GridPos` - 190 edges
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

## Communities (316 total, 18 thin omitted)

### Community 0 - ".new"
Cohesion: 0.03
Nodes (223): ArchetypeKind, generate_world(), generate_world_with_content(), accessibility_navigation_preserves_editable_text_focus(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_id_by_source() (+215 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.03
Nodes (35): BuildingBase, bool, float, int, List, UnityEvent, bool, Dictionary (+27 more)

### Community 2 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (131): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+123 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.07
Nodes (18): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+10 more)

### Community 4 - ".count"
Cohesion: 0.03
Nodes (99): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), authored_rotating_node_names(), begin_world_loading() (+91 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (298): accessibility_motion_preferences_preserve_authored_parameters(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_settings_selection(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, action_ranges_and_tower_acquisition_are_euclidean() (+290 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (28): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+20 more)

### Community 7 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (15): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+7 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Res"
Cohesion: 0.04
Nodes (277): Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, actor_material(), ActorNameOverlay (+269 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.06
Nodes (13): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+5 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.15
Nodes (21): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Option (+13 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (20): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+12 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (16): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy (+8 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "ContentCatalog"
Cohesion: 0.06
Nodes (131): ContentCatalog, DirtyRegion, GridPos, ActorState, String, GeneratedResource, GeneratedWorld, action_animation_speed() (+123 more)

### Community 18 - "MonoBehaviour"
Cohesion: 0.01
Nodes (99): Api, PersistentScoped, ContainerBuilder, Volume, PostProcessingInstaller, AudioMixerInstaller, AudioMixer, ContainerBuilder (+91 more)

### Community 19 - "command.rs"
Cohesion: 0.14
Nodes (37): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+29 more)

### Community 20 - "RenderAssets"
Cohesion: 0.05
Nodes (96): apply_authored_main_menu_camera(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_scene_rotation(), authored_ui_image(), authored_ui_image_with_corner_scale(), building_effect_material(), combat_material() (+88 more)

### Community 21 - "NavGrid"
Cohesion: 0.11
Nodes (28): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavGrid, NavigationError, neighbour_candidates() (+20 more)

### Community 22 - "String"
Cohesion: 0.05
Nodes (81): StreamUserType, active_event_text(), actor_role_level_cap(), animation_playback_for_selection(), building_blocks_navigation(), building_command_name(), building_construction_cost(), building_cost_reduction_percent() (+73 more)

### Community 23 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 24 - "StableId"
Cohesion: 0.04
Nodes (91): ObjectiveDef, ObjectiveKind, Display, FromStr, StableId, actor_appearance_hash(), actor_appearances_are_seeded_varied_and_persisted(), ActorCustomization (+83 more)

### Community 25 - "Node_SO"
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 26 - "GameConfig"
Cohesion: 0.07
Nodes (65): GameConfig, GameplayConfig, BTreeMap, StationDef, GeneratedFoliage, active_station_ids(), actor_idle_anchor(), append_world_diagnostic_quad() (+57 more)

### Community 27 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Ui"
Cohesion: 0.10
Nodes (65): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+57 more)

### Community 30 - "ResMut"
Cohesion: 0.03
Nodes (186): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AnyResult, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibilityAnnouncement (+178 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "Option"
Cohesion: 0.02
Nodes (153): AmbientLight, ActiveMaterialHandles, animate_loading_icon(), animate_main_menu_clouds(), apply_loading_icon_rotation(), authored_post_process_stack(), authored_rgb_filter(), AutomaticResumeRuntime (+145 more)

### Community 33 - "OperatorChatRuntime"
Cohesion: 0.19
Nodes (9): AccessibleNode, moderate_selected_operator_user(), send_operator_chat_message(), operator_chat_scroll_stays_anchored_when_new_messages_arrive(), OperatorChatBadges, OperatorChatLine, OperatorChatRuntime, Into (+1 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 36 - "PlayerSettings"
Cohesion: 0.12
Nodes (28): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+20 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "direct_broadcast.rs"
Cohesion: 0.05
Nodes (45): append_direct_broadcast_diagnostic_to(), AuthorizationEvent, bandwidth_test_url_is_constructed_without_logging_the_key(), bounded_history_f32(), BroadcastPrerequisites, build_ingest_url(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), draw_centered_label() (+37 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.16
Nodes (8): Action, CancellationToken, int, UTF8Encoding, BinarySaveCodec, int, PlayerRoleSaveData, BinaryWriter

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (8): NodeUnlockData, Action, Container, ContainerBuilder, EventType, IEnumerable, List, TechTreeProcessor

### Community 42 - "build_converted_animation"
Cohesion: 0.07
Nodes (45): AnimationClip, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve(), AnimatedCharacterShadowReceiver (+37 more)

### Community 43 - "stream_operator_chat_controls"
Cohesion: 0.07
Nodes (33): BroadcastMetricsSnapshot, DirectBroadcastPhase, operator_chat_scroll_rows(), operator_live_button_label(), operator_window_close_requests_exit(), prepared_broadcast_can_start(), Changed, Interaction (+25 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+18 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.13
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 47 - "encode_broadcast_session"
Cohesion: 0.08
Nodes (43): BroadcastConfig, BroadcastEncoderPreference, amf_quality_profile_keeps_static_grid_detail_between_keyframes(), AudioFrame, AudioInput, BroadcastEncoder, BroadcastMetrics, BroadcastTarget (+35 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (15): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+7 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 52 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - ".new"
Cohesion: 0.09
Nodes (31): bandwidth_test_never_claims_to_be_publicly_live(), broadcast_output_options(), closing_the_operator_window_requests_a_graceful_game_exit(), configure_direct_broadcast(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), DirectTwitchBroadcastPlugin, ending_stream_returns_the_operator_to_main_menu_after_shutdown() (+23 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "stream_town_domain/src/content.rs"
Cohesion: 0.07
Nodes (60): ArchetypeBounds, ArchetypeDef, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+52 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 58 - "IInstaller"
Cohesion: 0.01
Nodes (134): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, Camera, GameObject, ProjectCameraInstaller, ContainerBuilder (+126 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "PlayerProcessor"
Cohesion: 0.08
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 61 - "World.Generation"
Cohesion: 0.05
Nodes (31): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller (+23 more)

### Community 62 - "CameraController"
Cohesion: 0.11
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.05
Nodes (123): animation_state_id(), animation_state_machine_id(), animation_take_name(), append_vec3_keys(), avatar_mask_id(), clip_id(), color_value(), controller_id() (+115 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.14
Nodes (11): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, ContainerBuilder, ColorAdjustments (+3 more)

### Community 66 - "SelectedBuilding"
Cohesion: 0.09
Nodes (6): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler, SelectedBuilding

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (66): BTreeSet, TwitchConfig, SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault (+58 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "xtask/src/main.rs"
Cohesion: 0.05
Nodes (92): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+84 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "StationSensor"
Cohesion: 0.10
Nodes (9): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor, UnityEvent (+1 more)

### Community 74 - "Utils"
Cohesion: 0.03
Nodes (46): InputButton, SharedTypes, InputButton, STStateMachine.States, PlayerControls.ObjectSelection, Units, TownGoal.Data, Utils (+38 more)

### Community 75 - "String"
Cohesion: 0.12
Nodes (39): ability_choices(), action_animation_choices(), apply_role_draft(), building_model_node_choices(), cached_gltf_metadata(), delete_selected_role(), discover_model_assets(), discover_texture_assets() (+31 more)

### Community 76 - "Targetable"
Cohesion: 0.11
Nodes (7): bool, BoxCollider, float, int, Transform, Vector3, Targetable

### Community 77 - "Objective"
Cohesion: 0.09
Nodes (10): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.12
Nodes (38): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+30 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.27
Nodes (5): GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Instant"
Cohesion: 0.13
Nodes (13): BroadcastStopDisposition, CadenceTick, duration_as_micros(), Duration, Error, Instant, stream_readback_due(), twitch_live_request_timeout() (+5 more)

### Community 83 - "TerrainAppearanceConfig"
Cohesion: 0.28
Nodes (10): TerrainAppearanceConfig, foliage_should_be_hidden(), record_completed_cell_traversal(), traversal_score_for_rate(), traversal_wear_byte(), traversal_wear_fades_from_five_to_fifty_and_decays_by_half_life(), traversal_wear_fraction(), traversal_wear_records_only_completed_citizen_cell_entries() (+2 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Option"
Cohesion: 0.10
Nodes (39): AtomicBool, BroadcastController, capture_stream_only_target(), configure_stream_capture_ring(), copy_gpu_rows_into(), DirectBroadcastSnapshot, GpuStreamCaptureRing, GpuStreamCaptureShared (+31 more)

### Community 86 - "GridProcessor"
Cohesion: 0.06
Nodes (22): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, GridProcessorEditor, int (+14 more)

### Community 87 - "legacy.rs"
Cohesion: 0.07
Nodes (85): absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, BinaryParser<'a>, checked_in_schema_one_fixture_imports_retained_terrain() (+77 more)

### Community 88 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "GameEvent"
Cohesion: 0.05
Nodes (25): SortBuildingByLowerLevel, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+17 more)

### Community 92 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (38): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+30 more)

### Community 93 - ".Log"
Cohesion: 0.05
Nodes (21): Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, Container, ContainerBuilder, GameStateProcessor (+13 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "RoleHandler"
Cohesion: 0.05
Nodes (33): PlayerRoleData, AudioClip, bool, float, int, RoleData, AudioClip, bool (+25 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (41): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+33 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (20): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+12 more)

### Community 98 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 99 - "PresentationCatalog"
Cohesion: 0.05
Nodes (88): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef (+80 more)

### Community 100 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 101 - "StateMachine"
Cohesion: 0.07
Nodes (13): int, STSM_Helper_Attack, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "GameEventProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 104 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.08
Nodes (24): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+16 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "TargetProcessor"
Cohesion: 0.24
Nodes (4): Container, ContainerBuilder, List, TargetProcessor

### Community 109 - "CustomLogHandler"
Cohesion: 0.13
Nodes (11): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Dictionary, DebugSettings (+3 more)

### Community 110 - "BinaryReader"
Cohesion: 0.14
Nodes (3): Func, List, BinaryReader

### Community 111 - ".CreateEnumField"
Cohesion: 0.13
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - ".SetTargetType"
Cohesion: 0.18
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "world.rs"
Cohesion: 0.06
Nodes (66): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), cell_hash(), changing_seed_changes_world_hash() (+58 more)

### Community 116 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.14
Nodes (26): adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine(), format_number() (+18 more)

### Community 118 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 119 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 120 - "UserInterface_TownVote"
Cohesion: 0.14
Nodes (9): bool, Button, GameObject, List, Slider, TextMeshProUGUI, Transform, UnityAction (+1 more)

### Community 121 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 122 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (76): apply_building_draft(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authoring_config_save_bar(), authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft (+68 more)

### Community 123 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 124 - "AIPath"
Cohesion: 0.11
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 126 - "twitch_tab"
Cohesion: 0.20
Nodes (16): broadcast_encoder_label(), Duration, Sender, start_twitch_authorization(), start_twitch_broadcast_authorization(), start_twitch_broadcast_clear(), start_twitch_broadcast_diagnostic(), start_twitch_clear() (+8 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - ".from"
Cohesion: 0.04
Nodes (99): Default, TimeCycleConfig, ActorKind, RoleProgress, Default, actor_movement_speed(), actor_movement_speed_on_path(), agent_path() (+91 more)

### Community 129 - ".new"
Cohesion: 0.19
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "DirectBroadcastRuntime"
Cohesion: 0.08
Nodes (36): append_direct_broadcast_diagnostic(), apply_direct_broadcast_control(), AutomaticBroadcastStart, average_milliseconds(), begin_twitch_live_verification(), capture_direct_broadcast_frame(), direct_broadcast_log_path(), DirectBroadcastControl (+28 more)

### Community 132 - "SelectedObject"
Cohesion: 0.08
Nodes (7): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup, SelectedResource

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 135 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 136 - "TechnologyGraphLayout"
Cohesion: 0.12
Nodes (29): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), Default, Result (+21 more)

### Community 137 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 138 - "sync_stream_only_capture"
Cohesion: 0.11
Nodes (22): camera_targets_primary_window(), NativeGameAudioRouting, pcm16_wav_clip(), pcm16_wav_data(), Assets, Commands, Entity, Handle (+14 more)

### Community 139 - "Station"
Cohesion: 0.08
Nodes (15): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+7 more)

### Community 140 - "IRuntimeDataScriptable"
Cohesion: 0.07
Nodes (18): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, Dictionary, List (+10 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

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
Cohesion: 0.10
Nodes (21): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+13 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 150 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "UserInterface.MainMenu"
Cohesion: 0.10
Nodes (11): CameraProcessor, ContainerBuilder, MetaDataInstaller, Image, TextMeshProUGUI, StatusBar, SettingsInstaller, UserInterface.MainMenu (+3 more)

### Community 154 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 155 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 156 - "update_enemy_music_intensity"
Cohesion: 0.20
Nodes (12): intensity_smoothing_has_a_five_second_time_constant(), point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, Time, Vec2 (+4 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.09
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 159 - "TechNodeData"
Cohesion: 0.15
Nodes (10): List, Node_SO, TechNodeData, Button, GameObject, Image, Slider, TextMeshProUGUI (+2 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 163 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 164 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "RoleProcessor"
Cohesion: 0.06
Nodes (14): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+6 more)

### Community 167 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 170 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 171 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 172 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 173 - "Vec4"
Cohesion: 0.05
Nodes (37): AccessibilityMotionDefaults, BoundsMaterialExtension, BoundsMaterialUniform, building_material_cycle_stays_phase_aligned_without_cpu_asset_updates(), building_material_time_cycle(), BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension (+29 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 176 - "Access_Dropdown"
Cohesion: 0.06
Nodes (19): Camera, Quaternion, Vector3, ProjectCamera, Access_AADropdown, Access_AODropdown, Access_AutosaveTimerDropdown, Access_CameraAADropdown (+11 more)

### Community 177 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 178 - "VoteEvent"
Cohesion: 0.11
Nodes (14): int, List, NewKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float (+6 more)

### Community 179 - "String"
Cohesion: 0.09
Nodes (63): animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_controllers(), convert_embedded_model_clips() (+55 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy."
Cohesion: 0.50
Nodes (3): Answer, Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy., Source Nodes

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "Goal"
Cohesion: 0.15
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 185 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "PlayerInputProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor, bool, Dictionary, InputButton (+3 more)

### Community 189 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 190 - "UserInterface_Roles"
Cohesion: 0.14
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "VfxAnimationController"
Cohesion: 0.13
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "CommonEnums.cs"
Cohesion: 0.12
Nodes (11): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, ReviveType, RunAnimation, Seasons (+3 more)

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

### Community 203 - "TargetMask"
Cohesion: 0.19
Nodes (6): List, Vector3, Dictionary, List, TargetFlagHelper, TargetMask

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "drive_tidal_music"
Cohesion: 0.30
Nodes (14): AdaptiveMusicSignature, drive_tidal_music(), report_once(), NativeAudioRouting, Option, Res, ResMut, silence_music() (+6 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "LabelDisplayProcessor"
Cohesion: 0.09
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

### Community 209 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 210 - "TidalPcmMix"
Cohesion: 0.32
Nodes (4): stream_only_music_tap_mixes_pre_monitor_pcm_into_wasapi_audio(), stream_only_music_tap_resamples_to_the_twitch_clock(), TidalPcmMix, NativeAudioFrame

### Community 211 - "RoleDataContainer"
Cohesion: 0.15
Nodes (8): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings

### Community 212 - "stream_town_domain/src/lib.rs"
Cohesion: 0.40
Nodes (5): is_transient_surface_configuration_error(), stream_town_render_error_handler(), RenderError, RenderErrorPolicy, World

### Community 213 - "HealthModifier"
Cohesion: 0.22
Nodes (6): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit

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

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

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

### Community 234 - "AnimationHandler"
Cohesion: 0.06
Nodes (20): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+12 more)

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

### Community 243 - ".write"
Cohesion: 0.29
Nodes (8): PlayerSettingsStore, PlayerSettingsStoreError, Into, Path, PathBuf, Result, SpannedError, store_recovers_last_valid_backup()

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

### Community 253 - "PlacementProbeHandler"
Cohesion: 0.25
Nodes (4): PlacementProbe, float, PlacementProbeHandler, SurfaceType

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "SaveState"
Cohesion: 0.20
Nodes (6): ChannelData, string, Container, PSAccess, bool, SaveState

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "draw_world_preview"
Cohesion: 0.29
Nodes (8): draw_world_preview(), preview_grid_point(), preview_lerp_color(), Color32, Pos2, Rect, terrain_preview_color(), WorldPreviewLayer

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 268 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 269 - "stream_town_domain"
Cohesion: 0.40
Nodes (6): stream_town_domain, stream_town_ffmpeg_bridge, stream_town_game, stream_town_migrate, stream_town_tools, xtask

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

### Community 276 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.07
Nodes (55): apply_preview_material_overrides(), apply_preview_node_visibility(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene, player_preview_material_overrides() (+47 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.12
Nodes (16): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+8 more)

### Community 287 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 288 - "UIElementWrapper"
Cohesion: 0.25
Nodes (5): GameObject, List, PresetButtons, ContainerBuilder, UIElementWrapper

### Community 289 - "AdaptiveMusicConfig"
Cohesion: 0.22
Nodes (5): AdaptiveMusicConfig, Self, String, WindowConfig, intensity_cycles_per_second()

### Community 290 - "DataStructures"
Cohesion: 0.25
Nodes (4): int, ChangeTimeStamp, DataStructures, DateTime

### Community 291 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 292 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 293 - "PlayerSaveData"
Cohesion: 0.06
Nodes (26): Dictionary, Mesh, Vector3, SaveDataMapper, bool, int, List, string (+18 more)

### Community 294 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.05
Nodes (10): bool, UIGameObjectAccessor, ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, ConnectionTab, Reflex.Core (+2 more)

### Community 295 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), GpuImage, GpuRenderAssets, ErasedRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 298 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (69): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot (+61 more)

### Community 300 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 301 - "healing_burst_effect"
Cohesion: 0.62
Nodes (7): healing_burst_effect(), healing_channel_effect(), healing_effect_curves_preserve_authored_lifetimes_and_channel_keys(), healing_effect_duration(), healing_effect_extent_stays_within_character_scale(), healing_effect_sample(), HealingEffectSample

### Community 302 - "color_grading_for_state"
Cohesion: 0.67
Nodes (4): authored_color_grading(), color_grading_for_state(), scene_exposure_uses_approved_neutral_baselines_and_prism_clouds(), ColorGrading

### Community 303 - "Q: Why did live capture FPS fall below 28 during the night transition?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did live capture FPS fall below 28 during the night transition?, Source Nodes

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+4 more)

### Community 307 - "GpuStreamCaptureCounters"
Cohesion: 0.67
Nodes (3): AtomicUsize, GpuStreamCaptureCounters, AtomicU64

### Community 309 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "FoliageGenerationSettings"
Cohesion: 0.14
Nodes (12): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings, int (+4 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **401 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+396 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=3.399575666) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=2.427662932) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.220994104) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=1.857467961)
- `WorldSnapshot` (3× useful, score=1.77434555)
- `SkinnedMesh` (2× useful, score=1.497734921)
- `drive_tidal_music()` (2× useful, score=1.474473069)
- `WorldSimulation` (2× useful, score=1.288002666)
- `load_input()` (2× useful, score=1.182725203) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.160627988)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StableId` connect `StableId` to `.from`, `.new`, `.new`, `stream_town_migrate/src/content.rs`, `.count`, `stream_town_game/src/lib.rs`, `TechnologyGraphLayout`, `Res`, `draw_world_preview`, `config.rs`, `ContentCatalog`, `command.rs`, `RenderAssets`, `String`, `GameConfig`, `ModelPreviewRuntime`, `Ui`, `ResMut`, `Option`, `build_converted_animation`, `ToolState`, `AnimationControllerDef`, `healing_burst_effect`, `runtime_console.rs`, `String`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/presentation.rs`, `twitch.rs`, `xtask/src/main.rs`, `String`, `technology_graph.rs`, `legacy.rs`, `PresentationCatalog`, `world.rs`, `stream_town_tools/src/main.rs`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `Utils` connect `Utils` to `EnemySpawner`, `IRuntimeDataScriptable`, `Easings`, `MonoBehaviour`, `.DrawDataFieldAndLabel`, `TechTree.Elements`, `DataStructures`, `GenerationSettings`, `ScriptablesProcessorInfrastructure`, `RandomEnabler`, `AudioHandler`, `IInstaller`, `World.Generation`, `CommonEnums.cs`, `BuildingPlacer`, `RoleDataContainer`, `SnapToGridMouseMovement`, `StringUtils`, `FPSDisplay`, `Resource`, `.CreateEnumField`, `UpdateGraphBounds`, `PlacementProbeHandler`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `SelectedObject`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `MonoBehaviour`, `IProcessor`, `WorldGenProcessor`, `RoleProcessor`, `TechTreeProcessor`, `UIProcessor`, `VoteEvent`, `Player`, `StreamTownSessionBridge`, `TimeProcessor`, `TwitchClientProcessor`, `IInstaller`, `UserInterface_Roles`, `Utils`, `BuildingPlacer`, `GameEvent`, `ObjectPoolingProcessor`, `RoleHandler`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `SelectedPlayer`, `UserInterface_TownVote`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _401 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `.new` be split into smaller, more focused modules?**
  _Cohesion score 0.02588593037308487 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.0347661188369153 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06843559977888336 - nodes in this community are weakly interconnected._