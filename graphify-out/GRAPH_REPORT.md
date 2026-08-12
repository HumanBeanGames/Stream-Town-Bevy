# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 607 files · ~1,583,621 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6736 nodes · 17212 edges · 264 communities (232 shown, 32 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 992 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `da840bb8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- MonoBehaviour
- BuildingProcessor
- ContentCatalog
- stream_town_migrate/src/presentation.rs
- Utils
- TownGoal.Data
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- String
- SettingsProcessor
- WorldGenProcessor
- Player
- SelectedPlayerGroup
- TechTreeIOUtility
- SelectedPlayer
- Character
- twitch.rs
- RoleHandler
- WorldGenSaveData
- .GenerateFromSettings
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- CommonEnums.cs
- GenerationSettings
- RoleProcessor
- SaveFileData
- GameEventProcessor
- FoliageGenerationSettings
- PlayerInventory
- UserInterface
- WorldInstanceDeterminism
- Station
- UserInterface_Debug
- stream_town_game/src/lib.rs
- SettingsData
- AnimationHandler
- .Log
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BinarySaveCodec
- AnimationControllerDef
- legacy.rs
- SeasonProcessor
- STSM_Idle_Player
- TechTreeNode
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- GameEvent
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
- Result
- Objective
- TradeProcessor
- MeshData
- models.rs
- Tiler
- ScriptablesEditor
- BTreeMap
- UserInterface_ObjectSelection
- UserInterface.MainMenu
- UIElementWrapper
- GridProcessor
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- Goal
- RotationHandler
- Access_Text
- Coordinator
- Targetable
- UserInterface_TownVote
- STSM_Idle
- FoliageProcessor
- Option
- NavGrid
- UserInterface_Event
- IRuntimeDataScriptable
- convert_fbx_to_glb.py
- Access_Toggle
- stream_town_migrate/src/content.rs
- TwitchUser
- .StartupSequence
- stream_town_domain/src/presentation.rs
- Globals
- BuildingResourceModelHandler
- MainMenuManager
- TownGoalProcessor
- GameMasterCommands
- UnitHealthBar
- LoadingManager
- World.Generation
- ResourceRuntimeData
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- EditorUtils
- EnemyModelHandler
- RoleDataContainer
- VoteEvent
- Resource
- DebugProcessor
- process_injected_commands
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- PoolableObject
- BuildingBase
- CommandDictionary
- UpdateGraphBounds
- IProcessor
- stream_town_migrate/src/main.rs
- BuildingDamageMaterialHandler
- GateController
- AstarPath
- ConfirmCheck
- TreeMaterialExtension
- ToolState
- LabelDisplayProcessor
- SelectedResource
- UserInterface_GameMenu
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- PlayerSaveData
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- DontDestroyOnLoad
- PlayerRoleData
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- ResourceTarget
- STSM_HelperBase
- CampGenerationSettings
- .GetResourceAssets
- Stream Town Reloaded - Architecture Documentation
- WindController
- GridProcessor.cs
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- ResourceGenerationSettings
- NewKingVote
- .RenderResourceType
- RoleSlotModifier
- WeatherProcessor
- VFXArrowPointer
- EventProcessor
- TimeProcessor
- TargetSettings
- World.Generation.Settings
- SelectedBuilding
- EditorHelpers
- ResourceStorageModifier
- ParallelProgressReporter
- .Update
- SelectedEnemyCamp
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- WorldGenRuntimeData
- StringUtils
- BuildPlacerData
- STSM_Action_DepositResource
- VfxAnimationController
- .RefreshSceneBindingsAndTryGenerate
- RoleData
- BuildingDataSettings
- command.rs
- TechTreeSettings
- Sensors
- ResourceData
- IProcessor.cs
- PlayerSpawnPoint
- UILineRenderer
- UserInterface_DisplayUsernames
- StatusBar
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- PassiveResourceIncrementer
- ObjectSelectionProcessor.Editor.cs
- Key Rules
- SelectedEnemy
- Common Patterns
- Requirement
- .MoveCamera
- SimpleDisableAfterTime
- FPSDisplay.cs
- ObjectiveSaveData
- .GetCompatiblePorts
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- BuildingSettings
- VFX
- ScriptKeywordProcessor
- .InjectRuntimeData
- NodeGroup_SO
- Processor Template
- Common Patterns
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- Twitch setup
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Easings
- CreateProjectScopeProcessors.cs
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- AGENTS.md
- CustomLogger
- KeepKingVote
- extraction-spec.md
- UnitTravelToPosition
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- Autosave
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- TargetSensor
- Editor
- PlayerInputRuntimeData
- VfxParticlePosition
- TL_API
- UI_TechOption
- .InjectRuntimeData
- ScriptablesProcessorInfrastructure
- .RefreshSceneData
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- TransformSaveData

