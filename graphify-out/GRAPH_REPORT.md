# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 585 files · ~855,813 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 5505 nodes · 12319 edges · 257 communities (232 shown, 25 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 970 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b157149d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ScriptablesProcessorInfrastructure
- BuildingProcessor
- ScriptableObject
- Processors
- Player
- WorldGenProcessor
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- ObjectPoolingProcessor
- SettingsProcessor
- MonoBehaviour
- Target
- Resource
- TechTreeIOUtility
- FoliageProcessor
- PlayerRole
- World.Generation
- SaveProcessor.cs
- UserInterface
- .GenerateFromSettings
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- BinarySaveCodec
- World.Generation.Settings
- stream_town_game/src/lib.rs
- Utils
- SaveFileData
- GameEventProcessor
- .Log
- Targetable
- CommonEnums.cs
- CellSpacePartitioning
- LabelDisplayProcessor
- UserInterface_Debug
- HealthHandler
- SettingsData
- SaveProcessor
- .CreateEnumField
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- STSM_Idle
- GUIDProcessor
- .RestoreObjectiveProgress
- SeasonProcessor
- GUIDComponent
- BinaryReader
- VfxSeagullSpawner
- AudioHandler
- TargetSensor
- ResourceData
- TwitchClientProcessor
- StreamTownSessionBridge
- StationProcessor
- RoleDataSettings
- StableId
- STSM_GoToLocation
- TechTreeEditorWindow
- BuildingBase
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- RoleHandler
- Objective
- UserInterface_RulerVote
- MeshData
- ProjectCamera
- Tiler
- ScriptablesEditor
- EnemyModelHandler
- UserInterface_ObjectSelection
- .StartupSequence
- PlayerCommands
- GridProcessor
- TwitchBotSetupWindow
- GamestateJukebox
- WorldUtils
- Pet
- SensorProcessor
- Access_Text
- Station
- UIProcessor
- UserInterface_TownVote
- TechTreeNode
- Access_Dropdown
- AnimationHandler
- GridPos
- FoliageGenerationSettings
- RaidEvent
- STSM_StateAction
- .Draw
- save.rs
- PlayerInventory
- Coordinator
- ResourceGenerationSettings
- Editor
- STSM_Action_PlayerBase
- MainMenuManager
- TownGoalProcessor
- MiscCommands
- UnitHealthBar
- LoadingManager
- UIElementWrapper
- RoleData
- CustomLogHandler
- LevelHandler
- GlobalAudioController
- DayAndNightProcessor
- SelectedBuilding
- PlayerInputProcessor
- TransformSaveData
- TechTreeRuntimeData
- IProcessor.cs
- VoteEvent
- TimeProcessor
- ResourceRuntimeData
- Goal
- .SetTargetType
- SnapToGridMouseMovement
- Season
- GameStateProcessor
- PlayerInputRuntimeData
- CommandDictionary
- UpdateGraphBounds
- .InitializeAndActivateProcessorsAsync
- stream_town_migrate/src/main.rs
- PlayerSaveData
- SelectableObject
- WeatherProcessor
- ConfirmCheck
- SelectedPlayer
- tools_ui
- FrameCapture
- CampGenerationSettings
- ResourceTarget
- ResourceProcessor
- What You Must Do When Invoked
- RuntimeData Template
- WorldSaveData
- RuntimeData Template
- Key Rules
- LoadingProgressReporter
- UserInterface_TownGoal
- Access_Toggle
- BuildingResourceModelHandler
- GateController
- generate_world
- Stream Town Reloaded - Architecture Documentation
- WindController
- content.rs
- .RenderResourceType
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- UserInterface_GameMenu
- .new
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- ResourceHolder
- .Update
- command.rs
- BuildingSettings
- EditorUtils
- IRuntimeDataScriptable
- NewKingVote
- CreditsProcessor
- SelectedObject
- TargetProcessor
- TechNodeData
- UserInterface_BuildingHealthBar
- UnitTextDisplay
- EditorHelpers
- GameConfig
- SeasonAudioData
- SimpleMusicController
- TerrainGenSettings
- Settings Scriptable Template
- DebugSettings
- IProcessor
- WorldGenRuntimeData
- Easings
- SelectedEnemy
- SelectedResource
- SeasonDataSettings
- STSM_HelperDeposit
- BuildingDamageMaterialHandler
- SimpleCancelBuildingPlacer
- DayAndNightSettings
- .DrawDataFieldAndLabel
- WorldGenSaveData
- STSM_HelperBase
- UI_TechOption
- UILineRenderer
- UserInterface_DisplayUsernames
- Access_GOList
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- ResourceDataSettings
- GameEventSettings
- Key Rules
- GridProcessor.cs
- Common Patterns
- SelectedEnemyCamp
- ObjectPoolingSettings
- KeepKingVote
- FPSDisplay.cs
- Twitch setup
- RotationHandler
- Key Rules
- Requirement
- RuntimeData Template
- RandomEnabler
- StringUtils
- ScriptKeywordProcessor
- UnitTravelToPosition
- BuildingConfigSettings
- Processor Template
- Common Patterns
- PlayerRuntimeData
- STSM_Action_DepositResource
- graphify reference: query, path, explain
- TODO List
- StreamTownGamePlugin
- Stream Town Bevy
- graphify reference: add a URL and watch a folder
- EquipmentHandlerEditor
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- SimpleScreenShot
- IntWrapper
- TL_API
- CreateProjectScopeProcessors.cs
- Autosave
- .InjectRuntimeData
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- ForwardRendererInstaller
- RenderPipelineInstaller
- AGENTS.md
- .RefreshSceneBindingsAndTryGenerate
- ScriptableObjectAssetData
- StatusBar
- CustomLogger
- ObjectSelectionProcessor.Editor.cs
- extraction-spec.md
- README.md
- SavePlayersData
- TechTreeNodeType.cs
- PoolablePlayer.cs

