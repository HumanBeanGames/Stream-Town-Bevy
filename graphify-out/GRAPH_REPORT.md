# Graph Report - Stream-Town-Bevy  (2026-08-23)

## Corpus Check
- 644 files · ~1,687,039 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8135 nodes · 23196 edges · 303 communities (284 shown, 19 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1023 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c2481672`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UnityAsset
- BuildingProcessor
- world.rs
- BinarySaveCodec
- ScriptableObject
- .new
- TwitchChatProcessor
- BinaryWriter
- BottomBarInterface
- Commands
- SettingsProcessor
- Targetable
- Utils
- ArchetypeDef
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- config.rs
- DebugProcessor
- Station
- .Log
- BuildingPlacer
- WorldSimulation
- UnitHealthBar
- GridPos
- Query
- TechTreeGraphView
- SaveFileData
- Player
- Coordinator.cs
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- DayAndNightProcessor
- Res
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- stream_town_game/src/lib.rs
- component_field_value
- legacy.rs
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- StableId
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- ScriptablesProcessorInfrastructure
- Option
- String
- TechTreeEditorWindow
- Result
- BuildingBase
- CameraController
- PlayerProcessor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- STSM_StateAction
- GUIDComponent
- models.rs
- Tiler
- ScriptablesEditor
- STSM_Idle_Player
- UserInterface_ObjectSelection
- PlayerRoleData
- SensorProcessor
- BTreeMap
- TwitchBotSetupWindow
- Goal
- WorldUtils
- Node_SO
- SavingAndLoading.Structs
- Access_Text
- PlayerRole
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- String
- GameEvent
- ResourceRuntimeData
- STSM_GoToLocation
- convert_fbx_to_glb.py
- UserInterface
- SelectedBuilding
- SaveProcessor
- Coordinator
- PresentationCatalog
- World.Generation.Settings
- StateMachine
- FoliageGenerationSettings
- TownGoalProcessor
- MainMenuManager
- ResourceTarget
- LoadingManager
- Access_Toggle
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- .Update
- EnemyModelHandler
- Access_Dropdown
- GridProcessor
- stream_town_migrate/src/main.rs
- Buildings
- Resource
- VoteEvent
- unity_color_filter
- PlayerInventory
- SnapToGridMouseMovement
- AIPath
- ResourceHolder
- PlayerInputProcessor
- TechTreeNode
- UpdateGraphBounds
- Vec
- AnimationHandler
- CommonEnums.cs
- WeatherProcessor
- twitch_tab
- tools_ui
- ResourceProcessor
- runtime_console.rs
- GateController
- SelectableObject
- WorldGenRuntimeData
- .RenderResourceType
- What You Must Do When Invoked
- RuntimeData Template
- String
- RuntimeData Template
- Key Rules
- BuildingResourceModelHandler
- Pet
- add_file
- RoleHandler
- .default
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- stream_town_tools/src/main.rs
- IProcessor.cs
- stream_town_migrate/src/presentation.rs
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WindController
- IProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- .CreateEnumField
- GameConfig
- .SetTargetType
- SimpleMusicController
- WorldInstanceDeterminism
- Target
- xtask/src/main.rs
- MiscCommands
- TransformSaveData
- .InjectRuntimeData
- RotationHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- PlayerControls
- PlayerSaveData
- Age
- preview_lerp_color
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ProjectCamera
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- List
- MonoBehaviour
- WorldGenerationReferenceExporter
- GUIDProcessor
- GameStateProcessor
- UserInterface_GameMenu
- Easings
- stream_town_migrate/src/menu_scene.rs
- UserInterface_TownGoal
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SaveDataMapper
- StringUtils
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- UserInterface_RulerVote
- ResourceGenerationSettings
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- BuildingModelHandler
- BuildingDataSettings
- Requirement
- CommandDictionary
- BuildingDamageMaterialHandler
- animate_healing_effects
- Key Rules
- TimeProcessor
- RuntimeData Template
- PassiveResourceIncrementer
- TerrainGenSettings
- ScriptKeywordProcessor
- FPSDisplay
- BevyMigrationExporter
- Processor Template
- Common Patterns
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Editor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- UI_TechOption
- CreateProjectScopeProcessors.cs
- load_player_settings
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- ResourceDataSettings
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- GridSettings
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxSeagullSpawner
- extraction-spec.md
- Access_GOList
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- KeepKingVote
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- EquipmentHandlerEditor
- TL_API
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- GridProcessor.cs
- Q: If there is more to do, keep going.
- Processors
- EventProcessor
- stream_town_domain
- SimpleDisableAfterTime
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- DontDestroyOnLoad
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- NewKingVote
- CampGenerationSettings
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- Access_TextInput
- import_save
- SimpleScreenShot
- RandomEnabler
- VfxParticlePosition
- StreamTown.Migration
- IntWrapper
- IRuntimeDataScriptable
- .ExportModification
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- command.rs
- VideoSettingsPresetsInstaller
- StreamUserType
- VfxAnimationController
- UI_Objective
- SimpleRandomModelEnabled
- PlacementProbeHandler

## God Nodes (most connected - your core abstractions)
1. `StableId` - 341 edges
2. `WorldSimulation` - 164 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 138 edges
8. `WorldGenProcessor` - 114 edges
9. `SettingsProcessor` - 107 edges
10. `RenderAssets` - 105 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `begin_world_loading()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `bake()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/menu_scene.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (303 total, 19 thin omitted)

### Community 0 - "UnityAsset"
Cohesion: 0.19
Nodes (41): aged_buildings(), archetype_kind(), building_cost_reductions(), building_level_caps(), convert_export(), field_value(), foliage_layers(), generated_record_ids() (+33 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "world.rs"
Cohesion: 0.10
Nodes (48): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash() (+40 more)

### Community 3 - "BinarySaveCodec"
Cohesion: 0.15
Nodes (7): Func, int, UTF8Encoding, BinarySaveCodec, int, PlayerRoleSaveData, BinaryReader

### Community 4 - "ScriptableObject"
Cohesion: 0.03
Nodes (61): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+53 more)

### Community 5 - ".new"
Cohesion: 0.03
Nodes (172): generate_world(), generate_world_with_content(), action_ranges_and_tower_acquisition_are_euclidean(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), append_terrain_quad(), append_terrain_skirt(), archetype_id_by_source() (+164 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Commands"
Cohesion: 0.07
Nodes (124): GeneratedWorld, actor_material(), actor_scene_budget(), advance_falling_fish(), agent_is_moving(), AgentAnimation, animate_agents(), animate_falling_fish() (+116 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "Targetable"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 12 - "Utils"
Cohesion: 0.06
Nodes (6): Utils, Animation, STStateMachine, Pathfinding, Character, GameResources

### Community 13 - "ArchetypeDef"
Cohesion: 0.17
Nodes (22): ArchetypeDef, ArchetypeKind, ArchetypeScene, HealthDef, RotatingNodeDef, Option, archetype_by_source(), archetype_scene_for_age() (+14 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.09
Nodes (13): PlayerDeathHandler, bool, float, Vector3, Action, float, Enemy, Action (+5 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (44): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, EnemyDef, EnemyModelSetDef (+36 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "config.rs"
Cohesion: 0.12
Nodes (25): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet, Default (+17 more)

### Community 19 - "DebugProcessor"
Cohesion: 0.11
Nodes (10): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+2 more)

### Community 20 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 21 - ".Log"
Cohesion: 0.05
Nodes (38): bool, List, ObjectPoolingSettings, Action, bool, BoxCollider, CancellationToken, Container (+30 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "WorldSimulation"
Cohesion: 0.06
Nodes (47): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather() (+39 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "GridPos"
Cohesion: 0.08
Nodes (41): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError (+33 more)

### Community 26 - "Query"
Cohesion: 0.04
Nodes (171): Aabb, Added, AnimationGraphHandle, AnimationTransitions, AudioSink, BackgroundColor, ActorAnimationDriver, ActorHealthFill (+163 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (14): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+6 more)

### Community 30 - "Coordinator.cs"
Cohesion: 0.10
Nodes (19): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+11 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (23): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+15 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 36 - "Res"
Cohesion: 0.05
Nodes (122): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, PlayerSettings, Default, AgentCommandQueue, apply_player_settings(), apply_settings_draft() (+114 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 39 - "ContentCatalog"
Cohesion: 0.11
Nodes (60): ContentCatalog, ActorState, String, action_animation_speed(), action_cooldown(), actor_accepts_resource(), actor_archetype(), actor_health_bar_hide_seconds() (+52 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.14
Nodes (11): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.10
Nodes (43): PassiveResourceContribution, asset(), building_model_definitions(), building_node_age(), component(), component_at(), component_reference_name(), component_reference_names() (+35 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (205): active_event_text(), ActorHealthOverlay, adjust_settings_menu(), advance_loading_runtime(), animate_chimney_smoke_particles(), animate_fish_school(), animate_loading_icon(), animation_property_value() (+197 more)

### Community 46 - "component_field_value"
Cohesion: 0.21
Nodes (24): ArchetypesById, archetype_bounds(), building_placements(), BuildingPlacement, component_field_value(), component_type(), convert_archetypes(), disable_after_milliseconds() (+16 more)

### Community 47 - "legacy.rs"
Cohesion: 0.18
Nodes (38): ActorCustomization, binary_fixture(), BinaryParser, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal(), json_buildings(), json_customization() (+30 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.10
Nodes (19): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+11 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "StableId"
Cohesion: 0.05
Nodes (57): ObjectiveDef, FromStr, StableId, objective_increment(), ObjectiveEvent, ObjectiveProgress, Vec, TownGoalState (+49 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (5): int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 57 - "Option"
Cohesion: 0.03
Nodes (134): AmbientLight, Assets, actor_detail_budget(), animate_weather_particles(), animation_root_name(), apply_material_overrides(), bottom_bar_button_dispatches_through_the_typed_command_queue(), BottomBarActionEnabled (+126 more)

### Community 58 - "String"
Cohesion: 0.10
Nodes (52): animation_take_name(), convert_embedded_model_clips(), convert_post_process(), field_bool(), field_f32(), field_str(), field_u64(), field_value() (+44 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "BuildingBase"
Cohesion: 0.09
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "PlayerProcessor"
Cohesion: 0.08
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "twitch.rs"
Cohesion: 0.08
Nodes (41): channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join(), OAuthClient, OAuthErrorResponse (+33 more)

### Community 68 - "Objective"
Cohesion: 0.16
Nodes (3): Action, int, Objective

### Community 69 - "STSM_StateAction"
Cohesion: 0.10
Nodes (9): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction (+1 more)

### Community 70 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.07
Nodes (19): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+11 more)

### Community 74 - "STSM_Idle_Player"
Cohesion: 0.06
Nodes (16): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_GatherResource (+8 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.05
Nodes (21): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck, SelectedEnemy, SelectedEnemyCamp, SelectedResource (+13 more)

### Community 76 - "PlayerRoleData"
Cohesion: 0.08
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+6 more)

### Community 77 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 78 - "BTreeMap"
Cohesion: 0.11
Nodes (44): animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert(), convert_avatar_masks() (+36 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "SavingAndLoading.Structs"
Cohesion: 0.10
Nodes (3): SavingAndLoading, SavingAndLoading.Structs, World.Generation

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRole"
Cohesion: 0.04
Nodes (39): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+31 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "String"
Cohesion: 0.20
Nodes (16): animation_parameter_name(), archetype_scenes(), authored_mask(), child_technology_guids(), collect_model_dependencies(), decomposes_combined_unity_flag_values(), glb_asset_path(), mask_ids() (+8 more)

### Community 91 - "GameEvent"
Cohesion: 0.05
Nodes (23): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+15 more)

### Community 92 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 93 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "UserInterface"
Cohesion: 0.06
Nodes (13): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, UserInterface (+5 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (20): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+12 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "PresentationCatalog"
Cohesion: 0.05
Nodes (79): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+71 more)

### Community 100 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "FoliageGenerationSettings"
Cohesion: 0.10
Nodes (17): Material, materials, Mesh, meshes, int, List, string, FoliageGroupSaveData (+9 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "ResourceTarget"
Cohesion: 0.26
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 108 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 114 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 115 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 116 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "Buildings"
Cohesion: 0.08
Nodes (5): BuildCostModifier, PlayerControls.ObjectSelection, World, Level, Buildings

### Community 119 - "Resource"
Cohesion: 0.06
Nodes (21): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, Container (+13 more)

### Community 120 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "PlayerInventory"
Cohesion: 0.11
Nodes (8): PlayerInventory, Dictionary, ResourceInventory, bool, int, Dictionary, float, STSM_Action_DepositResource

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "Vec"
Cohesion: 0.06
Nodes (67): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, ActivePetVisual, add_animation_composition(), add_animation_layer_branch() (+59 more)

### Community 130 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 131 - "CommonEnums.cs"
Cohesion: 0.10
Nodes (19): List, Vector3, TargetSettings, TargetableData, Dictionary, List, Foliage, FoliageSaveType (+11 more)

### Community 132 - "WeatherProcessor"
Cohesion: 0.10
Nodes (17): float, int, Material, AllSeasonSettings, Container, ContainerBuilder, WeatherProcessor, Color (+9 more)

### Community 133 - "twitch_tab"
Cohesion: 0.29
Nodes (11): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+3 more)

### Community 134 - "tools_ui"
Cohesion: 0.18
Nodes (23): content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), poll_runtime_console(), preview_grid_point(), redo_catalog_edit(), role_i32() (+15 more)

### Community 135 - "ResourceProcessor"
Cohesion: 0.12
Nodes (12): Container, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int (+4 more)

### Community 136 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 139 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 140 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "String"
Cohesion: 0.10
Nodes (35): ActorKind, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert() (+27 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "RoleHandler"
Cohesion: 0.06
Nodes (10): RoleHandler, bool, Dictionary, UnityEvent, object, UnityAction, SelectedObject, SelectedPlayer (+2 more)

### Community 150 - ".default"
Cohesion: 0.18
Nodes (17): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_game_config(), main() (+9 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "stream_town_tools/src/main.rs"
Cohesion: 0.11
Nodes (56): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), bounded_ui_index(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role() (+48 more)

### Community 154 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 155 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (82): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id(), convert_chimney_smoke(), convert_clips() (+74 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 163 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 164 - "GameConfig"
Cohesion: 0.20
Nodes (21): GameConfig, RoleDef, BTreeSet, StationDef, apply_passive_building_income(), apply_recruit_group_order(), assigned_station(), best_station_id() (+13 more)

### Community 165 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 168 - "Target"
Cohesion: 0.12
Nodes (9): Units, Target, Utils.Pooling, Sensors, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 170 - "MiscCommands"
Cohesion: 0.14
Nodes (5): Dictionary, MiscCommands, Dictionary, MessageSender, EnemyType

### Community 171 - "TransformSaveData"
Cohesion: 0.13
Nodes (12): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+4 more)

### Community 173 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "PlayerControls"
Cohesion: 0.12
Nodes (5): InputButton, UserInterface.MainMenu, MetaData, Settings, PlayerControls

### Community 177 - "PlayerSaveData"
Cohesion: 0.11
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 178 - "Age"
Cohesion: 0.06
Nodes (24): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+16 more)

### Community 179 - "preview_lerp_color"
Cohesion: 1.00
Nodes (3): preview_lerp_color(), Color32, terrain_preview_color()

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 185 - "MonoBehaviour"
Cohesion: 0.02
Nodes (84): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, Camera (+76 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "GUIDProcessor"
Cohesion: 0.14
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 188 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SaveDataMapper"
Cohesion: 0.16
Nodes (10): Mesh, Vector3, SaveDataMapper, bool, int, MeshSaveData, float, Vector2SaveData (+2 more)

### Community 195 - "StringUtils"
Cohesion: 0.14
Nodes (4): RoleScriptablesEditor, Utils, StringUtils, ScriptablesEditor

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 200 - "ResourceGenerationSettings"
Cohesion: 0.18
Nodes (9): ContainerBuilder, ResourceGenSettingsInstaller, List, ResourceGenSettings, AnimationCurve, bool, int, List (+1 more)

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 209 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 212 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 213 - "animate_healing_effects"
Cohesion: 0.33
Nodes (11): animate_healing_effects(), gradient_material(), healing_burst_effect(), healing_channel_effect(), healing_effect_curves_preserve_authored_lifetimes_and_channel_keys(), healing_effect_duration(), healing_effect_sample(), HealingEffectKind (+3 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "TimeProcessor"
Cohesion: 0.11
Nodes (9): Container, ContainerBuilder, TimeProcessor, int, string, ObjectiveSaveData, float, int (+1 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 218 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

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

### Community 229 - "Twitch setup"
Cohesion: 0.07
Nodes (24): Audio provenance, Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows, Binaries, Commands (+16 more)

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

### Community 234 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "load_player_settings"
Cohesion: 0.32
Nodes (7): AnyResult, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), player_settings_path(), PathBuf, main()

### Community 243 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Q: How are shipping visual and audio parity implemented in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How are shipping visual and audio parity implemented in the Bevy migration?, Source Nodes

### Community 248 - "GridSettings"
Cohesion: 0.25
Nodes (6): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 253 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

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

### Community 261 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "Processors"
Cohesion: 0.07
Nodes (9): ObjectSelectionProcessor, STStateMachine.States, Processors, Behaviours, Processors.Editor, Audio, STStateMachine.Helpers, VFX (+1 more)

### Community 268 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 270 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 277 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 282 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 283 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 284 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 286 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 288 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 289 - "IRuntimeDataScriptable"
Cohesion: 0.11
Nodes (16): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+8 more)

### Community 291 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 292 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 293 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 294 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 295 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 296 - "command.rs"
Cohesion: 0.06
Nodes (64): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+56 more)

### Community 297 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 298 - "StreamUserType"
Cohesion: 0.40
Nodes (4): StreamUserType, pending_stream_user_type(), should_show_actor_name(), legacy_user_type()

### Community 299 - "VfxAnimationController"
Cohesion: 0.05
Nodes (20): PersistentScoped, ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingDataContainerInstaller, Transform, PlayerSpawnPoint, AllBuildingDataSettings (+12 more)

### Community 300 - "UI_Objective"
Cohesion: 0.50
Nodes (3): Slider, TextMeshProUGUI, UI_Objective

## Knowledge Gaps
- **300 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+295 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **19 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `RenderAssets` (4× useful, score=3.481802588) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.66401759) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.54480177)
- `WorldSimulation` (2× useful, score=1.847279109)
- `load_input()` (2× useful, score=1.696288071) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (2× useful, score=1.665761652) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.664595804)
- `MaterialDef` (2× useful, score=1.664127343) _(code changed — re-verify)_
- `BevyMigrationExporter` (2× useful, score=1.63601087)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `UpdateGraphBounds`, `CommonEnums.cs`, `Processors`, `SimpleDisableAfterTime`, `BuildingPlacer`, `LabelDisplayProcessor`, `RandomEnabler`, `Coordinator.cs`, `SimpleScreenShot`, `GenerationSettings`, `.CreateEnumField`, `Target`, `VfxAnimationController`, `SimpleRandomModelEnabled`, `PlayerControls`, `Age`, `ScriptablesProcessorInfrastructure`, `MonoBehaviour`, `Easings`, `StringUtils`, `SavingAndLoading.Structs`, `FPSDisplay`, `UserInterface`, `EnemySpawner`, `Buildings`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `ResourceProcessor`, `WorldGenRuntimeData`, `DebugProcessor`, `.Log`, `Player`, `IProcessor`, `UserInterface_Debug`, `Target`, `TwitchClientProcessor`, `ProjectCamera`, `MonoBehaviour`, `GUIDProcessor`, `GameStateProcessor`, `PlayerProcessor`, `ResourceGenerationSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `TerrainGenSettings`, `GameEvent`, `SaveProcessor`, `Coordinator`, `EnemySpawner`, `GridProcessor`, `AIPath`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `UnityAsset`, `Vec`, `world.rs`, `.new`, `tools_ui`, `runtime_console.rs`, `Commands`, `ArchetypeDef`, `String`, `stream_town_domain/src/content.rs`, `save.rs`, `config.rs`, `WorldSimulation`, `GridPos`, `Query`, `stream_town_migrate/src/presentation.rs`, `stream_town_tools/src/main.rs`, `GameConfig`, `ContentCatalog`, `command.rs`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `stream_town_game/src/lib.rs`, `component_field_value`, `Option`, `String`, `stream_town_migrate/src/menu_scene.rs`, `twitch.rs`, `BTreeMap`, `animate_healing_effects`, `String`, `PresentationCatalog`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _300 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.062146892655367235 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10062893081761007 - nodes in this community are weakly interconnected._
- **Should `BinarySaveCodec` be split into smaller, more focused modules?**
  _Cohesion score 0.14982578397212543 - nodes in this community are weakly interconnected._