# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 595 files · ~868,235 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 5787 nodes · 13246 edges · 270 communities (243 shown, 27 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 978 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1c56855b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ScriptablesProcessorInfrastructure
- BuildingProcessor
- ScriptableObject
- Processors
- TradeProcessor
- WorldGenProcessor
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- ObjectPoolingProcessor
- SettingsProcessor
- CampGenerationSettings
- Character
- Resource
- TechTreeIOUtility
- FoliageProcessor
- RoleHandler
- World.Generation
- STSM_Action_PlayerBase
- GameEvent
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
- GenerationSettings
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
- LabelDisplayProcessor
- legacy.rs
- SeasonProcessor
- ResourceHolder
- BinarySaveCodec
- VfxSeagullSpawner
- AudioHandler
- TargetSensor
- ResourceProcessor
- TwitchClientProcessor
- StreamTownSessionBridge
- BevyMigrationExporter
- AIPath
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
- models.rs
- Tiler
- ScriptablesEditor
- EnemyModelHandler
- UserInterface_ObjectSelection
- RoleSlot
- UserInterface_Resources
- GridNode
- TwitchBotSetupWindow
- GamestateJukebox
- WorldUtils
- Pet
- BinaryReader
- Access_Text
- UserInterface
- UIProcessor
- UserInterface_TownVote
- STSM_StateAction
- Access_Dropdown
- AnimationHandler
- GridPos
- TransformSaveData
- PlayerCommands
- convert_fbx_to_glb.py
- .Draw
- stream_town_migrate/src/content.rs
- MonoBehaviour
- Coordinator
- FrameCapture
- Editor
- .Log
- MainMenuManager
- TownGoalProcessor
- Player
- UnitHealthBar
- LoadingManager
- UIElementWrapper
- ResourceStorageModifier
- CustomLogHandler
- LevelHandler
- MeshSaveData
- EnemySpawner
- SelectedBuilding
- PlayerInputProcessor
- PlayerSaveData
- Goal
- IProcessor.cs
- VoteEvent
- SeasonDataSettings
- DayAndNightProcessor
- SelectedObject
- .SetTargetType
- SnapToGridMouseMovement
- ResourceGenerationSettings
- GameStateProcessor
- UserInterface_TownGoal
- CommandDictionary
- UpdateGraphBounds
- IProcessor
- stream_town_migrate/src/main.rs
- AudioSourcesProcessor
- DebugSettings
- WeatherProcessor
- ConfirmCheck
- Sensors
- tools_ui
- BuildPlacerData
- PlayerRoleData
- TownResourceRuntimeData
- GridProcessor
- What You Must Do When Invoked
- RuntimeData Template
- WorldSaveData
- RuntimeData Template
- Key Rules
- LoadingProgressReporter
- ChanceObjectList
- RaidEvent
- BuildingResourceModelHandler
- GateController
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- WindController
- stream_town_domain/src/content.rs
- RoleData
- ResourceDataSaveData
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
- MiscCommands
- NewKingVote
- .StartMusic
- DayAndNightRuntimeData
- TimeProcessor
- Vector3
- UserInterface_BuildingHealthBar
- UnitTextDisplay
- EditorHelpers
- generate_world
- GameConfig
- Targetable
- GameEventConfigSettingsInstaller
- Settings Scriptable Template
- select_grid_cell
- ProjectCamera
- WorldGenRuntimeData
- Easings
- SelectedEnemy
- ErrorData
- CommonEnums.cs
- Station
- EventProcessor
- Access_GOList
- RoleSlotModifier
- DayAndNightSettings
- .DrawDataFieldAndLabel
- PlayerDeathHandler
- ParallelProgressReporter
- UI_TechOption
- UILineRenderer
- UserInterface_DisplayUsernames
- SelectedPlayerGroup
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- AllSeasonSettings
- GridSettings
- Key Rules
- GridSystem.Partitioning
- Common Patterns
- BuildingSettings
- SelectedPlayer
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
- WorldGenDebugSettings
- Processor Template
- Common Patterns
- SelectedEnemyCamp
- TradeSettings
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
- IInstaller
- TL_API
- CreateProjectScopeProcessors.cs
- Autosave
- RoleDataSettings
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SelectedResource
- WeatherSettings
- AGENTS.md
- GameEventRuntimeData
- ResourceDataSettings
- CustomLogger
- PlayerInputRuntimeData
- extraction-spec.md
- .RefreshSceneBindingsAndTryGenerate
- WorldGenSaveData
- GameEventSettings
- KeepKingVote
- TechTreeNodeType.cs
- PoolablePlayer.cs
- BuildingRuntimeData
- PassiveResourceIncrementer
- SensorProcessor
- .InjectRuntimeData
- .InjectRuntimeData
- SimpleDisableAfterTime
- TimeSettings
- WaterFoliageGenSettings
- WorldGenScaleSettings

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