## God Nodes (most connected - your core abstractions)
1. `Utils` - 158 edges
2. `Processors` - 156 edges
3. `ScriptablesProcessorInfrastructure` - 150 edges
4. `Player` - 142 edges
5. `WorldGenProcessor` - 110 edges
6. `SettingsProcessor` - 107 edges
7. `Reflex.Core` - 103 edges
8. `SaveProcessor` - 88 edges
9. `BuildingProcessor` - 78 edges
10. `Resource` - 76 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `embedded_config_supports_vertical_slice_scale()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `world_tab()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_tools/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `StreamTownSessionBridge` --references--> `SaveProcessor`  [EXTRACTED]
  Assets/Editor/StreamTownSessionBridge.cs → Assets/Scripts/Core/Processors/SaveProcessor.cs

## Import Cycles
- None detected.

## Communities (257 total, 25 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.08
Nodes (10): Container, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost, maxLevel (+2 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.02
Nodes (65): ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, TimeDataSettingsInstaller, ContainerBuilder, WorldGenBehaviorSettingsInstaller, ContainerBuilder, WorldGenDebugSettingsInstaller (+57 more)

### Community 3 - "Processors"
Cohesion: 0.06
Nodes (9): InputButton, UserInterface.MainMenu, Processors, World, MetaData, Audio, Settings, Environment (+1 more)

### Community 4 - "Player"
Cohesion: 0.08
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+2 more)

### Community 5 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ObjectPoolingProcessor"
Cohesion: 0.06
Nodes (30): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+22 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "MonoBehaviour"
Cohesion: 0.02
Nodes (80): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+72 more)

### Community 12 - "Target"
Cohesion: 0.06
Nodes (15): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors (+7 more)

### Community 13 - "Resource"
Cohesion: 0.06
Nodes (21): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, bool (+13 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 16 - "PlayerRole"
Cohesion: 0.07
Nodes (14): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+6 more)

### Community 18 - "SaveProcessor.cs"
Cohesion: 0.10
Nodes (18): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+10 more)

