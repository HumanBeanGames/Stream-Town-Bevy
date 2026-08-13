# Graph Report - Stream-Town-Bevy  (2026-08-14)

## Corpus Check
- 629 files · ~1,639,098 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7519 nodes · 20516 edges · 288 communities (263 shown, 25 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1011 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a382c199`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- String
- BuildingProcessor
- .default
- .SendMessage
- TargetProcessor
- WorldGenProcessor
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- simulation.rs
- SettingsProcessor
- ResMut
- Enemy
- ShaderRef
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- SelectedBuilding
- Station
- ObjectiveSaveData
- stream_town_migrate/src/presentation.rs
- BuildingPlacer
- PlayerProcessor
- UnitHealthBar
- Targetable
- Res
- TechTreeGraphView
- SaveFileData
- Player
- stream_town_game/src/lib.rs
- TransformSaveData
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- STSM_Action_PlayerBase
- CommandDictionary
- SettingsData
- SeasonProcessor
- StableId
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- GameEvent
- AnimationControllerDef
- DebugProcessor
- .RecalculateStats
- legacy.rs
- MiscCommands
- .Draw
- AudioHandler
- StreamTownSessionBridge
- Utils
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Result
- SelectableObject
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- Option
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- settings.rs
- World.Generation.Settings
- models.rs
- Tiler
- ScriptablesEditor
- StringUtils
- UserInterface_ObjectSelection
- TimeProcessor
- Goal
- AnimationHandler
- TwitchBotSetupWindow
- Target
- WorldUtils
- SelectedObject
- convert
- Access_Text
- TL_API
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- RoleHandler
- PoolableObject
- command.rs
- BuildingResourceModelHandler
- convert_fbx_to_glb.py
- String
- RoleDataSettings
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- IProcessor.cs
- StateMachine
- BuildingBase
- TownGoalProcessor
- MainMenuManager
- RaidEvent
- LoadingManager
- SensorProcessor
- UserInterface_TownGoal
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- Access_Toggle
- EnemyModelHandler
- world.rs
- Vec
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- WorldInstanceDeterminism
- PlayerInventory
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- ScriptablesProcessorInfrastructure
- GameStateProcessor
- EditorUtils
- UpdateGraphBounds
- GlobalAudioController
- WindController
- ErrorData
- GridProcessor.cs
- BuildingSettings
- ConfirmCheck
- SeasonDataSettings
- ToolState
- GateController
- SelectedPlayerGroup
- DayAndNightSettings
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- .Log
- RuntimeData Template
- Key Rules
- stream_town_migrate/src/content.rs
- Pet
- add_file
- ResourceDataSettings
- UserInterface_GameMenu
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- UIElementWrapper
- SelectedPlayer
- UserInterface
- ResourceHolder
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- SelectedEnemyCamp
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- EventProcessor
- WeatherProcessor
- VfxAnimationController
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- UserInterface_Roles
- Processors
- Audio
- UserInterface_RulerVote
- BuildingDamageMaterialHandler
- EditorHelpers
- RoleProcessor
- SelectedEnemy
- TechTree.Elements
- HealthModifier
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- TerrainGenSettings
- FoliageGenerationSettings
- DayAndNightProcessor
- LabelDisplayProcessor
- RoleSlotModifier
- Coordinator.cs
- KeepKingVote
- PassiveResourceIncrementer
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- ObjectPoolingRuntimeData
- SimpleDisableAfterTime
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- World.Generation
- Editor
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- SelectedResource
- Common Patterns
- Easings
- CommonEnums.cs
- Requirement
- UI_TechOption
- DontDestroyOnLoad
- GridSettings
- Key Rules
- MonoBehaviour
- RuntimeData Template
- NewKingVote
- BuildingModelHandler
- ScriptKeywordProcessor
- FPSDisplay.cs
- import_save
- Processor Template
- Common Patterns
- TradeSettings
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- EquipmentHandlerEditor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- WorldSaveData
- CreateProjectScopeProcessors.cs
- PlayerSaveData
- IProcessor
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- ResourceDataSaveData
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- RotationHandler
- SimpleScreenShot
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- PlayerInputRuntimeData
- extraction-spec.md
- WorldGenSaveData
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- IntWrapper
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- PostProcessingInstaller
- AudioMixerInstaller
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- IRuntimeDataScriptable
- Q: If there is more to do, keep going.
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- PlacementProbeHandler
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- PlayerControls
- .InjectRuntimeData
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- RandomEnabler
- BuildingScriptablesEditor.cs
- Autosave
- TechTreeNode
- parse_transform_tracks
- ObjectSelectionProcessor.Editor.cs

## God Nodes (most connected - your core abstractions)
1. `StableId` - 296 edges
2. `WorldSimulation` - 162 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 126 edges
8. `WorldGenProcessor` - 110 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_assignment_penalty_spreads_farmers_across_farms()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `builder_completes_and_upgrades_authored_construction()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `agent_facing_matches_unity_rotation_and_action_targets()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (288 total, 25 thin omitted)

### Community 0 - "String"
Cohesion: 0.05
Nodes (70): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, active_event_text(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+62 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (15): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, Dictionary, int, List (+7 more)

### Community 2 - ".default"
Cohesion: 0.07
Nodes (60): App, generate_world(), archetype_id_by_source(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats(), authored_target_sizes_drive_unity_action_reach_formulas(), battering_ram_targets_and_damages_buildings_from_authored_mask(), broadcaster_gate_precedes_twitch_command_dispatch() (+52 more)

### Community 3 - ".SendMessage"
Cohesion: 0.08
Nodes (6): List, GameSettings, BuildingCommands, TwitchClientProcessor, PlayerCommands, RoleCommands

### Community 4 - "TargetProcessor"
Cohesion: 0.20
Nodes (6): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor

### Community 5 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (26): HashSet, Action, bool, BoxCollider, Container, Func, GameObject, HashSet (+18 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.05
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "simulation.rs"
Cohesion: 0.05
Nodes (41): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+33 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "ResMut"
Cohesion: 0.05
Nodes (79): AppExit, PlayerSettings, Default, advance_loading_phase(), AgentCommandQueue, autosave_game(), bottom_bar_action_buttons(), bottom_bar_input() (+71 more)

### Community 12 - "Enemy"
Cohesion: 0.11
Nodes (13): Action, float, Enemy, uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp (+5 more)

### Community 13 - "ShaderRef"
Cohesion: 0.08
Nodes (27): BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform, CritterMaterialExtension, CritterMaterialUniform (+19 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.15
Nodes (6): Action, bool, float, int, UnityEvent, HealthHandler

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.06
Nodes (66): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+58 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 19 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 20 - "ObjectiveSaveData"
Cohesion: 0.27
Nodes (5): Button, EnumField, ObjectiveVisualElement, ObjectiveSaveData, ObjectiveType

### Community 21 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.12
Nodes (40): animator_component(), animator_reference_path(), array_index(), color_value(), convert_prefab_bindings(), extracts_indexed_material_properties(), field_array(), field_bool() (+32 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.09
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "Targetable"
Cohesion: 0.08
Nodes (17): List, Vector3, List, bool, BoxCollider, float, int, Transform (+9 more)

### Community 26 - "Res"
Cohesion: 0.05
Nodes (169): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, BackgroundColor (+161 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.09
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, GameMasterCommands, ModeratorCommands, RulerCommands (+1 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (187): AnyResult, actor_detail_budget(), actor_scene_budget(), ActorHealthFill, ActorHealthOverlay, adjust_settings_menu(), advance_loading_runtime(), AgentEnemyModelPresentation (+179 more)

### Community 31 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - "STSM_Action_PlayerBase"
Cohesion: 0.09
Nodes (7): int, STSM_Helper_Attack, int, STSM_Action_Attack, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 36 - "CommandDictionary"
Cohesion: 0.27
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 39 - "StableId"
Cohesion: 0.04
Nodes (185): GameConfig, ContentCatalog, StationDef, FromStr, StableId, GridPos, ActorState, BuildingState (+177 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "DebugProcessor"
Cohesion: 0.08
Nodes (13): Dictionary, DebugSettings, Container, ContainerBuilder, DebugLogCategory, DebugProcessor, STSM_Action_GatherResource, bool (+5 more)

### Community 46 - ".RecalculateStats"
Cohesion: 0.17
Nodes (8): StatModifiers, Dictionary, Dictionary, List, Queue, Transform, PlayerRuntimeData, StatType

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (37): ActorCustomization, StreamUserType, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies() (+29 more)

### Community 48 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 49 - ".Draw"
Cohesion: 0.11
Nodes (18): NodeUnlockSaveData, Port, Button, EnumField, UnlockVisualElement, Action, Button, EnumField (+10 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "Utils"
Cohesion: 0.05
Nodes (10): STStateMachine.States, Units, Utils, Behaviours, Animation, Sensors, STStateMachine, Pathfinding (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (18): Client, TwitchClientRuntimeData, OnChatCommandReceivedArgs, Client, Container, ContainerBuilder, IEnumerator, LogType (+10 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Result"
Cohesion: 0.14
Nodes (36): MaterialDef, animation_take_name(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), clip_id(), collect_prefab_dependencies(), controller_id(), convert() (+28 more)

### Community 57 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.31
Nodes (4): BinaryParser<'a>, decode_binary(), Result, LegacyWorldState

### Community 61 - "Option"
Cohesion: 0.04
Nodes (147): AmbientLight, Assets, AssetServer, PresentationCatalog, actor_material(), agent_facing_matches_unity_rotation_and_action_targets(), animation_property_value(), animation_root_name() (+139 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.15
Nodes (12): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+4 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.08
Nodes (15): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+7 more)

### Community 67 - "twitch.rs"
Cohesion: 0.06
Nodes (54): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet, Default (+46 more)

### Community 68 - "Objective"
Cohesion: 0.15
Nodes (3): Action, int, Objective

### Community 69 - "settings.rs"
Cohesion: 0.11
Nodes (27): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+19 more)

### Community 70 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "TimeProcessor"
Cohesion: 0.21
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 77 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.08
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+7 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "Target"
Cohesion: 0.15
Nodes (7): Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies, GUIDSystem

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 83 - "convert"
Cohesion: 0.17
Nodes (17): ActorKind, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert() (+9 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "TL_API"
Cohesion: 0.06
Nodes (13): Api, PersistentScoped, TL_API, Image, TextMeshProUGUI, UIRoleDisplay, List, SimpleEventOnStart (+5 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.11
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "RoleHandler"
Cohesion: 0.07
Nodes (21): PlayerRoleData, AudioClip, bool, float, int, RoleData, AudioClip, bool (+13 more)

### Community 91 - "PoolableObject"
Cohesion: 0.07
Nodes (12): Container, ContainerBuilder, GUIDProcessor, Component, Transform, bool, Dictionary, GUIDRuntimeData (+4 more)

### Community 92 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 93 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "String"
Cohesion: 0.13
Nodes (36): AnimationParameterDef, animation_state_id(), animation_state_machine_id(), infer_missing_parameters(), inline_file_id(), normalized_path(), parse_avatar_mask(), parse_blend_tree() (+28 more)

### Community 96 - "RoleDataSettings"
Cohesion: 0.09
Nodes (16): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AudioClip, bool (+8 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (21): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+13 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (44): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+36 more)

### Community 100 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "BuildingBase"
Cohesion: 0.09
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.14
Nodes (8): LoadType, MetaData, Button, GameObject, IEnumerator, int, MainMenuManager, Inject

### Community 105 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 108 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 114 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 115 - "world.rs"
Cohesion: 0.05
Nodes (56): WorldGenConfig, round_trips_through_serde(), Err, Formatter, Into, Result, Self, String (+48 more)

### Community 116 - "Vec"
Cohesion: 0.16
Nodes (19): binary_fixture(), BinaryParser, legacy_objective_matches(), LegacyDecodedSave, LegacyGoal, LegacyObjective, objective_target_matches(), put_f32() (+11 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.06
Nodes (21): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, Container (+13 more)

### Community 120 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 121 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 122 - ".SetTargetType"
Cohesion: 0.17
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 126 - "GameStateProcessor"
Cohesion: 0.15
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 127 - "EditorUtils"
Cohesion: 0.15
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.13
Nodes (13): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+5 more)

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 132 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 133 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (86): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+78 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectedPlayerGroup"
Cohesion: 0.15
Nodes (3): List, List, SelectedPlayerGroup

### Community 139 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - ".Log"
Cohesion: 0.06
Nodes (23): HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder (+15 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (123): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+115 more)

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 155 - "UserInterface"
Cohesion: 0.06
Nodes (13): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, UserInterface (+5 more)

### Community 156 - "ResourceHolder"
Cohesion: 0.19
Nodes (7): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, SaveableResource

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 163 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 164 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 165 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 166 - "VfxAnimationController"
Cohesion: 0.13
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 167 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (19): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+11 more)

### Community 170 - "UserInterface_Roles"
Cohesion: 0.20
Nodes (6): bool, Dictionary, GameObject, Transform, UserInterface_Roles, Color32

### Community 171 - "Processors"
Cohesion: 0.07
Nodes (7): BuildCostModifier, PlayerControls.ObjectSelection, Processors, World, Level, Buildings, GameResources

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 174 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "RoleProcessor"
Cohesion: 0.18
Nodes (4): Container, ContainerBuilder, int, RoleProcessor

### Community 178 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 179 - "HealthModifier"
Cohesion: 0.22
Nodes (6): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 186 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 187 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 188 - "LabelDisplayProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 190 - "Coordinator.cs"
Cohesion: 0.08
Nodes (21): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+13 more)

### Community 192 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 195 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 200 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

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

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "CommonEnums.cs"
Cohesion: 0.10
Nodes (15): PlayerDeathHandler, bool, float, Vector3, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus (+7 more)

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 213 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "MonoBehaviour"
Cohesion: 0.02
Nodes (86): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, Camera (+78 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 218 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 221 - "import_save"
Cohesion: 0.43
Nodes (7): absolute_path(), backup_candidate(), import_preserves_source_and_recovers_named_backup(), import_save(), ImportReport, Path, PathBuf

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "TradeSettings"
Cohesion: 0.33
Nodes (5): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller

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
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

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

### Community 234 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "PlayerSaveData"
Cohesion: 0.06
Nodes (26): Dictionary, Mesh, Vector3, SaveDataMapper, bool, int, List, string (+18 more)

### Community 240 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 243 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 248 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 253 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 262 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "IRuntimeDataScriptable"
Cohesion: 0.11
Nodes (17): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, string (+9 more)

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 268 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 269 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 270 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "PlayerControls"
Cohesion: 0.12
Nodes (5): InputButton, UserInterface.MainMenu, MetaData, Settings, PlayerControls

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (62): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, WorldGenDebugSettingsInstaller, int, AudioSettings, List, CampGenSettings (+54 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 289 - "TechTreeNode"
Cohesion: 0.10
Nodes (13): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Group (+5 more)

### Community 293 - "parse_transform_tracks"
Cohesion: 0.33
Nodes (6): append_vec3_keys(), parse_inline_array(), parse_transform_tracks(), parses_unity_transform_curves_without_editor_types(), Item, Iterator

## Knowledge Gaps
- **284 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+279 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `.SendMessage`, `WorldGenProcessor`, `SettingsProcessor`, `.Log`, `UserInterface_GameMenu`, `PlayerProcessor`, `ScriptableObject`, `SaveFileData`, `GameEventProcessor`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `DebugProcessor`, `RoleProcessor`, `StreamTownSessionBridge`, `Coordinator.cs`, `TimeProcessor`, `MonoBehaviour`, `FoliageProcessor`, `PoolableObject`, `TownGoalProcessor`, `MainMenuManager`, `WorldSaveData`, `PlayerSaveData`, `IProcessor`, `ResourceDataSaveData`, `Resource`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `.Log`, `.InjectRuntimeData`, `PlayerProcessor`, `ScriptableObject`, `Player`, `WorldGenRuntimeData`, `UserInterface_Debug`, `GridProcessor`, `ResourceProcessor`, `DebugProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `TerrainGenSettings`, `Target`, `CellSpacePartitioning`, `MonoBehaviour`, `FoliageProcessor`, `PoolableObject`, `SaveProcessor`, `Coordinator`, `RaidEvent`, `EnemySpawner`, `IProcessor`, `AIPath`, `GameStateProcessor`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `.SendMessage`, `TwitchChatProcessor`, `Enemy`, `HealthHandler`, `Pet`, `Station`, `BuildingPlacer`, `PlayerProcessor`, `UserInterface_Debug`, `CommandDictionary`, `CharacterModelHandler`, `UserInterface_Roles`, `.RecalculateStats`, `RoleProcessor`, `MiscCommands`, `TwitchClientProcessor`, `LabelDisplayProcessor`, `Coordinator.cs`, `UserInterface_DisplayUsernames`, `Target`, `TargetSensor`, `RoleHandler`, `PoolableObject`, `SaveProcessor`, `RaidEvent`, `VoteEvent`, `.SetTargetType`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _284 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `String` be split into smaller, more focused modules?**
  _Cohesion score 0.0525879917184265 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07428571428571429 - nodes in this community are weakly interconnected._
- **Should `.default` be split into smaller, more focused modules?**
  _Cohesion score 0.06557377049180328 - nodes in this community are weakly interconnected._