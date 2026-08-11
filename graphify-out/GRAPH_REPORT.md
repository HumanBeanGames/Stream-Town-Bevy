# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 603 files · ~1,534,021 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6183 nodes · 14556 edges · 261 communities (235 shown, 26 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 983 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8db6ddab`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ScriptablesProcessorInfrastructure
- BuildingProcessor
- ScriptableObject
- Processors
- stream_town_migrate/src/presentation.rs
- PlayerCommands
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- ObjectPoolingProcessor
- SettingsProcessor
- WorldGenProcessor
- UserInterface
- Resource
- TechTreeIOUtility
- TerrainGenSettings
- PlayerRole
- Character
- .CreateEnumField
- GameEvent
- .GenerateFromSettings
- ObjectiveSaveData
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- World.Generation.Settings
- load_input
- .SendMessage
- SaveFileData
- GameEventProcessor
- retargeted_animation_clip
- GUIDProcessor
- RoleData
- CellSpacePartitioning
- .AddEvent
- UserInterface_Debug
- BuildingBase
- SettingsData
- SaveProcessor
- EnemyModelHandler
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- Targetable
- AnimationControllerRuntime
- legacy.rs
- IProcessor
- Enemy
- BinarySaveCodec
- VfxSeagullSpawner
- AudioHandler
- RoleDataContainer
- ResourceProcessor
- TwitchClientProcessor
- StreamTownSessionBridge
- BevyMigrationExporter
- AIPath
- StableId
- STSM_GoToLocation
- TechTreeEditorWindow
- TechTreeNode
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- UserInterface_TownGoal
- Objective
- StationSensor
- GenerationSettings
- models.rs
- Tiler
- ScriptablesEditor
- STSM_StateAction
- UserInterface_ObjectSelection
- SelectedPlayerGroup
- STSM_Idle
- GridProcessor
- TwitchBotSetupWindow
- GamestateJukebox
- WorldUtils
- Pet
- SimpleRotateOnAxis
- Access_Text
- Target
- UIProcessor
- UserInterface_TownVote
- Access_Toggle
- FoliageProcessor
- AnimationHandler
- GridPos
- SavingAndLoading.Structs
- FoliageGenerationSettings
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
- Access_Dropdown
- ResourceRuntimeData
- CustomLogHandler
- LevelHandler
- ResourceTarget
- DayAndNightProcessor
- SelectedBuilding
- UserInterface_RulerVote
- PlayerSaveData
- Goal
- ProcessorStartupReport
- VoteEvent
- TargetProcessor
- .RenderResourceType
- SelectedObject
- .SetTargetType
- SnapToGridMouseMovement
- UserInterface_GameMenu
- .Log
- ResourceStorageModifier
- CommandDictionary
- UpdateGraphBounds
- ResourceHolder
- stream_town_migrate/src/main.rs
- DebugProcessor
- STSM_Idle_Player
- .StartupSequence
- ConfirmCheck
- Sensors
- ToolState
- generate_and_spawn_world
- PlayerRoleData
- drive_converted_animations
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- WorldSaveData
- RuntimeData Template
- Key Rules
- BuildingSettings
- .GetMissingProcessorDependencies
- BuildPlacerData
- .Update
- GateController
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- WindController
- ErrorData
- UserInterface_Roles
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- TwitchConnection
- .new
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- AudioSourcesProcessor
- SensorProcessor
- command.rs
- Access_GOList
- EditorUtils
- BuildingDamageMaterialHandler
- ParallelProgressReporter
- STSM_HelperBase
- VfxAnimationController
- TimeProcessor
- SelectedResource
- UserInterface_BuildingHealthBar
- LabelDisplayProcessor
- EditorHelpers
- select_grid_cell
- GameConfig
- Access_TextInput
- PlacementProbeHandler
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ProjectCamera
- WorldGenRuntimeData
- SelectedEnemy
- StyleUtility
- UI_TechOption
- Station
- RaidEvent
- TechTree_SO
- TradeProcessor
- KeepKingVote
- Utils
- BuildingRuntimeData
- .InitializeAndActivateProcessorsAsync
- .InjectRuntimeData
- UILineRenderer
- UserInterface_DisplayUsernames
- UnitTravelToPosition
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- CommonEnums.cs
- Key Rules
- Common Patterns
- SimpleDisableAfterTime
- RoleHandler
- FPSDisplay.cs
- NewKingVote
- RotationHandler
- Key Rules
- Requirement
- RuntimeData Template
- RandomEnabler
- StringUtils
- ScriptKeywordProcessor
- TL_API
- IRuntimeDataScriptable
- Processor Template
- Common Patterns
- attach_converted_animations
- .RefreshSceneBindingsAndTryGenerate
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
- GridProcessor.cs
- EventProcessor
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SelectedEnemyCamp
- AGENTS.md
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- Autosave
- CustomLogger
- ForwardRendererInstaller
- extraction-spec.md
- RenderPipelineInstaller
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- VideoSettingsPresetsInstaller
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- TargetSensor
- MonoBehaviour
- VfxParticlePosition
- IntWrapper
- append_vec3_keys

