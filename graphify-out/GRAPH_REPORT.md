# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 604 files · ~1,562,471 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6475 nodes · 16165 edges · 257 communities (237 shown, 20 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 991 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d7972020`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ScriptablesProcessorInfrastructure
- BuildingProcessor
- String
- String
- process_injected_commands
- GameEvent
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- GUIDProcessor
- SettingsProcessor
- WorldGenProcessor
- TownGoal.Data
- BinaryReader
- TechTreeIOUtility
- .RecalculateStats
- Utils
- World.Generation
- .CreateEnumField
- Station
- .GenerateFromSettings
- ObjectiveSaveData
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- World.Generation.Settings
- TechTreeNode
- MiscCommands
- SaveFileData
- GameEventProcessor
- RoleHandler
- STSM_StateAction
- RaidEvent
- CellSpacePartitioning
- RoleSlot
- UserInterface_Debug
- BuildingBase
- SettingsData
- SaveProcessor
- EnemyModelHandler
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BinarySaveCodec
- AnimationControllerRuntime
- legacy.rs
- SeasonProcessor
- ContentCatalog
- PlayerRole
- VfxSeagullSpawner
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- STSM_HelperBase
- StableId
- STSM_GoToLocation
- TechTreeEditorWindow
- DebugProcessor
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- UserInterface_TownGoal
- Objective
- SensorProcessor
- GenerationSettings
- models.rs
- Tiler
- ScriptablesEditor
- stream_town_migrate/src/presentation.rs
- UserInterface_ObjectSelection
- GameConfig
- Access_Toggle
- IRuntimeDataScriptable
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- TwitchUser
- load_input
- Access_Text
- UserInterface
- PlayerSaveData
- UserInterface_TownVote
- CampGenerationSettings
- FoliageProcessor
- AnimationHandler
- NavGrid
- LabelDisplayProcessor
- SelectedPlayerGroup
- convert_fbx_to_glb.py
- .Draw
- stream_town_migrate/src/content.rs
- SelectableObject
- Coordinator
- stream_town_domain/src/presentation.rs
- Editor
- stream_town_game/src/lib.rs
- MainMenuManager
- TownGoalProcessor
- Player
- UnitHealthBar
- LoadingManager
- Enemy
- TransformSaveData
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedBuilding
- UserInterface_RulerVote
- DontDestroyOnLoad
- Goal
- EnemyWeaponModel
- VoteEvent
- Resource
- BuildingDamageMaterialHandler
- SelectedObject
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- ObjectPoolingProcessor
- Buildings
- CommandDictionary
- UpdateGraphBounds
- CommonEnums.cs
- stream_town_migrate/src/main.rs
- UserInterface_GameMenu
- .DrawDataFieldAndLabel
- .StartMusic
- ConfirmCheck
- RoleData
- ToolState
- VFXArrowPointer
- PlayerRoleData
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- GridProcessor
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- Character
- .RefreshSceneBindingsAndTryGenerate
- GateController
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- WindController
- VideoSettingsPresetsInstaller
- FoliageGenerationSettings
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- KeepKingVote
- GridProcessor.cs
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- Processors
- SimpleMusicController
- BuildingSettings
- SelectedEnemyCamp
- EditorUtils
- PlayerInventory
- PlayerDeathHandler
- NewKingVote
- EventProcessor
- TimeProcessor
- SelectedResource
- VfxAnimationController
- PlayerInputRuntimeData
- EditorHelpers
- UIElementWrapper
- GameStateProcessor
- VfxParticlePosition
- ScriptableObjectAssetData
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- WorldGenRuntimeData
- SimpleDisableAfterTime
- SelectedEnemy
- RotationHandler
- AudioMixerInstaller
- PlayerRoleSaveData
- PlayerCommands
- UnitTravelToPosition
- command.rs
- Season
- AutosaveIntervalsInstaller
- IProcessor.cs
- IProcessor
- STSM_Idle_Enemy
- UILineRenderer
- UserInterface_DisplayUsernames
- WeatherProcessor
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- PassiveResourceIncrementer
- ForwardRendererInstaller
- Key Rules
- DebugSettings
- Common Patterns
- FPSDisplay.cs
- Key Rules
- Requirement
- RuntimeData Template
- RandomEnabler
- StringUtils
- ScriptKeywordProcessor
- DayAndNightSettings
- Processor Template
- Common Patterns
- retargeted_animation_clip
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- Twitch setup
- graphify reference: add a URL and watch a folder
- EquipmentHandlerEditor
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- SimpleScreenShot
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Easings
- CreateProjectScopeProcessors.cs
- .RestoreObjectiveProgress
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- WorldGenSaveData
- AGENTS.md
- PostProcessingInstaller
- CustomLogger
- extraction-spec.md
- BuildPlacerData
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Targetable
- UserInterface_BuildingHealthBar
- MonoBehaviour
- UI_TechOption
- IntWrapper
- StatusBar
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- append_vec3_keys
- Autosave

