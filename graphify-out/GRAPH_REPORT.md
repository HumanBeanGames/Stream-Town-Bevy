# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 609 files · ~1,606,887 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7053 nodes · 18566 edges · 273 communities (250 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 994 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f7732437`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- xtask/src/main.rs
- BuildingProcessor
- stream_town_domain/src/content.rs
- stream_town_migrate/src/presentation.rs
- Character
- EnemyModelHandler
- TwitchChatProcessor
- TechTreeNode
- BottomBarInterface
- Target
- SettingsProcessor
- WorldGenProcessor
- Option
- GameMasterCommands
- TechTreeIOUtility
- World.Generation.Settings
- Utils
- Processors
- .GenerateFromSettings
- legacy.rs
- UnlockVisualElement
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- STSM_Action_GatherResource
- Goal
- PlayerSettings
- SaveFileData
- GameEventProcessor
- GridPos
- PlayerRoleData
- STSM_Action_PlayerBase
- SensorProcessor
- StationProcessor
- UserInterface_Debug
- CommandDictionary
- SettingsData
- AnimationHandler
- BinaryWriter
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- LabelDisplayProcessor
- AnimationControllerDef
- GameStateProcessor
- SeasonProcessor
- DebugProcessor
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Access_Dropdown
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
- twitch.rs
- Objective
- TransformSaveData
- GenerationSettings
- models.rs
- Tiler
- ScriptablesEditor
- BTreeMap
- UserInterface_ObjectSelection
- RoleHandler
- ProjectCamera
- GridSystem.Partitioning
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- UserInterface_TownGoal
- DontDestroyOnLoad
- Access_Text
- stream_town_game/src/lib.rs
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- GUIDProcessor
- PlayerRole
- RaidEvent
- EditorUtils
- convert_fbx_to_glb.py
- command.rs
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
- FoliageGenerationSettings
- ResourceHolder
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- Targetable
- SelectedResource
- IRuntimeDataScriptable
- VoteEvent
- Resource
- GameEvent
- Commands
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Station
- BuildingBase
- CommonEnums.cs
- UpdateGraphBounds
- SelectableObject
- stream_town_migrate/src/main.rs
- Res
- .RenderResourceType
- Enemy
- ConfirmCheck
- STSM_Idle_Player
- ToolState
- Globals
- TechnologyTreeGroup
- RoleData
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
- SelectedEnemyCamp
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- WeatherProcessor
- UserInterface
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- WindController
- RotationHandler
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- RoleSlot
- MiscCommands
- StringUtils
- CampGenerationSettings
- Easings
- VfxAnimationController
- STSM_StateAction
- TimeProcessor
- PlayerSaveData
- Access_GOList
- SelectedBuilding
- EditorHelpers
- BuildingResourceModelHandler
- IProcessor
- WorldGenSaveData
- TerrainGenSettings
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- PoolableObject
- .SetGeneratedResources
- GridNode
- TargetProcessor
- STSM_Helper_Attack
- KeepKingVote
- .BuildMatricesDictionary
- BuildingDataSettings
- settings.rs
- Player
- BuildingSettings
- WorldGenRuntimeData
- NewKingVote
- SelectedEnemy
- UILineRenderer
- UserInterface_DisplayUsernames
- GridProcessor
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- STSM_HelperBase
- SelectedPlayerGroup
- Key Rules
- SimpleDisableAfterTime
- Common Patterns
- FoliageData
- DayAndNightSettings
- PlayerSpawnPoint
- NodeUnlockData
- ResourceDataSettings
- PostProcessingInstaller
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- AudioMixerInstaller
- VfxParticlePosition
- ScriptKeywordProcessor
- IntWrapper
- AutosaveIntervalsInstaller
- Processor Template
- Common Patterns
- TechTree_SO
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- BuildPlacerData
- Twitch setup
- graphify reference: add a URL and watch a folder
- .RenderFoliageType
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- ForwardRendererInstaller
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- RenderPipelineInstaller
- CreateProjectScopeProcessors.cs
- VideoSettingsPresetsInstaller
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- HealthModifier
- AGENTS.md
- PlayerDeathHandler
- VFXArrowPointer
- PassiveResourceIncrementer
- TradeProcessor
- CustomLogger
- Access_TextInput
- extraction-spec.md
- FoliageRuntimeData
- PlayerInputRuntimeData
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- PlacementProbeHandler
- TechTreeNodeType.cs
- PoolablePlayer.cs
- ObjectSelectionProcessor.Editor.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- UnitTravelToPosition
- ObjectiveSaveData
- BuildingRuntimeData
- NodeGroup_SO
- .InjectRuntimeData
- ScriptablesProcessorInfrastructure
- MonoBehaviour
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

