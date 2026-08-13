# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 609 files · ~1,605,079 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7023 nodes · 18449 edges · 267 communities (249 shown, 18 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 994 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f34b3587`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- GridPos
- BuildingProcessor
- StableId
- stream_town_migrate/src/presentation.rs
- String
- EnemyModelHandler
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- Sensors
- SettingsProcessor
- WorldGenProcessor
- Player
- RoleHandler
- TechTreeIOUtility
- MonoBehaviour
- Character
- Utils
- .GenerateFromSettings
- legacy.rs
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- PlayerRoleData
- TechTreeRuntimeData
- Result
- SaveFileData
- GameEventProcessor
- ContentCatalog
- BuildingResourceModelHandler
- STSM_Idle_Player
- ResourceProcessor
- Station
- UserInterface_Debug
- MiscCommands
- SettingsData
- AnimationHandler
- StreamUserType
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- LabelDisplayProcessor
- AnimationControllerDef
- save.rs
- SeasonProcessor
- DebugSettings
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceData
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- SelectedPlayer
- WorldSimulation
- STSM_GoToLocation
- TechTreeEditorWindow
- VfxSeagullSpawner
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- EnemyCampSaveData
- MeshData
- models.rs
- Tiler
- ScriptablesEditor
- Result
- UserInterface_ObjectSelection
- STSM_Action_PlayerBase
- ProjectCamera
- GridProcessor
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- GameEvent
- update_environment_presentation
- Access_Text
- stream_town_game/src/lib.rs
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- Option
- RoleProcessor
- RaidEvent
- Access_Dropdown
- convert_fbx_to_glb.py
- .new
- stream_town_migrate/src/content.rs
- BinarySaveCodec
- Coordinator
- stream_town_domain/src/presentation.rs
- Editor
- IProcessor.cs
- ResourceRuntimeData
- TownGoalProcessor
- MainMenuManager
- UnitHealthBar
- LoadingManager
- WorldGenSaveData
- ResourceHolder
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- STSM_StateAction
- RotationHandler
- IRuntimeDataScriptable
- VoteEvent
- Resource
- PlayerInventory
- process_injected_commands
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Targetable
- BuildingBase
- CommandDictionary
- UpdateGraphBounds
- SelectableObject
- stream_town_migrate/src/main.rs
- WorldInstanceDeterminism
- .RenderResourceType
- GameStateProcessor
- ConfirmCheck
- STSM_Idle
- ToolState
- UnitTextDisplay
- SensorProcessor
- STSM_Helper_Attack
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- UI_TechOption
- Requirement
- DontDestroyOnLoad
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- WeatherProcessor
- UserInterface
- .GetResourceAssets
- Stream Town Reloaded - Architecture Documentation
- WindController
- TechTreeNode
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- FoliageGenerationSettings
- CommonEnums.cs
- CampGenerationSettings
- Easings
- EditorUtils
- NodeUnlockData
- TimeProcessor
- StringUtils
- Access_GOList
- SelectedBuilding
- EditorHelpers
- EventProcessor
- IProcessor
- InventorySaveData
- TerrainGenSettings
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- DebugProcessor
- SeasonDataSettings
- DayAndNightSettings
- AllBuildingDataSettings
- PassiveResourceIncrementer
- KeepKingVote
- WorldGenDebugSettings
- BuildingDataSettings
- command.rs
- PlayerCommands
- Target
- TL_API
- NewKingVote
- .StartMusic
- UILineRenderer
- UserInterface_DisplayUsernames
- ResourceTarget
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- STSM_HelperBase
- GridProcessor.cs
- Key Rules
- SimpleDisableAfterTime
- Common Patterns
- Processors
- BuildingConfigSettings
- HealthModifier
- FPSDisplay.cs
- PlayerDeathHandler
- AllSeasonSettings
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- .SpawnDebugPlayerCoroutine
- VfxParticlePosition
- ScriptKeywordProcessor
- IntWrapper
- Access_TextInput
- Processor Template
- Common Patterns
- WorldGenBehaviorSettings
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- BuildPlacerData
- Twitch setup
- graphify reference: add a URL and watch a folder
- TownGoalSettings
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- ObjectiveSaveData
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Goal
- CreateProjectScopeProcessors.cs
- PlayerInputRuntimeData
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- append_vec3_keys
- AGENTS.md
- TradeProcessor
- .RefreshSceneBindingsAndTryGenerate
- ScriptableObjectAssetData
- ResourceGenerationSettings
- CustomLogger
- UnitTravelToPosition
- extraction-spec.md
- .list
- RoleDataSettings
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- EquipmentHandlerEditor
- ScriptablesProcessorInfrastructure
- IInstaller
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- Autosave