## Communities (270 total, 27 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.05
Nodes (34): ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, WorldGenBehaviorSettingsInstaller, ContainerBuilder, WorldGenLayerSettingsInstaller, SaveSettings, float (+26 more)

### Community 3 - "Processors"
Cohesion: 0.08
Nodes (10): ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, Processors.Editor, MetaData, Audio, Settings (+2 more)

### Community 4 - "TradeProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, TradeProcessor

### Community 5 - "WorldGenProcessor"
Cohesion: 0.09
Nodes (12): Action, bool, Container, GameObject, IEnumerable, int, IReadOnlyList, string (+4 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ObjectPoolingProcessor"
Cohesion: 0.06
Nodes (32): bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller, Action, bool, BoxCollider (+24 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 12 - "Character"
Cohesion: 0.05
Nodes (28): ActivityStatus, InputButton, SharedTypes, bool, float, string, UserType, TwitchUser (+20 more)

### Community 13 - "Resource"
Cohesion: 0.10
Nodes (6): DepositResources, IResourceHolder, Container, Dictionary, TownResourceProcessor, Resource

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 16 - "RoleHandler"
Cohesion: 0.07
Nodes (10): RoleHandler, bool, Dictionary, UnityEvent, Container, ContainerBuilder, int, List (+2 more)

### Community 18 - "STSM_Action_PlayerBase"
Cohesion: 0.10
Nodes (10): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_Heal (+2 more)

### Community 19 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.11
Nodes (17): HashSet, Func, HashSet, List, Material, Mesh, Resource, Vector2 (+9 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.07
Nodes (21): int, ChangeTimeStamp, ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData (+13 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "TechTreeNode"
Cohesion: 0.12
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "stream_town_game/src/lib.rs"
Cohesion: 0.11
Nodes (55): AppExit, actor_color(), Agent, AgentAnimation, animate_agents(), camera_controls(), cleanup_state_entities(), cleanup_world() (+47 more)

### Community 27 - "Utils"
Cohesion: 0.05
Nodes (7): BuildCostModifier, Utils, World, Level, Buildings, SavingAndLoading.Structs, GameResources

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (20): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+12 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.08
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "PlayerInventory"
Cohesion: 0.22
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 31 - "PoolableObject"
Cohesion: 0.06
Nodes (27): Container, ContainerBuilder, GUIDProcessor, Action, float, Enemy, int, ActiveResourceIncrementer (+19 more)

### Community 32 - "GenerationSettings"
Cohesion: 0.11
Nodes (16): AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings, Action, IEnumerator (+8 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 34 - "GlobalAudioController"
Cohesion: 0.22
Nodes (6): GlobalAudioController, AudioSource, bool, float, IEnumerator, Season

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "HealthHandler"
Cohesion: 0.13
Nodes (7): Func, Action, bool, float, int, UnityEvent, HealthHandler

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.06
Nodes (30): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+22 more)

### Community 39 - ".CreateEnumField"
Cohesion: 0.12
Nodes (12): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+4 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.06
Nodes (22): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+14 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (11): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+3 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 44 - "LabelDisplayProcessor"
Cohesion: 0.14
Nodes (6): Container, ContainerBuilder, LabelDisplayProcessor, float, ParticleSystem, VFXArrowPointer

### Community 45 - "legacy.rs"
Cohesion: 0.06
Nodes (86): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+78 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 48 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (6): Action, CancellationToken, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 50 - "AudioHandler"
Cohesion: 0.15
Nodes (7): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler

### Community 51 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 52 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "StreamTownSessionBridge"
Cohesion: 0.19
Nodes (3): MenuItem, StreamTownSessionBridge, FrameCapture

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "AIPath"
Cohesion: 0.11
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 57 - "StableId"
Cohesion: 0.17
Nodes (19): FromStr, StableId, ActorState, BuildingState, complete_gameplay_scenario_round_trips(), deterministic_weather(), id(), BTreeMap (+11 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "BuildingBase"
Cohesion: 0.11
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 61 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 62 - "CameraController"
Cohesion: 0.10
Nodes (11): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+3 more)

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
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "IRuntimeDataScriptable"
Cohesion: 0.17
Nodes (10): Queue, AudioRuntimeData, Queue, AudioSourcesRuntimeData, CreditsRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+2 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 70 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 71 - "models.rs"
Cohesion: 0.24
Nodes (14): ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result, String (+6 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (11): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+3 more)

### Community 74 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (10): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+2 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 77 - "UserInterface_Resources"
Cohesion: 0.21
Nodes (7): Slider, TextMeshProUGUI, Color, GameObject, Slider, TextMeshProUGUI, UserInterface_Resources

### Community 78 - "GridNode"
Cohesion: 0.15
Nodes (10): int, List, Vector2, GridNode, Color, CollisionColours, CollisionType, float (+2 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GamestateJukebox"
Cohesion: 0.11
Nodes (13): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+5 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 83 - "BinaryReader"
Cohesion: 0.13
Nodes (5): Func, List, List, SavePlayersData, BinaryReader

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "UserInterface"
Cohesion: 0.09
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, UserInterface, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

### Community 86 - "UIProcessor"
Cohesion: 0.16
Nodes (3): Container, ContainerBuilder, UIProcessor

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "STSM_StateAction"
Cohesion: 0.12
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 89 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.11
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 91 - "GridPos"
Cohesion: 0.14
Nodes (20): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError, OpenNode (+12 more)

### Community 92 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 93 - "PlayerCommands"
Cohesion: 0.16
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.33
Nodes (12): arguments(), convert(), discover_sources(), inspect_glb(), main(), normalized_relative(), Path, Deterministically convert Stream Town FBX sources to self-contained GLB files. (+4 more)

### Community 95 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.16
Nodes (40): asset(), authored_value(), child_technology_guids(), ContentConversionReport, convert(), convert_export(), converts_active_catalog_references_and_round_trips_ron(), field() (+32 more)

### Community 97 - "MonoBehaviour"
Cohesion: 0.03
Nodes (39): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, MetaDataInstaller, PersistentScoped, ContainerBuilder, FoliageGenSettingsInstaller (+31 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "FrameCapture"
Cohesion: 0.22
Nodes (10): bool, double, float, int, IReadOnlyList, List, long, string (+2 more)

### Community 100 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - ".Log"
Cohesion: 0.07
Nodes (14): Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, int, STSM_Helper_Build, STSM_Action_GatherResource (+6 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.11
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "Player"
Cohesion: 0.07
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+1 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.07
Nodes (15): GameObject, List, PresetButtons, Access_AOToggle, Access_ChannelNameInput, ContainerBuilder, Access_EdgeScrollingToggle, Access_MouseControlsToggle (+7 more)

### Community 108 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 109 - "CustomLogHandler"
Cohesion: 0.17
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "MeshSaveData"
Cohesion: 0.18
Nodes (7): bool, int, MeshSaveData, float, Vector2SaveData, float, Vector3SaveData

### Community 112 - "EnemySpawner"
Cohesion: 0.15
Nodes (6): Transform, float, int, List, Transform, EnemySpawner

### Community 114 - "PlayerInputProcessor"
Cohesion: 0.12
Nodes (9): PlayerInputSettings, ContainerBuilder, PlayerInputSettingsInstaller, IMainThreadInitializableProcessor, Container, ContainerBuilder, InputButton, Vector2 (+1 more)

### Community 115 - "PlayerSaveData"
Cohesion: 0.06
Nodes (24): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, bool (+16 more)

### Community 116 - "Goal"
Cohesion: 0.10
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 117 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 120 - "DayAndNightProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, DayAndNightProcessor

### Community 121 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 122 - ".SetTargetType"
Cohesion: 0.15
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
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "IProcessor"
Cohesion: 0.23
Nodes (4): CancellationToken, Task, Container, IProcessor

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "AudioSourcesProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, AudioSourcesProcessor

### Community 132 - "DebugSettings"
Cohesion: 0.29
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 133 - "WeatherProcessor"
Cohesion: 0.23
Nodes (3): Container, ContainerBuilder, WeatherProcessor

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

### Community 138 - "PlayerRoleData"
Cohesion: 0.12
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, StatType

### Community 139 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 140 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "LoadingProgressReporter"
Cohesion: 0.24
Nodes (4): bool, float, string, LoadingProgressReporter

### Community 147 - "ChanceObjectList"
Cohesion: 0.27
Nodes (5): float, ChanceObject, float, List, ChanceObjectList

### Community 148 - "RaidEvent"
Cohesion: 0.07
Nodes (19): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+11 more)

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

### Community 154 - "stream_town_domain/src/content.rs"
Cohesion: 0.24
Nodes (15): AuthoredRecord, AuthoredValue, BuildingDef, ContentCatalog, ContentError, RoleDef, BTreeMap, BTreeSet (+7 more)

### Community 155 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

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
Cohesion: 0.12
Nodes (14): ContainerBuilder, AllBuildingDataSettingsInstaller, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingDataContainerInstaller, int, ResourceCostData (+6 more)

### Community 166 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 167 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 168 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 169 - ".StartMusic"
Cohesion: 0.56
Nodes (3): SeasonAudioData, AudioClip, List

### Community 170 - "DayAndNightRuntimeData"
Cohesion: 0.28
Nodes (3): bool, float, DayAndNightRuntimeData

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "generate_world"
Cohesion: 0.27
Nodes (12): cell_hash(), changing_seed_changes_world_hash(), generate_world(), GeneratedResource, GeneratedWorld, generation_is_deterministic(), hash_world(), String (+4 more)

### Community 177 - "GameConfig"
Cohesion: 0.16
Nodes (14): ConfigError, default_configuration_is_valid_and_round_trips_ron(), GameConfig, GameplayConfig, Default, Result, Self, String (+6 more)

### Community 178 - "Targetable"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 179 - "GameEventConfigSettingsInstaller"
Cohesion: 0.33
Nodes (4): ContainerBuilder, GameEventConfigSettingsInstaller, bool, GameEventConfigSettings

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "select_grid_cell"
Cohesion: 0.17
Nodes (13): Camera, Option, select_grid_cell(), SelectedCell, SelectionMarker, world_to_grid(), Camera2d, GlobalTransform (+5 more)

### Community 182 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 186 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 187 - "CommonEnums.cs"
Cohesion: 0.12
Nodes (15): TargetSettings, ContainerBuilder, TargetSettingsInstaller, TargetableData, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus (+7 more)

### Community 188 - "Station"
Cohesion: 0.06
Nodes (23): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+15 more)

### Community 189 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 190 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 192 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 193 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 194 - "PlayerDeathHandler"
Cohesion: 0.25
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 196 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "AllSeasonSettings"
Cohesion: 0.29
Nodes (6): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings

### Community 204 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "GridSystem.Partitioning"
Cohesion: 0.19
Nodes (4): GridProcessorEditor, GridSystem.Utils, GridSystem.Partitioning, GridSystem

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingSettings"
Cohesion: 0.20
Nodes (6): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller

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

### Community 221 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 225 - "TradeSettings"
Cohesion: 0.33
Nodes (5): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller

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

### Community 236 - "IInstaller"
Cohesion: 0.04
Nodes (33): ContainerBuilder, InstantiationBarrier, ContainerBuilder, Volume, PostProcessingInstaller, AudioMixerInstaller, AudioMixer, ContainerBuilder (+25 more)

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "RoleDataSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 244 - "WeatherSettings"
Cohesion: 0.33
Nodes (4): VisualEffect, WeatherSettings, ContainerBuilder, WeatherSettingsInstaller

### Community 246 - "GameEventRuntimeData"
Cohesion: 0.33
Nodes (6): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData

### Community 248 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 251 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 254 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 255 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 259 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 260 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 261 - "SensorProcessor"
Cohesion: 0.09
Nodes (11): float, SensorSettings, ContainerBuilder, SensorSettingsInstaller, float, List, SensorRuntimeData, SensorBase (+3 more)

### Community 264 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 268 - "TimeSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, TimeDataSettingsInstaller, int, TimeSettings

### Community 275 - "WaterFoliageGenSettings"
Cohesion: 0.40
Nodes (4): ContainerBuilder, WaterFoliageGenSettingsInstaller, List, WaterFoliageGenSettings

### Community 277 - "WorldGenScaleSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenScaleSettingsInstaller, float, WorldGenScaleSettings

## Knowledge Gaps
- **210 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+205 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **27 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `IProcessor`, `BuildingProcessor`, `WorldGenProcessor`, `ObjectPoolingProcessor`, `SettingsProcessor`, `Character`, `Resource`, `FoliageProcessor`, `WorldSaveData`, `RoleHandler`, `WaterFoliageGenSettings`, `PlayerProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `PoolableObject`, `UserInterface_GameMenu`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `ResourceProcessor`, `StreamTownSessionBridge`, `PlayerCommands`, `MonoBehaviour`, `.Log`, `MainMenuManager`, `TownGoalProcessor`, `IInstaller`, `MeshSaveData`, `PlayerSaveData`, `ResourceGenerationSettings`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `IProcessor`, `ScriptableObject`, `BuildingProcessor`, `.InjectRuntimeData`, `TwitchChatProcessor`, `ObjectPoolingProcessor`, `CampGenerationSettings`, `GridProcessor`, `FoliageProcessor`, `WaterFoliageGenSettings`, `.GenerateFromSettings`, `RaidEvent`, `WorldGenScaleSettings`, `PlayerProcessor`, `PoolableObject`, `GenerationSettings`, `CellSpacePartitioning`, `UserInterface_Debug`, `SaveProcessor`, `Vector3`, `ResourceProcessor`, `TwitchClientProcessor`, `ProjectCamera`, `WorldGenRuntimeData`, `AIPath`, `UserInterface`, `WorldGenDebugSettings`, `MonoBehaviour`, `Coordinator`, `.Log`, `Player`, `IInstaller`, `EnemySpawner`, `PlayerSaveData`, `ResourceGenerationSettings`, `GameStateProcessor`?**
  _High betweenness centrality (0.056) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `BuildingRuntimeData`, `TwitchChatProcessor`, `Character`, `RoleHandler`, `BuildingPlacer`, `PlayerProcessor`, `RoleData`, `GameEventProcessor`, `PoolableObject`, `UserInterface_Debug`, `HealthHandler`, `SaveProcessor`, `MiscCommands`, `CharacterModelHandler`, `LabelDisplayProcessor`, `UnitTextDisplay`, `TargetSensor`, `Station`, `UserInterface_DisplayUsernames`, `Pet`, `UserInterface`, `PlayerCommands`, `VoteEvent`, `.SetTargetType`, `CommandDictionary`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _210 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07135135135135136 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06641604010025062 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.04875886524822695 - nodes in this community are weakly interconnected._