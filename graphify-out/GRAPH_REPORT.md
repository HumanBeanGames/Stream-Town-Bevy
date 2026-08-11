# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 602 files · ~1,532,141 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6152 nodes · 14454 edges · 261 communities (236 shown, 25 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 982 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b8491e70`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ScriptablesProcessorInfrastructure
- BuildingProcessor
- ScriptableObject
- Coordinator.cs
- stream_town_migrate/src/presentation.rs
- twitch.rs
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- ObjectPoolingProcessor
- SettingsProcessor
- WorldGenProcessor
- Processors
- Resource
- TechTreeIOUtility
- FoliageProcessor
- RoleProcessor
- Character
- .CreateEnumField
- GameEvent
- .GenerateFromSettings
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- World.Generation.Settings
- select_grid_cell
- Player
- SaveFileData
- GameEventProcessor
- retargeted_animation_clip
- RoleDataSettings
- CellSpacePartitioning
- GlobalAudioController
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
- SeasonProcessor
- PoolableObject
- BinaryWriter
- VfxSeagullSpawner
- AudioHandler
- RaidEvent
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
- Utils
- MeshData
- models.rs
- Tiler
- ScriptablesEditor
- STSM_StateAction
- UserInterface_ObjectSelection
- ContentCatalog
- STSM_Idle
- GridProcessor
- TwitchBotSetupWindow
- GamestateJukebox
- WorldUtils
- Pet
- BinarySaveCodec
- Access_Text
- Target
- UIProcessor
- UserInterface_TownVote
- Access_Toggle
- FoliageData
- AnimationHandler
- GridPos
- TransformSaveData
- WorldGenSaveData
- convert_fbx_to_glb.py
- .Draw
- stream_town_migrate/src/content.rs
- AllSeasonSettings
- .StartupSequence
- stream_town_domain/src/presentation.rs
- Editor
- .default
- MainMenuManager
- TownGoalProcessor
- GameMasterCommands
- UnitHealthBar
- LoadingManager
- UIElementWrapper
- .Log
- CustomLogHandler
- LevelHandler
- stream_town_game/src/lib.rs
- EnemySpawner
- SelectedBuilding
- UserInterface_RulerVote
- PlayerSaveData
- Goal
- IProcessor.cs
- VoteEvent
- SeasonDataSettings
- DayAndNightProcessor
- SelectedObject
- .SetTargetType
- SnapToGridMouseMovement
- UserInterface_GameMenu
- GameStateProcessor
- RoleSlot
- CommandDictionary
- UpdateGraphBounds
- UserInterface_Resources
- stream_town_migrate/src/main.rs
- DebugSettings
- DebugProcessor
- WeatherProcessor
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
- BuildingDataSettings
- ChanceObjectList
- BuildPlacerData
- NodeUnlockData
- GateController
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- WindController
- ErrorData
- RoleHandler
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- TwitchTransport
- .new
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- SimpleMusicController
- Audio
- command.rs
- SimpleCancelBuildingPlacer
- EditorUtils
- MiscCommands
- BSPCell
- STSM_HelperBase
- ScriptableObjectAssetData
- TimeProcessor
- .RenderFoliageType
- UserInterface_BuildingHealthBar
- LabelDisplayProcessor
- EditorHelpers
- ObjectPoolingRuntimeData
- GameConfig
- .StartMusic
- SimpleRotateOnAxis
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- WorldGenRuntimeData
- SelectedEnemy
- FoliageRuntimeData
- UI_TechOption
- Station
- UserInterface_Event
- TechTree_SO
- TradeProcessor
- KeepKingVote
- .DrawDataFieldAndLabel
- PersistentScoped
- IProcessor
- IRuntimeDataScriptable
- UILineRenderer
- UserInterface_DisplayUsernames
- UnitTravelToPosition
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- CommonEnums.cs
- ParallelProgressReporter
- Key Rules
- ChangeTimeStamp
- Common Patterns
- SimpleDisableAfterTime
- SelectedPlayer
- UI_Objective
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
- SelectableObject
- Processor Template
- Common Patterns
- attach_converted_animations
- .RefreshSceneBindingsAndTryGenerate
- graphify reference: query, path, explain
- TODO List
- GeneratedWorld
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
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- AGENTS.md
- ResourceDataSettings
- Autosave
- CustomLogger
- PlayerSpawnPoint
- extraction-spec.md
- GenerationSettings
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- SensorProcessor
- VfxAnimationController
- MonoBehaviour
- VfxParticlePosition
- IntWrapper
- Coordinator
- append_vec3_keys
- ObjectSelectionProcessor.Editor.cs

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

## Communities (261 total, 25 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (17): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller, Container, ContainerBuilder (+9 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.03
Nodes (65): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+57 more)