## God Nodes (most connected - your core abstractions)
1. `Utils` - 158 edges
2. `Processors` - 156 edges
3. `ScriptablesProcessorInfrastructure` - 150 edges
4. `Player` - 142 edges
5. `WorldGenProcessor` - 110 edges
6. `SettingsProcessor` - 107 edges
7. `Reflex.Core` - 103 edges
8. `SaveProcessor` - 88 edges
9. `StableId` - 88 edges
10. `BuildingProcessor` - 78 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `world_tab()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_tools/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `stress()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/xtask/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `color_value()` --calls--> `component()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/presentation.rs → bevy-port/crates/stream_town_migrate/src/content.rs

## Import Cycles
- None detected.

## Communities (261 total, 26 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.03
Nodes (68): ContainerBuilder, AllBuildingDataSettingsInstaller, int, AudioSettings, List, CampGenSettings, float, Material (+60 more)

### Community 3 - "Processors"
Cohesion: 0.05
Nodes (12): ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, Processors.Editor, MetaData, Audio, Settings (+4 more)

### Community 4 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.09
Nodes (73): animation_state_id(), array_index(), assign_clip_rigs_and_reference_poses(), clip_id(), collect_prefab_dependencies(), color_value(), controller_id(), convert() (+65 more)

### Community 5 - "PlayerCommands"
Cohesion: 0.12
Nodes (6): List, GameSettings, TwitchClientProcessor, PlayerCommands, Dictionary, MessageSender

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, GroupSaveData, int, List, Port (+13 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (38): Action, CancellationToken, Task, IAsyncInitializableProcessor, ProcessorStartupContext, Action, bool, BoxCollider (+30 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.08
Nodes (14): bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int, IReadOnlyList (+6 more)

### Community 12 - "UserInterface"
Cohesion: 0.06
Nodes (13): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, UserInterface (+5 more)

### Community 13 - "Resource"
Cohesion: 0.04
Nodes (31): DepositResources, PlayerInventory, Dictionary, ResourceInventory, bool, int, int, ActiveResourceIncrementer (+23 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 16 - "PlayerRole"
Cohesion: 0.10
Nodes (7): RoleSlotModifier, int, Container, ContainerBuilder, int, RoleProcessor, PlayerRole

### Community 17 - "Character"
Cohesion: 0.07
Nodes (21): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+13 more)

### Community 18 - ".CreateEnumField"
Cohesion: 0.14
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 19 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+9 more)

### Community 21 - "ObjectiveSaveData"
Cohesion: 0.10
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.09
Nodes (11): Func, PlayerDeathHandler, bool, float, Vector3, Action, bool, float (+3 more)

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "load_input"
Cohesion: 0.14
Nodes (39): AppExit, Agent, AgentAnimation, animate_agents(), animate_weather_particles(), camera_controls(), capture_screenshot(), credits_input() (+31 more)

### Community 27 - ".SendMessage"
Cohesion: 0.09
Nodes (5): Vector3, BuildingCommands, Dictionary, MiscCommands, RoleCommands

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, bool, ParticleSystem (+6 more)

### Community 30 - "retargeted_animation_clip"
Cohesion: 0.24
Nodes (15): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), retargeted_animation_clip() (+7 more)

### Community 31 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 32 - "RoleData"
Cohesion: 0.14
Nodes (14): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+6 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "BuildingBase"
Cohesion: 0.11
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.10
Nodes (14): Action, CancellationToken, Container, ContainerBuilder, float, List, Task, SaveProcessor (+6 more)

### Community 39 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "Targetable"
Cohesion: 0.11
Nodes (10): Vector3, List, bool, BoxCollider, float, int, Transform, Vector3 (+2 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.14
Nodes (18): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+10 more)

### Community 45 - "legacy.rs"
Cohesion: 0.06
Nodes (86): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+78 more)

### Community 46 - "IProcessor"
Cohesion: 0.05
Nodes (27): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Container, IMainThreadInitializableProcessor (+19 more)

### Community 47 - "Enemy"
Cohesion: 0.10
Nodes (15): Action, float, Enemy, uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp (+7 more)

### Community 48 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.17
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 50 - "AudioHandler"
Cohesion: 0.13
Nodes (9): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Queue (+1 more)

### Community 51 - "RoleDataContainer"
Cohesion: 0.12
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.12
Nodes (12): Container, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int (+4 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (18): Client, TwitchClientRuntimeData, OnChatCommandReceivedArgs, Client, Container, ContainerBuilder, IEnumerator, LogType (+10 more)

### Community 54 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

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
Cohesion: 0.10
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "TechTreeNode"
Cohesion: 0.12
Nodes (10): Color, Foldout, List, Sprite, Vector2, VisualElement, TechTreeNode, Group (+2 more)

### Community 61 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.15
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (5): Action, int, Objective, ObjectiveType, ObjectiveData

### Community 69 - "StationSensor"
Cohesion: 0.15
Nodes (3): SensorBase, UnityEvent, StationSensor

### Community 70 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "STSM_StateAction"
Cohesion: 0.13
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "SelectedPlayerGroup"
Cohesion: 0.21
Nodes (3): List, List, SelectedPlayerGroup

### Community 77 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 78 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GamestateJukebox"
Cohesion: 0.07
Nodes (26): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox (+18 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 83 - "SimpleRotateOnAxis"
Cohesion: 0.10
Nodes (9): PersistentScoped, Slider, TextMeshProUGUI, UI_Objective, GameObject, SimpleRandomModelEnabled, float, Vector3 (+1 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Target"
Cohesion: 0.09
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

### Community 86 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.10
Nodes (11): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+3 more)

### Community 91 - "GridPos"
Cohesion: 0.14
Nodes (20): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError, OpenNode (+12 more)

### Community 92 - "SavingAndLoading.Structs"
Cohesion: 0.05
Nodes (33): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+25 more)

### Community 93 - "FoliageGenerationSettings"
Cohesion: 0.10
Nodes (17): Material, materials, Mesh, meshes, int, List, string, FoliageGroupSaveData (+9 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.23
Nodes (18): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+10 more)

### Community 95 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (82): ArchetypesById, ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, ContentCatalog, ContentError, RoleDef (+74 more)

### Community 97 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 98 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.13
Nodes (34): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationMotionDef, AnimationParameterDef, AnimationParameterKind, AnimationQuatKeyframe (+26 more)

### Community 100 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "stream_town_game/src/lib.rs"
Cohesion: 0.09
Nodes (39): AmbientLight, App, converted_animation_spec(), converted_asset_exists(), ConvertedAnimationApplied, ConvertedAnimationSpec, debug_weather_override(), embedded_content() (+31 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.09
Nodes (13): DontDestroyOnLoad, ContainerBuilder, LoadType, MetaData, bool, string, MainMenuRuntimeData, Button (+5 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "Player"
Cohesion: 0.11
Nodes (6): Player, Dictionary, GameObject, Vector3, GameMasterCommands, RulerCommands

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 108 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "ResourceTarget"
Cohesion: 0.26
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider (+2 more)

### Community 115 - "PlayerSaveData"
Cohesion: 0.07
Nodes (23): Component, Dictionary, Mesh, Transform, Vector3, SaveDataMapper, bool, int (+15 more)

### Community 116 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 117 - "ProcessorStartupReport"
Cohesion: 0.67
Nodes (3): Exception, ProcessorStartupReport, ProcessorStartupStage

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "TargetProcessor"
Cohesion: 0.20
Nodes (6): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor

### Community 120 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 121 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 125 - ".Log"
Cohesion: 0.12
Nodes (7): Action, Container, ContainerBuilder, GameStateProcessor, LoadSceneMode, Scene, ResourceData[]&gt;

### Community 126 - "ResourceStorageModifier"
Cohesion: 0.13
Nodes (7): BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, UnityEvent, StorageStatus

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "DebugProcessor"
Cohesion: 0.11
Nodes (11): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+3 more)

### Community 132 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "Sensors"
Cohesion: 0.07
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 136 - "ToolState"
Cohesion: 0.05
Nodes (79): TwitchConfig, CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse, Arc, Client (+71 more)

### Community 137 - "generate_and_spawn_world"
Cohesion: 0.09
Nodes (45): AnimationGraph, Assets, AssetServer, ArchetypeDef, ArchetypeKind, ArchetypeScene, cell_hash(), changing_seed_changes_world_hash() (+37 more)

### Community 138 - "PlayerRoleData"
Cohesion: 0.11
Nodes (9): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+1 more)

### Community 139 - "drive_converted_animations"
Cohesion: 0.27
Nodes (13): AnimationNodeIndex, AnimationPlayer, ActorAnimationDriver, animation_nodes_for_selection(), apply_animation_blend(), ConvertedAnimationDriver, current_normalized_time(), drive_converted_animations() (+5 more)

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "BuildingSettings"
Cohesion: 0.11
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 147 - ".GetMissingProcessorDependencies"
Cohesion: 0.27
Nodes (3): Container, IEnumerable, Type

### Community 148 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 149 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

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

### Community 154 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 155 - "UserInterface_Roles"
Cohesion: 0.20
Nodes (6): bool, Dictionary, GameObject, Transform, UserInterface_Roles, Color32

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "TwitchConnection"
Cohesion: 0.25
Nodes (11): broadcaster_gate_precedes_twitch_command_dispatch(), generate_connect_code(), handle_twitch_event(), InjectedCommands, PendingChatCommand, poll_twitch_transport(), Default, String (+3 more)

### Community 159 - ".new"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "AudioSourcesProcessor"
Cohesion: 0.31
Nodes (3): Container, ContainerBuilder, AudioSourcesProcessor

### Community 163 - "SensorProcessor"
Cohesion: 0.31
Nodes (3): Container, ContainerBuilder, SensorProcessor

### Community 164 - "command.rs"
Cohesion: 0.26
Nodes (11): ChatCommand, CommandParseError, no_argument(), Err, FromStr, Option, Result, Self (+3 more)

### Community 165 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 166 - "EditorUtils"
Cohesion: 0.15
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 167 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 169 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 170 - "VfxAnimationController"
Cohesion: 0.25
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - "LabelDisplayProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "select_grid_cell"
Cohesion: 0.20
Nodes (10): Camera, select_grid_cell(), SelectedCell, SelectionMarker, GlobalTransform, MouseButton, PrimaryWindow, SpatialQuery (+2 more)

### Community 177 - "GameConfig"
Cohesion: 0.12
Nodes (21): AnyResult, ConfigError, default_configuration_is_valid_and_round_trips_ron(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, Default, Result (+13 more)

### Community 178 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 187 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 188 - "Station"
Cohesion: 0.08
Nodes (16): Station, Dictionary, float, int, List, Queue, Transform, Container (+8 more)

### Community 189 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 190 - "TechTree_SO"
Cohesion: 0.33
Nodes (3): NodeGroup_SO, List, TechTree_SO

### Community 191 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 193 - "Utils"
Cohesion: 0.05
Nodes (8): BuildCostModifier, RoleScriptablesEditor, Utils, World, Level, ScriptablesEditor, Buildings, GameResources

### Community 194 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

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

### Community 203 - "CommonEnums.cs"
Cohesion: 0.12
Nodes (15): TargetableData, Dictionary, List, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+7 more)

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 209 - "RoleHandler"
Cohesion: 0.11
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 213 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

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

### Community 221 - "IRuntimeDataScriptable"
Cohesion: 0.09
Nodes (20): Queue, AudioRuntimeData, CreditsRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+12 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "attach_converted_animations"
Cohesion: 0.27
Nodes (17): AnimationGraphHandle, apply_material_overrides(), attach_converted_animations(), attach_native_animations(), cleanup_state_entities(), cleanup_world(), collect_animation_targets(), find_named_descendant() (+9 more)

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

### Community 239 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 240 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 246 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 247 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 248 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 251 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 253 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 264 - "MonoBehaviour"
Cohesion: 0.02
Nodes (92): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, Camera (+84 more)

### Community 275 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 276 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 284 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

## Knowledge Gaps
- **223 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+218 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.998426795)
- `RenderAssets` (2× useful, score=1.998426795) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `DebugProcessor`, `PlayerCommands`, `MonoBehaviour`, `ObjectPoolingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `Resource`, `WorldSaveData`, `PlayerRole`, `Character`, `PlayerProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `GUIDProcessor`, `.AddEvent`, `TechTreeProcessor`, `TimeProcessor`, `IProcessor`, `ResourceProcessor`, `StreamTownSessionBridge`, `FoliageProcessor`, `SavingAndLoading.Structs`, `FoliageGenerationSettings`, `MainMenuManager`, `TownGoalProcessor`, `PlayerSaveData`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `DebugProcessor`, `TwitchChatProcessor`, `MonoBehaviour`, `ObjectPoolingProcessor`, `TerrainGenSettings`, `.GenerateFromSettings`, `PlayerProcessor`, `GUIDProcessor`, `CellSpacePartitioning`, `UserInterface_Debug`, `SaveProcessor`, `IProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `ProjectCamera`, `WorldGenRuntimeData`, `AIPath`, `RaidEvent`, `GridProcessor`, `Target`, `FoliageProcessor`, `Player`, `DayAndNightProcessor`, `PlayerSaveData`, `.Log`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `ScriptablesProcessorInfrastructure`, `GraphicsProcessor`, `Access_Slider`, `SettingsData`, `SaveProcessor`, `MainMenuManager`, `Access_GOList`, `MonoBehaviour`, `Access_Dropdown`, `IProcessor`, `Access_TextInput`, `Access_Text`, `ProjectCamera`, `Access_Toggle`, `Autosave`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _223 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07950310559006211 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06578947368421052 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.029519331243469175 - nodes in this community are weakly interconnected._