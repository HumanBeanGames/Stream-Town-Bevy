# Graph Report - .  (2026-08-05)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 4980 nodes · 11402 edges · 261 communities (232 shown, 29 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 964 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `27f0d297`
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
- IInstaller
- Units
- Resource
- TechTreeIOUtility
- FoliageProcessor
- PlayerRole
- Target
- Character
- UserInterface
- .GenerateFromSettings
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- BinarySaveCodec
- World.Generation.Settings
- MonoBehaviour
- Utils
- SaveFileData
- GameEventProcessor
- DebugProcessor
- Targetable
- CommonEnums.cs
- CellSpacePartitioning
- LabelDisplayProcessor
- UserInterface_Debug
- HealthHandler
- SettingsData
- .LoadGameAsync
- .CreateEnumField
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- STSM_Idle_Player
- PoolableObject
- SaveProcessor
- SeasonProcessor
- Enemy
- BinaryReader
- VfxSeagullSpawner
- AudioHandler
- TargetSensor
- ResourceProcessor
- TwitchClientProcessor
- StreamTownSessionBridge
- Station
- RoleDataSettings
- .Log
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
- ResourceInventory
- UIProcessor
- UserInterface_TownVote
- TechTreeNode
- Access_Dropdown
- AnimationHandler
- PlayerRoleData
- FoliageData
- RaidEvent
- STSM_StateAction
- .Draw
- .EnsureValidCredentials
- PlayerInventory
- Coordinator
- ResourceGenerationSettings
- Editor
- GameEvent
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
- EnemySpawner
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
- IRuntimeDataScriptable
- CommandDictionary
- UpdateGraphBounds
- IProcessor
- DayAndNightProcessor
- .CapturePlayers
- SelectableObject
- WeatherProcessor
- ConfirmCheck
- SelectedPlayer
- WorldInstanceDeterminism
- .ExerciseHealthBarCategory
- CampGenerationSettings
- ResourceTarget
- ResourceData
- UserInterface_Resources
- BuildingCommands
- WorldSaveData
- TL_Secrets
- .UserIsSubscribed
- .LoadSceneAsync
- UserInterface_TownGoal
- Access_Toggle
- BuildingResourceModelHandler
- GateController
- ResourceStorageModifier
- TwitchUser
- WindController
- FishGodEvent
- .RenderResourceType
- ResourceDataSaveData
- FoliageGroupSaveData
- UserInterface_GameMenu
- DontDestroyOnLoad
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- ResourceHolder
- .Update
- SaveDataMapper
- BuildingDataSettings
- EditorUtils
- UserInterface_Event
- NewKingVote
- CreditsProcessor
- SelectedObject
- PlayerSaveData
- TechNodeData
- UserInterface_BuildingHealthBar
- .ValidateTokenAsync
- EditorHelpers
- EnemyWeaponModel
- .StartMusic
- SimpleMusicController
- TerrainGenSettings
- BuildingSettings
- DebugSettings
- TradeProcessor
- WorldGenRuntimeData
- Easings
- SelectedEnemy
- SelectedResource
- TechVoteSaveData
- ChanceObjectList
- PlayerRoleSaveData
- DayAndNightRuntimeData
- DayAndNightSettings
- .DrawDataFieldAndLabel
- WorldGenSaveData
- STSM_HelperBase
- UI_TechOption
- UILineRenderer
- UserInterface_DisplayUsernames
- Access_GOList
- BuildingModelHandler
- PlayerDeathHandler
- AllBuildingDataSettings
- ResourceDataSettings
- GameEventSettings
- PassiveResourceIncrementer
- GridProcessor.cs
- EventProcessor
- SelectedEnemyCamp
- SelectedPlayerGroup
- ParallelProgressReporter
- FPSDisplay.cs
- SimpleDisableAfterTime
- RotationHandler
- GridSettings
- Requirement
- InventoryEntrySaveData
- RandomEnabler
- StringUtils
- ScriptKeywordProcessor
- UnitTravelToPosition
- BuildingConfigSettings
- FoliageGenSettings
- TimeSettings
- WaterFoliageGenSettings
- WorldGenBehaviorSettings
- SensorSettings
- TownGoalSettings
- WeatherSettings
- EquipmentHandlerEditor
- GameEventRuntimeData
- VideoSettingsPreset
- UnityGraphics
- SimpleScreenShot
- IntWrapper
- TL_API
- CreateProjectScopeProcessors.cs
- .SaveGame
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- .RefreshSceneBindingsAndTryGenerate
- ScriptableObjectAssetData
- StatusBar
- SimpleRotateOnAxis
- CustomLogger
- ObjectSelectionProcessor.Editor.cs
- PathProbe
- .InjectRuntimeData
- FoliageSaveData
- SavePlayersData
- TwitchClientRuntimeData
- TechTreeNodeType.cs
- PoolablePlayer.cs
- .RefreshSceneData
- SelectionBase.cs

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
- `StreamTownSessionBridge` --references--> `SaveProcessor`  [EXTRACTED]
  Assets/Editor/StreamTownSessionBridge.cs → Assets/Scripts/Core/Processors/SaveProcessor.cs
