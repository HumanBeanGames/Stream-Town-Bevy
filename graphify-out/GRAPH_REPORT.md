# Graph Report - Stream-Town-Bevy  (2026-09-05)

## Corpus Check
- 675 files · ~1,855,516 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9854 nodes · 30541 edges · 323 communities (301 shown, 22 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1071 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `edb774f3`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .new
- BuildingProcessor
- stream_town_migrate/src/content.rs
- SeasonProcessor
- advance_world_loading_cover
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- STSM_Idle_Player
- BottomBarInterface
- Res
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- TownGoalProcessor
- GeneratedWorld
- ScriptablesProcessorInfrastructure
- command.rs
- MonoBehaviour
- NavGrid
- PlayerRoleData
- UnityGraphics
- StableId
- Node_SO
- String
- TechTree.Elements
- SaveFileData
- Ui
- RuntimeConfig
- WorldGenProcessor
- setup_rendering
- UnityAsset
- GenerationSettings
- Option
- settings.rs
- SettingsData
- direct_broadcast.rs
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- Vec
- runtime_console.rs
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- Result
- .GetResourceAssets
- process_injected_commands
- AudioHandler
- StreamTownSessionBridge
- ContentCatalog
- TwitchClientProcessor
- .new
- .SerializeComponent
- ResourceGenerationSettings
- BevyMigrationExporter
- IInstaller
- TechTreeEditorWindow
- PlayerProcessor
- stream_town_domain/src/content.rs
- CameraController
- stream_town_migrate/src/presentation.rs
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- twitch.rs
- GamestateJukebox
- xtask/src/main.rs
- models.rs
- Tiler
- apply_player_settings
- Utils
- roles_tab
- Station
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- WorldGenSaveData
- Access_Text
- Option
- GridProcessor
- legacy.rs
- CharacterModelHandler
- FoliageProcessor
- SnapToGridMouseMovement
- GameEvent
- ObjectPoolingProcessor
- save.rs
- convert_fbx_to_glb.py
- .new
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- TraversalWearRuntime
- StateMachine
- GameEventProcessor
- ObjectSelectionProcessor.Editor.cs
- TwitchUser
- ResourceProcessor
- LoadingManager
- Result
- Goal
- CustomLogHandler
- LevelHandler
- .CreateEnumField
- UpdateGraphBounds
- GlobalAudioController
- ScriptablesEditor
- world.rs
- .Draw
- tidal_music.rs
- DayAndNightProcessor
- VfxSeagullSpawner
- UserInterface_TownVote
- TechTreeGraphView
- stream_town_tools/src/main.rs
- TimeProcessor
- AIPath
- .Log
- GateController
- String
- .new
- Coordinator
- DirectBroadcastRuntime
- RoleData
- EnemySpawner
- .write
- Targetable
- component_field_value
- TargetSensor
- sync_stream_only_capture
- StationProcessor
- SensorProcessor
- What You Must Do When Invoked
- RuntimeData Template
- WeatherProcessor
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldSaveData
- xtask/src/lib.rs
- TechTreeNode
- CommandDictionary
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- GridPos
- STSM_StateAction
- UnitHealthBar
- update_enemy_music_intensity
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- EnemyModelHandler
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- runtime_tab
- UserInterface_ObjectSelection
- TransformSaveData
- UserInterface_DisplayUsernames
- RoleHandler
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- SimpleMusicController
- EditorHelpers
- GUIDComponent
- .default
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- PlacementProbeHandler
- Access_Dropdown
- List
- VoteEvent
- Option
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Q: The game has slowed to about 0.3 fps. Look into why, fix it and redeploy.
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- DebugProcessor
- UIElementWrapper
- WorldGenerationReferenceExporter
- PlayerInputProcessor
- CommonEnums.cs
- UserInterface_BuildingHealthBar
- stream_town_migrate/src/menu_scene.rs
- Access_Toggle
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- VfxAnimationController
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- SelectedBuilding
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- .SetTargetType
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- drive_tidal_music
- Common Patterns
- LabelDisplayProcessor
- Requirement
- BuildingBase
- building_region
- AdaptiveMusicConfig
- String
- Key Rules
- BuildingModelHandler
- RuntimeData Template
- Character Animation Regression Checklist
- IRuntimeDataScriptable
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
- AnimationHandler
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
- BuildingDataSettings
- extraction-spec.md
- GameStateProcessor
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TownGoal.Data
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- capture_stream_only_target
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- CampGenerationSettings
- Q: If there is more to do, keep going.
- ResourceHolder
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
- SelectedObject
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch command reference
- UserInterface_Roles
- Instant
- DontDestroyOnLoad
- SelectedEnemy
- SelectedResource
- SelectedPlayerGroup
- PlayerSaveData
- CreateProjectScopeProcessors.cs
- STSM_HelperBase
- .ExportModification
- RotationHandler
- VFXArrowPointer
- ToolState
- SelectedEnemyCamp
- resolve_combat_projectile_impact
- scroll_town_load_list
- Q: Why did live capture FPS fall below 28 during the night transition?
- STSM_Idle_Enemy
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- player_window_mode
- Autosave
- tidal_plugin
- SaveState
- Projectile
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- stream_town_domain/src/lib.rs
- ErrorData
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- FoliageGenerationSettings
- vcpkg.json
- .RestoreObjectiveProgress
- preview_grid_point
- FFmpeg runtime and relinking
- StreamTown.Migration

## God Nodes (most connected - your core abstractions)
1. `StableId` - 492 edges
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
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs

## Import Cycles
- None detected.

## Communities (323 total, 22 thin omitted)

### Community 0 - ".new"
Cohesion: 0.04
Nodes (130): RoleProgress, Default, generate_world(), generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean(), agent_facing_matches_unity_rotation_and_action_targets() (+122 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (19): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+11 more)

### Community 2 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (56): animation_parameter_name(), archetype_scenes(), asset(), authored_mask(), building_placements(), BuildingPlacement, child_technology_guids(), collect_model_dependencies() (+48 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.07
Nodes (18): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+10 more)

