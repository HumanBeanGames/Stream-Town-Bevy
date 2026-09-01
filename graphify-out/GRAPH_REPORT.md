# Graph Report - Stream-Town-Bevy  (2026-09-01)

## Corpus Check
- 670 files · ~1,815,082 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9363 nodes · 28109 edges · 307 communities (281 shown, 26 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1059 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0fe5f5dd`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- WorldGenRuntimeData
- BuildingProcessor
- stream_town_migrate/src/presentation.rs
- SeasonProcessor
- Res
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- Utils
- BottomBarInterface
- .new
- SettingsProcessor
- UserInterface_Debug
- config.rs
- ScriptablesProcessorInfrastructure
- TechTreeIOUtility
- HealthHandler
- .count
- save.rs
- String
- command.rs
- twitch.rs
- NavGrid
- .Log
- .default
- Result
- generate_and_spawn_world
- Node_SO
- SimulationError
- SaveFileData
- Option
- ResMut
- WorldGenProcessor
- CellSpacePartitioning
- String
- GenerationSettings
- .SetTargetType
- UnityAsset
- SettingsData
- RenderAssets
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BuildingBase
- AnimationControllerDef
- MainMenuManager
- legacy.rs
- Option
- .GetResourceAssets
- stream_town_migrate/src/main.rs
- AudioHandler
- StreamTownSessionBridge
- GlobalAudioController
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- Value
- Station
- MonoBehaviour
- TechTreeEditorWindow
- drive_tidal_music
- stream_town_domain/src/content.rs
- CameraController
- STSM_StateAction
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- GameEventProcessor
- GamestateJukebox
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- simulation.rs
- GameStateProcessor
- runtime_console.rs
- ObjectPoolingProcessor
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- UIElementWrapper
- Access_Text
- TwitchUser
- UI_TechOption
- .new
- settings.rs
- SaveProcessor
- SnapToGridMouseMovement
- RaidEvent
- world.rs
- String
- convert_fbx_to_glb.py
- StableId
- Resource
- .LoadGameAsync
- WorldSimulation
- PresentationCatalog
- Coordinator
- StateMachine
- component_field_value
- TownGoalProcessor
- .RenderResourceType
- ResourceProcessor
- LoadingManager
- DayAndNightProcessor
- GUIDComponent
- CustomLogHandler
- LevelHandler
- ResourceHolder
- TargetSensor
- BuildingDataSettings
- ScriptablesEditor
- RoleHandler
- stream_town_tools/src/main.rs
- PlayerInputProcessor
- List
- .EnsureValidCredentials
- VoteEvent
- ResourceRuntimeData
- SelectedPlayerGroup
- SelectableObject
- AIPath
- Targetable
- GateController
- direct_broadcast.rs
- WindController
- .StartupSequence
- Option
- TimeProcessor
- EnemySpawner
- stream_town_domain/src/lib.rs
- TownGoal.Data
- Editor
- SensorProcessor
- DirectBroadcastRuntime
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- DirectBroadcastControl
- RuntimeData Template
- Key Rules
- ConfirmCheck
- DontDestroyOnLoad
- xtask/src/lib.rs
- WeatherProcessor
- World.Generation
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- TargetProcessor
- TL_Secrets
- LoadingWorkNode
- .Draw
- Stream Town Reloaded - Architecture Documentation
- .UserIsSubscribed
- Instant
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_ObjectSelection
- DirectBroadcastSnapshot
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- SimpleMusicController
- Q: There are still no animations.
- xtask/src/main.rs
- NativeGameAudioRouting
- UserInterface_BuildingHealthBar
- .new
- CommonEnums.cs
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- Access_Toggle
- SelectedObject
- ErrorData
- scalar
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- VfxAnimationController
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- BuildingDamageMaterialHandler
- Easings
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- SelectedEnemy
- FoliageGenerationSettings.cs
- stream_town_migrate/src/menu_scene.rs
- SelectedResource
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- foliage_visual_variant
- IRuntimeDataScriptable
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- BevyMigrationExporter
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- .RefreshSceneBindingsAndTryGenerate
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- SimpleDisableAfterTime
- draw_world_preview
- Vec
- StationProcessor
- DebugSettings
- SelectedEnemyCamp
- Key Rules
- .InitializeAndActivateProcessorsAsync
- RuntimeData Template
- Character Animation Regression Checklist
- OpenNode
- ScriptKeywordProcessor
- FPSDisplay
- TechTreeNode
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
- hash_world
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- RandomEnabler
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- record_gpu_readiness
- extraction-spec.md
- SelectedBuilding
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- SimpleScreenShot
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- PlayerInputRuntimeData
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Sensors
- Q: If there is more to do, keep going.
- Target
- PlacementProbeHandler
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Processors
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- SimpleRotateOnAxis
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxSeagullSpawner
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch commands
- horizontal_hash
- Character
- StreamTown.Migration
- UI_Objective
- Requirement
- UIRoleDisplay
- TL_API
- CreateProjectScopeProcessors.cs
- TwitchClientRuntimeData
- ToolState
- SaveDataMapper
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- Autosave
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- main
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- vcpkg.json
- .ExportModification
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 419 edges
2. `ContentCatalog` - 182 edges
3. `WorldSimulation` - 178 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `ToolState` - 138 edges
9. `RenderAssets` - 134 edges
10. `WorldGenProcessor` - 114 edges

## Surprising Connections (you probably didn't know these)
- `generate_world()` --calls--> `default_resource_generation_layers()`  [INFERRED]
  bevy-port/crates/stream_town_domain/src/world.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `embedded_config_matches_shipping_starting_roster()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generated_terrain_chunks_cover_the_grid_with_watertight_seams()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (307 total, 26 thin omitted)

### Community 0 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.05
Nodes (20): BuildPlacerData, GameObject, Renderer, string, Vector2, bool, Dictionary, int (+12 more)

### Community 2 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (105): animation_state_id(), animation_state_machine_id(), animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), clip_id(), color_value() (+97 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 4 - "Res"
Cohesion: 0.05
Nodes (190): AnimationTransitions, AudioSink, ActivePetVisual, ActorNameOverlay, Agent, agent_is_moving(), AgentAnimation, animate_agents() (+182 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (259): AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, actor_detail_budget(), actor_scene_budget(), ActorHealthFill, ActorHealthOverlay, adjust_settings_menu() (+251 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (28): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+20 more)

### Community 7 - "Utils"
Cohesion: 0.05
Nodes (14): BuildCostModifier, DisableOnAwake, SelectionBase, List, SimpleEventOnStart, SimpleHideRendererOnAwake, GameObject, SimpleRandomModelEnabled (+6 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - ".new"
Cohesion: 0.02
Nodes (174): AccessibilityActionRequest, AmbientLight, accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean(), ActiveMaterialHandles (+166 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 12 - "config.rs"
Cohesion: 0.11
Nodes (27): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig (+19 more)

### Community 13 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (5): int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (14): Func, Action, float, Enemy, Action, Container, ContainerBuilder, EventProcessor (+6 more)

### Community 16 - ".count"
Cohesion: 0.06
Nodes (66): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), authored_rotating_node_names(), begin_world_loading() (+58 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "String"
Cohesion: 0.02
Nodes (179): AccessibleNode, AnimatedBy, AnimationClip, AnimationGraphHandle, AnimationTargetId, active_event_text(), ActorAnimationDriver, add_animation_composition() (+171 more)

### Community 19 - "command.rs"
Cohesion: 0.17
Nodes (33): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+25 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (65): BTreeSet, TwitchConfig, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization (+57 more)

### Community 21 - "NavGrid"
Cohesion: 0.17
Nodes (15): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building(), reconstruct_path() (+7 more)

### Community 22 - ".Log"
Cohesion: 0.05
Nodes (20): Action, Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, int, STSM_Helper_Build (+12 more)

### Community 23 - ".default"
Cohesion: 0.08
Nodes (46): authority_tab(), character_model_choices_include_converted_hierarchy_nodes(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_presentation_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation() (+38 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - "generate_and_spawn_world"
Cohesion: 0.04
Nodes (121): ArchetypeScene, actor_combat_visual(), apply_authored_main_menu_camera(), archetype_by_source(), archetype_scene_for_age(), authored_scene_rotation(), building_effect_material(), BuildingEffectKind (+113 more)

### Community 26 - "Node_SO"
Cohesion: 0.11
Nodes (14): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+6 more)

### Community 27 - "SimulationError"
Cohesion: 0.16
Nodes (5): complete_gameplay_scenario_round_trips(), Result, SimulationError, technology_vote_starts_persistent_goal_and_unlocks_after_all_objectives(), validate_trade_resource()

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Option"
Cohesion: 0.26
Nodes (22): animation_assets_editor(), archetype_kind_choice(), content_tab(), content_tab_contents(), draw_building_visual(), draw_footprint_grid(), draw_model_preview(), enemy_camp_generation_editor() (+14 more)

### Community 30 - "ResMut"
Cohesion: 0.03
Nodes (164): AccessibilityFocusVisualQuery, AnyResult, PlayerSettings, Default, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_settings_selection() (+156 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (29): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+21 more)

### Community 32 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 33 - "String"
Cohesion: 0.11
Nodes (45): AnimationParameterDef, animator_component(), animator_reference_path(), collect_prefab_dependencies(), convert_prefab_bindings(), convert_prefab_materials(), infer_missing_parameters(), inline_file_id() (+37 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "UnityAsset"
Cohesion: 0.20
Nodes (42): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), enemy_camp_generation_layers(), field_value(), generated_record_ids(), objective_definitions() (+34 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "RenderAssets"
Cohesion: 0.03
Nodes (114): Added, AccessibilityMotionDefaults, actor_material(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension (+106 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.05
Nodes (19): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+11 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "BuildingBase"
Cohesion: 0.09
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (28): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+20 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "legacy.rs"
Cohesion: 0.11
Nodes (47): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id() (+39 more)

### Community 47 - "Option"
Cohesion: 0.03
Nodes (259): GameConfig, ContentCatalog, DirtyRegion, GridPos, ActorState, String, generate_world(), generate_world_with_content() (+251 more)

### Community 48 - ".GetResourceAssets"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 49 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.08
Nodes (18): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+10 more)

### Community 52 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "Value"
Cohesion: 0.20
Nodes (26): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+18 more)

### Community 57 - "Station"
Cohesion: 0.09
Nodes (12): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+4 more)

### Community 58 - "MonoBehaviour"
Cohesion: 0.01
Nodes (144): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+136 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "drive_tidal_music"
Cohesion: 0.17
Nodes (26): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature, player_music_gain() (+18 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (50): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+42 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "STSM_StateAction"
Cohesion: 0.05
Nodes (20): RotationHandler, float, Quaternion, Vector3, bool, int, List, EnemyModelHandler (+12 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (56): animation_parameter_name(), archetype_scenes(), asset(), authored_mask(), building_placements(), BuildingPlacement, child_technology_guids(), collect_model_dependencies() (+48 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "simulation.rs"
Cohesion: 0.10
Nodes (27): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+19 more)

### Community 74 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 75 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 76 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (47): Container, ContainerBuilder, GUIDProcessor, Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor (+39 more)

### Community 77 - "Objective"
Cohesion: 0.08
Nodes (12): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+4 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.06
Nodes (69): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+61 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.10
Nodes (20): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+12 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 83 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 86 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 87 - ".new"
Cohesion: 0.13
Nodes (18): apply_direct_broadcast_control(), closing_the_operator_window_requests_a_graceful_game_exit(), configure_direct_broadcast(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting() (+10 more)

### Community 88 - "settings.rs"
Cohesion: 0.10
Nodes (33): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+25 more)

### Community 89 - "SaveProcessor"
Cohesion: 0.05
Nodes (44): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+36 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - "world.rs"
Cohesion: 0.16
Nodes (30): WorldGenConfig, authored_grid_centre(), authored_world_to_grid(), cell_hash(), changing_seed_changes_world_hash(), foliage_horizontal_hash(), generate_authored_resources(), generate_candidate_mask() (+22 more)

### Community 93 - "String"
Cohesion: 0.09
Nodes (70): ability_choices(), action_animation_choices(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice() (+62 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "StableId"
Cohesion: 0.11
Nodes (20): ObjectiveDef, Display, FromStr, StableId, BuildingState, objective_increment(), ObjectiveEvent, ObjectiveProgress (+12 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (43): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+35 more)

### Community 97 - ".LoadGameAsync"
Cohesion: 0.07
Nodes (34): Action, CancellationToken, List, Task, Transform, int, List, string (+26 more)

### Community 98 - "WorldSimulation"
Cohesion: 0.12
Nodes (13): EnemyCampState, FishGodState, RaidState, BTreeSet, Option, VecDeque, ruler_vote_rejects_duplicates_and_invalid_candidates(), TownEvent (+5 more)

### Community 99 - "PresentationCatalog"
Cohesion: 0.05
Nodes (90): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef (+82 more)

### Community 100 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 102 - "component_field_value"
Cohesion: 0.18
Nodes (30): ArchetypesById, EnemyDef, ResourceReward, archetype_bounds(), building_model_definitions(), building_node_age(), component_field_value(), component_reference_name() (+22 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.08
Nodes (24): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+16 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "DayAndNightProcessor"
Cohesion: 0.13
Nodes (7): Container, ContainerBuilder, DayAndNightProcessor, IPostInitializeProcessor, bool, float, DayAndNightRuntimeData

### Community 108 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.17
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 112 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 113 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 114 - "ScriptablesEditor"
Cohesion: 0.06
Nodes (20): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+12 more)

### Community 115 - "RoleHandler"
Cohesion: 0.02
Nodes (58): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleData (+50 more)

### Community 116 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (63): animation_property_curves_editor(), apply_building_draft(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft (+55 more)

### Community 117 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 118 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 119 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 120 - "VoteEvent"
Cohesion: 0.06
Nodes (23): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, TechVote (+15 more)

### Community 121 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 123 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 124 - "AIPath"
Cohesion: 0.11
Nodes (21): bool, float, int, string, Type, Vector3, AIPath, AstarData (+13 more)

### Community 126 - "Targetable"
Cohesion: 0.14
Nodes (7): bool, BoxCollider, float, int, Transform, Vector3, Targetable

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (72): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), AuthorizationEvent, bandwidth_test_url_is_constructed_without_logging_the_key(), bounded_history_f32(), BroadcastPrerequisites, build_ingest_url(), CadenceTick, configure_amf_quality() (+64 more)

### Community 129 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 130 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 131 - "Option"
Cohesion: 0.11
Nodes (39): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, BroadcastTarget, capture_process_audio(), discard_pending_audio() (+31 more)

### Community 132 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 135 - "TownGoal.Data"
Cohesion: 0.05
Nodes (23): InputButton, SharedTypes, int, ChangeTimeStamp, Vector2, GroupSaveData, List, TechTreeSaveData_SO (+15 more)

### Community 136 - "Editor"
Cohesion: 0.04
Nodes (24): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, RoleScriptablesEditor (+16 more)

### Community 137 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 138 - "DirectBroadcastRuntime"
Cohesion: 0.06
Nodes (60): arm_stream_only_readback(), BroadcastMetricsSnapshot, camera_targets_primary_window(), capture_direct_broadcast_frame(), cleanup_completed_stream_only_readbacks(), DirectBroadcastPhase, DirectBroadcastRuntime, disarm_stream_only_readbacks() (+52 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "Result"
Cohesion: 0.10
Nodes (27): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), encoder_candidates(), encoder_input_format(), encoder_is_hardware() (+19 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "DirectBroadcastControl"
Cohesion: 0.15
Nodes (10): DirectBroadcastControl, exit_after_broadcast_stops(), operator_window_close_requests_exit(), AppExit, MessageReader, MessageWriter, stream_operator_restart_button(), StreamOperatorWindow (+2 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "WeatherProcessor"
Cohesion: 0.10
Nodes (17): float, int, Material, AllSeasonSettings, Container, ContainerBuilder, WeatherProcessor, Color (+9 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 154 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 155 - "LoadingWorkNode"
Cohesion: 0.36
Nodes (5): LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, WorldLoadingWork

### Community 156 - ".Draw"
Cohesion: 0.07
Nodes (31): List, Vector2, NodeSaveData, NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Port (+23 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 159 - "Instant"
Cohesion: 0.22
Nodes (9): bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastStopDisposition, duration_as_micros(), Duration, Instant, stream_readback_due(), video_cadence_skips_stale_slots_instead_of_bursting_after_a_stall() (+1 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 163 - "DirectBroadcastSnapshot"
Cohesion: 0.20
Nodes (11): append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), average_milliseconds(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), direct_broadcast_log_path(), DirectBroadcastSnapshot, micros_to_milliseconds(), rate_per_second() (+3 more)

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 167 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "NativeGameAudioRouting"
Cohesion: 0.23
Nodes (5): NativeGameAudioRouting, pcm16_wav_clip(), pcm16_wav_data(), Default, stream_only_game_audio_is_muted_locally_and_mixed_before_the_monitor()

### Community 171 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 172 - ".new"
Cohesion: 0.27
Nodes (7): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), generated_instance_counts_match_the_sanitized_unity_save_oracle(), positive_noise_offset(), Self, SystemRandom, two_variant_foliage_hash_does_not_form_coordinate_parity_rows()

### Community 173 - "CommonEnums.cs"
Cohesion: 0.05
Nodes (26): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+18 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 177 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 178 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 179 - "scalar"
Cohesion: 0.12
Nodes (37): append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id(), json_f32(), parses_authored_fireworks_graph_parameters(), prefab_disable_lifetime() (+29 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (84): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+76 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 190 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "foliage_visual_variant"
Cohesion: 0.28
Nodes (9): avalanche_instance_hash(), fnv_mix(), foliage_visual_variant(), foliage_visual_yaw_milliradians(), resource_visual_variant(), stable_string_hash(), two_variant_resource_hash_does_not_form_coordinate_parity_rows(), unity_instance_index() (+1 more)

### Community 195 - "IRuntimeDataScriptable"
Cohesion: 0.03
Nodes (37): Container, ContainerBuilder, LabelDisplayProcessor, Container, ContainerBuilder, GridProcessor, CreditsRuntimeData, float (+29 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.22
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

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.07
Nodes (24): int, string, ObjectiveSaveData, List, SaveGameData, bool, float, List (+16 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 209 - "draw_world_preview"
Cohesion: 0.29
Nodes (8): draw_world_preview(), preview_grid_point(), preview_lerp_color(), Color32, Pos2, Rect, terrain_preview_color(), WorldPreviewLayer

### Community 210 - "Vec"
Cohesion: 0.43
Nodes (8): binary_fixture(), BinaryParser, put_f32(), put_i32(), put_string(), put_u32(), Vec, Cursor

### Community 211 - "StationProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 212 - "DebugSettings"
Cohesion: 0.48
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.17
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "TechTreeNode"
Cohesion: 0.05
Nodes (32): ChildrenSaveData, Color, float, string, TechnologyTreeGroup, Color, Foldout, List (+24 more)

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
Cohesion: 0.20
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

### Community 234 - "hash_world"
Cohesion: 0.52
Nodes (7): hash_world(), legacy_resource_navigation(), legacy_v1_world_hash(), legacy_v2_world_hash(), legacy_v3_world_hash(), legacy_variable_resource_amounts(), String

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

### Community 243 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

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

### Community 251 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), ErasedRenderAssets, GpuImage, GpuRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "Sensors"
Cohesion: 0.07
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "Target"
Cohesion: 0.09
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

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

### Community 273 - "Processors"
Cohesion: 0.04
Nodes (17): ObjectSelectionProcessor, InputButton, Transform, PlayerSpawnPoint, Vector3, UnityGraphics, UserInterface.MainMenu, Processors (+9 more)

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.08
Nodes (42): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+34 more)

### Community 285 - "Stream Town Twitch commands"
Cohesion: 0.40
Nodes (4): Moderator and game-master commands, Player commands, Ruler and operator commands, Stream Town Twitch commands

### Community 287 - "horizontal_hash"
Cohesion: 0.67
Nodes (4): horizontal_hash(), Item, Iterator, shoreline_approaches()

### Community 288 - "Character"
Cohesion: 0.09
Nodes (16): TextMeshProUGUI, UI_VoteObjectiveRow, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Character.Enumerations, Core, Pets (+8 more)

### Community 290 - "UI_Objective"
Cohesion: 0.50
Nodes (3): Slider, TextMeshProUGUI, UI_Objective

### Community 291 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 292 - "UIRoleDisplay"
Cohesion: 0.50
Nodes (3): Image, TextMeshProUGUI, UIRoleDisplay

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 299 - "ToolState"
Cohesion: 0.08
Nodes (72): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot() (+64 more)

### Community 304 - "SaveDataMapper"
Cohesion: 0.08
Nodes (19): Dictionary, Mesh, Vector3, SaveDataMapper, bool, int, List, string (+11 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.04
Nodes (22): Player, Dictionary, GameObject, Vector3, Vector3, Action, Container, ContainerBuilder (+14 more)

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **383 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+378 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=4.53424798) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=3.237941093) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.962292656) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.477432826) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.366566747)
- `SkinnedMesh` (2× useful, score=1.997632118)
- `drive_tidal_music()` (2× useful, score=1.966606185) _(code changed — re-verify)_
- `WorldSimulation` (2× useful, score=1.71789778) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `SimpleScreenShot`, `EnemySpawner`, `TownGoal.Data`, `Sensors`, `Editor`, `Target`, `ScriptablesProcessorInfrastructure`, `Processors`, `SimpleRotateOnAxis`, `World.Generation`, `Character`, `GenerationSettings`, `UpdateGraphBounds`, `CommonEnums.cs`, `Easings`, `IRuntimeDataScriptable`, `BuildingPlacer`, `SimpleDisableAfterTime`, `SnapToGridMouseMovement`, `FPSDisplay`, `ScriptablesEditor`, `RandomEnabler`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `stream_town_migrate/src/presentation.rs`, `Res`, `stream_town_game/src/lib.rs`, `stream_town_domain/src/lib.rs`, `.new`, `config.rs`, `save.rs`, `String`, `command.rs`, `twitch.rs`, `.default`, `generate_and_spawn_world`, `SimulationError`, `Option`, `String`, `UnityAsset`, `RenderAssets`, `ToolState`, `AnimationControllerDef`, `legacy.rs`, `Option`, `scalar`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `foliage_visual_variant`, `stream_town_migrate/src/content.rs`, `simulation.rs`, `runtime_console.rs`, `technology_graph.rs`, `draw_world_preview`, `String`, `WorldSimulation`, `PresentationCatalog`, `component_field_value`, `stream_town_tools/src/main.rs`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `BuildingProcessor`, `TimeProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `Target`, `UserInterface_Debug`, `HealthHandler`, `WorldGenProcessor`, `TechTreeProcessor`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `IRuntimeDataScriptable`, `GameEventProcessor`, `ObjectPoolingProcessor`, `BuildingPlacer`, `SaveProcessor`, `RaidEvent`, `Resource`, `RoleHandler`, `VoteEvent`, `SelectedPlayerGroup`?**
  _High betweenness centrality (0.023) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _383 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05110809588421529 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/presentation.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05691471287801563 - nodes in this community are weakly interconnected._
- **Should `SeasonProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.1053763440860215 - nodes in this community are weakly interconnected._