- `SettingsProcessor` --references--> `GraphicsProcessor`  [EXTRACTED]
  Assets/Scripts/Settings/SettingsProcessor.cs → Assets/GraphicsProcessor.cs
- `WorldGenProcessor` --references--> `ProjectCamera`  [EXTRACTED]
  Assets/Scripts/Core/Processors/WorldGenProcessor.cs → Assets/ProjectCamera.cs
- `SettingsProcessor` --references--> `ProjectCamera`  [EXTRACTED]
  Assets/Scripts/Settings/SettingsProcessor.cs → Assets/ProjectCamera.cs
- `SettingsProcessor` --references--> `PresetButtons`  [EXTRACTED]
  Assets/Scripts/Settings/SettingsProcessor.cs → Assets/ReflexDI/PresetButtons.cs

## Import Cycles
- None detected.

## Communities (261 total, 29 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.04
Nodes (44): ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, WorldGenDebugSettingsInstaller, ContainerBuilder, WorldGenLayerSettingsInstaller, ContainerBuilder, WorldGenScaleSettingsInstaller (+36 more)

### Community 3 - "Processors"
Cohesion: 0.06
Nodes (11): BuildCostModifier, InputButton, UserInterface.MainMenu, Processors, World, Level, MetaData, Buildings (+3 more)

### Community 4 - "Player"
Cohesion: 0.10
Nodes (7): Player, Dictionary, GameObject, Vector3, GameMasterCommands, RoleCommands, RulerCommands

### Community 5 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (14): bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int, IReadOnlyList (+6 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ObjectPoolingProcessor"
Cohesion: 0.07
Nodes (21): bool, List, ObjectPoolingSettings, Action, bool, BoxCollider, CancellationToken, Container (+13 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (12): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+4 more)

### Community 11 - "IInstaller"
Cohesion: 0.04
Nodes (31): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ChannelDataInstaller, ContainerBuilder (+23 more)

### Community 12 - "Units"
Cohesion: 0.07
Nodes (12): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, PlayerControls.ObjectSelection (+4 more)

### Community 13 - "Resource"
Cohesion: 0.08
Nodes (9): int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor, Resource (+1 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "FoliageProcessor"
Cohesion: 0.09
Nodes (24): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+16 more)

### Community 16 - "PlayerRole"
Cohesion: 0.08
Nodes (8): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, PlayerRole

### Community 17 - "Target"
Cohesion: 0.09
Nodes (9): Target, Utils.Pooling, GridSystem.Partitioning, Combat, Environment, SavingAndLoading.SavableObjects, Enemies, GUIDSystem (+1 more)

### Community 18 - "Character"
Cohesion: 0.11
Nodes (11): Pets.Enumerations, TownGoal, Character.Enumerations, Core, Pets, GameEventSystem, GameEventSystem.Events, Twitch.Commands (+3 more)

### Community 19 - "UserInterface"
Cohesion: 0.06
Nodes (13): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, UserInterface (+5 more)

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.10
Nodes (22): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+14 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.06
Nodes (15): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+7 more)

### Community 24 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (7): Action, CancellationToken, int, List, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "MonoBehaviour"
Cohesion: 0.05
Nodes (20): CameraProcessor, PersistentScoped, Projectile, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective (+12 more)

### Community 27 - "Utils"
Cohesion: 0.06
Nodes (5): Utils, Animation, STStateMachine, Audio, SavingAndLoading.Structs

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (20): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+12 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 30 - "DebugProcessor"
Cohesion: 0.08
Nodes (9): Container, ContainerBuilder, DebugProcessor, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack (+1 more)

### Community 31 - "Targetable"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+7 more)