## God Nodes (most connected - your core abstractions)
1. `StableId` - 203 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldGenProcessor` - 110 edges
7. `SettingsProcessor` - 107 edges
8. `Reflex.Core` - 103 edges
9. `WorldSimulation` - 100 edges
10. `SaveProcessor` - 88 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generate_and_spawn_world()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `world_tab()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_tools/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (257 total, 20 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.08
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "String"
Cohesion: 0.15
Nodes (20): active_event_text(), building_definition_id(), CommandFeedback, equipment_node_names(), generate_connect_code(), initial_actor_identity(), item_info(), normalized_content_name() (+12 more)

### Community 3 - "String"
Cohesion: 0.21
Nodes (26): inline_file_id(), parse_blend_tree(), parse_conditions(), parse_controller(), parse_parameters(), parse_reference_list(), parse_state_motions(), parse_yaml_documents() (+18 more)

### Community 4 - "process_injected_commands"
Cohesion: 0.10
Nodes (66): AnimationGraph, AssetServer, ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, EnemyDef, GridPos (+58 more)

### Community 5 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.06
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "GUIDProcessor"
Cohesion: 0.09
Nodes (7): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, PoolType

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 12 - "TownGoal.Data"
Cohesion: 0.08
Nodes (10): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 13 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - ".RecalculateStats"
Cohesion: 0.17
Nodes (8): StatModifiers, Dictionary, Dictionary, List, Queue, Transform, PlayerRuntimeData, StatType

### Community 16 - "Utils"
Cohesion: 0.05
Nodes (11): STStateMachine.States, Utils, Behaviours, Animation, Sensors, GridSystem.Partitioning, STStateMachine, Pathfinding (+3 more)

### Community 18 - ".CreateEnumField"
Cohesion: 0.13
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 19 - "Station"
Cohesion: 0.05
Nodes (26): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+18 more)

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+9 more)

### Community 21 - "ObjectiveSaveData"
Cohesion: 0.06
Nodes (25): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+17 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (10): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, ModeratorCommands (+2 more)

### Community 24 - "HealthHandler"
Cohesion: 0.12
Nodes (7): Action, bool, float, int, UnityEvent, HealthHandler, ReviveType

### Community 25 - "World.Generation.Settings"
Cohesion: 0.04
Nodes (36): Slider, TextMeshProUGUI, UI_Objective, Image, TextMeshProUGUI, UIRoleDisplay, CampGenerationSettings, List (+28 more)

### Community 26 - "TechTreeNode"
Cohesion: 0.13
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 27 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 30 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 31 - "STSM_StateAction"
Cohesion: 0.13
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 32 - "RaidEvent"
Cohesion: 0.08
Nodes (16): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+8 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 34 - "RoleSlot"
Cohesion: 0.14
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "BuildingBase"
Cohesion: 0.11
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.07
Nodes (24): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+16 more)

### Community 39 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.14
Nodes (18): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+10 more)

### Community 45 - "legacy.rs"
Cohesion: 0.05
Nodes (104): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+96 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "ContentCatalog"
Cohesion: 0.10
Nodes (45): AuthoredRecord, AuthoredValue, BuildingDef, ContentCatalog, ContentError, EnemySpawnerDef, HealthDef, ProjectileShooterDef (+37 more)