### Community 19 - "UserInterface"
Cohesion: 0.06
Nodes (14): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, TownGoal (+6 more)

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.08
Nodes (27): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+19 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.06
Nodes (24): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+16 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (19): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+11 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.07
Nodes (10): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, ModeratorCommands (+2 more)

### Community 24 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "stream_town_game/src/lib.rs"
Cohesion: 0.10
Nodes (50): AppExit, Agent, cleanup_state_entities(), cleanup_world(), credits_input(), finish_boot(), game_input(), GameState (+42 more)

### Community 27 - "Utils"
Cohesion: 0.05
Nodes (7): BuildCostModifier, Utils, Level, Buildings, SavingAndLoading.Structs, Character, GameResources

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (20): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+12 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 30 - ".Log"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor, int, STSM_Helper_Build (+9 more)

### Community 31 - "Targetable"
Cohesion: 0.11
Nodes (7): bool, BoxCollider, float, int, Transform, Vector3, Targetable

### Community 32 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (16): TargetSettings, ContainerBuilder, TargetSettingsInstaller, TargetableData, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus (+8 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 34 - "LabelDisplayProcessor"
Cohesion: 0.12
Nodes (9): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, float, ParticleSystem (+1 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "HealthHandler"
Cohesion: 0.10
Nodes (10): Func, Action, float, Enemy, Action, bool, float, int (+2 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.08
Nodes (18): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+10 more)

### Community 39 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.10
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "STSM_Idle"
Cohesion: 0.17
Nodes (6): AIPath, bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 44 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 45 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 48 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 52 - "ResourceData"
Cohesion: 0.18
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "StreamTownSessionBridge"
Cohesion: 0.19
Nodes (3): MenuItem, StreamTownSessionBridge, FrameCapture

### Community 55 - "StationProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 56 - "RoleDataSettings"
Cohesion: 0.16
Nodes (11): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+3 more)

### Community 57 - "StableId"
Cohesion: 0.17
Nodes (19): FromStr, StableId, ActorState, BuildingState, complete_gameplay_scenario_round_trips(), deterministic_weather(), id(), BTreeMap (+11 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.16
Nodes (7): AIPath, bool, float, GameObject, int, Transform, STSM_GoToLocation

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "BuildingBase"
Cohesion: 0.11
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 61 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "RoleHandler"
Cohesion: 0.07
Nodes (15): PlayerRoleData, AIPath, AudioClip, bool, float, int, RoleHandler, AIPath (+7 more)

### Community 68 - "Objective"
Cohesion: 0.13
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 70 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 71 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 77 - "PlayerCommands"
Cohesion: 0.13
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 78 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GamestateJukebox"
Cohesion: 0.11
Nodes (13): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+5 more)

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 83 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Station"
Cohesion: 0.09
Nodes (12): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+4 more)

### Community 86 - "UIProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, float, UISettings, ContainerBuilder (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 89 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.08
Nodes (17): AnimationHandler, AIPath, Animator, bool, Dictionary, float, int, PlayerDeathHandler (+9 more)

### Community 91 - "GridPos"
Cohesion: 0.14
Nodes (20): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError, OpenNode (+12 more)

### Community 92 - "FoliageGenerationSettings"
Cohesion: 0.15
Nodes (11): List, FoliageGenSettings, List, WaterFoliageGenSettings, List, Material, Mesh, string (+3 more)

### Community 93 - "RaidEvent"
Cohesion: 0.08
Nodes (16): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+8 more)

### Community 94 - "STSM_StateAction"
Cohesion: 0.11
Nodes (9): int, STSM_Helper_Attack, int, STSM_Action_Attack, AIPath, bool, float, int (+1 more)

### Community 95 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 96 - "save.rs"
Cohesion: 0.16
Nodes (25): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacySaveInfo, LegacySaveKind, load_native(), native_save_is_atomic_and_keeps_backup() (+17 more)

