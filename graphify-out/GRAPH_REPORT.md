# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 609 files · ~1,606,635 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7047 nodes · 18544 edges · 278 communities (247 shown, 31 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 994 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `45c28320`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- world.rs
- BuildingProcessor
- stream_town_domain/src/content.rs
- stream_town_migrate/src/presentation.rs
- Character
- EnemyModelHandler
- TwitchChatProcessor
- TechTreeNode
- BottomBarInterface
- Units
- SettingsProcessor
- WorldGenProcessor
- save.rs
- Result
- TechTreeIOUtility
- World.Generation.Settings
- Utils
- Processors
- .GenerateFromSettings
- legacy.rs
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- STSM_Idle_Player
- Goal
- ResMut
- SaveFileData
- GameEventProcessor
- ContentCatalog
- convert
- STSM_Action_PlayerBase
- SensorProcessor
- IInstaller
- UserInterface_Debug
- CommandDictionary
- SettingsData
- AnimationHandler
- WorldInstanceDeterminism
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- LabelDisplayProcessor
- AnimationControllerDef
- GameStateProcessor
- SeasonProcessor
- DebugSettings
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- WeatherProcessor
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
- MeshData
- models.rs
- Tiler
- ScriptablesEditor
- BTreeMap
- UserInterface_ObjectSelection
- runtime_console.rs
- Access_Dropdown
- GridProcessor.cs
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
- RoleHandler
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
- WorldGenSaveData
- ResourceHolder
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- .StartMusic
- SelectedResource
- IRuntimeDataScriptable
- VoteEvent
- Resource
- ChanceObjectList
- Option
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Targetable
- BuildingBase
- CommonEnums.cs
- UpdateGraphBounds
- SelectableObject
- stream_town_migrate/src/main.rs
- Query
- .RenderResourceType
- DayAndNightRuntimeData
- ConfirmCheck
- STSM_Idle
- ToolState
- .DrawDataFieldAndLabel
- UserInterface_GameMenu
- GameEventSettings
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
- UIElementWrapper
- Season
- UserInterface
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- WindController
- RotationHandler
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- FoliageGenerationSettings.cs
- StringUtils
- CampGenerationSettings
- Easings
- VfxAnimationController
- BuildingConfigSettings
- IProcessor
- PlayerSaveData
- Access_GOList
- SelectedBuilding
- EditorHelpers
- GameEventConfigSettingsInstaller
- .InitializeAndActivateProcessorsAsync
- .CapturePlayers
- TerrainGenSettings
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- PoolableObject
- TimeSettings
- ObjectPoolingSettings
- BuildingDamageMaterialHandler
- UISettings
- KeepKingVote
- WorldGenDebugSettings
- BuildingDataSettings
- settings.rs
- Player
- Target
- WorldGenRuntimeData
- NewKingVote
- SelectedEnemy
- UILineRenderer
- UserInterface_DisplayUsernames
- ResourceTarget
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- STSM_HelperBase
- WeatherSettings
- Key Rules
- SimpleDisableAfterTime
- Common Patterns
- .CalculateDayCount
- import_save
- TL_API
- FPSDisplay.cs
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
- WorldGenScaleSettings
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- BuildPlacerData
- Twitch setup
- graphify reference: add a URL and watch a folder
- SensorSettings
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
- ScriptableObjectAssetData
- AGENTS.md
- .RegisterSceneLoadHook
- .RefreshSceneBindingsAndTryGenerate
- PlayerSpawnPoint
- ResourceGenerationSettings
- CustomLogger
- TwitchCameraRequest
- extraction-spec.md
- InstantiationBarrier
- PersistentScoped
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- ChannelDataInstaller
- TechTreeNodeType.cs
- PoolablePlayer.cs
- SaveStateInstaller
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- UnitTravelToPosition
- .AddGoalFollowed
- EquipmentHandlerEditor
- .GetCompatiblePorts
- SimpleEventOnStart
- FloatWrapper
- .InjectRuntimeData
- .InjectRuntimeData
- NodeSaveData
- .SetGroupSelectionArea
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
- `GridNode` --references--> `CollisionType`  [EXTRACTED]
  Assets/Scripts/GridSystem/GridNode.cs → Assets/Scripts/GridSystem/Utils/Utils.cs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (278 total, 31 thin omitted)

### Community 0 - "world.rs"
Cohesion: 0.11
Nodes (29): WorldGenConfig, FoliageHabitat, FoliageLayerDef, FoliageVariantDef, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash() (+21 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (21): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller, Container, ContainerBuilder (+13 more)

### Community 2 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (49): ArchetypeBounds, ArchetypeDef, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+41 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "Character"
Cohesion: 0.07
Nodes (18): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+10 more)

