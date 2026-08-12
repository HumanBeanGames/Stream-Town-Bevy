# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 607 files · ~1,593,856 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6833 nodes · 17760 edges · 277 communities (245 shown, 32 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 993 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `13ec3b86`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- GeneratedWorld
- BuildingProcessor
- stream_town_domain/src/content.rs
- stream_town_migrate/src/presentation.rs
- Target
- EnemyModelHandler
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- process_injected_commands
- SettingsProcessor
- WorldGenProcessor
- Player
- SelectedPlayerGroup
- TechTreeIOUtility
- World.Generation.Settings
- Character
- Result
- Processors
- PlayerCommands
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- RoleHandler
- Goal
- UIElementWrapper
- SaveFileData
- GameEventProcessor
- stream_town_game/src/lib.rs
- BuildingResourceModelHandler
- PlayerInventory
- .GetResourceTarget
- StationProcessor
- UserInterface_Debug
- MiscCommands
- SettingsData
- AnimationHandler
- ObjectPoolingProcessor
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BinarySaveCodec
- AnimationControllerDef
- legacy.rs
- SeasonProcessor
- STSM_Idle_Player
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- PlayerRole
- StableId
- STSM_GoToLocation
- TechTreeEditorWindow
- VfxSeagullSpawner
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- drive_converted_animations
- Objective
- WorldGenSaveData
- MeshData
- models.rs
- Tiler
- ScriptablesEditor
- BTreeMap
- UserInterface_ObjectSelection
- TechTreeNode
- Access_Dropdown
- GridProcessor
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- SaveDataMapper
- ResourceRuntimeData
- Access_Text
- update_environment_presentation
- CellSpacePartitioning
- UserInterface_TownVote
- Station
- FoliageProcessor
- generate_and_spawn_world
- RoleData
- GameEvent
- IRuntimeDataScriptable
- convert_fbx_to_glb.py
- KeepKingVote
- stream_town_migrate/src/content.rs
- SelectedPlayer
- Coordinator
- stream_town_domain/src/presentation.rs
- Editor
- SelectableObject
- run_transport
- TownGoalProcessor
- MainMenuManager
- UnitHealthBar
- LoadingManager
- STSM_Idle
- .SetGeneratedResources
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- EditorUtils
- STSM_StateAction
- DontDestroyOnLoad
- VoteEvent
- Resource
- DebugProcessor
- Res
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Targetable
- BuildingBase
- CommandDictionary
- UpdateGraphBounds
- IProcessor.cs
- stream_town_migrate/src/main.rs
- .GenerateFromSettings
- FoliageSaveData
- WeatherProcessor
- ConfirmCheck
- config.rs
- ToolState
- LabelDisplayProcessor
- SensorProcessor
- BuildingDamageMaterialHandler
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- GameStateProcessor
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- UI_TechOption
- Requirement
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- TwitchUser
- UserInterface
- .GetResourceAssets
- Stream Town Reloaded - Architecture Documentation
- WindController
- GridProcessor.cs
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- FoliageGenerationSettings.cs
- TechTree_SO
- Season
- .RenderResourceType
- BinaryReader
- EventProcessor
- TimeProcessor
- .RefreshSceneBindingsAndTryGenerate
- UserInterface_TownGoal
- SelectedBuilding
- EditorHelpers
- UserInterface.MainMenu
- IProcessor
- ResourceHolder
- ResourceStorageModifier
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- GUIDComponent
- WorldGenRuntimeData
- StringUtils
- UserInterface_GameMenu
- Autosave
- PassiveResourceIncrementer
- ErrorData
- CampGenerationSettings
- BuildingDataSettings
- command.rs
- RotationHandler
- Utils
- CommonEnums.cs
- SimpleDisableAfterTime
- setup_camera
- UILineRenderer
- UserInterface_DisplayUsernames
- NewKingVote
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- .RestoreObjectiveProgress
- Audio
- Key Rules
- SelectedEnemy
- Common Patterns
- World.Generation
- VfxParticlePosition
- .StartMusic
- FPSDisplay.cs
- BuildingSettings
- STSM_HelperBase
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- NodeUnlockData
- ObjectSelectionProcessor.Editor.cs
- ScriptKeywordProcessor
- .RestoreWorldState
- STSM_Idle_Enemy
- Processor Template
- Common Patterns
- TradeSettings
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- AllBuildingDataSettings
- Twitch setup
- graphify reference: add a URL and watch a folder
- PostProcessingInstaller
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- VfxAnimationController
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Easings
- CreateProjectScopeProcessors.cs
- PlacementProbeHandler
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- PlayerSpawnPoint
- AGENTS.md
- StatusBar
- ChannelDataInstaller
- SaveStateInstaller
- twitch.rs
- CustomLogger
- FloatWrapper
- extraction-spec.md
- .InjectRuntimeData
- RoleDataSettings
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- .RegisterSceneLoadHook
- TechTreeNodeType.cs
- PoolablePlayer.cs
- .RefreshSceneData
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- TargetSensor
- EquipmentHandlerEditor
- AudioMixerInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- UnitTravelToPosition
- ScriptablesProcessorInfrastructure
- IntWrapper
- MonoBehaviour
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- AutosaveIntervalsInstaller
- PlayerSaveData