## God Nodes (most connected - your core abstractions)
1. `StableId` - 253 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldSimulation` - 129 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `SaveProcessor` - 88 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (267 total, 18 thin omitted)

### Community 0 - "GridPos"
Cohesion: 0.07
Nodes (65): GameConfig, WorldGenConfig, FoliageHabitat, FoliageLayerDef, FoliageVariantDef, StationDef, GridPos, authored_foliage_is_deterministic_and_respects_habitat_and_resources() (+57 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (17): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller, Container, ContainerBuilder (+9 more)

### Community 2 - "StableId"
Cohesion: 0.05
Nodes (79): GameplayConfig, BTreeMap, ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue (+71 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.11
Nodes (60): array_index(), clip_id(), color_value(), convert_clips(), convert_prefab_renderer_materials(), extracts_indexed_material_properties(), field_array(), field_bool() (+52 more)

### Community 4 - "String"
Cohesion: 0.06
Nodes (74): Added, AnimationClip, AnimationGraph, AnimationGraphHandle, AnimationTargetId, active_event_text(), add_animation_layer_branch(), add_rotation_curve() (+66 more)

### Community 5 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (10): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+2 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Sensors"
Cohesion: 0.09
Nodes (7): STStateMachine.States, Behaviours, Animation, Sensors, STStateMachine, Pathfinding, STStateMachine.Helpers

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (17): Action, bool, Container, ContainerBuilder, GameObject, IEnumerable, int, IReadOnlyList (+9 more)

### Community 12 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+2 more)

### Community 13 - "RoleHandler"
Cohesion: 0.07
Nodes (18): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+10 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "MonoBehaviour"
Cohesion: 0.02
Nodes (57): CameraProcessor, PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+49 more)

### Community 16 - "Character"
Cohesion: 0.07
Nodes (14): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+6 more)

### Community 17 - "Utils"
Cohesion: 0.04
Nodes (10): BuildCostModifier, RoleScriptablesEditor, Utils, Level, ScriptablesEditor, Buildings, SavingAndLoading, SavingAndLoading.Structs (+2 more)

### Community 18 - ".GenerateFromSettings"
Cohesion: 0.11
Nodes (20): HashSet, BoxCollider, Func, HashSet, Vector2, Vector3, GenerateDebugPositions(), GetPositiveNoiseOffset() (+12 more)

### Community 19 - "legacy.rs"
Cohesion: 0.13
Nodes (51): binary_fixture(), BinaryParser, clamped_cell(), decode_json(), ImportReport, json_active_goal(), json_buildings(), json_customization() (+43 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (12): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+4 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.08
Nodes (17): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, VisualElement (+9 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.07
Nodes (15): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy, Action (+7 more)

### Community 25 - "PlayerRoleData"
Cohesion: 0.11
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, StatType

### Community 26 - "TechTreeRuntimeData"
Cohesion: 0.16
Nodes (5): bool, Dictionary, float, int, TechTreeRuntimeData

### Community 27 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "ContentCatalog"
Cohesion: 0.05
Nodes (88): App, ContentCatalog, ActorState, RoleProgress, Default, String, generate_world(), action_animation_speed() (+80 more)

### Community 31 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 32 - "STSM_Idle_Player"
Cohesion: 0.12
Nodes (6): STSM_Action_GatherResource, bool, float, uint, Vector3, STSM_Idle_Player

### Community 33 - "ResourceProcessor"
Cohesion: 0.21
Nodes (8): Container, ContainerBuilder, List, Material, materials, Mesh, meshes, ResourceProcessor

### Community 34 - "Station"
Cohesion: 0.06
Nodes (19): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+11 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (7): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, UserInterface_Debug

### Community 36 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.13
Nodes (9): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+1 more)

### Community 39 - "StreamUserType"
Cohesion: 0.50
Nodes (3): StreamUserType, should_show_actor_name(), legacy_user_type()

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.10
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "LabelDisplayProcessor"
Cohesion: 0.09
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller (+7 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "save.rs"
Cohesion: 0.15
Nodes (31): detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native(), native_save_is_atomic_and_keeps_backup() (+23 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "DebugSettings"
Cohesion: 0.20
Nodes (6): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, DebugLogCategory, SerializedScriptableObject

### Community 48 - "NavGrid"
Cohesion: 0.09
Nodes (29): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+21 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "ResourceData"
Cohesion: 0.15
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (18): Client, TwitchClientRuntimeData, OnChatCommandReceivedArgs, Client, Container, ContainerBuilder, IEnumerator, LogType (+10 more)

### Community 54 - "UIProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, float, UISettings, ContainerBuilder (+6 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 57 - "WorldSimulation"
Cohesion: 0.05
Nodes (49): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown() (+41 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 61 - "StateMachine"
Cohesion: 0.12
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

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
Cohesion: 0.06
Nodes (20): int, ChangeTimeStamp, Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, Vector2 (+12 more)

### Community 67 - "twitch.rs"
Cohesion: 0.06
Nodes (52): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet, Default, Result, Self (+44 more)

### Community 68 - "Objective"
Cohesion: 0.07
Nodes (14): Slider, TextMeshProUGUI, UIRuntimeData, Action, int, Objective, Dictionary, GameObject (+6 more)

### Community 69 - "EnemyCampSaveData"
Cohesion: 0.50
Nodes (3): int, uint, EnemyCampSaveData

### Community 70 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "Result"
Cohesion: 0.11
Nodes (40): TextureDef, animation_state_id(), animation_state_machine_id(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), controller_id(), convert() (+32 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "STSM_Action_PlayerBase"
Cohesion: 0.13
Nodes (5): AttackUnit, STSM_Action_Build, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.22
Nodes (6): GlobalAudioController, AudioSource, bool, float, IEnumerator, Season

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 83 - "update_environment_presentation"
Cohesion: 0.07
Nodes (45): AmbientLight, Assets, building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialExtension, BuildingMaterialInstance, BuildingMaterialInstanced (+37 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_game/src/lib.rs"
Cohesion: 0.03
Nodes (112): AnimationNodeIndex, AnimationPlayer, AnyResult, ActorCustomization, ActorAnimationDriver, adjust_settings_menu(), advance_animation_crossfade(), AgentEquipmentPresentation (+104 more)

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

### Community 90 - "Option"
Cohesion: 0.08
Nodes (90): AssetServer, PresentationCatalog, actor_detail_budget(), actor_material(), actor_scene_budget(), animate_combat_effects(), animation_property_value(), archetype_by_source() (+82 more)

### Community 91 - "RoleProcessor"
Cohesion: 0.08
Nodes (9): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, List (+1 more)

### Community 92 - "RaidEvent"
Cohesion: 0.08
Nodes (16): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+8 more)

### Community 93 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - ".new"
Cohesion: 0.15
Nodes (22): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+14 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 97 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (13): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, string (+5 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (46): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+38 more)

### Community 100 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "IProcessor.cs"
Cohesion: 0.20
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 102 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.08
Nodes (13): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+5 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "WorldGenSaveData"
Cohesion: 0.10
Nodes (17): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+9 more)

### Community 108 - "ResourceHolder"
Cohesion: 0.13
Nodes (7): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, SelectedResource

### Community 109 - "CustomLogHandler"
Cohesion: 0.21
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.07
Nodes (21): Transform, Action, bool, Dictionary, float, GameObject, int, List (+13 more)

### Community 113 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemy, SelectedEnemyCamp, object, UnityAction, SelectedObject

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "STSM_StateAction"
Cohesion: 0.19
Nodes (5): bool, float, int, STSM_StateAction, AnimationName

### Community 116 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 117 - "IRuntimeDataScriptable"
Cohesion: 0.10
Nodes (22): Queue, AudioRuntimeData, Dictionary, int, List, BuildingRuntimeData, CreditsRuntimeData, UnityEvent (+14 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.06
Nodes (18): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, Container (+10 more)

### Community 120 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 121 - "process_injected_commands"
Cohesion: 0.06
Nodes (120): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, AudioSink, BackgroundColor, ActorNameOverlay, Agent, agent_is_moving() (+112 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.17
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "Targetable"
Cohesion: 0.08
Nodes (18): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+10 more)

### Community 126 - "BuildingBase"
Cohesion: 0.10
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 132 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 133 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "STSM_Idle"
Cohesion: 0.17
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (86): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+78 more)

### Community 137 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 138 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 139 - "STSM_Helper_Attack"
Cohesion: 0.18
Nodes (4): int, STSM_Helper_Attack, int, STSM_Action_Attack

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "ObjectPoolingProcessor"
Cohesion: 0.07
Nodes (20): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+12 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 147 - "Pet"
Cohesion: 0.10
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 150 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 151 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 154 - "WeatherProcessor"
Cohesion: 0.23
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 155 - "UserInterface"
Cohesion: 0.06
Nodes (17): InputButton, SharedTypes, TownGoal.Data, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Core, World (+9 more)

### Community 156 - ".GetResourceAssets"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

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

### Community 164 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 165 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 166 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (24): TargetSettings, Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData (+16 more)

### Community 167 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 169 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 177 - "IProcessor"
Cohesion: 0.13
Nodes (7): CancellationToken, Task, Container, IPostInitializeProcessor, IProcessor, Dictionary, ParallelProgressReporter

### Community 178 - "InventorySaveData"
Cohesion: 0.22
Nodes (7): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData

### Community 179 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "DebugProcessor"
Cohesion: 0.07
Nodes (10): Container, ContainerBuilder, GUIDProcessor, Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor (+2 more)

### Community 183 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 184 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 185 - "AllBuildingDataSettings"
Cohesion: 0.20
Nodes (7): ContainerBuilder, AllBuildingDataSettingsInstaller, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingDataContainerInstaller, AllBuildingDataSettings

### Community 187 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 189 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

### Community 190 - "BuildingDataSettings"
Cohesion: 0.15
Nodes (11): int, ResourceCostData, bool, float, Sprite, string, BuildingDataSettings, List (+3 more)

### Community 191 - "command.rs"
Cohesion: 0.05
Nodes (68): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+60 more)

### Community 192 - "PlayerCommands"
Cohesion: 0.13
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 193 - "Target"
Cohesion: 0.10
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

### Community 195 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 196 - ".StartMusic"
Cohesion: 0.56
Nodes (3): SeasonAudioData, AudioClip, List

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 204 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "Processors"
Cohesion: 0.07
Nodes (10): ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, Processors.Editor, MetaData, Audio, Settings (+2 more)

### Community 209 - "BuildingConfigSettings"
Cohesion: 0.33
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 210 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "PlayerDeathHandler"
Cohesion: 0.29
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 213 - "AllSeasonSettings"
Cohesion: 0.29
Nodes (6): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 218 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 221 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "WorldGenBehaviorSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenBehaviorSettingsInstaller, bool, WorldGenBehaviorSettings

### Community 225 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "TownGoalSettings"
Cohesion: 0.33
Nodes (4): int, TownGoalSettings, ContainerBuilder, TownGoalSettingsInstaller

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 235 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "Goal"
Cohesion: 0.16
Nodes (4): EventType, Action, Dictionary, Goal

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.05
Nodes (42): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+34 more)

### Community 244 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

### Community 246 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 248 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 249 - "ResourceGenerationSettings"
Cohesion: 0.12
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 253 - ".list"
Cohesion: 0.50
Nodes (3): FnMut, Self, T

### Community 254 - "RoleDataSettings"
Cohesion: 0.08
Nodes (18): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip (+10 more)

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 263 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 279 - "IInstaller"
Cohesion: 0.03
Nodes (41): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, Volume (+33 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.02
Nodes (70): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller, ContainerBuilder, TimeDataSettingsInstaller (+62 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `ObjectPoolingProcessor`, `MonoBehaviour`, `PlayerProcessor`, `ScriptableObject`, `IInstaller`, `UserInterface`, `SaveFileData`, `.GetResourceAssets`, `GameEventProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `IProcessor`, `StreamTownSessionBridge`, `DebugProcessor`, `PlayerCommands`, `FoliageProcessor`, `RoleProcessor`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `Resource`, `ResourceGenerationSettings`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `GameStateProcessor`, `TwitchChatProcessor`, `Player`, `ObjectPoolingProcessor`, `MonoBehaviour`, `.GenerateFromSettings`, `PlayerProcessor`, `ScriptableObject`, `IInstaller`, `ResourceProcessor`, `UserInterface_Debug`, `CampGenerationSettings`, `IProcessor`, `TerrainGenSettings`, `ResourceData`, `TwitchClientProcessor`, `DebugProcessor`, `WorldGenDebugSettings`, `Target`, `ProjectCamera`, `GridProcessor`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `WorldGenBehaviorSettings`, `EnemySpawner`, `SaveProcessor`, `ResourceGenerationSettings`, `AIPath`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `TwitchChatProcessor`, `UnitTextDisplay`, `RoleHandler`, `ObjectPoolingProcessor`, `Character`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `GameEventProcessor`, `Station`, `UserInterface_Debug`, `MiscCommands`, `CharacterModelHandler`, `LabelDisplayProcessor`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `PlayerCommands`, `Target`, `UserInterface_DisplayUsernames`, `TargetSensor`, `RoleProcessor`, `SaveProcessor`, `IRuntimeDataScriptable`, `VoteEvent`, `.SetTargetType`, `Targetable`, `CommandDictionary`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `GridPos` be split into smaller, more focused modules?**
  _Cohesion score 0.07319347319347319 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.0632996632996633 - nodes in this community are weakly interconnected._
- **Should `StableId` be split into smaller, more focused modules?**
  _Cohesion score 0.05116279069767442 - nodes in this community are weakly interconnected._