### Community 5 - "EnemyModelHandler"
Cohesion: 0.07
Nodes (14): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, int, STSM_Helper_Attack, int (+6 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): float, Func, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string, TwitchChatRuntimeData, OnChatCommandReceivedArgs (+15 more)

### Community 7 - "TechTreeNode"
Cohesion: 0.07
Nodes (20): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+12 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Units"
Cohesion: 0.06
Nodes (9): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Animation, Sensors, STStateMachine, Pathfinding (+1 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (17): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+9 more)

### Community 12 - "save.rs"
Cohesion: 0.15
Nodes (31): detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native(), native_save_is_atomic_and_keeps_backup() (+23 more)

### Community 13 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 16 - "Utils"
Cohesion: 0.05
Nodes (7): BuildCostModifier, Utils, Level, Buildings, SavingAndLoading, SavingAndLoading.Structs, World.Generation

### Community 17 - "Processors"
Cohesion: 0.05
Nodes (10): ObjectSelectionProcessor, InputButton, Processors, World, Processors.Editor, MetaData, Audio, Settings (+2 more)

### Community 18 - ".GenerateFromSettings"
Cohesion: 0.11
Nodes (20): HashSet, Func, HashSet, List, Vector2, Vector3, GenerateDebugPositions(), GetPositiveNoiseOffset() (+12 more)