## God Nodes (most connected - your core abstractions)
1. `StableId` - 239 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldSimulation` - 117 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `SaveProcessor` - 88 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_enemies_drive_damage_range_cadence_and_weighted_spawning()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (264 total, 32 thin omitted)

### Community 0 - "MonoBehaviour"
Cohesion: 0.01
Nodes (102): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+94 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "ContentCatalog"
Cohesion: 0.06
Nodes (69): ConfigError, default_configuration_is_valid_and_round_trips_ron(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, BTreeMap, Default, Result (+61 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "Utils"
Cohesion: 0.05
Nodes (14): BuildCostModifier, RoleScriptablesEditor, InputButton, Utils, Processors, World, Level, ScriptablesEditor (+6 more)

### Community 5 - "TownGoal.Data"
Cohesion: 0.08
Nodes (11): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, TechTree.Data (+3 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): List, GameSettings, bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate (+17 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (20): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, Vector2 (+12 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "String"
Cohesion: 0.05
Nodes (81): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, active_event_text(), add_animation_layer_branch(), add_rotation_curve() (+73 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.08
Nodes (14): Action, bool, BoxCollider, Container, GameObject, IEnumerable, int, IReadOnlyList (+6 more)

### Community 12 - "Player"
Cohesion: 0.08
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, OnChatCommandReceivedArgs, TwitchClientProcessor (+2 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+11 more)

### Community 16 - "Character"
Cohesion: 0.08
Nodes (14): Pets.Enumerations, TownGoal, Character.Enumerations, Core, Pets, GameEventSystem, GameEventSystem.Events, SavingAndLoading (+6 more)

### Community 17 - "twitch.rs"
Cohesion: 0.13
Nodes (22): channel_point_reward_tag_survives_privmsg_conversion(), envelope_from_privmsg(), OAuthErrorResponse, Arc, Mutex, Option, Receiver, String (+14 more)

### Community 18 - "RoleHandler"
Cohesion: 0.11
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, PlayerRole

### Community 19 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 20 - ".GenerateFromSettings"
Cohesion: 0.18
Nodes (10): HashSet, Func, HashSet, List, Vector2, Vector3, GenerateDebugPositions(), GetPositiveNoiseOffset() (+2 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.06
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.06
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.12
Nodes (8): Func, Action, bool, float, int, UnityEvent, HealthHandler, ReviveType

### Community 25 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (21): ContainerBuilder, AllRoleDataSettingsInstaller, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings, AudioClip, bool, float (+13 more)

### Community 26 - "GenerationSettings"
Cohesion: 0.11
Nodes (16): AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings, Action, IEnumerator (+8 more)

### Community 27 - "RoleProcessor"
Cohesion: 0.08
Nodes (9): Container, ContainerBuilder, int, List, RoleProcessor, Dictionary, MiscCommands, Dictionary (+1 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "FoliageGenerationSettings"
Cohesion: 0.18
Nodes (9): List, FoliageGenSettings, List, Material, Mesh, string, Vector3, FoliageGenerationSettings (+1 more)

### Community 31 - "PlayerInventory"
Cohesion: 0.22
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 32 - "UserInterface"
Cohesion: 0.09
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, UserInterface, Combat, SavingAndLoading.SavableObjects (+1 more)

### Community 33 - "WorldInstanceDeterminism"
Cohesion: 0.24
Nodes (7): Material, Resource, int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 34 - "Station"
Cohesion: 0.06
Nodes (24): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+16 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "stream_town_game/src/lib.rs"
Cohesion: 0.03
Nodes (138): GridPos, ActorState, RoleProgress, Default, String, cell_hash(), changing_seed_changes_world_hash(), generate_world() (+130 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.09
Nodes (14): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+6 more)

### Community 39 - ".Log"
Cohesion: 0.05
Nodes (24): Action, HideInCallstack, Object, DebugLogCategory, Container, ContainerBuilder, GameStateProcessor, Action (+16 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.07
Nodes (20): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+12 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "legacy.rs"
Cohesion: 0.05
Nodes (112): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+104 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "STSM_Idle_Player"
Cohesion: 0.04
Nodes (26): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, int, STSM_Helper_Attack (+18 more)

### Community 48 - "TechTreeNode"
Cohesion: 0.09
Nodes (19): Button, EnumField, ObjectiveVisualElement, Color, Foldout, List, Sprite, VisualElement (+11 more)

### Community 49 - ".Draw"
Cohesion: 0.11
Nodes (18): NodeUnlockSaveData, Port, Button, EnumField, UnlockVisualElement, Action, Button, EnumField (+10 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (12): bool, double, float, int, List, long, MenuItem, string (+4 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.22
Nodes (5): Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "GameEvent"
Cohesion: 0.07
Nodes (14): Transform, bool, IEnumerator, int, List, string, RaidEvent, Action (+6 more)

### Community 57 - "StableId"
Cohesion: 0.05
Nodes (65): ObjectiveDef, ObjectiveKind, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips() (+57 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.14
Nodes (8): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 61 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (9): bool, Camera, float, int, PlayerInput, Transform, Vector2, CameraController (+1 more)

### Community 63 - "Node_SO"
Cohesion: 0.14
Nodes (12): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+4 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (10): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, ICollection, IDictionary, ISerializationCallbackReceiver (+2 more)

### Community 67 - "Result"
Cohesion: 0.14
Nodes (16): BTreeSet, String, TwitchConfig, CredentialVault, DeviceAuthorization, OAuthClient, Client, Formatter (+8 more)

### Community 68 - "Objective"
Cohesion: 0.08
Nodes (11): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+3 more)

### Community 69 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

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
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "UserInterface.MainMenu"
Cohesion: 0.18
Nodes (3): UserInterface.MainMenu, MetaData, Settings

### Community 77 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 78 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 83 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 86 - "Targetable"
Cohesion: 0.05
Nodes (28): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+20 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "STSM_Idle"
Cohesion: 0.17
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "Option"
Cohesion: 0.08
Nodes (56): AmbientLight, AnyResult, Assets, AssetServer, PresentationCatalog, animation_property_value(), animation_root_name(), archetype_by_source() (+48 more)

### Community 91 - "NavGrid"
Cohesion: 0.13
Nodes (19): can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode, path_routes_around_dynamic_building() (+11 more)

### Community 92 - "UserInterface_Event"
Cohesion: 0.12
Nodes (11): Animator, GameObject, IEnumerator, int, FishGodEvent, OnMessageReceivedArgs, EventCommands, GameObject (+3 more)

### Community 93 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (15): CreditsRuntimeData, bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData, bool (+7 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.09
Nodes (97): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+89 more)

### Community 97 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 98 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "Globals"
Cohesion: 0.20
Nodes (3): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, Globals

### Community 101 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 102 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "GameMasterCommands"
Cohesion: 0.12
Nodes (3): GameMasterCommands, RulerCommands, Vector3

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 108 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

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
Cohesion: 0.12
Nodes (10): float, int, List, Transform, EnemySpawner, float, ChanceObject, float (+2 more)

### Community 113 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 116 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (10): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+2 more)

### Community 117 - "RoleDataContainer"
Cohesion: 0.12
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 118 - "VoteEvent"
Cohesion: 0.18
Nodes (10): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+2 more)

### Community 119 - "Resource"
Cohesion: 0.08
Nodes (9): DepositResources, int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor (+1 more)

### Community 120 - "DebugProcessor"
Cohesion: 0.15
Nodes (9): Dictionary, DebugSettings, IMainThreadInitializableProcessor, Container, ContainerBuilder, DebugProcessor, UnityEvent, DebugRuntimeData (+1 more)

### Community 121 - "process_injected_commands"
Cohesion: 0.05
Nodes (149): Added, AnimationGraphHandle, App, AppExit, BackgroundColor, actor_detail_budget(), actor_material(), actor_scene_budget() (+141 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.11
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.18
Nodes (14): bool, float, int, Vector3, AIPath, GraphCollision, GraphNode, GraphUpdateObject (+6 more)

### Community 125 - "PoolableObject"
Cohesion: 0.04
Nodes (38): CollectResource, Container, ContainerBuilder, GUIDProcessor, Action, float, Enemy, AnimationCurve (+30 more)

### Community 126 - "BuildingBase"
Cohesion: 0.11
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 127 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "IProcessor"
Cohesion: 0.23
Nodes (4): CancellationToken, Task, Container, IProcessor

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 132 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 133 - "AstarPath"
Cohesion: 0.31
Nodes (5): string, Type, AstarData, AstarPath, NavGraph

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "TreeMaterialExtension"
Cohesion: 0.16
Nodes (15): BuildingMaterialExtension, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform, Image, TerrainMaterialExtension, TerrainMaterialUniform, tree_season_controls() (+7 more)

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (47): apply_technology_draft(), bounded_ui_index(), content_tab(), draw_world_preview(), inspector_tab(), main(), migration_tab(), poll_tool_job_events() (+39 more)

### Community 137 - "LabelDisplayProcessor"
Cohesion: 0.06
Nodes (23): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller (+15 more)

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "PlayerSaveData"
Cohesion: 0.06
Nodes (26): Dictionary, Mesh, Vector3, SaveDataMapper, bool, int, List, string (+18 more)

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

### Community 149 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 150 - "PlayerRoleData"
Cohesion: 0.10
Nodes (9): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+1 more)

### Community 151 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "ResourceTarget"
Cohesion: 0.24
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 154 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 155 - "CampGenerationSettings"
Cohesion: 0.22
Nodes (7): List, CampGenSettings, float, int, string, Vector2, CampGenerationSettings

### Community 156 - ".GetResourceAssets"
Cohesion: 0.12
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

### Community 164 - "ResourceGenerationSettings"
Cohesion: 0.20
Nodes (9): List, ResourceGenSettings, List, WaterResourceGenSettings, AnimationCurve, bool, int, List (+1 more)

### Community 165 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 166 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 168 - "WeatherProcessor"
Cohesion: 0.08
Nodes (19): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Container, ContainerBuilder (+11 more)

### Community 169 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 170 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - "TargetSettings"
Cohesion: 0.29
Nodes (4): TargetSettings, ContainerBuilder, TargetSettingsInstaller, TargetableData

### Community 173 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 178 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

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

### Community 185 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 186 - "STSM_Action_DepositResource"
Cohesion: 0.33
Nodes (3): float, STSM_Action_DepositResource, STStateBase

### Community 187 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 189 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 190 - "BuildingDataSettings"
Cohesion: 0.12
Nodes (14): ContainerBuilder, AllBuildingDataSettingsInstaller, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingDataContainerInstaller, int, ResourceCostData (+6 more)

### Community 191 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 192 - "TechTreeSettings"
Cohesion: 0.40
Nodes (4): int, TechTreeSettings, ContainerBuilder, TechTreeSettingsInstaller

### Community 193 - "Sensors"
Cohesion: 0.09
Nodes (7): STStateMachine.States, Behaviours, Animation, Sensors, STStateMachine, Pathfinding, STStateMachine.Helpers

### Community 194 - "ResourceData"
Cohesion: 0.21
Nodes (6): bool, int, Matrix4x4, uint, Vector3, ResourceData

### Community 195 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 200 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 210 - "SimpleDisableAfterTime"
Cohesion: 0.15
Nodes (6): Slider, TextMeshProUGUI, UI_Objective, float, GameObject, SimpleDisableAfterTime

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 213 - ".GetCompatiblePorts"
Cohesion: 0.50
Nodes (3): List, Port, NodeAdapter

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "BuildingSettings"
Cohesion: 0.14
Nodes (7): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller, Age

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 225 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

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
Cohesion: 0.07
Nodes (20): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+12 more)

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 263 - "Editor"
Cohesion: 0.18
Nodes (6): GameObject, List, EquipmentHandlerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 266 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 267 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 269 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (54): float, Material, Volume, DayAndNightSettings, bool, ParticleSystem, Transform, GameEventSettings (+46 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 285 - "TransformSaveData"
Cohesion: 0.13
Nodes (12): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+4 more)

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

- **Why does `SaveProcessor` connect `SaveProcessor` to `MonoBehaviour`, `IProcessor`, `BuildingProcessor`, `TwitchChatProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `UserInterface_GameMenu`, `PlayerSaveData`, `Character`, `PlayerProcessor`, `ScriptableObject`, `RoleProcessor`, `.GetResourceAssets`, `GameEventProcessor`, `FoliageGenerationSettings`, `SaveFileData`, `ResourceGenerationSettings`, `.Log`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `StreamTownSessionBridge`, `ResourceProcessor`, `FoliageProcessor`, `WorldSaveData`, `MainMenuManager`, `TownGoalProcessor`, `Resource`, `DebugProcessor`, `PoolableObject`?**
  _High betweenness centrality (0.052) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `MonoBehaviour`, `IProcessor`, `BuildingProcessor`, `TwitchChatProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `ScriptableObject`, `GenerationSettings`, `CampGenerationSettings`, `FoliageGenerationSettings`, `UserInterface`, `WorldInstanceDeterminism`, `UserInterface_Debug`, `ResourceGenerationSettings`, `.Log`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `WorldGenRuntimeData`, `GameEvent`, `GridProcessor`, `Targetable`, `FoliageProcessor`, `.InjectRuntimeData`, `.StartupSequence`, `GameMasterCommands`, `EnemySpawner`, `SaveProcessor`, `DebugProcessor`, `PoolableObject`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `Autosave`, `GraphicsProcessor`, `IProcessor`, `Access_Slider`, `MonoBehaviour`, `SettingsData`, `MainMenuManager`, `UserInterface_GameMenu`, `UIElementWrapper`, `SaveProcessor`, `ScriptablesProcessorInfrastructure`, `Access_Text`, `Access_Dropdown`, `Access_Toggle`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MonoBehaviour` be split into smaller, more focused modules?**
  _Cohesion score 0.01454234388366125 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07390648567119155 - nodes in this community are weakly interconnected._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.06254272043745727 - nodes in this community are weakly interconnected._