### Community 97 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 98 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 99 - "ResourceGenerationSettings"
Cohesion: 0.20
Nodes (9): List, ResourceGenSettings, List, WaterResourceGenSettings, AnimationCurve, bool, int, List (+1 more)

### Community 100 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "STSM_Action_PlayerBase"
Cohesion: 0.10
Nodes (9): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Heal, STSM_Action_PlayerAttack (+1 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MiscCommands"
Cohesion: 0.15
Nodes (5): Dictionary, MiscCommands, OnChatCommandReceivedArgs, Dictionary, MessageSender

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.11
Nodes (11): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+3 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.12
Nodes (9): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_TextInput, TMP_InputField, ContainerBuilder (+1 more)

### Community 108 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 114 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 115 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 116 - "TechTreeRuntimeData"
Cohesion: 0.16
Nodes (5): bool, Dictionary, float, int, TechTreeRuntimeData

### Community 117 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "TimeProcessor"
Cohesion: 0.13
Nodes (7): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData, IEnumerable

### Community 120 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 121 - "Goal"
Cohesion: 0.16
Nodes (4): EventType, Action, Dictionary, Goal

### Community 122 - ".SetTargetType"
Cohesion: 0.11
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "Season"
Cohesion: 0.20
Nodes (7): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Season

### Community 125 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 126 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.17
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.21
Nodes (24): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+16 more)

### Community 131 - "PlayerSaveData"
Cohesion: 0.07
Nodes (23): Dictionary, Mesh, Transform, Vector3, SaveDataMapper, bool, int, List (+15 more)

### Community 132 - "SelectableObject"
Cohesion: 0.11
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 133 - "WeatherProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "SelectedPlayer"
Cohesion: 0.10
Nodes (4): List, SelectedPlayer, List, SelectedPlayerGroup

### Community 136 - "tools_ui"
Cohesion: 0.18
Nodes (23): content_tab(), inspector_tab(), main(), migration_tab(), Commands, Default, Option, ResMut (+15 more)

### Community 137 - "FrameCapture"
Cohesion: 0.22
Nodes (10): bool, double, float, int, IReadOnlyList, List, long, string (+2 more)

### Community 138 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 139 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 140 - "ResourceProcessor"
Cohesion: 0.14
Nodes (3): Container, ContainerBuilder, ResourceProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WorldSaveData"
Cohesion: 0.13
Nodes (17): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+9 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "LoadingProgressReporter"
Cohesion: 0.24
Nodes (4): bool, float, string, LoadingProgressReporter

### Community 147 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 148 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 149 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 150 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 151 - "generate_world"
Cohesion: 0.18
Nodes (17): cell_hash(), changing_seed_changes_world_hash(), generate_world(), GeneratedResource, GeneratedWorld, generation_is_deterministic(), hash_world(), String (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 154 - "content.rs"
Cohesion: 0.26
Nodes (11): BuildingDef, ContentCatalog, ContentError, RoleDef, BTreeMap, BTreeSet, Result, String (+3 more)

### Community 155 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - ".new"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Into, Result, Self, String, StableIdError, Formatter

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Attributes, GUIContent, PropertyAttribute, PropertyDrawer, Rect, SerializedProperty

### Community 162 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 163 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 164 - "command.rs"
Cohesion: 0.26
Nodes (11): ChatCommand, CommandParseError, no_argument(), Err, FromStr, Option, Result, Self (+3 more)

### Community 165 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 166 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 167 - "IRuntimeDataScriptable"
Cohesion: 0.25
Nodes (7): CreditsRuntimeData, IRuntimeDataScriptable, List, TownGoalRuntimeData, Slider, TextMeshProUGUI, UIRuntimeData

### Community 168 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 169 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 170 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 171 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 172 - "TechNodeData"
Cohesion: 0.39
Nodes (3): List, Node_SO, TechNodeData

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - "UnitTextDisplay"
Cohesion: 0.15
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "GameConfig"
Cohesion: 0.23
Nodes (10): ConfigError, default_configuration_is_valid_and_round_trips_ron(), GameConfig, GameplayConfig, Default, Result, Self, String (+2 more)

### Community 177 - "SeasonAudioData"
Cohesion: 0.57
Nodes (3): SeasonAudioData, AudioClip, List

### Community 178 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 179 - "TerrainGenSettings"
Cohesion: 0.33
Nodes (6): AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "DebugSettings"
Cohesion: 0.29
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 182 - "IProcessor"
Cohesion: 0.08
Nodes (14): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, IProcessor, Action (+6 more)

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 187 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 189 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 192 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 193 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 194 - "WorldGenSaveData"
Cohesion: 0.09
Nodes (17): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+9 more)