## God Nodes (most connected - your core abstractions)
1. `StableId` - 249 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldSimulation` - 126 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `SaveProcessor` - 88 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_enemies_drive_damage_range_cadence_and_weighted_spawning()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `battering_ram_targets_and_damages_buildings_from_authored_mask()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `builder_completes_and_upgrades_authored_construction()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `combat_goal_damages_kills_and_respawns()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (277 total, 32 thin omitted)

### Community 0 - "GeneratedWorld"
Cohesion: 0.09
Nodes (40): AnyResult, WorldGenConfig, FoliageHabitat, FoliageLayerDef, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash() (+32 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (10): Container, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost, maxLevel (+2 more)

### Community 2 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (49): ArchetypeBounds, ArchetypeDef, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+41 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "Target"
Cohesion: 0.13
Nodes (7): Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, GUIDSystem

### Community 5 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "process_injected_commands"
Cohesion: 0.07
Nodes (47): active_event_text(), AgentCommand, AgentCommandQueue, broadcaster_gate_precedes_twitch_command_dispatch(), building_age(), building_definition_id(), building_model_node_names(), BuildingCommandQueue (+39 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (16): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+8 more)

### Community 12 - "Player"
Cohesion: 0.07
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+1 more)

### Community 13 - "SelectedPlayerGroup"
Cohesion: 0.21
Nodes (3): List, List, SelectedPlayerGroup

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 16 - "Character"
Cohesion: 0.09
Nodes (13): Pets.Enumerations, TownGoal, Character.Enumerations, Core, Pets, GameEventSystem, GameEventSystem.Events, SavingAndLoading (+5 more)

### Community 17 - "Result"
Cohesion: 0.18
Nodes (14): DeviceAuthorization, OAuthClient, Client, Formatter, Into, Result, String, Vec (+6 more)

### Community 18 - "Processors"
Cohesion: 0.06
Nodes (10): BuildCostModifier, InputButton, PlayerControls.ObjectSelection, Processors, World, Level, Buildings, Enemies (+2 more)

### Community 19 - "PlayerCommands"
Cohesion: 0.16
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 20 - ".CreateEnumField"
Cohesion: 0.13
Nodes (12): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+4 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.07
Nodes (22): int, ChangeTimeStamp, ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData (+14 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 24 - "HealthHandler"
Cohesion: 0.10
Nodes (10): Func, Action, float, Enemy, Action, bool, float, int (+2 more)

### Community 25 - "RoleHandler"
Cohesion: 0.06
Nodes (21): PlayerRoleData, AudioClip, bool, float, int, RoleHandler, bool, Dictionary (+13 more)

### Community 26 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 27 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.04
Nodes (145): GameConfig, Default, ArchetypeKind, ContentCatalog, StationDef, GridPos, ActorState, RoleProgress (+137 more)

### Community 31 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 32 - "PlayerInventory"
Cohesion: 0.14
Nodes (6): PlayerInventory, Dictionary, ResourceInventory, bool, int, Dictionary

### Community 33 - ".GetResourceTarget"
Cohesion: 0.18
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 34 - "StationProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.08
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+7 more)