### Community 4 - "advance_world_loading_cover"
Cohesion: 0.04
Nodes (76): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), AutomaticResumeRuntime, begin_world_loading() (+68 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (286): accessibility_settings_selection(), accessibility_should_clear_focus(), AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityCandidate, actor_detail_budget(), actor_health_fill_color(), actor_name_color() (+278 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (27): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+19 more)

### Community 7 - "STSM_Idle_Player"
Cohesion: 0.06
Nodes (15): AttackUnit, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float (+7 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Res"
Cohesion: 0.03
Nodes (256): Added, AmbientLight, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, AccessibilityHighContrastText, ActivePetVisual (+248 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

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
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.06
Nodes (15): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy, Action (+7 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "GeneratedWorld"
Cohesion: 0.07
Nodes (73): GameConfig, GameplayConfig, BTreeMap, BuildingDef, PassiveResourceContribution, RoleSlotContribution, StorageContribution, GeneratedFoliage (+65 more)

### Community 18 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.06
Nodes (10): ContainerBuilder, MetaDataInstaller, ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers (+2 more)

### Community 19 - "command.rs"
Cohesion: 0.14
Nodes (37): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+29 more)

### Community 20 - "MonoBehaviour"
Cohesion: 0.01
Nodes (105): Api, CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, PersistentScoped, ContainerBuilder, Volume, PostProcessingInstaller (+97 more)

### Community 21 - "NavGrid"
Cohesion: 0.10
Nodes (30): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavGrid, NavigationError, neighbour_candidates() (+22 more)

### Community 22 - "PlayerRoleData"
Cohesion: 0.07
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+6 more)

### Community 23 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 24 - "StableId"
Cohesion: 0.04
Nodes (94): ObjectiveDef, Display, FromStr, StableId, actor_appearance_hash(), actor_appearances_are_seeded_varied_and_persisted(), ActorCustomization, authored_trade_rates_clamp_to_stock_gold_and_capacity() (+86 more)

### Community 25 - "Node_SO"
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 26 - "String"
Cohesion: 0.11
Nodes (39): AnimationEventDef, AnimationObjectReference, AnimationParameterDef, infer_missing_parameters(), inline_file_id(), inline_mapping_value(), parse_animation_events(), parse_blend_tree() (+31 more)

### Community 27 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Ui"
Cohesion: 0.09
Nodes (69): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+61 more)

### Community 30 - "RuntimeConfig"
Cohesion: 0.03
Nodes (181): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AnyResult, PlayerSettings, Default, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active() (+173 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (26): HashSet, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject, HashSet (+18 more)

### Community 32 - "setup_rendering"
Cohesion: 0.05
Nodes (80): ActiveMaterialHandles, animate_main_menu_clouds(), authored_post_process_stack(), bounds_material(), building_bounds_material_preserves_unity_placement_contract(), building_material(), BuildingMaterialInstance, BuildingMaterialInstances (+72 more)

### Community 33 - "UnityAsset"
Cohesion: 0.20
Nodes (42): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), enemy_camp_generation_layers(), field_value(), generated_record_ids(), objective_definitions() (+34 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "Option"
Cohesion: 0.05
Nodes (158): ArchetypeScene, MainMenuModelInstance, MainMenuSceneReference, Option, String, PresentationCatalog, actor_material(), animated_pets_resolve_their_own_unity_controllers_and_rigs() (+150 more)

### Community 36 - "settings.rs"
Cohesion: 0.12
Nodes (25): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), imports_unity_json_indices_and_clamps_values() (+17 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (62): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), AuthorizationEvent, average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), BroadcastPrerequisites, build_ingest_url() (+54 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "Vec"
Cohesion: 0.04
Nodes (98): AnimationClip, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve(), advance_animation_crossfade() (+90 more)

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
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 47 - "Result"
Cohesion: 0.10
Nodes (30): AtomicUsize, BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, BroadcastMetrics, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), encoder_candidates() (+22 more)