### Community 32 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (17): Vector3, TargetSettings, TargetableData, Dictionary, List, Foliage, FoliageSaveType, FoliageType (+9 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.10
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 34 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "HealthHandler"
Cohesion: 0.09
Nodes (12): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, STSM_Action_Heal, Action, bool, float (+4 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (14): string, GameIO, SaveFileType, bool, float, int, string, SettingsData (+6 more)

### Community 38 - ".LoadGameAsync"
Cohesion: 0.18
Nodes (10): Action, CancellationToken, List, Task, SaveOperationState, SaveRuntimeData, building, data (+2 more)

### Community 39 - ".CreateEnumField"
Cohesion: 0.11
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.10
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "STSM_Idle_Player"
Cohesion: 0.10
Nodes (11): AIPath, bool, float, Vector3, STSM_Idle_Enemy, bool, float, uint (+3 more)

### Community 44 - "PoolableObject"
Cohesion: 0.11
Nodes (17): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, Dictionary, float (+9 more)

### Community 45 - "SaveProcessor"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, Dictionary, float, Material, materialIndex, materials, Mesh (+6 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "Enemy"
Cohesion: 0.10
Nodes (14): Action, float, Enemy, uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp (+6 more)

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "TargetSensor"
Cohesion: 0.11
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 52 - "ResourceProcessor"
Cohesion: 0.22
Nodes (5): Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 55 - "Station"
Cohesion: 0.07
Nodes (18): Station, Dictionary, float, int, List, Queue, Transform, Container (+10 more)

### Community 56 - "RoleDataSettings"
Cohesion: 0.08
Nodes (18): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip (+10 more)

### Community 57 - ".Log"
Cohesion: 0.13
Nodes (8): Action, HideInCallstack, Object, DebugLogCategory, LoadSceneMode, Scene, GridGraph, ResourceData[]&gt;

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.12
Nodes (9): STSM_HelperDeposit, AIPath, bool, float, GameObject, int, Transform, Vector3 (+1 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "BuildingBase"
Cohesion: 0.10
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 61 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "RoleHandler"
Cohesion: 0.13
Nodes (8): RoleHandler, AIPath, bool, Dictionary, UnityEvent, StatModifiers, Dictionary, StatType

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 70 - "MeshData"
Cohesion: 0.16
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
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.14
Nodes (13): bool, CancellationTokenSource, int, long, MenuItem, string, DeviceCodeResponse, ErrorResponse (+5 more)

### Community 80 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 83 - "SensorProcessor"
Cohesion: 0.15
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "ResourceInventory"
Cohesion: 0.10
Nodes (14): ResourceInventory, bool, int, float, int, Queue, ResourceRateOfChange, Dictionary (+6 more)

### Community 86 - "UIProcessor"
Cohesion: 0.16
Nodes (3): Container, ContainerBuilder, UIProcessor

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TechTreeNode"
Cohesion: 0.13
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 89 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.14
Nodes (8): AnimationHandler, AIPath, Animator, bool, Dictionary, float, int, AnimationName

### Community 91 - "PlayerRoleData"
Cohesion: 0.14
Nodes (7): PlayerRoleData, AIPath, AudioClip, bool, float, int, List

### Community 92 - "FoliageData"
Cohesion: 0.13
Nodes (12): Material, Mesh, Quaternion, Vector3, FoliageData, List, Material, Mesh (+4 more)

### Community 93 - "RaidEvent"
Cohesion: 0.15
Nodes (7): bool, IEnumerator, int, List, string, RaidEvent, GameEvent

### Community 94 - "STSM_StateAction"
Cohesion: 0.13
Nodes (9): int, STSM_Helper_Attack, int, STSM_Action_Attack, AIPath, bool, float, int (+1 more)

### Community 95 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 96 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 97 - "PlayerInventory"
Cohesion: 0.15
Nodes (6): DepositResources, PlayerInventory, Dictionary, float, STSM_Action_DepositResource, STStateBase

### Community 98 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 99 - "ResourceGenerationSettings"
Cohesion: 0.11
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 100 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 102 - "MainMenuManager"
Cohesion: 0.15
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.13
Nodes (10): Dictionary, float, GameObject, Image, string, TextMeshProUGUI, Transform, LoadingManager (+2 more)

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

### Community 112 - "EnemySpawner"
Cohesion: 0.15
Nodes (6): Transform, float, int, List, Transform, EnemySpawner

### Community 114 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 115 - "TransformSaveData"
Cohesion: 0.12
Nodes (14): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+6 more)

### Community 116 - "TechTreeRuntimeData"
Cohesion: 0.16
Nodes (5): bool, Dictionary, float, int, TechTreeRuntimeData

### Community 117 - "IProcessor.cs"
Cohesion: 0.15
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 120 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 121 - "Goal"
Cohesion: 0.16
Nodes (4): EventType, Action, Dictionary, Goal

### Community 122 - ".SetTargetType"
Cohesion: 0.20
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "Season"
Cohesion: 0.18
Nodes (11): float, int, Material, AllSeasonSettings, Color, float, int, VisualEffect (+3 more)

### Community 125 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 126 - "IRuntimeDataScriptable"
Cohesion: 0.13
Nodes (14): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+6 more)