### Community 3 - "Coordinator.cs"
Cohesion: 0.10
Nodes (7): InputButton, UserInterface.MainMenu, World, MetaData, GameEventSystem, Settings, PlayerControls

### Community 4 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.09
Nodes (73): animation_state_id(), array_index(), assign_clip_rigs_and_reference_poses(), clip_id(), collect_prefab_dependencies(), color_value(), controller_id(), convert() (+65 more)

### Community 5 - "twitch.rs"
Cohesion: 0.12
Nodes (26): CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse, Client, Formatter, Into (+18 more)

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
Cohesion: 0.07
Nodes (20): HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder (+12 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.08
Nodes (14): bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int, IReadOnlyList (+6 more)

### Community 12 - "Processors"
Cohesion: 0.07
Nodes (13): InputButton, SharedTypes, DataStructures, TownGoal.Data, Processors, StreamTown.EditorTools, TownGoal, UserInterface (+5 more)

### Community 13 - "Resource"
Cohesion: 0.04
Nodes (34): DepositResources, ResourceStorageModifier, float, int, PlayerInventory, Dictionary, ResourceInventory, bool (+26 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "FoliageProcessor"
Cohesion: 0.14
Nodes (6): Bounds, Container, ContainerBuilder, HashSet, List, FoliageProcessor

### Community 16 - "RoleProcessor"
Cohesion: 0.08
Nodes (9): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, List (+1 more)

### Community 17 - "Character"
Cohesion: 0.05
Nodes (21): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+13 more)

### Community 18 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 19 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+9 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (18): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+10 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.06
Nodes (15): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+7 more)

### Community 24 - "HealthHandler"
Cohesion: 0.06
Nodes (20): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy, Action (+12 more)

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "select_grid_cell"
Cohesion: 0.13
Nodes (29): AppExit, camera_controls(), capture_screenshot(), credits_input(), finish_boot(), game_input(), GameState, main_menu_input() (+21 more)

### Community 27 - "Player"
Cohesion: 0.09
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, TwitchClientProcessor, PlayerCommands (+1 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 30 - "retargeted_animation_clip"
Cohesion: 0.24
Nodes (15): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), retargeted_animation_clip() (+7 more)

### Community 32 - "RoleDataSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.17
Nodes (10): Bounds, CellPartitioningEditor, Dictionary, float, int, List, Resource, Vector2 (+2 more)

### Community 34 - "GlobalAudioController"
Cohesion: 0.22
Nodes (6): GlobalAudioController, AudioSource, bool, float, IEnumerator, Season

### Community 35 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "BuildingBase"
Cohesion: 0.11
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.08
Nodes (22): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+14 more)

### Community 39 - "EnemyModelHandler"
Cohesion: 0.20
Nodes (5): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, EventType, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "Targetable"
Cohesion: 0.04
Nodes (33): ProjectileShooter, float, int, string, List, Vector3, TargetSettings, ContainerBuilder (+25 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.14
Nodes (18): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+10 more)

### Community 45 - "legacy.rs"
Cohesion: 0.06
Nodes (86): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+78 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "PoolableObject"
Cohesion: 0.06
Nodes (28): CollectResource, Container, ContainerBuilder, GUIDProcessor, AnimationCurve, bool, int, object (+20 more)

### Community 48 - "BinaryWriter"
Cohesion: 0.17
Nodes (3): Action, CancellationToken, BinaryWriter

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "RaidEvent"
Cohesion: 0.19
Nodes (6): bool, IEnumerator, int, List, string, RaidEvent

### Community 52 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (18): Client, TwitchClientRuntimeData, OnChatCommandReceivedArgs, Client, Container, ContainerBuilder, IEnumerator, LogType (+10 more)

### Community 54 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

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
Cohesion: 0.11
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

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
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "Utils"
Cohesion: 0.07
Nodes (5): BuildCostModifier, Utils, Level, Buildings, GameResources

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

### Community 74 - "STSM_StateAction"
Cohesion: 0.10
Nodes (10): int, STSM_Action_Attack, bool, float, Vector3, STSM_Action_EnemyAttack, bool, float (+2 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "ContentCatalog"
Cohesion: 0.15
Nodes (21): App, ArchetypeDef, ArchetypeKind, ArchetypeScene, ContentCatalog, archetype_by_source(), converted_animation_spec(), ConvertedAnimationSpec (+13 more)

### Community 77 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 78 - "GridProcessor"
Cohesion: 0.10
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

### Community 83 - "BinarySaveCodec"
Cohesion: 0.14
Nodes (8): Func, int, List, UTF8Encoding, BinarySaveCodec, int, PlayerRoleSaveData, BinaryReader

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Target"
Cohesion: 0.07
Nodes (12): GridProcessorEditor, GridSystem.Utils, PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat (+4 more)

### Community 86 - "UIProcessor"
Cohesion: 0.16
Nodes (3): Container, ContainerBuilder, UIProcessor

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 89 - "FoliageData"
Cohesion: 0.24
Nodes (11): Dictionary, Material, Matrix4x4, Mesh, Material, Mesh, Quaternion, Vector3 (+3 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.08
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+7 more)

### Community 91 - "GridPos"
Cohesion: 0.14
Nodes (20): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError, OpenNode (+12 more)

### Community 92 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 93 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.23
Nodes (18): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+10 more)