### Community 39 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (38): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+30 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "BinarySaveCodec"
Cohesion: 0.19
Nodes (6): Action, CancellationToken, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "legacy.rs"
Cohesion: 0.05
Nodes (111): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+103 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 48 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.13
Nodes (12): bool, double, float, int, List, long, MenuItem, string (+4 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.17
Nodes (11): Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int, Matrix4x4 (+3 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "PlayerRole"
Cohesion: 0.07
Nodes (14): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+6 more)

### Community 57 - "StableId"
Cohesion: 0.05
Nodes (65): GameplayConfig, BTreeMap, ObjectiveDef, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState (+57 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.10
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
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.12
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "drive_converted_animations"
Cohesion: 0.06
Nodes (60): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, ActorAnimationDriver, add_animation_layer_branch(), add_rotation_curve() (+52 more)

### Community 68 - "Objective"
Cohesion: 0.10
Nodes (10): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData, Action, int (+2 more)

### Community 69 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

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

### Community 74 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 77 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 78 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "SaveDataMapper"
Cohesion: 0.16
Nodes (10): Mesh, Vector3, SaveDataMapper, bool, int, MeshSaveData, float, Vector2SaveData (+2 more)

### Community 83 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "update_environment_presentation"
Cohesion: 0.08
Nodes (34): AmbientLight, building_snow_strength(), BuildingMaterialExtension, BuildingMaterialInstance, BuildingMaterialInstances, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform (+26 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "Station"
Cohesion: 0.09
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "generate_and_spawn_world"
Cohesion: 0.07
Nodes (94): App, AssetServer, PresentationCatalog, actor_material(), apply_material_overrides(), building_effect_material(), building_material(), BuildingEffectKind (+86 more)

### Community 91 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 92 - "GameEvent"
Cohesion: 0.05
Nodes (22): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+14 more)

### Community 93 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (14): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+6 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 98 - "Coordinator"
Cohesion: 0.10
Nodes (16): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+8 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 102 - "run_transport"
Cohesion: 0.29
Nodes (7): BTreeSet, TwitchConfig, CredentialVault, Self, Sender, run_transport(), UnboundedReceiver

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "STSM_Idle"
Cohesion: 0.25
Nodes (4): bool, float, Vector3, STSM_Idle

### Community 108 - ".SetGeneratedResources"
Cohesion: 0.40
Nodes (5): List, Material, materials, Mesh, meshes

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
Cohesion: 0.11
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "EditorUtils"
Cohesion: 0.15
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 116 - "STSM_StateAction"
Cohesion: 0.11
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 117 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 118 - "VoteEvent"
Cohesion: 0.18
Nodes (10): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+2 more)

### Community 119 - "Resource"
Cohesion: 0.08
Nodes (12): DepositResources, int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor (+4 more)

### Community 120 - "DebugProcessor"
Cohesion: 0.07
Nodes (13): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, HideInCallstack, Object (+5 more)

### Community 121 - "Res"
Cohesion: 0.06
Nodes (113): Added, AnimationGraphHandle, AppExit, Assets, BackgroundColor, ActorCustomization, Agent, agent_is_moving() (+105 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.12
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 125 - "Targetable"
Cohesion: 0.07
Nodes (17): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, List, TargetProcessor, Dictionary (+9 more)

### Community 126 - "BuildingBase"
Cohesion: 0.09
Nodes (13): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+5 more)

### Community 127 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - ".GenerateFromSettings"
Cohesion: 0.07
Nodes (31): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+23 more)

### Community 133 - "WeatherProcessor"
Cohesion: 0.19
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "config.rs"
Cohesion: 0.23
Nodes (9): ConfigError, default_configuration_is_valid_and_round_trips_ron(), enabled_twitch_requires_public_configuration(), Result, Self, String, stable_id(), valid_twitch_login() (+1 more)

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (45): apply_technology_draft(), bounded_ui_index(), content_tab(), draw_world_preview(), inspector_tab(), main(), migration_tab(), poll_tool_job_events() (+37 more)

### Community 137 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 138 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 139 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

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

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 150 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 151 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 154 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 155 - "UserInterface"
Cohesion: 0.07
Nodes (9): InputButton, SharedTypes, TownGoal.Data, StreamTown.EditorTools, UserInterface, TechTree.Data, TechTree.ScriptableObjects, Data (+1 more)

### Community 156 - ".GetResourceAssets"
Cohesion: 0.11
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

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

### Community 165 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 166 - "TechTree_SO"
Cohesion: 0.17
Nodes (7): int, TechTreeSettings, ContainerBuilder, TechTreeSettingsInstaller, NodeGroup_SO, List, TechTree_SO