### Community 127 - "CommandDictionary"
Cohesion: 0.22
Nodes (6): Action, Dictionary, IReadOnlyList, List, CommandDictionary, ModeratorCommands

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "IProcessor"
Cohesion: 0.23
Nodes (4): CancellationToken, Task, Container, IProcessor

### Community 130 - "DayAndNightProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, DayAndNightProcessor

### Community 132 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 133 - "WeatherProcessor"
Cohesion: 0.19
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 136 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 137 - ".ExerciseHealthBarCategory"
Cohesion: 0.18
Nodes (11): bool, double, float, Func, int, IReadOnlyList, List, long (+3 more)

### Community 138 - "CampGenerationSettings"
Cohesion: 0.15
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 139 - "ResourceTarget"
Cohesion: 0.24
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 140 - "ResourceData"
Cohesion: 0.21
Nodes (6): bool, int, Matrix4x4, uint, Vector3, ResourceData

### Community 141 - "UserInterface_Resources"
Cohesion: 0.21
Nodes (7): Slider, TextMeshProUGUI, Color, GameObject, Slider, TextMeshProUGUI, UserInterface_Resources

### Community 143 - "WorldSaveData"
Cohesion: 0.18
Nodes (10): List, SaveGameData, bool, float, int, List, string, ResourceAmountSaveData (+2 more)

### Community 144 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 145 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 146 - ".LoadSceneAsync"
Cohesion: 0.21
Nodes (5): Task, bool, float, string, LoadingProgressReporter

### Community 147 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 148 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 149 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

### Community 150 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 151 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 152 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 153 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 154 - "FishGodEvent"
Cohesion: 0.21
Nodes (5): Animator, GameObject, IEnumerator, int, FishGodEvent

### Community 155 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.24
Nodes (10): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+2 more)

### Community 157 - "FoliageGroupSaveData"
Cohesion: 0.38
Nodes (6): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData

### Community 159 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

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

### Community 164 - "SaveDataMapper"
Cohesion: 0.27
Nodes (5): Mesh, Vector3, SaveDataMapper, float, Vector3SaveData

### Community 165 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 166 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 167 - "UserInterface_Event"
Cohesion: 0.20
Nodes (7): Slider, TextMeshProUGUI, UIRuntimeData, GameObject, Slider, TextMeshProUGUI, UserInterface_Event

### Community 168 - "NewKingVote"
Cohesion: 0.24
Nodes (3): int, List, NewKingVote

### Community 169 - "CreditsProcessor"
Cohesion: 0.22
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 170 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 171 - "PlayerSaveData"
Cohesion: 0.18
Nodes (9): int, PlayerCustomizationSaveData, bool, int, List, string, uint, UserType (+1 more)

### Community 172 - "TechNodeData"
Cohesion: 0.25
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - ".ValidateTokenAsync"
Cohesion: 0.33
Nodes (6): CancellationToken, Dictionary, Task, UnityWebRequest, TokenValidationResponse, WebResponse

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

### Community 177 - ".StartMusic"
Cohesion: 0.49
Nodes (3): SeasonAudioData, AudioClip, List

### Community 178 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 179 - "TerrainGenSettings"
Cohesion: 0.20
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 180 - "BuildingSettings"
Cohesion: 0.20
Nodes (4): bool, Dictionary, int, BuildingSettings

### Community 181 - "DebugSettings"
Cohesion: 0.29
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 182 - "TradeProcessor"
Cohesion: 0.20
Nodes (6): Dictionary, float, TradeSettings, Container, ContainerBuilder, TradeProcessor

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 187 - "TechVoteSaveData"
Cohesion: 0.33
Nodes (7): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData

### Community 189 - "ChanceObjectList"
Cohesion: 0.27
Nodes (5): float, ChanceObject, float, List, ChanceObjectList

### Community 191 - "DayAndNightRuntimeData"
Cohesion: 0.28
Nodes (3): bool, float, DayAndNightRuntimeData