### Community 95 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (81): ArchetypesById, ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, ContentError, RoleDef, BTreeMap (+73 more)

### Community 97 - "AllSeasonSettings"
Cohesion: 0.29
Nodes (6): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings

### Community 98 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.13
Nodes (34): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationMotionDef, AnimationParameterDef, AnimationParameterKind, AnimationQuatKeyframe (+26 more)

### Community 100 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - ".default"
Cohesion: 0.19
Nodes (14): broadcaster_gate_precedes_twitch_command_dispatch(), embedded_config_supports_vertical_slice_scale(), generate_connect_code(), handle_twitch_event(), InjectedCommands, PendingChatCommand, poll_twitch_transport(), Default (+6 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "GameMasterCommands"
Cohesion: 0.11
Nodes (5): List, GameSettings, GameMasterCommands, RulerCommands, Vector3

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - ".Log"
Cohesion: 0.14
Nodes (4): Action, LoadSceneMode, Scene, ResourceData[]&gt;

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "stream_town_game/src/lib.rs"
Cohesion: 0.20
Nodes (27): ActorAnimationDriver, Agent, AgentAnimation, drive_native_animations(), grid_to_world(), GridLocation, headless_vertical_slice_spawns_three_hundred_agents(), Hud (+19 more)

### Community 112 - "EnemySpawner"
Cohesion: 0.15
Nodes (6): Transform, float, int, List, Transform, EnemySpawner

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "PlayerSaveData"
Cohesion: 0.06
Nodes (27): Component, Dictionary, Mesh, Transform, SaveDataMapper, bool, int, List (+19 more)

### Community 116 - "Goal"
Cohesion: 0.15
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 117 - "IProcessor.cs"
Cohesion: 0.15
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 120 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 121 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 125 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 126 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "UserInterface_Resources"
Cohesion: 0.21
Nodes (7): Slider, TextMeshProUGUI, Color, GameObject, Slider, TextMeshProUGUI, UserInterface_Resources

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "DebugSettings"
Cohesion: 0.22
Nodes (6): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, DebugLogCategory, SerializedScriptableObject

### Community 132 - "DebugProcessor"
Cohesion: 0.06
Nodes (15): Container, ContainerBuilder, DebugProcessor, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal (+7 more)

### Community 133 - "WeatherProcessor"
Cohesion: 0.23
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "Sensors"
Cohesion: 0.06
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (44): apply_technology_draft(), bounded_ui_index(), content_tab(), draw_world_preview(), inspector_tab(), main(), migration_tab(), poll_twitch_tool_events() (+36 more)

### Community 137 - "generate_and_spawn_world"
Cohesion: 0.19
Nodes (24): AnimationGraph, Assets, AssetServer, actor_material(), animate_agents(), animation_root_name(), apply_material_overrides(), converted_asset_exists() (+16 more)

### Community 138 - "PlayerRoleData"
Cohesion: 0.10
Nodes (9): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+1 more)

### Community 139 - "drive_converted_animations"
Cohesion: 0.38
Nodes (10): AnimationNodeIndex, AnimationPlayer, animation_nodes_for_selection(), apply_animation_blend(), ConvertedAnimationDriver, current_normalized_time(), drive_converted_animations(), BTreeMap (+2 more)

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
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "BuildingDataSettings"
Cohesion: 0.11
Nodes (14): ContainerBuilder, AllBuildingDataSettingsInstaller, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingDataContainerInstaller, int, ResourceCostData (+6 more)

### Community 147 - "ChanceObjectList"
Cohesion: 0.27
Nodes (5): float, ChanceObject, float, List, ChanceObjectList

### Community 148 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

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

### Community 155 - "RoleHandler"
Cohesion: 0.07
Nodes (18): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+10 more)

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "TwitchTransport"
Cohesion: 0.22
Nodes (8): Arc, Mutex, Receiver, TwitchEvent, TwitchStatus, TwitchTransport, Drop, UnboundedSender

### Community 159 - ".new"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 164 - "command.rs"
Cohesion: 0.26
Nodes (11): ChatCommand, CommandParseError, no_argument(), Err, FromStr, Option, Result, Self (+3 more)