### Community 167 - "Season"
Cohesion: 0.14
Nodes (13): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Color, float (+5 more)

### Community 168 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 169 - "BinaryReader"
Cohesion: 0.14
Nodes (3): Func, List, BinaryReader

### Community 170 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 171 - "TimeProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, TimeProcessor

### Community 173 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UserInterface.MainMenu"
Cohesion: 0.22
Nodes (3): UserInterface.MainMenu, MetaData, Settings

### Community 177 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 178 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 179 - "ResourceStorageModifier"
Cohesion: 0.27
Nodes (3): ResourceStorageModifier, float, int

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 187 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 188 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 189 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 190 - "BuildingDataSettings"
Cohesion: 0.17
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 191 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 192 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 193 - "Utils"
Cohesion: 0.05
Nodes (16): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, RoleScriptablesEditor, STStateMachine.States (+8 more)

### Community 194 - "CommonEnums.cs"
Cohesion: 0.18
Nodes (10): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, ReviveType, Seasons, TimeOfDay (+2 more)

### Community 195 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 210 - ".StartMusic"
Cohesion: 0.49
Nodes (3): SeasonAudioData, AudioClip, List

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "BuildingSettings"
Cohesion: 0.20
Nodes (4): bool, Dictionary, int, BuildingSettings

### Community 213 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - ".RestoreWorldState"
Cohesion: 0.28
Nodes (3): float, int, TimeRuntimeData

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "TradeSettings"
Cohesion: 0.33
Nodes (5): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller

### Community 225 - "WorldSaveData"
Cohesion: 0.17
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "AllBuildingDataSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingDataContainerInstaller, AllBuildingDataSettings

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 235 - "VfxAnimationController"
Cohesion: 0.05
Nodes (20): Api, PersistentScoped, TL_API, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI, UI_VoteObjectiveRow (+12 more)

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.06
Nodes (30): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+22 more)

### Community 246 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 249 - "twitch.rs"
Cohesion: 0.14
Nodes (16): channel_point_reward_tag_survives_privmsg_conversion(), envelope_from_privmsg(), OAuthErrorResponse, Arc, Mutex, Option, Receiver, token_response_keeps_rotated_refresh_token() (+8 more)

### Community 254 - "RoleDataSettings"
Cohesion: 0.18
Nodes (10): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+2 more)

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 263 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 266 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 267 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 269 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 270 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 277 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 279 - "MonoBehaviour"
Cohesion: 0.02
Nodes (72): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder (+64 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (76): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+68 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 285 - "PlayerSaveData"
Cohesion: 0.11
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **32 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `.GenerateFromSettings`, `SettingsProcessor`, `WorldGenProcessor`, `Character`, `PlayerCommands`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `.GetResourceAssets`, `GameEventProcessor`, `SaveFileData`, `ObjectPoolingProcessor`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `IProcessor`, `StreamTownSessionBridge`, `ResourceProcessor`, `PlayerRole`, `UserInterface_GameMenu`, `.RestoreObjectiveProgress`, `SaveDataMapper`, `FoliageProcessor`, `.RestoreWorldState`, `TownGoalProcessor`, `MainMenuManager`, `Resource`, `DebugProcessor`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `.GenerateFromSettings`, `Target`, `TwitchChatProcessor`, `Player`, `GameStateProcessor`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `UserInterface_Debug`, `ObjectPoolingProcessor`, `IProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `WorldGenRuntimeData`, `Access_Dropdown`, `GridProcessor`, `CellSpacePartitioning`, `FoliageProcessor`, `GameEvent`, `Coordinator`, `EnemySpawner`, `SaveProcessor`, `DebugProcessor`, `AIPath`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `Access_Slider`, `GraphicsProcessor`, `UserInterface_GameMenu`, `SettingsData`, `MainMenuManager`, `Access_Dropdown`, `IProcessor`, `SaveProcessor`, `ScriptablesProcessorInfrastructure`, `Access_Text`, `MonoBehaviour`, `Access_Toggle`, `Autosave`, `UIElementWrapper`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `GeneratedWorld` be split into smaller, more focused modules?**
  _Cohesion score 0.09175377468060394 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07315233785822021 - nodes in this community are weakly interconnected._
- **Should `stream_town_domain/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08735150244584207 - nodes in this community are weakly interconnected._