# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 590 files · ~864,469 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 5708 nodes · 13025 edges · 276 communities (249 shown, 27 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 978 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `87773924`
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
- Character
- Resource
- TechTreeIOUtility
- FoliageProcessor
- PlayerRole
- World.Generation
- ResourceProcessor
- TownGoal.Data
- .GenerateFromSettings
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- TechTreeNode
- World.Generation.Settings
- stream_town_game/src/lib.rs
- Utils
- SaveFileData
- GameEventProcessor
- PlayerInventory
- PoolableObject
- Station
- CellSpacePartitioning
- GlobalAudioController
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
- legacy.rs
- SeasonProcessor
- ResourceHolder
- BinaryWriter
- VfxSeagullSpawner
- AudioHandler
- TargetSensor
- ResourceData
- TwitchClientProcessor
- StreamTownSessionBridge
- BevyMigrationExporter
- PathfindingMigrationStubs.cs
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
- IRuntimeDataScriptable
- Objective
- UserInterface_RulerVote
- MeshData
- STSM_Action_GatherResource
- Tiler
- ScriptablesEditor
- EnemyModelHandler
- UserInterface_ObjectSelection
- ResourceRuntimeData
- UserInterface_Resources
- GridProcessor
- TwitchBotSetupWindow
- GamestateJukebox
- WorldUtils
- Pet
- BinarySaveCodec
- Access_Text
- UserInterface
- UIProcessor
- UserInterface_TownVote
- STSM_Action_PlayerAttack
- Access_Dropdown
- AnimationHandler
- GridPos
- TransformSaveData
- RaidEvent
- STSM_StateAction
- .Draw
- save.rs
- PlayerSpawnPoint
- Coordinator
- Access_Toggle
- Editor
- .Log
- MainMenuManager
- TownGoalProcessor
- .SendMessage
- UnitHealthBar
- LoadingManager
- UIElementWrapper
- ResourceStorageModifier
- CustomLogHandler
- LevelHandler
- .RenderResourceType
- DayAndNightProcessor
- SelectedBuilding
- PlayerInputProcessor
- PlayerSaveData
- Goal
- IProcessor.cs
- VoteEvent
- .RestoreObjectiveProgress
- CampGenerationSettings
- AIPath
- .SetTargetType
- SnapToGridMouseMovement
- ResourceGenerationSettings
- GameStateProcessor
- UserInterface_TownGoal
- CommandDictionary
- UpdateGraphBounds
- IProcessor
- stream_town_migrate/src/main.rs
- .Update
- SelectableObject
- WeatherProcessor
- ConfirmCheck
- Sensors
- tools_ui
- BuildPlacerData
- PlayerCommands
- TownResourceRuntimeData
- ObjectSelectionProcessor.Editor.cs
- What You Must Do When Invoked
- RuntimeData Template
- WorldSaveData
- RuntimeData Template
- Key Rules
- .LoadSceneAsync
- VFXArrowPointer
- GameEvent
- BuildingResourceModelHandler
- GateController
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- WindController
- content.rs
- TwitchUser
- .CaptureFoliageGroups
- Stream Town Reloaded - Architecture Documentation
- UserInterface_GameMenu
- .new
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- SimpleMusicController
- BuildingDamageMaterialHandler
- command.rs
- BuildingDataSettings
- EditorUtils
- WorldInstanceDeterminism
- NewKingVote
- CreditsProcessor
- SelectedObject
- TimeProcessor
- .EnsureValidCredentials
- UserInterface_BuildingHealthBar
- LabelDisplayProcessor
- EditorHelpers
- GameConfig
- .ExerciseHealthBarCategory
- Targetable
- TerrainGenSettings
- Settings Scriptable Template
- TechTree_SO
- FishGodEvent
- WorldGenRuntimeData
- Easings
- SelectedEnemy
- ErrorData
- SimpleCancelBuildingPlacer
- CommonEnums.cs
- EventProcessor
- ResourceTarget
- BuildingConfigSettings
- DayAndNightSettings
- .DrawDataFieldAndLabel
- WorldGenDebugSettings
- STSM_HelperBase
- UI_TechOption
- UILineRenderer
- UserInterface_DisplayUsernames
- DebugSettings
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- WorldGenLayerSettings
- GridSettings
- Key Rules
- GridProcessor.cs
- Common Patterns
- BuildingSettings
- RoleHandler
- TownGoalSettings
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
- STSM_Action_DepositResource
- Processor Template
- Common Patterns
- PlayerRoleData
- PostProcessingInstaller
- graphify reference: query, path, explain
- TODO List
- StreamTownGamePlugin
- Bevy Migration Status
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
- RoleDataSettings
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SelectedResource
- RenderPipelineInstaller
- AGENTS.md
- ForwardRendererInstaller
- .GetCompatiblePorts
- .InjectRuntimeData
- CustomLogger
- TL_Secrets
- extraction-spec.md
- .UserIsSubscribed
- SaveDataMapper
- .TryTakeReviveCost
- SavePlayersData
- TechTreeNodeType.cs
- PoolablePlayer.cs
- PlayerDeathHandler
- PassiveResourceIncrementer
- SensorProcessor
- Enemy
- GameEventRuntimeData
- SimpleDisableAfterTime
- ProjectCameraInstaller
- .InjectRuntimeData
- TradeProcessor
- SelectedEnemyCamp
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- VideoSettingsPresetsInstaller
- FoliageGenerationSettings
- TwitchClientRuntimeData
- .InjectRuntimeData

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
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `world_tab()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_tools/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (276 total, 27 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.08
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.03
Nodes (52): ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller, ContainerBuilder, TimeDataSettingsInstaller, ContainerBuilder, WorldGenBehaviorSettingsInstaller (+44 more)

### Community 3 - "Processors"
Cohesion: 0.06
Nodes (12): InputButton, UserInterface.MainMenu, Processors, Core, World, MetaData, Audio, Settings (+4 more)

### Community 4 - "Player"
Cohesion: 0.11
Nodes (7): Player, Dictionary, GameObject, Vector3, GameMasterCommands, RulerCommands, Vector3

### Community 5 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (16): Action, bool, BoxCollider, Container, GameObject, IEnumerable, int, IReadOnlyList (+8 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (20): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, Vector2 (+12 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ObjectPoolingProcessor"
Cohesion: 0.06
Nodes (30): bool, List, ObjectPoolingSettings, Action, bool, BoxCollider, CancellationToken, Container (+22 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "MonoBehaviour"
Cohesion: 0.03
Nodes (58): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ChannelDataInstaller (+50 more)

### Community 12 - "Character"
Cohesion: 0.08
Nodes (13): Pets.Enumerations, StreamTown.EditorTools, TownGoal, Pets, GameEventSystem, GameEventSystem.Events, SavingAndLoading, Twitch.Commands (+5 more)

### Community 13 - "Resource"
Cohesion: 0.09
Nodes (8): DepositResources, int, ActiveResourceIncrementer, IResourceHolder, Container, Dictionary, TownResourceProcessor, Resource

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 16 - "PlayerRole"
Cohesion: 0.05
Nodes (23): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+15 more)

### Community 18 - "ResourceProcessor"
Cohesion: 0.16
Nodes (3): Container, ContainerBuilder, ResourceProcessor

### Community 19 - "TownGoal.Data"
Cohesion: 0.09
Nodes (10): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.11
Nodes (20): HashSet, Func, HashSet, List, Vector2, Vector3, GenerateDebugPositions(), GetPositiveNoiseOffset() (+12 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.09
Nodes (17): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+9 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.09
Nodes (14): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+6 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.10
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 24 - "TechTreeNode"
Cohesion: 0.14
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "stream_town_game/src/lib.rs"
Cohesion: 0.08
Nodes (71): AppExit, actor_color(), Agent, AgentAnimation, animate_agents(), camera_controls(), cleanup_state_entities(), cleanup_world() (+63 more)

### Community 27 - "Utils"
Cohesion: 0.08
Nodes (4): BuildCostModifier, Utils, Level, Buildings

### Community 28 - "SaveFileData"
Cohesion: 0.11
Nodes (19): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+11 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.08
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "PlayerInventory"
Cohesion: 0.14
Nodes (6): PlayerInventory, Dictionary, ResourceInventory, bool, int, Dictionary

### Community 31 - "PoolableObject"
Cohesion: 0.12
Nodes (15): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+7 more)

### Community 32 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 34 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 36 - "HealthHandler"
Cohesion: 0.14
Nodes (8): int, STSM_Helper_Attack, Action, bool, float, int, UnityEvent, HealthHandler

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.08
Nodes (15): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+7 more)

### Community 39 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "STSM_Idle"
Cohesion: 0.17
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 44 - "GUIDProcessor"
Cohesion: 0.11
Nodes (7): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, PoolType

### Community 45 - "legacy.rs"
Cohesion: 0.10
Nodes (59): ActorKind, SavedActor, SavedTerrainMesh, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer() (+51 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.07
Nodes (18): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+10 more)

### Community 47 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 48 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

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
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "StreamTownSessionBridge"
Cohesion: 0.18
Nodes (3): MenuItem, StreamTownSessionBridge, FrameCapture

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "PathfindingMigrationStubs.cs"
Cohesion: 0.16
Nodes (12): Action, bool, float, int, string, Type, AstarData, AstarPath (+4 more)

### Community 57 - "StableId"
Cohesion: 0.17
Nodes (19): FromStr, StableId, ActorState, BuildingState, complete_gameplay_scenario_round_trips(), deterministic_weather(), id(), BTreeMap (+11 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.14
Nodes (8): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.10
Nodes (10): GroupSaveData, TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement (+2 more)

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
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (10): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, ICollection, IDictionary, ISerializationCallbackReceiver (+2 more)

### Community 67 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (14): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+6 more)

### Community 68 - "Objective"
Cohesion: 0.16
Nodes (4): Action, int, Objective, EnemyType

### Community 69 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 70 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (11): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+3 more)

### Community 74 - "EnemyModelHandler"
Cohesion: 0.14
Nodes (8): bool, int, List, EnemyModelHandler, bool, float, Vector3, STSM_Action_EnemyAttack

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 77 - "UserInterface_Resources"
Cohesion: 0.21
Nodes (7): Slider, TextMeshProUGUI, Color, GameObject, Slider, TextMeshProUGUI, UserInterface_Resources

### Community 78 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 83 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "UserInterface"
Cohesion: 0.08
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, UserInterface, Combat, SavingAndLoading.SavableObjects (+1 more)

### Community 86 - "UIProcessor"
Cohesion: 0.16
Nodes (3): Container, ContainerBuilder, UIProcessor

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "STSM_Action_PlayerAttack"
Cohesion: 0.12
Nodes (8): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Heal, STSM_Action_PlayerAttack

### Community 89 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 91 - "GridPos"
Cohesion: 0.14
Nodes (20): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError, OpenNode (+12 more)

### Community 92 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 93 - "RaidEvent"
Cohesion: 0.11
Nodes (13): bool, IEnumerator, int, List, string, RaidEvent, Slider, TextMeshProUGUI (+5 more)

### Community 94 - "STSM_StateAction"
Cohesion: 0.13
Nodes (7): int, STSM_Action_Attack, bool, float, int, STSM_StateAction, AnimationName

### Community 95 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 96 - "save.rs"
Cohesion: 0.18
Nodes (24): detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native(), native_save_is_atomic_and_keeps_backup() (+16 more)

### Community 97 - "PlayerSpawnPoint"
Cohesion: 0.06
Nodes (13): PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, List, SimpleEventOnStart (+5 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 100 - "Editor"
Cohesion: 0.10
Nodes (8): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - ".Log"
Cohesion: 0.08
Nodes (17): IMainThreadInitializableProcessor, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor, int (+9 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.11
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - ".SendMessage"
Cohesion: 0.07
Nodes (7): Vector3, BuildingCommands, Dictionary, MiscCommands, RoleCommands, Dictionary, MessageSender

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.08
Nodes (15): DontDestroyOnLoad, Dictionary, float, GameObject, Image, string, TextMeshProUGUI, Transform (+7 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 114 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 115 - "PlayerSaveData"
Cohesion: 0.11
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 116 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 117 - "IProcessor.cs"
Cohesion: 0.22
Nodes (8): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport, ProcessorStartupStage

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 120 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 121 - "AIPath"
Cohesion: 0.22
Nodes (8): Vector3, AIPath, GraphNode, Int3, NNConstraint, NNInfo, PathUtilities, PathProbe

### Community 122 - ".SetTargetType"
Cohesion: 0.11
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "ResourceGenerationSettings"
Cohesion: 0.20
Nodes (9): List, ResourceGenSettings, List, WaterResourceGenSettings, AnimationCurve, bool, int, List (+1 more)

### Community 125 - "GameStateProcessor"
Cohesion: 0.19
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 126 - "UserInterface_TownGoal"
Cohesion: 0.17
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "IProcessor"
Cohesion: 0.14
Nodes (7): CancellationToken, Task, Container, IPostInitializeProcessor, IProcessor, Dictionary, ParallelProgressReporter

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 132 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 133 - "WeatherProcessor"
Cohesion: 0.12
Nodes (11): VisualEffect, WeatherSettings, ContainerBuilder, WeatherSettingsInstaller, Container, ContainerBuilder, WeatherProcessor, bool (+3 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "Sensors"
Cohesion: 0.09
Nodes (7): STStateMachine.States, Behaviours, Animation, Sensors, STStateMachine, Pathfinding, STStateMachine.Helpers

### Community 136 - "tools_ui"
Cohesion: 0.18
Nodes (23): content_tab(), inspector_tab(), main(), migration_tab(), Commands, Default, Option, ResMut (+15 more)

### Community 137 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 138 - "PlayerCommands"
Cohesion: 0.11
Nodes (7): List, GameSettings, OnMessageReceivedArgs, EventCommands, OnChatCommandReceivedArgs, TwitchClientProcessor, PlayerCommands

### Community 139 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WorldSaveData"
Cohesion: 0.16
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - ".LoadSceneAsync"
Cohesion: 0.21
Nodes (5): Task, bool, float, string, LoadingProgressReporter

### Community 147 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 148 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 149 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

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

### Community 154 - "content.rs"
Cohesion: 0.26
Nodes (11): BuildingDef, ContentCatalog, ContentError, RoleDef, BTreeMap, BTreeSet, Result, String (+3 more)

### Community 155 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 156 - ".CaptureFoliageGroups"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

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
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 163 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 164 - "command.rs"
Cohesion: 0.26
Nodes (11): ChatCommand, CommandParseError, no_argument(), Err, FromStr, Option, Result, Self (+3 more)

### Community 165 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 166 - "EditorUtils"
Cohesion: 0.15
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 167 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 168 - "NewKingVote"
Cohesion: 0.24
Nodes (3): int, List, NewKingVote

### Community 169 - "CreditsProcessor"
Cohesion: 0.22
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 170 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 171 - "TimeProcessor"
Cohesion: 0.16
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+6 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "GameConfig"
Cohesion: 0.12
Nodes (23): ConfigError, default_configuration_is_valid_and_round_trips_ron(), GameConfig, GameplayConfig, Default, Result, Self, String (+15 more)

### Community 177 - ".ExerciseHealthBarCategory"
Cohesion: 0.18
Nodes (11): bool, double, float, Func, int, IReadOnlyList, List, long (+3 more)

### Community 178 - "Targetable"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 179 - "TerrainGenSettings"
Cohesion: 0.33
Nodes (6): AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "TechTree_SO"
Cohesion: 0.18
Nodes (7): int, TechTreeSettings, ContainerBuilder, TechTreeSettingsInstaller, NodeGroup_SO, List, TechTree_SO

### Community 182 - "FishGodEvent"
Cohesion: 0.22
Nodes (6): Animator, GameObject, IEnumerator, int, FishGodEvent, GameEvent

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 186 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 188 - "CommonEnums.cs"
Cohesion: 0.10
Nodes (18): List, Vector3, TargetSettings, TargetableData, Dictionary, List, Foliage, FoliageSaveType (+10 more)

### Community 189 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 190 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 191 - "BuildingConfigSettings"
Cohesion: 0.33
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 192 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 193 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 194 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

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

### Community 199 - "DebugSettings"
Cohesion: 0.24
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "WorldGenLayerSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenLayerSettingsInstaller, LayerMask, WorldGenLayerSettings

### Community 204 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingSettings"
Cohesion: 0.20
Nodes (4): bool, Dictionary, int, BuildingSettings

### Community 209 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 210 - "TownGoalSettings"
Cohesion: 0.33
Nodes (4): int, TownGoalSettings, ContainerBuilder, TownGoalSettingsInstaller

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

### Community 221 - "STSM_Action_DepositResource"
Cohesion: 0.33
Nodes (3): float, STSM_Action_DepositResource, STStateBase

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "PlayerRoleData"
Cohesion: 0.09
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 225 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "StreamTownGamePlugin"
Cohesion: 0.50
Nodes (3): App, StreamTownGamePlugin, Plugin

### Community 229 - "Bevy Migration Status"
Cohesion: 0.17
Nodes (10): Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity, Validation (+2 more)

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

### Community 240 - "RoleDataSettings"
Cohesion: 0.08
Nodes (18): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip (+10 more)

### Community 244 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 246 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 247 - ".GetCompatiblePorts"
Cohesion: 0.50
Nodes (3): List, Port, NodeAdapter

### Community 251 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 253 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 254 - "SaveDataMapper"
Cohesion: 0.07
Nodes (20): List, Mesh, Transform, Vector3, SaveDataMapper, bool, int, MeshSaveData (+12 more)

### Community 259 - "PlayerDeathHandler"
Cohesion: 0.29
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 260 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 261 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 262 - "Enemy"
Cohesion: 0.21
Nodes (3): Action, float, Enemy

### Community 263 - "GameEventRuntimeData"
Cohesion: 0.33
Nodes (6): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData

### Community 264 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 265 - "ProjectCameraInstaller"
Cohesion: 0.33
Nodes (4): Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 267 - "TradeProcessor"
Cohesion: 0.20
Nodes (6): Dictionary, float, TradeSettings, Container, ContainerBuilder, TradeProcessor

### Community 269 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 270 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 271 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 272 - "FoliageGenerationSettings"
Cohesion: 0.15
Nodes (11): List, FoliageGenSettings, List, WaterFoliageGenSettings, List, Material, Mesh, string (+3 more)

## Knowledge Gaps
- **210 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+205 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **27 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `IProcessor`, `BuildingProcessor`, `WorldGenProcessor`, `ObjectPoolingProcessor`, `PlayerCommands`, `MonoBehaviour`, `Character`, `Resource`, `SettingsProcessor`, `FoliageProcessor`, `FoliageGenerationSettings`, `PlayerRole`, `ResourceProcessor`, `PlayerProcessor`, `.CaptureFoliageGroups`, `GameEventProcessor`, `SaveFileData`, `UserInterface_GameMenu`, `TechTreeProcessor`, `TimeProcessor`, `GUIDProcessor`, `SeasonProcessor`, `StreamTownSessionBridge`, `.Log`, `MainMenuManager`, `TownGoalProcessor`, `.RestoreObjectiveProgress`, `ResourceGenerationSettings`, `SaveDataMapper`?**
  _High betweenness centrality (0.068) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `IProcessor`, `ScriptableObject`, `BuildingProcessor`, `Player`, `TwitchChatProcessor`, `ObjectPoolingProcessor`, `MonoBehaviour`, `FoliageProcessor`, `FoliageGenerationSettings`, `ResourceProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `CellSpacePartitioning`, `UserInterface_Debug`, `SaveProcessor`, `GUIDProcessor`, `TerrainGenSettings`, `ResourceData`, `TwitchClientProcessor`, `WorldGenRuntimeData`, `PathfindingMigrationStubs.cs`, `WorldGenDebugSettings`, `WorldGenLayerSettings`, `GridProcessor`, `UserInterface`, `Access_Dropdown`, `CampGenerationSettings`, `RaidEvent`, `Coordinator`, `.Log`, `DayAndNightProcessor`, `.InjectRuntimeData`, `ResourceGenerationSettings`, `GameStateProcessor`, `SaveDataMapper`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `TwitchChatProcessor`, `PlayerCommands`, `PlayerRole`, `VFXArrowPointer`, `BuildingPlacer`, `PlayerProcessor`, `TwitchUser`, `GameEventProcessor`, `PoolableObject`, `Station`, `UserInterface_Debug`, `HealthHandler`, `SaveProcessor`, `CharacterModelHandler`, `LabelDisplayProcessor`, `TargetSensor`, `SimpleCancelBuildingPlacer`, `BuildingBase`, `UserInterface_DisplayUsernames`, `RoleHandler`, `Pet`, `UserInterface`, `PlayerRoleData`, `.Log`, `.SendMessage`, `VoteEvent`, `.SetTargetType`, `CommandDictionary`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _210 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07052631578947369 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07673469387755102 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.03109959274342836 - nodes in this community are weakly interconnected._