### Community 48 - ".GetResourceAssets"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 49 - "process_injected_commands"
Cohesion: 0.09
Nodes (41): StreamUserType, building_blocks_navigation(), building_cost_summary(), building_definition_id(), building_instance_ids(), building_upgrade_cost(), buy_town_resource(), CommandOrigin (+33 more)

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (16): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+8 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "ContentCatalog"
Cohesion: 0.06
Nodes (102): ContentCatalog, StationDef, TargetingScoreDef, ActorState, String, action_animation_speed(), action_cooldown(), active_station_ids() (+94 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - ".new"
Cohesion: 0.12
Nodes (26): apply_direct_broadcast_control(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), closing_the_operator_window_requests_a_graceful_game_exit(), configure_direct_broadcast(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), DirectTwitchBroadcastPlugin (+18 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "ResourceGenerationSettings"
Cohesion: 0.11
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 58 - "IInstaller"
Cohesion: 0.01
Nodes (136): ContainerBuilder, InstantiationBarrier, Camera, GameObject, ProjectCameraInstaller, ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder (+128 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "PlayerProcessor"
Cohesion: 0.08
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.07
Nodes (53): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingModelDef, ContentError, default_resource_generation_layers() (+45 more)

### Community 62 - "CameraController"
Cohesion: 0.11
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (97): animation_state_id(), animation_state_machine_id(), animation_take_name(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id(), convert() (+89 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (15): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+7 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (65): BTreeSet, TwitchConfig, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization (+57 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "xtask/src/main.rs"
Cohesion: 0.10
Nodes (52): archive_purge_backup_history(), AudioBaselineManifest, Cli, Command, fine_navigation_town_reset_preserves_only_requested_progress(), glb_animation_count(), glb_document_from_bytes(), glb_node_names() (+44 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "apply_player_settings"
Cohesion: 0.20
Nodes (10): apply_player_settings(), constrain_town_camera_position(), player_msaa(), WinitSettings, town_camera_keeps_the_unity_angle_with_the_wider_upper_third_frame(), town_camera_projection(), DirectionalLight, DirectionalLightShadowMap (+2 more)

### Community 74 - "Utils"
Cohesion: 0.03
Nodes (43): BuildCostModifier, InputButton, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Utils, Processors (+35 more)

### Community 75 - "roles_tab"
Cohesion: 0.17
Nodes (23): ability_choices(), action_animation_choices(), animation_quat_keys_editor(), animation_transform_tracks_editor(), animation_vec3_keys_editor(), building_model_node_choices(), buildings_tab(), cached_gltf_metadata() (+15 more)

### Community 76 - "Station"
Cohesion: 0.09
Nodes (13): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+5 more)

### Community 77 - "Objective"
Cohesion: 0.09
Nodes (12): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+4 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.06
Nodes (67): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), Default, Result (+59 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.27
Nodes (5): GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.14
Nodes (8): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation

### Community 83 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Option"
Cohesion: 0.09
Nodes (49): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastTarget, capture_process_audio(), discard_pending_audio(), encode_broadcast_session() (+41 more)

### Community 86 - "GridProcessor"
Cohesion: 0.06
Nodes (22): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, GridProcessorEditor, int (+14 more)

### Community 87 - "legacy.rs"
Cohesion: 0.17
Nodes (39): binary_fixture(), BinaryParser, clamped_cell(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies() (+31 more)

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
Nodes (23): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+15 more)

### Community 92 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (37): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+29 more)

### Community 93 - "save.rs"
Cohesion: 0.13
Nodes (40): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+32 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - ".new"
Cohesion: 0.16
Nodes (22): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+14 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (47): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+39 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.07
Nodes (20): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+12 more)

### Community 98 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (74): AnimationClipDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+66 more)

### Community 100 - "TraversalWearRuntime"
Cohesion: 0.08
Nodes (35): SeasonalTerrainPalette, TerrainAppearanceConfig, autosave_game(), building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialUpdateRuntime, CommandSaveRuntime (+27 more)

### Community 101 - "StateMachine"
Cohesion: 0.12
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 104 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "Result"
Cohesion: 0.24
Nodes (7): BinaryParser<'a>, legacy_user_type(), FnMut, Result, Self, T, LegacyWorldState

### Community 108 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 114 - "ScriptablesEditor"
Cohesion: 0.07
Nodes (19): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+11 more)

### Community 115 - "world.rs"
Cohesion: 0.06
Nodes (66): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), cell_hash(), changing_seed_changes_world_hash() (+58 more)

### Community 116 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.16
Nodes (24): adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine(), format_number() (+16 more)

### Community 118 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 119 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 120 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 121 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 122 - "stream_town_tools/src/main.rs"
Cohesion: 0.05
Nodes (90): apply_building_draft(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authoring_config_save_bar(), authority_tab(), broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record() (+82 more)

### Community 123 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 126 - ".Log"
Cohesion: 0.13
Nodes (6): Action, IEnumerable, HideInCallstack, Object, DebugLogCategory, Vector2Int

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "String"
Cohesion: 0.05
Nodes (54): AccessibleNode, active_event_text(), announce_citizen_deaths(), authored_rotating_node_names(), building_command_name(), building_model_node_names(), citizen_death_announcement(), CitizenDeathAnnouncementRuntime (+46 more)

### Community 129 - ".new"
Cohesion: 0.19
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "DirectBroadcastRuntime"
Cohesion: 0.05
Nodes (53): AutomaticBroadcastStart, bounded_history_f32(), BroadcastMetricsSnapshot, capture_direct_broadcast_frame(), DirectBroadcastControl, DirectBroadcastPhase, DirectBroadcastRuntime, operator_live_button_label() (+45 more)

### Community 132 - "RoleData"
Cohesion: 0.07
Nodes (22): RoleData, AudioClip, bool, float, int, Sprite, string, ContainerBuilder (+14 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - ".write"
Cohesion: 0.33
Nodes (6): PlayerSettingsStore, Into, Path, PathBuf, Result, store_recovers_last_valid_backup()

### Community 135 - "Targetable"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 136 - "component_field_value"
Cohesion: 0.21
Nodes (27): ArchetypesById, archetype_bounds(), building_model_definitions(), building_node_age(), component_field_value(), component_reference_name(), component_reference_names(), component_type() (+19 more)

### Community 137 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 138 - "sync_stream_only_capture"
Cohesion: 0.09
Nodes (24): camera_targets_primary_window(), NativeGameAudioMix, NativeGameAudioRouting, NativeGameAudioVoice, pcm16_wav_clip(), pcm16_wav_data(), Assets, Commands (+16 more)

### Community 139 - "StationProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 140 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

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

### Community 149 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 150 - "CommandDictionary"
Cohesion: 0.13
Nodes (8): Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary, MiscCommands, EnemyType

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "GridPos"
Cohesion: 0.06
Nodes (72): GridPos, EnemyCampState, active_resource_at(), agent_path(), AgentCommand, build_enemy_navigation_field(), building_direction_commands_follow_the_visible_town_axes(), building_fine_navigation_cells() (+64 more)

### Community 154 - "STSM_StateAction"
Cohesion: 0.12
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 155 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 156 - "update_enemy_music_intensity"
Cohesion: 0.18
Nodes (13): intensity_smoothing_has_a_five_second_time_constant(), point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, Res, Time (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.08
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 159 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "runtime_tab"
Cohesion: 0.29
Nodes (8): format_runtime_frame_times(), inject_runtime_command(), launch_runtime_game(), poll_runtime_console(), runtime_actions_sequence_after_latest_acknowledgement(), runtime_console_attached(), runtime_tab(), send_runtime_action()

### Community 163 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 164 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "RoleHandler"
Cohesion: 0.04
Nodes (20): RoleSlotModifier, int, RoleHandler, bool, Dictionary, UnityEvent, RoleSlot, bool (+12 more)

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

### Community 172 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 173 - ".default"
Cohesion: 0.02
Nodes (149): accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), AccessibilityMotionDefaults, authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform (+141 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "PlacementProbeHandler"
Cohesion: 0.14
Nodes (6): UnitTravelToPosition, Vector3, PlacementProbe, float, PlacementProbeHandler, SurfaceType

### Community 176 - "Access_Dropdown"
Cohesion: 0.06
Nodes (18): Camera, Quaternion, Vector3, ProjectCamera, Access_AADropdown, Access_AODropdown, Access_AutosaveTimerDropdown, Access_CameraAADropdown (+10 more)

### Community 177 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 178 - "VoteEvent"
Cohesion: 0.06
Nodes (23): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, TechVote (+15 more)

### Community 179 - "Option"
Cohesion: 0.08
Nodes (52): PrefabPresentationBinding, animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert_materials() (+44 more)

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

### Community 184 - "DebugProcessor"
Cohesion: 0.14
Nodes (8): Dictionary, DebugSettings, Container, ContainerBuilder, DebugProcessor, int, STSM_Helper_Build, SerializedScriptableObject

### Community 185 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "PlayerInputProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor, bool, Dictionary, InputButton (+3 more)

### Community 189 - "CommonEnums.cs"
Cohesion: 0.18
Nodes (10): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, SaveItem, Seasons, TimeOfDay (+2 more)

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

### Community 195 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

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

### Community 203 - ".SetTargetType"
Cohesion: 0.21
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

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

### Community 208 - "LabelDisplayProcessor"
Cohesion: 0.10
Nodes (12): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+4 more)

### Community 209 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 210 - "BuildingBase"
Cohesion: 0.08
Nodes (18): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Action (+10 more)

### Community 211 - "building_region"
Cohesion: 0.24
Nodes (15): DirtyRegion, building_navigation_region(), building_region(), building_site_is_available(), building_site_is_available_for_simulation(), coarse_building_occupancy_preserves_authored_placement_footprints(), enemy_camp_has_town_route(), enemy_camp_navigation_region() (+7 more)

### Community 212 - "AdaptiveMusicConfig"
Cohesion: 0.25
Nodes (5): AdaptiveMusicConfig, Default, Self, TimeCycleConfig, intensity_cycles_per_second()

### Community 213 - "String"
Cohesion: 0.20
Nodes (18): decode_binary(), ImportReport, json_pet_name(), legacy_objective_matches(), legacy_pet_name(), LegacyDecodedSave, LegacyGoal, LegacyObjective (+10 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "IRuntimeDataScriptable"
Cohesion: 0.15
Nodes (11): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, List, TownGoalRuntimeData (+3 more)

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

### Community 234 - "AnimationHandler"
Cohesion: 0.08
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+7 more)

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

### Community 251 - "BuildingDataSettings"
Cohesion: 0.17
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 253 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "TownGoal.Data"
Cohesion: 0.09
Nodes (9): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, Data (+1 more)

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "capture_stream_only_target"
Cohesion: 0.18
Nodes (10): capture_stream_only_target(), configure_stream_capture_ring(), copy_gpu_rows_into(), exit_after_broadcast_stops(), AppExit, GpuImage, GpuRenderAssets, MessageWriter (+2 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "CampGenerationSettings"
Cohesion: 0.15
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 268 - "Editor"
Cohesion: 0.07
Nodes (10): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, RoleScriptablesEditor, Utils, WindControllerEditor, GridSystemEditor, StringUtils, Globals (+2 more)

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
Cohesion: 0.15
Nodes (8): HealthModifier, bool, float, GameObject, HealUnit, int, string, SimpleScreenShot

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.08
Nodes (53): apply_preview_material_overrides(), apply_preview_node_visibility(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene, player_preview_material_overrides() (+45 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.12
Nodes (16): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+8 more)

### Community 287 - "UserInterface_Roles"
Cohesion: 0.20
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 288 - "Instant"
Cohesion: 0.12
Nodes (14): BroadcastStopDisposition, CadenceTick, duration_as_micros(), Duration, Error, Instant, stream_readback_due(), twitch_live_request_timeout() (+6 more)

### Community 293 - "PlayerSaveData"
Cohesion: 0.12
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 297 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 298 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 299 - "ToolState"
Cohesion: 0.08
Nodes (82): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot() (+74 more)

### Community 301 - "resolve_combat_projectile_impact"
Cohesion: 0.38
Nodes (7): ActionPresentation, actor_combat_visual(), CombatProjectile, CombatVisualKind, ProjectileSource, ProjectileSpawn, resolve_combat_projectile_impact()

### Community 302 - "scroll_town_load_list"
Cohesion: 0.33
Nodes (6): MessageReader, MouseWheel, scroll_town_load_list(), TownLoadList, ComputedNode, ScrollPosition

### Community 303 - "Q: Why did live capture FPS fall below 28 during the night transition?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did live capture FPS fall below 28 during the night transition?, Source Nodes

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (11): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+3 more)

### Community 307 - "player_window_mode"
Cohesion: 0.67
Nodes (4): DisplayMode, player_window_mode(), startup_window_mode(), WindowMode

### Community 309 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 310 - "SaveState"
Cohesion: 0.20
Nodes (6): ChannelData, string, Container, PSAccess, bool, SaveState

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "stream_town_domain/src/lib.rs"
Cohesion: 0.40
Nodes (5): is_transient_surface_configuration_error(), stream_town_render_error_handler(), RenderError, RenderErrorPolicy, World

### Community 315 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "FoliageGenerationSettings"
Cohesion: 0.20
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 321 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 324 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

## Knowledge Gaps
- **400 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+395 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `StableId` connect `StableId` to `.new`, `.new`, `String`, `stream_town_migrate/src/content.rs`, `stream_town_game/src/lib.rs`, `component_field_value`, `Res`, `config.rs`, `GeneratedWorld`, `command.rs`, `GridPos`, `String`, `Ui`, `RuntimeConfig`, `setup_rendering`, `UnityAsset`, `Option`, `Vec`, `runtime_console.rs`, `AnimationControllerDef`, `resolve_combat_projectile_impact`, `.default`, `ToolState`, `process_injected_commands`, `Option`, `ContentCatalog`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/presentation.rs`, `twitch.rs`, `xtask/src/main.rs`, `roles_tab`, `technology_graph.rs`, `String`, `save.rs`, `.new`, `stream_town_domain/src/presentation.rs`, `TraversalWearRuntime`, `world.rs`, `stream_town_tools/src/main.rs`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `Utils` connect `Utils` to `TownGoal.Data`, `RoleData`, `EnemySpawner`, `Editor`, `Easings`, `ScriptablesProcessorInfrastructure`, `MonoBehaviour`, `HealthModifier`, `TechTree.Elements`, `GenerationSettings`, `PlacementProbeHandler`, `AudioHandler`, `ResourceGenerationSettings`, `IInstaller`, `CommonEnums.cs`, `BuildingPlacer`, `SnapToGridMouseMovement`, `FPSDisplay`, `Resource`, `.CreateEnumField`, `UpdateGraphBounds`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `MonoBehaviour`, `PlayerRoleData`, `IProcessor`, `WorldGenProcessor`, `UserInterface_Roles`, `SelectedPlayerGroup`, `RoleHandler`, `TechTreeProcessor`, `UIProcessor`, `VoteEvent`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `IInstaller`, `Utils`, `BuildingPlacer`, `GameEvent`, `ObjectPoolingProcessor`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `UserInterface_TownVote`, `TimeProcessor`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _400 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `.new` be split into smaller, more focused modules?**
  _Cohesion score 0.03899001761597182 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.055811571940604196 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07570621468926554 - nodes in this community are weakly interconnected._