## Communities (273 total, 23 thin omitted)

### Community 0 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (55): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+47 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "Character"
Cohesion: 0.07
Nodes (19): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+11 more)

### Community 5 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "TechTreeNode"
Cohesion: 0.06
Nodes (23): Color, Foldout, List, Sprite, Vector2, VisualElement, TechTreeNode, Port (+15 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Target"
Cohesion: 0.07
Nodes (14): STStateMachine.States, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors, STStateMachine (+6 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 12 - "Option"
Cohesion: 0.05
Nodes (81): AmbientLight, App, AssetServer, PresentationCatalog, actor_detail_budget(), animation_property_value(), animation_root_name(), building_material() (+73 more)

### Community 13 - "GameMasterCommands"
Cohesion: 0.09
Nodes (5): List, GameSettings, GameMasterCommands, RulerCommands, Vector3

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.08
Nodes (20): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+12 more)

### Community 15 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 16 - "Utils"
Cohesion: 0.05
Nodes (7): RoleScriptablesEditor, Utils, ScriptablesEditor, SavingAndLoading, SavingAndLoading.Structs, GameResources, World.Generation

### Community 17 - "Processors"
Cohesion: 0.05
Nodes (14): BuildCostModifier, InputButton, UserInterface.MainMenu, PlayerControls.ObjectSelection, Processors, World, Level, MetaData (+6 more)

### Community 18 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+9 more)

### Community 19 - "legacy.rs"
Cohesion: 0.05
Nodes (111): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+103 more)

### Community 20 - "UnlockVisualElement"
Cohesion: 0.22
Nodes (5): NodeUnlockSaveData, Button, EnumField, UnlockVisualElement, TechType

### Community 21 - "TechTree.Elements"
Cohesion: 0.06
Nodes (26): int, ChangeTimeStamp, Color, ErrorData, List, GroupErrorData, List, NodeErrorData (+18 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (12): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+4 more)

### Community 24 - "HealthHandler"
Cohesion: 0.10
Nodes (12): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+4 more)

### Community 25 - "STSM_Action_GatherResource"
Cohesion: 0.13
Nodes (7): float, int, Resource, uint, Vector3, ResourceTarget, STSM_Action_GatherResource