### Community 19 - "legacy.rs"
Cohesion: 0.15
Nodes (41): ActorCustomization, StreamUserType, binary_fixture(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal() (+33 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.10
Nodes (14): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, VisualElement, Button, EnumField, UnlockVisualElement (+6 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.07
Nodes (22): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, VisualElement (+14 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.05
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, List (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.09
Nodes (11): Func, Action, float, Enemy, Action, bool, float, int (+3 more)

### Community 25 - "STSM_Idle_Player"
Cohesion: 0.10
Nodes (8): int, STSM_Helper_Build, STSM_Action_GatherResource, bool, float, uint, Vector3, STSM_Idle_Player

### Community 26 - "Goal"
Cohesion: 0.12
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 27 - "ResMut"
Cohesion: 0.10
Nodes (45): AppExit, autosave_game(), broadcaster_gate_precedes_twitch_command_dispatch(), capture_screenshot(), credits_input(), CreditsTimeline, finish_boot(), fish_god_channel_reward_dispatches_praise_without_command_text() (+37 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 30 - "ContentCatalog"
Cohesion: 0.04
Nodes (147): GameConfig, ArchetypeKind, ContentCatalog, StationDef, GridPos, ActorState, RoleProgress, Default (+139 more)

### Community 31 - "convert"
Cohesion: 0.12
Nodes (29): ActorKind, SavedActor, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert() (+21 more)

### Community 32 - "STSM_Action_PlayerBase"
Cohesion: 0.09
Nodes (10): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_Heal (+2 more)

### Community 33 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 34 - "IInstaller"
Cohesion: 0.05
Nodes (28): ContainerBuilder, AllRoleDataSettingsInstaller, ContainerBuilder, GameSettingsInstaller, List, GameSettings, Dictionary, int (+20 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.06
Nodes (13): IMainThreadInitializableProcessor, Container, ContainerBuilder, DebugProcessor, bool, GameObject, IEnumerator, object (+5 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.11
Nodes (10): IReadOnlyList, List, Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary (+2 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.09
Nodes (14): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+6 more)

### Community 39 - "WorldInstanceDeterminism"
Cohesion: 0.24
Nodes (7): Material, Resource, int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - "DebugSettings"
Cohesion: 0.29
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 48 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.13
Nodes (11): bool, double, float, int, long, MenuItem, string, FrameCapture (+3 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.11
Nodes (17): Container, Dictionary, List, Material, materialIndex, materials, Matrix4x4, Mesh (+9 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "WeatherProcessor"
Cohesion: 0.19
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 57 - "StableId"
Cohesion: 0.05
Nodes (66): ObjectiveDef, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips() (+58 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 61 - "StateMachine"
Cohesion: 0.09
Nodes (12): PlayerDeathHandler, bool, float, Vector3, bool, List, string, uint (+4 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.14
Nodes (13): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+5 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.06
Nodes (20): int, ChangeTimeStamp, Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, Vector2 (+12 more)

### Community 67 - "twitch.rs"
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.15
Nodes (3): Action, int, Objective

### Community 69 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

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
Cohesion: 0.16
Nodes (12): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+4 more)

### Community 76 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 77 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 78 - "GridProcessor.cs"
Cohesion: 0.18
Nodes (6): GridProcessorEditor, Color, CollisionColours, CollisionType, GridSystem.Utils, GridSystem

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 81 - "WorldUtils"
Cohesion: 0.23
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
Nodes (192): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, AnyResult, AudioSink, active_event_text() (+184 more)

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
Cohesion: 0.05
Nodes (39): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+31 more)

### Community 90 - "GUIDProcessor"
Cohesion: 0.22
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 91 - "RoleHandler"
Cohesion: 0.03
Nodes (42): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleData (+34 more)

### Community 92 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 93 - "EditorUtils"
Cohesion: 0.22
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "command.rs"
Cohesion: 0.07
Nodes (53): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+45 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 97 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 98 - "Coordinator"
Cohesion: 0.11
Nodes (15): Coordinator, StartupState, bool, CancellationTokenSource, Container, Dictionary, GameObject, IEnumerable (+7 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "IProcessor.cs"
Cohesion: 0.22
Nodes (8): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport, ProcessorStartupStage

### Community 102 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.14
Nodes (8): LoadType, MetaData, Button, GameObject, IEnumerator, int, MainMenuManager, Inject

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "WorldGenSaveData"
Cohesion: 0.14
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 108 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 109 - "CustomLogHandler"
Cohesion: 0.17
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.15
Nodes (6): Transform, float, int, List, Transform, EnemySpawner

### Community 113 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - ".StartMusic"
Cohesion: 0.49
Nodes (3): SeasonAudioData, AudioClip, List

### Community 117 - "IRuntimeDataScriptable"
Cohesion: 0.11
Nodes (18): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, string (+10 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.03
Nodes (38): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+30 more)

### Community 120 - "ChanceObjectList"
Cohesion: 0.27
Nodes (5): float, ChanceObject, float, List, ChanceObjectList

### Community 121 - "Option"
Cohesion: 0.07
Nodes (126): AmbientLight, App, Assets, AssetServer, PresentationCatalog, actor_detail_budget(), actor_material(), actor_scene_budget() (+118 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "Targetable"
Cohesion: 0.05
Nodes (30): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+22 more)

### Community 126 - "BuildingBase"
Cohesion: 0.10
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 127 - "CommonEnums.cs"
Cohesion: 0.20
Nodes (9): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, Seasons, TimeOfDay, WallType (+1 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "Query"
Cohesion: 0.05
Nodes (99): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimationGraphHandle, BackgroundColor, ActorAnimationDriver, ActorNameOverlay, Agent (+91 more)

### Community 132 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 133 - "DayAndNightRuntimeData"
Cohesion: 0.28
Nodes (3): bool, float, DayAndNightRuntimeData

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "STSM_Idle"
Cohesion: 0.17
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 136 - "ToolState"
Cohesion: 0.09
Nodes (65): apply_technology_draft(), bounded_ui_index(), commit_catalog_candidate(), content_tab(), create_technology_group(), create_technology_node(), default_catalog_path(), delete_selected_technology_group() (+57 more)

### Community 137 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 139 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor, bool, Dictionary, InputButton (+3 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "ObjectPoolingProcessor"
Cohesion: 0.07
Nodes (20): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+12 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

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
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "UIElementWrapper"
Cohesion: 0.07
Nodes (15): GameObject, List, PresetButtons, Access_AOToggle, Access_ChannelNameInput, ContainerBuilder, Access_EdgeScrollingToggle, Access_MouseControlsToggle (+7 more)

### Community 154 - "Season"
Cohesion: 0.14
Nodes (13): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Color, float (+5 more)

### Community 155 - "UserInterface"
Cohesion: 0.07
Nodes (10): InputButton, SharedTypes, TownGoal.Data, StreamTown.EditorTools, TownGoal, UserInterface, TechTree.Data, TechTree.ScriptableObjects (+2 more)

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

### Community 164 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 165 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 167 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 169 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 170 - "BuildingConfigSettings"
Cohesion: 0.33
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 171 - "IProcessor"
Cohesion: 0.11
Nodes (9): Container, IProcessor, Action, Container, ContainerBuilder, EventProcessor, Container, ContainerBuilder (+1 more)

### Community 172 - "PlayerSaveData"
Cohesion: 0.08
Nodes (19): ResourceInventory, bool, int, Dictionary, bool, int, List, string (+11 more)

### Community 173 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 174 - "SelectedBuilding"
Cohesion: 0.12
Nodes (4): bool, float, PassiveResourceIncrementer, SelectedBuilding

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "GameEventConfigSettingsInstaller"
Cohesion: 0.33
Nodes (4): ContainerBuilder, GameEventConfigSettingsInstaller, bool, GameEventConfigSettings

### Community 177 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.15
Nodes (5): CancellationToken, Task, IPostInitializeProcessor, Dictionary, ParallelProgressReporter

### Community 178 - ".CapturePlayers"
Cohesion: 0.09
Nodes (14): Component, List, Mesh, Transform, Vector3, bool, int, MeshSaveData (+6 more)

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
Cohesion: 0.08
Nodes (25): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+17 more)

### Community 183 - "TimeSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, TimeDataSettingsInstaller, int, TimeSettings

### Community 184 - "ObjectPoolingSettings"
Cohesion: 0.33
Nodes (5): bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller

### Community 185 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 187 - "UISettings"
Cohesion: 0.33
Nodes (4): float, UISettings, ContainerBuilder, UISettingsInstaller

### Community 189 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

### Community 190 - "BuildingDataSettings"
Cohesion: 0.12
Nodes (14): ContainerBuilder, AllBuildingDataSettingsInstaller, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingDataContainerInstaller, int, ResourceCostData (+6 more)

### Community 191 - "settings.rs"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 192 - "Player"
Cohesion: 0.05
Nodes (14): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, ModeratorCommands (+6 more)

### Community 193 - "Target"
Cohesion: 0.12
Nodes (11): UserInterface.MainMenu, Target, Utils.Pooling, Pets, GridSystem.Partitioning, Combat, Environment, SavingAndLoading.SavableObjects (+3 more)

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

### Community 199 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 204 - "WeatherSettings"
Cohesion: 0.33
Nodes (4): VisualEffect, WeatherSettings, ContainerBuilder, WeatherSettingsInstaller

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - ".CalculateDayCount"
Cohesion: 0.40
Nodes (3): float, int, TimeRuntimeData

### Community 209 - "import_save"
Cohesion: 0.53
Nodes (6): absolute_path(), backup_candidate(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "ResourceDataSettings"
Cohesion: 0.13
Nodes (10): ContainerBuilder, ResourceDataSettingsInstaller, ContainerBuilder, WorldGenLayerSettingsInstaller, LayerMask, WorldGenLayerSettings, bool, int (+2 more)

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

### Community 224 - "WorldGenScaleSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenScaleSettingsInstaller, float, WorldGenScaleSettings

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
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "SensorSettings"
Cohesion: 0.33
Nodes (4): float, SensorSettings, ContainerBuilder, SensorSettingsInstaller

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
Cohesion: 0.07
Nodes (26): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+18 more)

### Community 244 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 249 - "ResourceGenerationSettings"
Cohesion: 0.12
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 251 - "TwitchCameraRequest"
Cohesion: 0.40
Nodes (4): bool, int, Vector3, TwitchCameraRequest

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 263 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 264 - ".GetCompatiblePorts"
Cohesion: 0.50
Nodes (3): List, Port, NodeAdapter

### Community 270 - "NodeSaveData"
Cohesion: 0.67
Nodes (3): List, Vector2, NodeSaveData

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 279 - "MonoBehaviour"
Cohesion: 0.04
Nodes (28): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, MetaDataInstaller, ContainerBuilder, TargetSettingsInstaller, Slider (+20 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (51): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, ContainerBuilder, WorldGenBehaviorSettingsInstaller, float, Material (+43 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **31 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `UserInterface_GameMenu`, `ObjectPoolingProcessor`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `IInstaller`, `UserInterface_Debug`, `TechTreeProcessor`, `IProcessor`, `SeasonProcessor`, `.CapturePlayers`, `StreamTownSessionBridge`, `ResourceProcessor`, `Player`, `Target`, `FoliageProcessor`, `GUIDProcessor`, `RoleHandler`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `WorldGenSaveData`, `Resource`, `ResourceGenerationSettings`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `ObjectPoolingProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `IInstaller`, `UserInterface_Debug`, `WorldInstanceDeterminism`, `CampGenerationSettings`, `IProcessor`, `GameStateProcessor`, `TerrainGenSettings`, `ResourceProcessor`, `TwitchClientProcessor`, `WorldGenDebugSettings`, `Player`, `Target`, `WorldGenRuntimeData`, `Access_Dropdown`, `ResourceDataSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `GUIDProcessor`, `RaidEvent`, `WorldGenScaleSettings`, `Coordinator`, `EnemySpawner`, `SaveProcessor`, `ResourceGenerationSettings`, `AIPath`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `Character`, `TwitchChatProcessor`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `IInstaller`, `UserInterface_Debug`, `CommandDictionary`, `CharacterModelHandler`, `LabelDisplayProcessor`, `PoolableObject`, `Target`, `UserInterface_DisplayUsernames`, `TargetSensor`, `RoleHandler`, `RaidEvent`, `SaveProcessor`, `VoteEvent`, `.SetTargetType`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11397849462365592 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05807622504537205 - nodes in this community are weakly interconnected._
- **Should `stream_town_domain/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08484848484848485 - nodes in this community are weakly interconnected._