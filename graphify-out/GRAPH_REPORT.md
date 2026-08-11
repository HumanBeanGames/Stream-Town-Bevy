# Graph Report - Stream-Town-Bevy  (2026-08-12)

## Corpus Check
- 603 files · ~1,535,372 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6197 nodes · 14648 edges · 272 communities (248 shown, 24 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 984 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6f8e9bb0`
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
- TechTreeProcessor.cs
- Resource
- TechTreeIOUtility
- TerrainGenSettings
- RoleProcessor
- Character
- .CreateEnumField
- GameEvent
- .GenerateFromSettings
- Age
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- World.Generation.Settings
- select_grid_cell
- MiscCommands
- SaveFileData
- GameEventProcessor
- retargeted_animation_clip
- BinaryWriter
- CellSpacePartitioning
- ContentCatalog
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
- GUIDComponent
- BinarySaveCodec
- VfxSeagullSpawner
- AudioHandler
- RoleDataSettings
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
- SensorProcessor
- MeshData
- models.rs
- Tiler
- ScriptablesEditor
- String
- UserInterface_ObjectSelection
- .SerializeComponent
- STSM_Idle_Player
- GridNode
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- Pet
- MonoBehaviour
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
- UIElementWrapper
- DayAndNightProcessor
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedBuilding
- UserInterface_RulerVote
- PlayerSaveData
- Goal
- IProcessor.cs
- VoteEvent
- PlayerInventory
- ResourceGenerationSettings
- SelectedObject
- .SetTargetType
- SnapToGridMouseMovement
- UserInterface_GameMenu
- GameStateProcessor
- AllSeasonSettings
- CommandDictionary
- UpdateGraphBounds
- ResourceHolder
- stream_town_migrate/src/main.rs
- DebugProcessor
- STSM_Action_GatherResource
- BottomBarButton
- ConfirmCheck
- STStateMachine.States
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
- UserInterface_Resources
- BuildPlacerData
- CampGenerationSettings
- GateController
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- WindController
- DontDestroyOnLoad
- RoleHandler
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- WeatherProcessor
- TechNodeData
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- .ShowContext
- SimpleMusicController
- command.rs
- SimpleCancelBuildingPlacer
- EditorUtils
- BuildingDamageMaterialHandler
- ParallelProgressReporter
- BuildingSettings
- TechnologyTreeGroup
- TimeProcessor
- SelectedResource
- UserInterface_BuildingHealthBar
- LabelDisplayProcessor
- EditorHelpers
- DayAndNightSettings
- GameConfig
- RoleSlotModifier
- ResourceDataSettings
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- WorldGenRuntimeData
- FoliageGroupSaveData
- SelectedEnemy
- .RestoreObjectiveProgress
- GameEventSettings
- Station
- RaidEvent
- ObjectPoolingSettings
- TradeSettings
- ObjectPoolingRuntimeData
- Utils
- AllBuildingDataSettings
- IProcessor
- WorldGenDebugSettings
- UILineRenderer
- UserInterface_DisplayUsernames
- GridSettings
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- CommonEnums.cs
- BuildingConfigSettings
- Key Rules
- GameEventConfigSettingsInstaller
- Common Patterns
- SimpleDisableAfterTime
- SelectedPlayer
- TimeSettings
- FPSDisplay.cs
- WorldGenBehaviorSettings
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
- WorldGenLayerSettings
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
- WorldGenScaleSettings
- AGENTS.md
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- Autosave
- CustomLogger
- ForwardRendererInstaller
- extraction-spec.md
- RenderPipelineInstaller
- SensorSettings
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- VideoSettingsPresetsInstaller
- TechTreeNodeType.cs
- PoolablePlayer.cs
- UISettings
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- TargetSensor
- .StartGoalFromNode
- ScriptableObjectAssetData
- IInstaller
- ChannelDataInstaller
- FloatWrapper
- ContextItem
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
8. `StableId` - 97 edges
9. `SaveProcessor` - 88 edges
10. `BuildingProcessor` - 78 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `town_resource_amount()` --references--> `WorldSimulation`  [EXTRACTED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/simulation.rs
- `generate_and_spawn_world()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (272 total, 24 thin omitted)

### Community 0 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (5): int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "ScriptableObject"
Cohesion: 0.04
Nodes (43): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, GameSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, List, FoliageGenSettings (+35 more)

### Community 3 - "Processors"
Cohesion: 0.06
Nodes (10): ObjectSelectionProcessor, InputButton, Processors, StreamTown.EditorTools, Processors.Editor, MetaData, UserInterface, Audio (+2 more)

### Community 4 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.12
Nodes (46): animation_state_id(), array_index(), assign_clip_rigs_and_reference_poses(), clip_id(), collect_prefab_dependencies(), color_value(), controller_id(), convert() (+38 more)

### Community 5 - "PlayerCommands"
Cohesion: 0.12
Nodes (5): OnMessageReceivedArgs, EventCommands, OnChatCommandReceivedArgs, TwitchClientProcessor, PlayerCommands

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.08
Nodes (17): Vector2, Group, int, List, Port, Vector2, TechTreeGraphView, List (+9 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.09
Nodes (12): Action, Button, Dictionary, GameObject, int, List, RectTransform, UnityAction (+4 more)

### Community 9 - "ObjectPoolingProcessor"
Cohesion: 0.06
Nodes (24): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+16 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (19): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+11 more)

### Community 12 - "TechTreeProcessor.cs"
Cohesion: 0.08
Nodes (11): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TownGoal, TechTree.Data (+3 more)

### Community 13 - "Resource"
Cohesion: 0.04
Nodes (36): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer (+28 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+11 more)

### Community 15 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 16 - "RoleProcessor"
Cohesion: 0.09
Nodes (7): Container, ContainerBuilder, int, List, RoleProcessor, List, SelectedPlayerGroup

### Community 17 - "Character"
Cohesion: 0.06
Nodes (20): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+12 more)