### Community 48 - "PlayerRole"
Cohesion: 0.10
Nodes (7): RoleSlotModifier, int, Container, ContainerBuilder, int, RoleProcessor, PlayerRole

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 57 - "StableId"
Cohesion: 0.06
Nodes (60): ObjectiveDef, ObjectiveKind, FromStr, StableId, ActorState, authored_trade_rates_clamp_to_stock_gold_and_capacity(), BuildingState, capped_deposit_preserves_inventory_overflow() (+52 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "DebugProcessor"
Cohesion: 0.04
Nodes (27): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, Container, ContainerBuilder (+19 more)

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
Cohesion: 0.08
Nodes (15): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+7 more)

### Community 67 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 68 - "Objective"
Cohesion: 0.13
Nodes (4): Action, int, Objective, EnemyType

### Community 69 - "SensorProcessor"
Cohesion: 0.12
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 70 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.11
Nodes (47): animation_state_id(), array_index(), assign_clip_rigs_and_reference_poses(), clip_id(), collect_prefab_dependencies(), color_value(), controller_id(), convert() (+39 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.13
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "GameConfig"
Cohesion: 0.13
Nodes (18): AnyResult, ConfigError, default_configuration_is_valid_and_round_trips_ron(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, BTreeMap, Default (+10 more)

### Community 77 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 78 - "IRuntimeDataScriptable"
Cohesion: 0.08
Nodes (19): int, List, Vector2, GridNode, Color, CollisionColours, CollisionType, CreditsRuntimeData (+11 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 83 - "load_input"
Cohesion: 0.06
Nodes (106): AmbientLight, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, App, AppExit, Assets, ActorAnimationDriver (+98 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "UserInterface"
Cohesion: 0.10
Nodes (8): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, UserInterface, SavingAndLoading.SavableObjects, Enemies, GUIDSystem

### Community 86 - "PlayerSaveData"
Cohesion: 0.11
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.15
Nodes (7): AnimationHandler, Animator, bool, Dictionary, float, int, AnimationName

### Community 91 - "NavGrid"
Cohesion: 0.13
Nodes (19): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode, path_routes_around_dynamic_building() (+11 more)

### Community 92 - "LabelDisplayProcessor"
Cohesion: 0.06
Nodes (23): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller (+15 more)

### Community 93 - "SelectedPlayerGroup"
Cohesion: 0.23
Nodes (3): List, List, SelectedPlayerGroup

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.23
Nodes (18): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+10 more)

### Community 95 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.09
Nodes (96): ArchetypesById, aged_buildings(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask(), authored_value() (+88 more)

### Community 97 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.12
Nodes (33): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationMotionDef, AnimationParameterDef, AnimationParameterKind, AnimationQuatKeyframe (+25 more)

### Community 100 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "stream_town_game/src/lib.rs"
Cohesion: 0.04
Nodes (105): cell_hash(), changing_seed_changes_world_hash(), generate_world(), GeneratedResource, generation_is_deterministic(), hash_world(), String, action_cooldown() (+97 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+2 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "Enemy"
Cohesion: 0.07
Nodes (22): CollectResource, Action, float, Enemy, int, ActiveResourceIncrementer, AnimationCurve, bool (+14 more)

### Community 108 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

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
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 116 - "Goal"
Cohesion: 0.10
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 117 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (31): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, IResourceHolder, float (+23 more)

### Community 120 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 121 - "SelectedObject"
Cohesion: 0.14
Nodes (3): object, UnityAction, SelectedObject

### Community 122 - ".SetTargetType"
Cohesion: 0.11
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "ObjectPoolingProcessor"
Cohesion: 0.06
Nodes (32): Action, CancellationToken, Task, IAsyncInitializableProcessor, ProcessorStartupContext, Action, bool, BoxCollider (+24 more)

### Community 126 - "Buildings"
Cohesion: 0.12
Nodes (4): BuildCostModifier, Level, Buildings, Combat

### Community 127 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "CommonEnums.cs"
Cohesion: 0.09
Nodes (20): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+12 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 132 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 133 - ".StartMusic"
Cohesion: 0.49
Nodes (3): SeasonAudioData, AudioClip, List

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 136 - "ToolState"
Cohesion: 0.05
Nodes (80): TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse, Arc (+72 more)

### Community 137 - "VFXArrowPointer"
Cohesion: 0.22
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 138 - "PlayerRoleData"
Cohesion: 0.17
Nodes (5): PlayerRoleData, AudioClip, bool, float, int

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GridProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, GridProcessor

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
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "Character"
Cohesion: 0.09
Nodes (13): Pets.Enumerations, StreamTown.EditorTools, TownGoal, Character.Enumerations, Core, Pets, GameEventSystem, GameEventSystem.Events (+5 more)

### Community 150 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 151 - "xtask/src/main.rs"
Cohesion: 0.39
Nodes (7): Cli, Command, main(), Command, Result, stress(), validate()

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 154 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 155 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "Processors"
Cohesion: 0.05
Nodes (12): ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, World, Processors.Editor, MetaData, Audio (+4 more)

### Community 163 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 164 - "BuildingSettings"
Cohesion: 0.10
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 166 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 167 - "PlayerInventory"
Cohesion: 0.22
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 168 - "PlayerDeathHandler"
Cohesion: 0.29
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 169 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 170 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 171 - "TimeProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 174 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 177 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 178 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 179 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 184 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 186 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 187 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 188 - "PlayerRoleSaveData"
Cohesion: 0.40
Nodes (3): List, int, PlayerRoleSaveData

### Community 189 - "PlayerCommands"
Cohesion: 0.13
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 191 - "command.rs"
Cohesion: 0.11
Nodes (35): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+27 more)

### Community 192 - "Season"
Cohesion: 0.14
Nodes (13): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Color, float (+5 more)

### Community 193 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 194 - "IProcessor.cs"
Cohesion: 0.33
Nodes (5): Exception, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupReport, ProcessorStartupStage

### Community 195 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "WeatherProcessor"
Cohesion: 0.19
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 204 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "DebugSettings"
Cohesion: 0.22
Nodes (6): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, DebugLogCategory, SerializedScriptableObject

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "retargeted_animation_clip"
Cohesion: 0.23
Nodes (14): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), retargeted_animation_clip() (+6 more)

### Community 225 - "WorldSaveData"
Cohesion: 0.13
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 229 - "Twitch setup"
Cohesion: 0.10
Nodes (17): Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity, Validation (+9 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 235 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 243 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 246 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 254 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Targetable"
Cohesion: 0.05
Nodes (22): ProjectileShooter, float, int, string, Container, ContainerBuilder, List, TargetProcessor (+14 more)

### Community 263 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 264 - "MonoBehaviour"
Cohesion: 0.01
Nodes (100): Api, CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller (+92 more)

### Community 269 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 276 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 279 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (73): List, CampGenSettings, List, FoliageGenSettings, bool, ParticleSystem, Transform, GameEventSettings (+65 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 284 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **20 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `UserInterface_GameMenu`, `MonoBehaviour`, `GUIDProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `Character`, `PlayerProcessor`, `ScriptableObject`, `SaveFileData`, `ResourceDataSaveData`, `GameEventProcessor`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `PlayerRole`, `StreamTownSessionBridge`, `ResourceProcessor`, `DebugProcessor`, `PlayerCommands`, `IProcessor`, `FoliageProcessor`, `WorldSaveData`, `MainMenuManager`, `TownGoalProcessor`, `.RestoreObjectiveProgress`, `WorldGenSaveData`, `Resource`, `ObjectPoolingProcessor`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `MonoBehaviour`, `GUIDProcessor`, `GridProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `ScriptableObject`, `RaidEvent`, `CellSpacePartitioning`, `UserInterface_Debug`, `SaveProcessor`, `GameStateProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `WorldGenRuntimeData`, `DebugProcessor`, `IProcessor`, `UserInterface`, `FoliageProcessor`, `Player`, `EnemySpawner`, `AIPath`, `ObjectPoolingProcessor`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `ScriptablesProcessorInfrastructure`, `GraphicsProcessor`, `Access_Slider`, `IProcessor`, `UserInterface_GameMenu`, `SettingsData`, `SaveProcessor`, `MainMenuManager`, `MonoBehaviour`, `Autosave`, `Access_Toggle`, `UIElementWrapper`, `Access_Text`, `Access_Dropdown`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07052631578947369 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07755102040816327 - nodes in this community are weakly interconnected._
- **Should `String` be split into smaller, more focused modules?**
  _Cohesion score 0.14736842105263157 - nodes in this community are weakly interconnected._