### Community 195 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 196 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 204 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "ObjectPoolingSettings"
Cohesion: 0.25
Nodes (7): bool, List, ObjectPoolingSettings, GameObject, int, string, PooledObjectData

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "Twitch setup"
Cohesion: 0.25
Nodes (7): 1. Secure the old credentials, 2. Register the Twitch application, 3. Authorize `HumanBeanBot`, 4. Prepare the channel, 5. Configure OBS, Connection controls and diagnostics, Twitch setup

### Community 213 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "UnitTravelToPosition"
Cohesion: 0.33
Nodes (3): UnitTravelToPosition, AIPath, Vector3

### Community 221 - "BuildingConfigSettings"
Cohesion: 0.33
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "PlayerRuntimeData"
Cohesion: 0.47
Nodes (5): Dictionary, List, Queue, Transform, PlayerRuntimeData

### Community 225 - "STSM_Action_DepositResource"
Cohesion: 0.33
Nodes (3): float, STSM_Action_DepositResource, STStateBase

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "StreamTownGamePlugin"
Cohesion: 0.50
Nodes (3): App, StreamTownGamePlugin, Plugin

### Community 229 - "Stream Town Bevy"
Cohesion: 0.50
Nodes (3): Binaries, Commands, Stream Town Bevy

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

### Community 236 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 243 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 244 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 247 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 248 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

## Knowledge Gaps
- **205 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+200 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `ScriptablesProcessorInfrastructure`, `UpdateGraphBounds`, `Processors`, `MonoBehaviour`, `Target`, `World.Generation`, `SaveProcessor.cs`, `UserInterface`, `TechTree.Elements`, `BuildingPlacer`, `CommonEnums.cs`, `.CreateEnumField`, `UnitTextDisplay`, `Easings`, `.DrawDataFieldAndLabel`, `MeshData`, `FPSDisplay.cs`, `RandomEnabler`, `StringUtils`, `SimpleScreenShot`, `DayAndNightProcessor`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.076) - this node is a cross-community bridge._
- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `PlayerSaveData`, `WorldGenProcessor`, `ObjectPoolingProcessor`, `SettingsProcessor`, `MonoBehaviour`, `ResourceProcessor`, `Resource`, `FoliageProcessor`, `PlayerRole`, `SaveProcessor.cs`, `PlayerProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `.Log`, `SaveFileData`, `UserInterface_GameMenu`, `TechTreeProcessor`, `GUIDProcessor`, `.RestoreObjectiveProgress`, `SeasonProcessor`, `StreamTownSessionBridge`, `IProcessor`, `WorldGenSaveData`, `PlayerCommands`, `FoliageGenerationSettings`, `ResourceGenerationSettings`, `MainMenuManager`, `TownGoalProcessor`, `TimeProcessor`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `ScriptablesProcessorInfrastructure`, `GraphicsProcessor`, `Access_Slider`, `SettingsData`, `SaveProcessor`, `ProjectCamera`, `MainMenuManager`, `Access_GOList`, `UIElementWrapper`, `MonoBehaviour`, `Autosave`, `Access_Text`, `Access_Toggle`, `IProcessor`, `Access_Dropdown`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _205 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07256571640133284 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.08233117483811286 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.02463605823068309 - nodes in this community are weakly interconnected._