### Community 18 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 19 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.09
Nodes (25): HashSet, Func, HashSet, List, Vector2, Vector3, GenerateDebugPositions(), GetPositiveNoiseOffset() (+17 more)

### Community 21 - "Age"
Cohesion: 0.06
Nodes (24): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+16 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.09
Nodes (14): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+6 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.09
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.09
Nodes (12): Func, Action, float, Enemy, Action, bool, float, int (+4 more)

### Community 25 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 26 - "select_grid_cell"
Cohesion: 0.13
Nodes (29): AppExit, animate_weather_particles(), camera_controls(), capture_screenshot(), credits_input(), finish_boot(), game_input(), GameState (+21 more)

### Community 27 - "MiscCommands"
Cohesion: 0.16
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 28 - "SaveFileData"
Cohesion: 0.11
Nodes (20): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+12 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.08
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "retargeted_animation_clip"
Cohesion: 0.24
Nodes (15): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), retargeted_animation_clip() (+7 more)

### Community 32 - "BinaryWriter"
Cohesion: 0.18
Nodes (3): Action, List, BinaryWriter

### Community 33 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 34 - "ContentCatalog"
Cohesion: 0.18
Nodes (23): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, ContentCatalog (+15 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "BuildingBase"
Cohesion: 0.10
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "SaveProcessor"
Cohesion: 0.06
Nodes (22): Container, ContainerBuilder, GUIDProcessor, Action, CancellationToken, Component, Container, ContainerBuilder (+14 more)

### Community 39 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.11
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "Targetable"
Cohesion: 0.06
Nodes (23): List, Vector3, TargetSettings, Container, ContainerBuilder, List, TargetProcessor, Dictionary (+15 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.14
Nodes (19): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+11 more)

### Community 45 - "legacy.rs"
Cohesion: 0.06
Nodes (84): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+76 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.10
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 48 - "BinarySaveCodec"
Cohesion: 0.18
Nodes (6): CancellationToken, Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 49 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "RoleDataSettings"
Cohesion: 0.08
Nodes (18): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip (+10 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.12
Nodes (22): bool, GameObject, HashSet, int, List, long, MenuItem, string (+14 more)

### Community 56 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 57 - "StableId"
Cohesion: 0.10
Nodes (29): round_trips_through_serde(), Err, Formatter, FromStr, Into, Result, Self, String (+21 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "TechTreeNode"
Cohesion: 0.12
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 61 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 62 - "CameraController"
Cohesion: 0.11
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (15): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+7 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 68 - "Objective"
Cohesion: 0.16
Nodes (3): Action, int, Objective

### Community 69 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

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

### Community 74 - "String"
Cohesion: 0.19
Nodes (27): inline_file_id(), parse_blend_tree(), parse_conditions(), parse_controller(), parse_parameters(), parse_reference_list(), parse_state_motions(), parse_yaml_documents() (+19 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - ".SerializeComponent"
Cohesion: 0.11
Nodes (13): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+5 more)

### Community 77 - "STSM_Idle_Player"
Cohesion: 0.10
Nodes (10): bool, float, Vector3, STSM_Idle_Enemy, bool, float, uint, Vector3 (+2 more)

### Community 78 - "GridNode"
Cohesion: 0.14
Nodes (10): int, List, Vector2, GridNode, Color, CollisionColours, CollisionType, float (+2 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 83 - "MonoBehaviour"
Cohesion: 0.04
Nodes (25): CameraProcessor, PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+17 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Target"
Cohesion: 0.09
Nodes (12): UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Sensors, GridSystem.Partitioning, Combat (+4 more)

### Community 86 - "UIProcessor"
Cohesion: 0.14
Nodes (3): Container, ContainerBuilder, UIProcessor

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.08
Nodes (18): Dictionary, TechVote, Button, GameObject, Image, Slider, TextMeshProUGUI, Transform (+10 more)

### Community 88 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "AnimationHandler"
Cohesion: 0.06
Nodes (19): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+11 more)

### Community 91 - "GridPos"
Cohesion: 0.08
Nodes (42): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError, OpenNode (+34 more)

### Community 92 - "SavingAndLoading.Structs"
Cohesion: 0.05
Nodes (36): Mesh, Transform, Vector3, SaveDataMapper, int, List, string, uint (+28 more)

### Community 93 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.23
Nodes (18): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+10 more)

### Community 95 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.10
Nodes (66): ArchetypesById, archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_value(), building_placements(), BuildingPlacement (+58 more)

### Community 97 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.14
Nodes (33): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationMotionDef, AnimationParameterDef, AnimationQuatKeyframe, AnimationStateDef (+25 more)

### Community 100 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "stream_town_game/src/lib.rs"
Cohesion: 0.07
Nodes (48): AnyResult, ActorAnimationDriver, AgentAnimation, broadcaster_gate_precedes_twitch_command_dispatch(), CommandFeedback, drive_native_animations(), embedded_unity_content_catalog_is_valid(), environment_palette() (+40 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "Player"
Cohesion: 0.07
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+1 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

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

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 115 - "PlayerSaveData"
Cohesion: 0.09
Nodes (16): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int (+8 more)

### Community 116 - "Goal"
Cohesion: 0.11
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 117 - "IProcessor.cs"
Cohesion: 0.15
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 118 - "VoteEvent"
Cohesion: 0.11
Nodes (13): int, List, NewKingVote, PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent (+5 more)

### Community 119 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 120 - "ResourceGenerationSettings"
Cohesion: 0.12
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 121 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 122 - ".SetTargetType"
Cohesion: 0.15
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 125 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 126 - "AllSeasonSettings"
Cohesion: 0.13
Nodes (14): float, int, Material, AllSeasonSettings, Color, float, int, VisualEffect (+6 more)

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
Cohesion: 0.08
Nodes (12): Dictionary, DebugSettings, Container, ContainerBuilder, GridProcessor, Container, ContainerBuilder, DebugLogCategory (+4 more)

### Community 132 - "STSM_Action_GatherResource"
Cohesion: 0.11
Nodes (5): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 133 - "BottomBarButton"
Cohesion: 0.22
Nodes (4): bool, IEnumerator, Image, BottomBarButton

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "STStateMachine.States"
Cohesion: 0.07
Nodes (12): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+4 more)

### Community 136 - "ToolState"
Cohesion: 0.05
Nodes (78): CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse, Arc, Client, Formatter (+70 more)

### Community 137 - "generate_and_spawn_world"
Cohesion: 0.12
Nodes (50): AnimationGraph, App, Assets, AssetServer, actor_material(), Agent, animate_agents(), animation_root_name() (+42 more)

### Community 138 - "PlayerRoleData"
Cohesion: 0.11
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
Cohesion: 0.12
Nodes (17): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+9 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 147 - "UserInterface_Resources"
Cohesion: 0.21
Nodes (7): Slider, TextMeshProUGUI, Color, GameObject, Slider, TextMeshProUGUI, UserInterface_Resources

### Community 148 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 149 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

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

### Community 154 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 155 - "RoleHandler"
Cohesion: 0.07
Nodes (18): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+10 more)

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 159 - "TechNodeData"
Cohesion: 0.25
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 163 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 164 - "command.rs"
Cohesion: 0.26
Nodes (11): ChatCommand, CommandParseError, no_argument(), Err, FromStr, Option, Result, Self (+3 more)

### Community 166 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 167 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 169 - "BuildingSettings"
Cohesion: 0.20
Nodes (4): bool, Dictionary, int, BuildingSettings

### Community 170 - "TechnologyTreeGroup"
Cohesion: 0.29
Nodes (5): Color, float, string, TechnologyTreeGroup, Group

### Community 171 - "TimeProcessor"
Cohesion: 0.16
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 174 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 177 - "GameConfig"
Cohesion: 0.16
Nodes (18): ConfigError, default_configuration_is_valid_and_round_trips_ron(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, BTreeMap, Default, Result (+10 more)

### Community 179 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

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

### Community 184 - "FoliageGroupSaveData"
Cohesion: 0.32
Nodes (6): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData

### Community 186 - ".RestoreObjectiveProgress"
Cohesion: 0.29
Nodes (3): int, string, ObjectiveSaveData

### Community 187 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 188 - "Station"
Cohesion: 0.08
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 189 - "RaidEvent"
Cohesion: 0.08
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 190 - "ObjectPoolingSettings"
Cohesion: 0.25
Nodes (7): bool, List, ObjectPoolingSettings, GameObject, int, string, PooledObjectData

### Community 191 - "TradeSettings"
Cohesion: 0.33
Nodes (5): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller

### Community 192 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 193 - "Utils"
Cohesion: 0.04
Nodes (9): BuildCostModifier, RoleScriptablesEditor, Utils, World, Level, ScriptablesEditor, Buildings, GameResources (+1 more)

### Community 194 - "AllBuildingDataSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingDataContainerInstaller, AllBuildingDataSettings

### Community 195 - "IProcessor"
Cohesion: 0.23
Nodes (4): CancellationToken, Task, Container, IProcessor

### Community 196 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

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
Cohesion: 0.11
Nodes (13): GameObject, int, EnemyWeaponModel, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+5 more)

### Community 204 - "BuildingConfigSettings"
Cohesion: 0.33
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "GameEventConfigSettingsInstaller"
Cohesion: 0.33
Nodes (4): ContainerBuilder, GameEventConfigSettingsInstaller, bool, GameEventConfigSettings

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 210 - "TimeSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, TimeDataSettingsInstaller, int, TimeSettings

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "WorldGenBehaviorSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenBehaviorSettingsInstaller, bool, WorldGenBehaviorSettings

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

### Community 221 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (14): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+6 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "attach_converted_animations"
Cohesion: 0.15
Nodes (26): AmbientLight, AnimationGraphHandle, apply_material_overrides(), attach_converted_animations(), attach_native_animations(), cleanup_state_entities(), cleanup_world(), collect_animation_targets() (+18 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "WorldGenLayerSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenLayerSettingsInstaller, LayerMask, WorldGenLayerSettings

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
Cohesion: 0.28
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 240 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 244 - "WorldGenScaleSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenScaleSettingsInstaller, float, WorldGenScaleSettings

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

### Community 254 - "SensorSettings"
Cohesion: 0.33
Nodes (4): float, SensorSettings, ContainerBuilder, SensorSettingsInstaller

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 259 - "UISettings"
Cohesion: 0.33
Nodes (4): float, UISettings, ContainerBuilder, UISettingsInstaller

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 263 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 264 - "IInstaller"
Cohesion: 0.04
Nodes (29): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, SaveStateInstaller (+21 more)

### Community 268 - "ContextItem"
Cohesion: 0.67
Nodes (3): bool, Sprite, ContextItem

### Community 276 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 284 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

## Knowledge Gaps
- **223 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+218 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.998426795)
- `RenderAssets` (2× useful, score=1.998426795) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `DebugProcessor`, `IInstaller`, `ObjectPoolingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `Resource`, `RoleProcessor`, `PlayerProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `.PrepareRuntimeForLoad`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `ResourceProcessor`, `StreamTownSessionBridge`, `FoliageGroupSaveData`, `.RestoreObjectiveProgress`, `IProcessor`, `MonoBehaviour`, `Target`, `FoliageProcessor`, `SavingAndLoading.Structs`, `MainMenuManager`, `TownGoalProcessor`, `ResourceGenerationSettings`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `DebugProcessor`, `TwitchChatProcessor`, `IInstaller`, `ObjectPoolingProcessor`, `TerrainGenSettings`, `.GenerateFromSettings`, `CampGenerationSettings`, `PlayerProcessor`, `CellSpacePartitioning`, `UserInterface_Debug`, `SaveProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `WorldGenRuntimeData`, `AIPath`, `RaidEvent`, `IProcessor`, `WorldGenDebugSettings`, `MonoBehaviour`, `WorldGenBehaviorSettings`, `Target`, `FoliageProcessor`, `SavingAndLoading.Structs`, `WorldGenLayerSettings`, `Player`, `EnemySpawner`, `WorldGenScaleSettings`, `ResourceGenerationSettings`, `GameStateProcessor`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `ScriptablesProcessorInfrastructure`, `GraphicsProcessor`, `Access_Slider`, `IProcessor`, `SettingsData`, `SaveProcessor`, `MainMenuManager`, `IInstaller`, `UIElementWrapper`, `MonoBehaviour`, `Access_Text`, `Access_Dropdown`, `Access_Toggle`, `Autosave`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _223 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07496194824961948 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06766917293233082 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.03954802259887006 - nodes in this community are weakly interconnected._