### Community 26 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 27 - "PlayerSettings"
Cohesion: 0.08
Nodes (42): AnyResult, AppExit, PlayerSettings, Default, broadcaster_gate_precedes_twitch_command_dispatch(), capture_screenshot(), credits_input(), CreditsTimeline (+34 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 30 - "GridPos"
Cohesion: 0.05
Nodes (86): WorldGenConfig, FoliageHabitat, FoliageLayerDef, FoliageVariantDef, GridPos, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash() (+78 more)

### Community 31 - "PlayerRoleData"
Cohesion: 0.09
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 32 - "STSM_Action_PlayerBase"
Cohesion: 0.12
Nodes (5): AttackUnit, STSM_Action_Build, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 33 - "SensorProcessor"
Cohesion: 0.15
Nodes (4): SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 34 - "StationProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 39 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+6 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "DebugProcessor"
Cohesion: 0.11
Nodes (11): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, HideInCallstack, Object (+3 more)

### Community 48 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 49 - ".Draw"
Cohesion: 0.11
Nodes (19): Button, EnumField, ObjectiveVisualElement, Port, Action, Button, EnumField, Foldout (+11 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.14
Nodes (12): Container, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int (+4 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 57 - "StableId"
Cohesion: 0.04
Nodes (109): ContentCatalog, ObjectiveDef, FromStr, StableId, ActorState, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState (+101 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.14
Nodes (7): bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "VfxSeagullSpawner"
Cohesion: 0.07
Nodes (20): GateController, Animator, bool, int, List, IEnumerator, FPS(), Start() (+12 more)

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
Cohesion: 0.11
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "twitch.rs"
Cohesion: 0.06
Nodes (54): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet, Default (+46 more)

### Community 68 - "Objective"
Cohesion: 0.13
Nodes (4): Action, int, Objective, EnemyType

### Community 69 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

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
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "GridSystem.Partitioning"
Cohesion: 0.18
Nodes (4): GridProcessorEditor, GridSystem.Utils, GridSystem.Partitioning, GridSystem

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 83 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (190): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, ActorCustomization, StreamUserType, ActionPresentation (+182 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.18
Nodes (5): Bounds, Container, ContainerBuilder, HashSet, FoliageProcessor

### Community 90 - "GUIDProcessor"
Cohesion: 0.14
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 91 - "PlayerRole"
Cohesion: 0.09
Nodes (8): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, PlayerRole

### Community 92 - "RaidEvent"
Cohesion: 0.07
Nodes (19): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+11 more)

### Community 93 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 97 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "Editor"
Cohesion: 0.18
Nodes (6): GameObject, List, EquipmentHandlerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 101 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

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

### Community 107 - "FoliageGenerationSettings"
Cohesion: 0.13
Nodes (13): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, Material (+5 more)

### Community 108 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

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

### Community 113 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "Targetable"
Cohesion: 0.11
Nodes (9): int, STSM_Helper_Build, bool, BoxCollider, float, int, Transform, Vector3 (+1 more)

### Community 117 - "IRuntimeDataScriptable"
Cohesion: 0.13
Nodes (13): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+5 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (29): DepositResources, ResourceStorageModifier, float, int, PlayerInventory, Dictionary, ResourceInventory, bool (+21 more)

### Community 120 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 121 - "Commands"
Cohesion: 0.11
Nodes (70): GameConfig, actor_material(), actor_scene_budget(), building_effect_material(), BuildingEffectKind, BuildingEffectParticle, BuildingPresentation, combat_material() (+62 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.20
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "Station"
Cohesion: 0.09
Nodes (15): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+7 more)

### Community 126 - "BuildingBase"
Cohesion: 0.11
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 127 - "CommonEnums.cs"
Cohesion: 0.09
Nodes (20): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+12 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "Res"
Cohesion: 0.07
Nodes (121): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimationGraphHandle, Assets, AudioSink, BackgroundColor, ActorAnimationDriver (+113 more)

### Community 132 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 133 - "Enemy"
Cohesion: 0.12
Nodes (9): Action, float, Enemy, int, ActiveResourceIncrementer, Action, Container, ContainerBuilder (+1 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "STSM_Idle_Player"
Cohesion: 0.11
Nodes (10): bool, float, Vector3, STSM_Idle_Enemy, bool, float, uint, Vector3 (+2 more)

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (86): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+78 more)

### Community 138 - "TechnologyTreeGroup"
Cohesion: 0.15
Nodes (11): Color, float, string, TechnologyTreeGroup, List, Texture2D, TechTreeSearchWindow, Group (+3 more)

### Community 139 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

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
Cohesion: 0.08
Nodes (22): Action, bool, CancellationToken, Container, ContainerBuilder, float, int, LoadSceneMode (+14 more)

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

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 154 - "WeatherProcessor"
Cohesion: 0.08
Nodes (19): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Container, ContainerBuilder (+11 more)

### Community 155 - "UserInterface"
Cohesion: 0.07
Nodes (12): InputButton, SharedTypes, TownGoal.Enumerations, TownGoal.Data, StreamTown.EditorTools, TownGoal, UserInterface, TechTree.Data (+4 more)

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

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

### Community 164 - "RoleSlot"
Cohesion: 0.14
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 165 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 167 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 169 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 170 - "STSM_StateAction"
Cohesion: 0.19
Nodes (5): bool, float, int, STSM_StateAction, AnimationName

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - "PlayerSaveData"
Cohesion: 0.07
Nodes (24): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, bool (+16 more)

### Community 173 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "BuildingResourceModelHandler"
Cohesion: 0.17
Nodes (5): BuildingResourceModelHandler, GameObject, BuildingResourceModelHandlerEditor, UnityEvent, StorageStatus

### Community 177 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 178 - "WorldGenSaveData"
Cohesion: 0.11
Nodes (13): bool, int, MeshSaveData, List, SaveGameData, float, Vector2SaveData, float (+5 more)

### Community 179 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "PoolableObject"
Cohesion: 0.07
Nodes (24): uint, GUIDComponent, BoxCollider, List, Quaternion, Vector3, SaveableBuilding, SaveableEnemy (+16 more)

### Community 183 - ".SetGeneratedResources"
Cohesion: 0.40
Nodes (5): List, Material, materials, Mesh, meshes

### Community 184 - "GridNode"
Cohesion: 0.14
Nodes (10): int, List, Vector2, GridNode, Color, CollisionColours, CollisionType, float (+2 more)

### Community 185 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 187 - "STSM_Helper_Attack"
Cohesion: 0.18
Nodes (4): int, STSM_Helper_Attack, int, STSM_Action_Attack

### Community 189 - ".BuildMatricesDictionary"
Cohesion: 0.51
Nodes (6): Dictionary, Material, Matrix4x4, Mesh, material, mesh

### Community 190 - "BuildingDataSettings"
Cohesion: 0.17
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 191 - "settings.rs"
Cohesion: 0.11
Nodes (27): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+19 more)

### Community 192 - "Player"
Cohesion: 0.08
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, OnChatCommandReceivedArgs, TwitchClientProcessor (+2 more)

### Community 193 - "BuildingSettings"
Cohesion: 0.18
Nodes (5): bool, Dictionary, int, BuildingSettings, Age

### Community 194 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 195 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 200 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "FoliageData"
Cohesion: 0.24
Nodes (6): List, Material, Mesh, Quaternion, Vector3, FoliageData

### Community 209 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 210 - "PlayerSpawnPoint"
Cohesion: 0.05
Nodes (16): Api, PersistentScoped, Transform, PlayerSpawnPoint, TL_API, Slider, TextMeshProUGUI, UI_Objective (+8 more)

### Community 212 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 213 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 218 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 221 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "TechTree_SO"
Cohesion: 0.25
Nodes (6): int, TechTreeSettings, ContainerBuilder, TechTreeSettingsInstaller, List, TechTree_SO

### Community 225 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - ".RenderFoliageType"
Cohesion: 0.32
Nodes (6): Dictionary, int, Material, Matrix4x4, Mesh, FoliageRenderer

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 235 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.09
Nodes (18): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+10 more)

### Community 244 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 246 - "PlayerDeathHandler"
Cohesion: 0.29
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 247 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 248 - "PassiveResourceIncrementer"
Cohesion: 0.33
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 249 - "TradeProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, TradeProcessor

### Community 251 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 253 - "FoliageRuntimeData"
Cohesion: 0.33
Nodes (6): Dictionary, List, Material, Matrix4x4, Mesh, FoliageRuntimeData

### Community 254 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 262 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 263 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 279 - "MonoBehaviour"
Cohesion: 0.02
Nodes (78): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ChannelDataInstaller (+70 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (64): ContainerBuilder, WaterResourceGenSettingsInstaller, List, FoliageGenSettings, bool, ParticleSystem, Transform, GameEventSettings (+56 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `Character`, `SettingsProcessor`, `WorldGenProcessor`, `GameMasterCommands`, `ObjectPoolingProcessor`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `TechTreeProcessor`, `TimeProcessor`, `PlayerSaveData`, `SeasonProcessor`, `DebugProcessor`, `IProcessor`, `WorldGenSaveData`, `StreamTownSessionBridge`, `ResourceProcessor`, `PoolableObject`, `FoliageProcessor`, `GUIDProcessor`, `PlayerRole`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `FoliageGenerationSettings`, `Resource`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `Target`, `GameMasterCommands`, `ObjectPoolingProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `UserInterface_Debug`, `CampGenerationSettings`, `GameStateProcessor`, `DebugProcessor`, `IProcessor`, `TerrainGenSettings`, `ResourceProcessor`, `TwitchClientProcessor`, `WorldGenRuntimeData`, `GridProcessor`, `ProjectCamera`, `CellSpacePartitioning`, `FoliageProcessor`, `GUIDProcessor`, `RaidEvent`, `EnemySpawner`, `SaveProcessor`, `AIPath`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `Character`, `TwitchChatProcessor`, `BuildingRuntimeData`, `Target`, `RoleData`, `GameMasterCommands`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `GameEventProcessor`, `PlayerRoleData`, `StationProcessor`, `UserInterface_Debug`, `CommandDictionary`, `MiscCommands`, `CharacterModelHandler`, `LabelDisplayProcessor`, `PoolableObject`, `UserInterface_DisplayUsernames`, `RoleHandler`, `TargetSensor`, `PlayerRole`, `RaidEvent`, `SaveProcessor`, `VoteEvent`, `VFXArrowPointer`, `.SetTargetType`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07315233785822021 - nodes in this community are weakly interconnected._
- **Should `stream_town_domain/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07650273224043716 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/presentation.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10367063492063493 - nodes in this community are weakly interconnected._