### Community 166 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 167 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 168 - "BSPCell"
Cohesion: 0.28
Nodes (4): bool, List, Vector2, BSPCell

### Community 169 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 170 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - ".RenderFoliageType"
Cohesion: 0.32
Nodes (6): Dictionary, int, Material, Matrix4x4, Mesh, FoliageRenderer

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 177 - "GameConfig"
Cohesion: 0.15
Nodes (16): AnyResult, ConfigError, default_configuration_is_valid_and_round_trips_ron(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, Default, Result (+8 more)

### Community 178 - ".StartMusic"
Cohesion: 0.56
Nodes (3): SeasonAudioData, AudioClip, List

### Community 179 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

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

### Community 186 - "FoliageRuntimeData"
Cohesion: 0.33
Nodes (6): Dictionary, List, Material, Matrix4x4, Mesh, FoliageRuntimeData

### Community 187 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 188 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 189 - "UserInterface_Event"
Cohesion: 0.11
Nodes (12): Animator, GameObject, IEnumerator, int, FishGodEvent, OnMessageReceivedArgs, EventCommands, GameObject (+4 more)

### Community 190 - "TechTree_SO"
Cohesion: 0.33
Nodes (3): NodeGroup_SO, List, TechTree_SO

### Community 191 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 193 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 195 - "IProcessor"
Cohesion: 0.23
Nodes (4): CancellationToken, Task, Container, IProcessor

### Community 196 - "IRuntimeDataScriptable"
Cohesion: 0.13
Nodes (14): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+6 more)

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
Cohesion: 0.09
Nodes (14): BuildingResourceModelHandler, GameObject, UnityEvent, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+6 more)

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "ChangeTimeStamp"
Cohesion: 0.50
Nodes (3): int, ChangeTimeStamp, DateTime

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "SimpleDisableAfterTime"
Cohesion: 0.15
Nodes (6): Image, TextMeshProUGUI, UIRoleDisplay, float, GameObject, SimpleDisableAfterTime

### Community 210 - "UI_Objective"
Cohesion: 0.50
Nodes (3): Slider, TextMeshProUGUI, UI_Objective

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

### Community 221 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "attach_converted_animations"
Cohesion: 0.19
Nodes (20): AnimationGraphHandle, attach_converted_animations(), attach_native_animations(), cleanup_state_entities(), cleanup_world(), collect_animation_targets(), ConvertedAnimationApplied, find_named_descendant() (+12 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "GeneratedWorld"
Cohesion: 0.33
Nodes (10): cell_hash(), changing_seed_changes_world_hash(), generate_world(), GeneratedResource, GeneratedWorld, generation_is_deterministic(), hash_world(), String (+2 more)

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

### Community 248 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 254 - "GenerationSettings"
Cohesion: 0.05
Nodes (33): AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings, Action, IEnumerator (+25 more)

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 261 - "SensorProcessor"
Cohesion: 0.12
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 263 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 264 - "MonoBehaviour"
Cohesion: 0.02
Nodes (92): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder (+84 more)

### Community 275 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 276 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 283 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 284 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

## Knowledge Gaps
- **220 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+215 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.998426795)
- `RenderAssets` (2× useful, score=1.998426795) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `DebugProcessor`, `MonoBehaviour`, `ObjectPoolingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `Resource`, `FoliageProcessor`, `WorldSaveData`, `Character`, `RoleProcessor`, `PlayerProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `.PrepareRuntimeForLoad`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `PoolableObject`, `ResourceProcessor`, `StreamTownSessionBridge`, `IProcessor`, `FoliageData`, `MainMenuManager`, `TownGoalProcessor`, `GameMasterCommands`, `PlayerSaveData`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `DebugProcessor`, `TwitchChatProcessor`, `MonoBehaviour`, `ObjectPoolingProcessor`, `FoliageProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `CellSpacePartitioning`, `UserInterface_Debug`, `SaveProcessor`, `PoolableObject`, `RaidEvent`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `WorldGenRuntimeData`, `AIPath`, `IProcessor`, `GridProcessor`, `Target`, `.StartupSequence`, `GameMasterCommands`, `.Log`, `EnemySpawner`, `PlayerSaveData`, `GameStateProcessor`, `GenerationSettings`?**
  _High betweenness centrality (0.052) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `ScriptablesProcessorInfrastructure`, `GraphicsProcessor`, `Access_Slider`, `IProcessor`, `SettingsData`, `SaveProcessor`, `MainMenuManager`, `MonoBehaviour`, `UIElementWrapper`, `Access_Text`, `Access_Dropdown`, `Access_Toggle`, `Autosave`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _220 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07135135135135136 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05632360471070148 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.031148986188657066 - nodes in this community are weakly interconnected._