### Community 192 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 193 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 194 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (9): bool, int, MeshSaveData, float, Vector2SaveData, bool, int, List (+1 more)

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

### Community 201 - "PlayerDeathHandler"
Cohesion: 0.25
Nodes (5): PlayerDeathHandler, AIPath, bool, float, Vector3

### Community 202 - "AllBuildingDataSettings"
Cohesion: 0.25
Nodes (5): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingDataContainerInstaller, AllBuildingDataSettings

### Community 203 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 204 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 205 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 206 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 207 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 213 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 214 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 215 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 216 - "InventoryEntrySaveData"
Cohesion: 0.33
Nodes (4): bool, int, string, InventoryEntrySaveData

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

### Community 222 - "FoliageGenSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, FoliageGenSettingsInstaller, List, FoliageGenSettings

### Community 223 - "TimeSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, TimeDataSettingsInstaller, int, TimeSettings

### Community 224 - "WaterFoliageGenSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WaterFoliageGenSettingsInstaller, List, WaterFoliageGenSettings

### Community 225 - "WorldGenBehaviorSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenBehaviorSettingsInstaller, bool, WorldGenBehaviorSettings

### Community 228 - "SensorSettings"
Cohesion: 0.33
Nodes (4): float, SensorSettings, ContainerBuilder, SensorSettingsInstaller

### Community 229 - "TownGoalSettings"
Cohesion: 0.33
Nodes (4): int, TownGoalSettings, ContainerBuilder, TownGoalSettingsInstaller

### Community 230 - "WeatherSettings"
Cohesion: 0.33
Nodes (4): VisualEffect, WeatherSettings, ContainerBuilder, WeatherSettingsInstaller

### Community 231 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 232 - "GameEventRuntimeData"
Cohesion: 0.33
Nodes (6): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData

### Community 233 - "VideoSettingsPreset"
Cohesion: 0.33
Nodes (4): bool, int, string, VideoSettingsPreset

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

### Community 240 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 241 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 242 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 243 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 244 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 245 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 247 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 248 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 249 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

## Knowledge Gaps
- **25 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+20 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **29 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `ScriptablesProcessorInfrastructure`, `UpdateGraphBounds`, `Processors`, `SelectionBase.cs`, `Units`, `Target`, `Character`, `UserInterface`, `.GenerateFromSettings`, `TechTree.Elements`, `BuildingPlacer`, `MonoBehaviour`, `CommonEnums.cs`, `LabelDisplayProcessor`, `.CreateEnumField`, `Easings`, `ChanceObjectList`, `.DrawDataFieldAndLabel`, `MeshData`, `FPSDisplay.cs`, `SimpleDisableAfterTime`, `RandomEnabler`, `StringUtils`, `ResourceGenerationSettings`, `SimpleScreenShot`, `SimpleRotateOnAxis`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Why does `SaveProcessor` connect `SaveProcessor` to `IProcessor`, `BuildingProcessor`, `.CapturePlayers`, `WorldGenProcessor`, `ObjectPoolingProcessor`, `SettingsProcessor`, `IInstaller`, `Resource`, `FoliageProcessor`, `PlayerRole`, `Character`, `PlayerProcessor`, `MonoBehaviour`, `SaveFileData`, `GameEventProcessor`, `DebugProcessor`, `UserInterface_GameMenu`, `SaveDataMapper`, `.LoadGameAsync`, `TechTreeProcessor`, `PoolableObject`, `SeasonProcessor`, `ResourceProcessor`, `StreamTownSessionBridge`, `PlayerCommands`, `FoliageData`, `FoliageGenSettings`, `WaterFoliageGenSettings`, `.PrepareRuntimeForLoad`, `ResourceGenerationSettings`, `MainMenuManager`, `TownGoalProcessor`, `.SaveGame`, `TimeProcessor`?**
  _High betweenness centrality (0.072) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `ScriptablesProcessorInfrastructure`, `GraphicsProcessor`, `IProcessor`, `Access_Slider`, `SettingsData`, `MainMenuManager`, `ProjectCamera`, `Access_GOList`, `VideoSettingsPreset`, `UIElementWrapper`, `IInstaller`, `SaveProcessor`, `Access_Text`, `Access_Toggle`, `Access_Dropdown`, `MonoBehaviour`, `.LoadWorldScene`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _25 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07052631578947369 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07450980392156863 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.03670634920634921 - nodes in this community are weakly interconnected._