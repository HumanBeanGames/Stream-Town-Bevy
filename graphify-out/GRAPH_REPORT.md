# Graph Report - Stream-Town-Bevy  (2026-09-04)

## Corpus Check
- 672 files · ~1,852,164 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9799 nodes · 30380 edges · 314 communities (288 shown, 26 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1071 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `58da903a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Option
- BuildingProcessor
- UnityAsset
- SeasonProcessor
- advance_world_loading_cover
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- StableId
- BottomBarInterface
- STSM_StateAction
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- TownGoalProcessor
- Vec
- ScriptablesProcessorInfrastructure
- command.rs
- MonoBehaviour
- NavGrid
- ResourceGenerationSettings
- UnityGraphics
- simulation.rs
- Node_SO
- String
- TechTree.Elements
- SaveFileData
- Ui
- Res
- WorldGenProcessor
- MenuRuntime
- xtask/src/main.rs
- MeshData
- .new
- PlayerSettings
- SettingsData
- PresentationCatalog
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- String
- WorldSimulation
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- Result
- .GetResourceAssets
- ResourceHolder
- AudioHandler
- StreamTownSessionBridge
- VfxSeagullSpawner
- TwitchClientProcessor
- .new
- .SerializeComponent
- legacy.rs
- string
- IInstaller
- TechTreeEditorWindow
- DayAndNightProcessor
- stream_town_domain/src/content.rs
- CameraController
- stream_town_migrate/src/presentation.rs
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- twitch.rs
- GamestateJukebox
- stream_operator_chat_controls
- models.rs
- UnitHealthBar
- .GenerateFromSettings
- Utils
- stream_town_migrate/src/content.rs
- Station
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_Idle_Player
- WorldGenSaveData
- Access_Text
- Option
- runtime_console.rs
- RoleData
- CharacterModelHandler
- FoliageProcessor
- SnapToGridMouseMovement
- RaidEvent
- .Log
- .EnsureValidCredentials
- convert_fbx_to_glb.py
- generate_world_from_layers
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- Node_SO.cs
- StateMachine
- GameEventProcessor
- component_field_value
- TwitchUser
- ResourceProcessor
- LoadingManager
- LabelDisplayProcessor
- AnimationHandler
- CustomLogHandler
- LevelHandler
- WorldInstanceDeterminism
- UpdateGraphBounds
- GlobalAudioController
- ScriptablesEditor
- world.rs
- .SetGeneratedResources
- tidal_music.rs
- .Draw
- .CreateEnumField
- UserInterface_TownVote
- TechTreeNode
- roles_tab
- EnemyModelHandler
- AIPath
- PlayerInputProcessor
- GateController
- direct_broadcast.rs
- stream_town_domain/src/lib.rs
- Coordinator
- DirectBroadcastControl
- TL_Secrets
- IRuntimeDataScriptable
- update_enemy_music_intensity
- SelectedPlayer
- .RenderResourceType
- TargetSensor
- DirectBroadcastRuntime
- BottomBarButton
- WeatherProcessor
- What You Must Do When Invoked
- RuntimeData Template
- GridProcessor
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldSaveData
- xtask/src/lib.rs
- Goal
- IProcessor.cs
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- GeneratedResource
- SelectedPlayerGroup
- String
- SelectableObject
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- UserInterface_ObjectSelection
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- .default
- SelectedObject
- TransformSaveData
- UserInterface_DisplayUsernames
- RoleHandler
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- SimpleMusicController
- EditorHelpers
- verify_twitch_public_stream
- RenderAssets
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- SelectedEnemy
- Access_Dropdown
- List
- VoteEvent
- BTreeMap
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- BevyMigrationExporter
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Easings
- SelectedResource
- WorldGenerationReferenceExporter
- FoliageGenerationSettings
- ResourceRuntimeData
- SelectedEnemyCamp
- UserInterface_BuildingHealthBar
- stream_town_migrate/src/menu_scene.rs
- UIElementWrapper
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- TargetProcessor
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- SelectedBuilding
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- .walkable_neighbours_with
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- drive_tidal_music
- Common Patterns
- tidal_plugin
- .UserIsSubscribed
- UserInterface_TownGoal
- NativeGameAudioRouting
- .write
- stream_town_tools/src/main.rs
- Key Rules
- BuildingModelHandler
- RuntimeData Template
- Character Animation Regression Checklist
- HealthModifier
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
- LoadingWorkNode
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- CommandDictionary
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- Access_Toggle
- extraction-spec.md
- .ValidateTokenAsync
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- Globals
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- convert_post_process
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- ObjectiveDef
- Q: If there is more to do, keep going.
- TechTreeSearchWindow
- Editor
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- RotationHandler
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- CampGenerationSettings
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- main
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch command reference
- VideoSettings
- ObjectSelectionProcessor.Editor.cs
- DontDestroyOnLoad
- StringUtils
- Requirement
- ObjectiveSaveData
- PlayerSaveData
- CreateProjectScopeProcessors.cs
- RandomEnabler
- .ExportModification
- .InjectRuntimeData
- preview_grid_point
- ToolState
- PlacementProbeHandler
- update_environment_presentation
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Autosave
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- Targetable
- TL_API
- vcpkg.json
- TwitchClientRuntimeData
- FFmpeg runtime and relinking
- StreamTown.Migration

## God Nodes (most connected - your core abstractions)
1. `StableId` - 489 edges
2. `WorldSimulation` - 228 edges
3. `ContentCatalog` - 224 edges
4. `GridPos` - 188 edges
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

## Communities (314 total, 26 thin omitted)

### Community 0 - "Option"
Cohesion: 0.03
Nodes (285): GameConfig, ContentCatalog, DirtyRegion, GridPos, ActorState, String, generate_world_with_content(), GeneratedFoliage (+277 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.03
Nodes (39): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, TilerBuilding (+31 more)

### Community 2 - "UnityAsset"
Cohesion: 0.17
Nodes (47): aged_buildings(), archetype_kind(), authored_mask(), building_cost_reductions(), building_level_caps(), convert_export(), enemy_camp_generation_layers(), field_value() (+39 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.07
Nodes (19): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+11 more)

### Community 4 - "advance_world_loading_cover"
Cohesion: 0.05
Nodes (69): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), authored_rotating_node_names(), begin_menu_loading() (+61 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (266): AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, actor_detail_budget(), actor_health_fill_color(), actor_name_color(), actor_role_level_cap(), actor_scene_budget() (+258 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (27): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+19 more)

### Community 7 - "StableId"
Cohesion: 0.14
Nodes (9): Display, FromStr, StableId, complete_gameplay_scenario_round_trips(), BTreeMap, Result, SimulationError, validate_trade_resource() (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.07
Nodes (17): BottomBarContext, Action, bool, Button, Dictionary, GameObject, int, List (+9 more)

### Community 9 - "STSM_StateAction"
Cohesion: 0.13
Nodes (7): int, STSM_Action_Attack, bool, float, int, STSM_StateAction, AnimationName

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.10
Nodes (27): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig (+19 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.14
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.04
Nodes (25): AttackUnit, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, PlayerDeathHandler, bool, float (+17 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.13
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "Vec"
Cohesion: 0.04
Nodes (80): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), advance_animation_crossfade(), animation_event_occurrences(), animation_nodes_for_selection() (+72 more)

### Community 18 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (8): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, Settings, ScriptablesProcessorInfrastructure

### Community 19 - "command.rs"
Cohesion: 0.15
Nodes (35): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+27 more)

### Community 20 - "MonoBehaviour"
Cohesion: 0.01
Nodes (101): CameraProcessor, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped, ContainerBuilder, Volume (+93 more)

### Community 21 - "NavGrid"
Cohesion: 0.17
Nodes (17): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavGrid, NavigationError, octile_distance() (+9 more)

### Community 22 - "ResourceGenerationSettings"
Cohesion: 0.33
Nodes (5): AnimationCurve, bool, int, List, ResourceGenerationSettings

### Community 23 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 24 - "simulation.rs"
Cohesion: 0.08
Nodes (37): actor_appearance_hash(), actor_appearances_are_seeded_varied_and_persisted(), ActorCustomization, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase(), capped_deposit_preserves_inventory_overflow(), capped_role_progression_discards_excess_at_the_requested_level() (+29 more)

### Community 25 - "Node_SO"
Cohesion: 0.14
Nodes (13): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+5 more)

### Community 26 - "String"
Cohesion: 0.11
Nodes (45): AnimationParameterDef, animator_component(), animator_reference_path(), convert_prefab_bindings(), fixture_asset(), glb_animation_names(), infer_missing_parameters(), inline_file_id() (+37 more)

### Community 27 - "TechTree.Elements"
Cohesion: 0.05
Nodes (28): int, ChangeTimeStamp, Color, ErrorData, List, GroupErrorData, List, NodeErrorData (+20 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "Ui"
Cohesion: 0.10
Nodes (67): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+59 more)

### Community 30 - "Res"
Cohesion: 0.03
Nodes (271): Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver, ActorNameOverlay (+263 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (14): Action, bool, BoxCollider, Container, GameObject, IEnumerable, int, IReadOnlyList (+6 more)

### Community 32 - "MenuRuntime"
Cohesion: 0.03
Nodes (180): AccessibilityFocusVisualQuery, AnyResult, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_settings_selection(), accessibility_should_clear_focus(), AccessibilityAnnouncement (+172 more)

### Community 33 - "xtask/src/main.rs"
Cohesion: 0.06
Nodes (84): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+76 more)

### Community 34 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 35 - ".new"
Cohesion: 0.02
Nodes (179): AccessibilityActionRequest, generate_world(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean(), animated_character_receiver_scope_follows_only_the_player_rig_hierarchy() (+171 more)

### Community 36 - "PlayerSettings"
Cohesion: 0.12
Nodes (27): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), imports_unity_json_indices_and_clamps_values() (+19 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "PresentationCatalog"
Cohesion: 0.03
Nodes (168): ArchetypeScene, MainMenuSceneReference, Option, PresentationCatalog, ActiveMaterialHandles, animated_pets_resolve_their_own_unity_controllers_and_rigs(), animation_selection_duration(), apply_authored_main_menu_camera() (+160 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "String"
Cohesion: 0.03
Nodes (139): AccessibleNode, active_event_text(), add_animation_composition(), add_animation_layer_branch(), announce_citizen_deaths(), announce_technology_vote(), authored_layer_weight_and_mask_configure_bevy_graph_branch(), AuthoredCreditsElement (+131 more)

### Community 43 - "WorldSimulation"
Cohesion: 0.08
Nodes (21): BuildingState, EnemyCampState, FishGodState, ObjectiveProgress, RaidState, BTreeSet, Option, Vec (+13 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (30): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+22 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 47 - "Result"
Cohesion: 0.12
Nodes (24): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), encoder_candidates(), encoder_input_format(), encoder_is_hardware() (+16 more)

### Community 48 - ".GetResourceAssets"
Cohesion: 0.11
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 49 - "ResourceHolder"
Cohesion: 0.09
Nodes (17): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent (+9 more)

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (15): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+7 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - ".new"
Cohesion: 0.10
Nodes (22): BroadcastMetricsSnapshot, closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting() (+14 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.07
Nodes (89): ActorKind, Vec, SavedTerrainMesh, StreamUserType, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture() (+81 more)

### Community 57 - "string"
Cohesion: 0.22
Nodes (11): bool, int, long, string, NeutralComponent, NeutralExport, NeutralField, NeutralGameObject (+3 more)

### Community 58 - "IInstaller"
Cohesion: 0.01
Nodes (138): CellSpacePartitioningInstaller, ContainerBuilder, Camera, GameObject, ProjectCameraInstaller, ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder (+130 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (54): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+46 more)

### Community 62 - "CameraController"
Cohesion: 0.11
Nodes (11): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+3 more)

### Community 63 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (78): AnimationFloatKeyframe, AnimationTangent, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert_fireworks() (+70 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (15): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+7 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.12
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "twitch.rs"
Cohesion: 0.06
Nodes (65): BTreeSet, TwitchConfig, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization (+57 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_operator_chat_controls"
Cohesion: 0.10
Nodes (22): moderate_selected_operator_user(), operator_chat_scroll_rows(), Changed, Interaction, MouseWheel, Window, send_operator_chat_message(), stream_operator_chat_controls() (+14 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "UnitHealthBar"
Cohesion: 0.07
Nodes (14): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate (+6 more)

### Community 73 - ".GenerateFromSettings"
Cohesion: 0.11
Nodes (17): HashSet, Func, HashSet, List, Vector2, Vector3, Action, IEnumerator (+9 more)

### Community 74 - "Utils"
Cohesion: 0.03
Nodes (48): UnitTravelToPosition, Vector3, BuildCostModifier, InputButton, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units (+40 more)

### Community 75 - "stream_town_migrate/src/content.rs"
Cohesion: 0.11
Nodes (39): asset(), authored_value(), building_model_definitions(), building_node_age(), component(), component_at(), component_reference_name(), component_reference_names() (+31 more)

### Community 76 - "Station"
Cohesion: 0.05
Nodes (33): Station, Dictionary, float, int, List, Queue, Transform, Container (+25 more)

### Community 77 - "Objective"
Cohesion: 0.13
Nodes (4): Action, int, Objective, EnemyType

### Community 78 - "technology_graph.rs"
Cohesion: 0.06
Nodes (67): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), Default, Result (+59 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.14
Nodes (13): bool, CancellationTokenSource, int, long, MenuItem, string, DeviceCodeResponse, ErrorResponse (+5 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_Idle_Player"
Cohesion: 0.06
Nodes (18): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+10 more)

### Community 83 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Option"
Cohesion: 0.12
Nodes (38): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, BroadcastTarget, capture_process_audio(), DirectBroadcastSnapshot (+30 more)

### Community 86 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 87 - "RoleData"
Cohesion: 0.07
Nodes (22): RoleData, AudioClip, bool, float, int, Sprite, string, ContainerBuilder (+14 more)

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
Cohesion: 0.03
Nodes (48): Container, ContainerBuilder, GUIDProcessor, Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack (+40 more)

### Community 93 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "generate_world_from_layers"
Cohesion: 0.18
Nodes (23): WorldGenConfig, authored_grid_centre(), authored_world_to_grid(), foliage_horizontal_hash(), generate_authored_resources(), generate_candidate_mask(), generate_foliage(), generate_world_from_layers() (+15 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (47): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+39 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (19): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+11 more)

### Community 98 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (69): AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+61 more)

### Community 100 - "Node_SO.cs"
Cohesion: 0.40
Nodes (3): InputButton, SharedTypes, Data

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 102 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 103 - "component_field_value"
Cohesion: 0.21
Nodes (25): ArchetypesById, archetype_bounds(), building_placements(), BuildingPlacement, component_field_value(), component_type(), convert_archetypes(), disable_after_milliseconds() (+17 more)

### Community 104 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.11
Nodes (13): Container, ContainerBuilder, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool (+5 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "LabelDisplayProcessor"
Cohesion: 0.05
Nodes (26): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller (+18 more)

### Community 108 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "WorldInstanceDeterminism"
Cohesion: 0.23
Nodes (7): Material, Resource, int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 114 - "ScriptablesEditor"
Cohesion: 0.10
Nodes (14): Color, Texture2D, BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int (+6 more)

### Community 115 - "world.rs"
Cohesion: 0.12
Nodes (25): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), avalanche_instance_hash(), changing_seed_changes_world_hash(), different_town_seeds_produce_different_resource_and_foliage_layouts(), fnv_mix(), foliage_visual_variant(), foliage_visual_yaw_milliradians() (+17 more)

### Community 116 - ".SetGeneratedResources"
Cohesion: 0.22
Nodes (11): float, int, List, Material, materials, Mesh, meshes, Resource (+3 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.15
Nodes (26): AdaptiveMusicConfig, adaptive_music_energy(), adaptive_music_signature(), adaptive_song_program(), authorable_live_variables_participate_in_program_refreshes(), composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine() (+18 more)

### Community 118 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 119 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 120 - "UserInterface_TownVote"
Cohesion: 0.08
Nodes (18): Dictionary, TechVote, Button, GameObject, Image, Slider, TextMeshProUGUI, Transform (+10 more)

### Community 121 - "TechTreeNode"
Cohesion: 0.06
Nodes (24): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+16 more)

### Community 122 - "roles_tab"
Cohesion: 0.13
Nodes (27): ability_choices(), action_animation_choices(), animation_quat_keys_editor(), animation_transform_tracks_editor(), animation_vec3_keys_editor(), building_model_node_choices(), cached_gltf_metadata(), character_model_choices_include_converted_hierarchy_nodes() (+19 more)

### Community 123 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor, bool, Dictionary, InputButton (+3 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (63): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), AuthorizationEvent, average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), bounded_history_f32(), BroadcastPrerequisites (+55 more)

### Community 129 - "stream_town_domain/src/lib.rs"
Cohesion: 0.13
Nodes (13): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError (+5 more)

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "DirectBroadcastControl"
Cohesion: 0.14
Nodes (12): BroadcastStopDisposition, DirectBroadcastControl, exit_after_broadcast_stops(), operator_window_close_requests_exit(), request_automatic_broadcast_start(), AppExit, MessageReader, MessageWriter (+4 more)

### Community 132 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 133 - "IRuntimeDataScriptable"
Cohesion: 0.04
Nodes (38): Transform, Action, bool, Dictionary, float, GameObject, int, List (+30 more)

### Community 134 - "update_enemy_music_intensity"
Cohesion: 0.18
Nodes (13): intensity_smoothing_has_a_five_second_time_constant(), point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, ResMut, Time (+5 more)

### Community 136 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 137 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 138 - "DirectBroadcastRuntime"
Cohesion: 0.07
Nodes (57): apply_direct_broadcast_control(), arm_stream_only_readback(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), camera_targets_primary_window(), capture_direct_broadcast_frame(), cleanup_completed_stream_only_readbacks(), configure_direct_broadcast() (+49 more)

### Community 139 - "BottomBarButton"
Cohesion: 0.22
Nodes (4): bool, IEnumerator, Image, BottomBarButton

### Community 140 - "WeatherProcessor"
Cohesion: 0.19
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GridProcessor"
Cohesion: 0.06
Nodes (22): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, GridProcessorEditor, int (+14 more)

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

### Community 149 - "Goal"
Cohesion: 0.10
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 150 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "GeneratedResource"
Cohesion: 0.24
Nodes (14): cell_hash(), generate_shoreline_resources(), GeneratedResource, hash_world(), horizontal_hash(), legacy_resource_navigation(), legacy_v1_world_hash(), legacy_v2_world_hash() (+6 more)

### Community 155 - "String"
Cohesion: 0.13
Nodes (20): animation_parameter_name(), archetype_scenes(), child_technology_guids(), collect_model_dependencies(), ContentConversionReport, convert(), glb_asset_path(), model_age() (+12 more)

### Community 156 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 159 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - ".default"
Cohesion: 0.07
Nodes (43): apply_building_draft(), authoring_apply_preserves_the_complete_local_twitch_setup(), authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, checked_in_authoring_assets_pass_headless_validation(), default_catalog_path() (+35 more)

### Community 163 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 164 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "RoleHandler"
Cohesion: 0.03
Nodes (39): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleHandler (+31 more)

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

### Community 172 - "verify_twitch_public_stream"
Cohesion: 0.14
Nodes (11): DirectTwitchBroadcastPlugin, duration_as_micros(), LiveVerificationTarget, App, Duration, Error, Plugin, twitch_live_request_timeout() (+3 more)

### Community 173 - "RenderAssets"
Cohesion: 0.03
Nodes (129): AccessibilityMotionDefaults, actor_combat_visual(), actor_material(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension (+121 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 176 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 177 - "List"
Cohesion: 0.31
Nodes (6): GameObject, List, NeutralAsset, NeutralScene, NeutralGameObject, NeutralScene

### Community 178 - "VoteEvent"
Cohesion: 0.06
Nodes (21): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, float (+13 more)

### Community 179 - "BTreeMap"
Cohesion: 0.10
Nodes (48): MaterialDef, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert(), convert_avatar_masks() (+40 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "BevyMigrationExporter"
Cohesion: 0.29
Nodes (4): HashSet, MenuItem, BevyMigrationExporter, NeutralAsset

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 188 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 190 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, String, Vec (+36 more)

### Community 192 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "TargetProcessor"
Cohesion: 0.18
Nodes (6): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor

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

### Community 203 - ".walkable_neighbours_with"
Cohesion: 0.23
Nodes (9): neighbour_candidates(), offset(), OpenNode, Option, Ord, Ordering, PartialOrd, Self (+1 more)

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "drive_tidal_music"
Cohesion: 0.29
Nodes (14): AdaptiveMusicSignature, drive_tidal_music(), intensity_program_needs_update(), report_once(), NativeAudioRouting, Option, Res, silence_music() (+6 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 209 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 210 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 211 - "NativeGameAudioRouting"
Cohesion: 0.11
Nodes (13): AutomaticBroadcastStart, NativeGameAudioClip, NativeGameAudioRouting, NativeGameAudioState, pcm16_wav_clip(), pcm16_wav_data(), Default, VecDeque (+5 more)

### Community 212 - ".write"
Cohesion: 0.33
Nodes (6): PlayerSettingsStore, Into, Path, PathBuf, Result, store_recovers_last_valid_backup()

### Community 213 - "stream_town_tools/src/main.rs"
Cohesion: 0.07
Nodes (58): AssetEditorSection, authoring_config_save_bar(), broadcast_encoder_label(), canonical_preview_node_name(), debug_fingerprint(), default_float_key(), default_navigation_footprint_thirds(), default_role_preview_animation() (+50 more)

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

### Community 218 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), ErasedRenderAssets, GpuImage, GpuRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

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

### Community 234 - "LoadingWorkNode"
Cohesion: 0.36
Nodes (5): LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, WorldLoadingWork

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

### Community 243 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

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

### Community 251 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 253 - ".ValidateTokenAsync"
Cohesion: 0.33
Nodes (6): CancellationToken, Dictionary, Task, UnityWebRequest, TokenValidationResponse, WebResponse

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "Globals"
Cohesion: 0.18
Nodes (6): List, EditorUtils, string, ScriptableObjectAssetData, Globals, DirectoryInfo

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "convert_post_process"
Cohesion: 0.15
Nodes (22): AnimationClipDef, animation_take_name(), clip_id(), convert_clips(), convert_embedded_model_clips(), convert_post_process(), embedded_clip_id(), field_bool() (+14 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "ObjectiveDef"
Cohesion: 0.31
Nodes (6): ObjectiveDef, common_gathered_resource(), is_gold_collect_objective(), objective_increment(), ObjectiveEvent, ObjectiveDraft

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "TechTreeSearchWindow"
Cohesion: 0.32
Nodes (6): List, Texture2D, TechTreeSearchWindow, ISearchWindowProvider, SearchTreeEntry, SearchWindowContext

### Community 268 - "Editor"
Cohesion: 0.11
Nodes (6): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, CellPartitioningEditor, GridSystemEditor, Editor

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

### Community 273 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

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
Cohesion: 0.08
Nodes (53): apply_preview_material_overrides(), apply_preview_node_visibility(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene, player_preview_material_overrides() (+45 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.12
Nodes (16): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+8 more)

### Community 287 - "VideoSettings"
Cohesion: 0.29
Nodes (7): DisplayMode, PostProcessAntiAliasing, Option, UnitySettingsData, VideoSettings, startup_window_mode(), WindowMode

### Community 290 - "StringUtils"
Cohesion: 0.14
Nodes (4): RoleScriptablesEditor, Utils, StringUtils, ScriptablesEditor

### Community 291 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 292 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 293 - "PlayerSaveData"
Cohesion: 0.10
Nodes (16): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int (+8 more)

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 298 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (76): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot() (+68 more)

### Community 304 - "update_environment_presentation"
Cohesion: 0.06
Nodes (51): AmbientLight, SeasonalTerrainPalette, TerrainAppearanceConfig, authored_rgb_filter(), autosave_game(), blend_environment_palette(), BuildingMaterialInstance, CommandSaveRuntime (+43 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.04
Nodes (21): Player, Dictionary, GameObject, Vector3, Vector3, Action, Container, ContainerBuilder (+13 more)

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "Targetable"
Cohesion: 0.06
Nodes (17): Vector3, List, Dictionary, List, TargetRuntimeData, bool, BoxCollider, float (+9 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **395 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+390 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=4.53424798) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=3.237941093) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.962292656) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.477432826) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.366566747) _(code changed — re-verify)_
- `SkinnedMesh` (2× useful, score=1.997632118)
- `drive_tidal_music()` (2× useful, score=1.966606185) _(code changed — re-verify)_
- `WorldSimulation` (2× useful, score=1.71789778) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `IRuntimeDataScriptable`, `ScriptablesProcessorInfrastructure`, `MonoBehaviour`, `TechTree.Elements`, `StringUtils`, `MeshData`, `RandomEnabler`, `AudioHandler`, `Easings`, `IInstaller`, `Station`, `BuildingPlacer`, `RoleData`, `SnapToGridMouseMovement`, `FPSDisplay`, `Resource`, `Node_SO.cs`, `LabelDisplayProcessor`, `UpdateGraphBounds`, `.CreateEnumField`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `Option`, `stream_town_domain/src/lib.rs`, `UnityAsset`, `stream_town_game/src/lib.rs`, `convert_post_process`, `ObjectiveDef`, `config.rs`, `Vec`, `command.rs`, `simulation.rs`, `GeneratedResource`, `String`, `Ui`, `Res`, `xtask/src/main.rs`, `.default`, `.new`, `PresentationCatalog`, `String`, `WorldSimulation`, `AnimationControllerDef`, `RenderAssets`, `ToolState`, `BTreeMap`, `legacy.rs`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/presentation.rs`, `twitch.rs`, `technology_graph.rs`, `stream_town_tools/src/main.rs`, `runtime_console.rs`, `stream_town_domain/src/presentation.rs`, `component_field_value`, `world.rs`, `roles_tab`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `BuildingProcessor`, `IRuntimeDataScriptable`, `TwitchChatProcessor`, `SelectedPlayer`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `TownGoalProcessor`, `MonoBehaviour`, `SelectedPlayerGroup`, `IProcessor`, `WorldGenProcessor`, `RoleHandler`, `TechTreeProcessor`, `UIProcessor`, `VoteEvent`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `IInstaller`, `Utils`, `BuildingPlacer`, `RaidEvent`, `.Log`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `UserInterface_TownVote`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _395 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Option` be split into smaller, more focused modules?**
  _Cohesion score 0.03043298165249385 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.031267217630853994 - nodes in this community are weakly interconnected._
- **Should `SeasonProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07400555041628122 - nodes in this community are weakly interconnected._