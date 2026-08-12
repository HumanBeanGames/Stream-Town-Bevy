# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 607 files · ~1,590,287 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6797 nodes · 17558 edges · 256 communities (237 shown, 19 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 993 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7c2fd929`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- MonoBehaviour
- BuildingProcessor
- stream_town_domain/src/content.rs
- stream_town_migrate/src/presentation.rs
- Target
- Utils
- TwitchChatProcessor
- TechTreeNode
- BottomBarInterface
- stream_town_game/src/lib.rs
- SettingsProcessor
- WorldGenProcessor
- Player
- PlayerRole
- TechTreeIOUtility
- String
- Character
- Result
- Processors
- BinarySaveCodec
- Targetable
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- PlayerRoleData
- PoolableObject
- Access_Dropdown
- SaveFileData
- GameEventProcessor
- STSM_Idle_Player
- BuildingResourceModelHandler
- RoleDataSettings
- .GenerateFromSettings
- Station
- UserInterface_Debug
- ContentCatalog
- SettingsData
- AnimationHandler
- .Log
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BinaryReader
- AnimationControllerDef
- legacy.rs
- SeasonProcessor
- DebugProcessor
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceData
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
- run_transport
- Objective
- PlayerCommands
- GenerationSettings
- models.rs
- Tiler
- ScriptablesEditor
- BTreeMap
- UserInterface_ObjectSelection
- UserInterface_TownGoal
- ProjectCamera
- GridProcessor
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- Goal
- MiscCommands
- Access_Text
- TreeMaterialExtension
- CellSpacePartitioning
- UserInterface_TownVote
- CommonEnums.cs
- FoliageProcessor
- Option
- RoleData
- RaidEvent
- IRuntimeDataScriptable
- convert_fbx_to_glb.py
- KeepKingVote
- stream_town_migrate/src/content.rs
- RoleHandler
- Coordinator
- stream_town_domain/src/presentation.rs
- Globals
- SelectableObject
- BuildPlacerData
- TownGoalProcessor
- MainMenuManager
- UnitHealthBar
- LoadingManager
- CampGenerationSettings
- ResourceRuntimeData
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- EditorUtils
- EnemyModelHandler
- DontDestroyOnLoad
- VoteEvent
- Resource
- DebugSettings
- generate_and_spawn_world
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Enemy
- BuildingBase
- CommandDictionary
- UpdateGraphBounds
- IProcessor
- stream_town_migrate/src/main.rs
- .CreateEnumField
- NodeUnlockData
- DayAndNightSettings
- ConfirmCheck
- config.rs
- ToolState
- LabelDisplayProcessor
- SensorProcessor
- PlayerInventory
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- PlayerSaveData
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- UI_TechOption
- Requirement
- xtask/src/main.rs
- Stream Town Reloaded - Architecture Documentation
- ResourceTarget
- Access_GOList
- UserInterface
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- WindController
- GridProcessor.cs
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- FoliageGenerationSettings
- .RenderResourceType
- TechTreeSearchWindow
- WeatherProcessor
- GridSettings
- EventProcessor
- TimeProcessor
- ResourceGenerationSettings
- FoliageGroupSaveData
- SelectedBuilding
- EditorHelpers
- ResourceDataSettings
- Access_TextInput
- .Update
- GameEventSettings
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ResourceHolder
- WorldGenRuntimeData
- StringUtils
- SensorSettings
- GameStateProcessor
- TownGoalSettings
- TargetSettings
- PassiveResourceIncrementer
- BuildingDataSettings
- command.rs
- RotationHandler
- Sensors
- ObjectiveSaveData
- SimpleDisableAfterTime
- WorldGenDebugSettings
- UILineRenderer
- UserInterface_DisplayUsernames
- NewKingVote
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- .InjectRuntimeData
- Key Rules
- SelectedEnemy
- Common Patterns
- WorldGenLayerSettings
- VfxParticlePosition
- FPSDisplay.cs
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- ObjectSelectionProcessor.Editor.cs
- ScriptKeywordProcessor
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
- Autosave
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- AGENTS.md
- setup_camera
- Access_Toggle
- twitch.rs
- CustomLogger
- extraction-spec.md
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- TargetSensor
- Editor
- TL_API
- ResourceProcessor
- ScriptablesProcessorInfrastructure
- IntWrapper
- IInstaller
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- AutosaveIntervalsInstaller
- SaveDataMapper

## God Nodes (most connected - your core abstractions)
1. `StableId` - 244 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldSimulation` - 120 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `SaveProcessor` - 88 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `combat_goal_damages_kills_and_respawns()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (256 total, 19 thin omitted)

### Community 0 - "MonoBehaviour"
Cohesion: 0.02
Nodes (59): CameraProcessor, PersistentScoped, UnitTravelToPosition, Vector3, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI (+51 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "stream_town_domain/src/content.rs"
Cohesion: 0.13
Nodes (30): AuthoredRecord, AuthoredValue, BuildingDef, ContentError, EnemyDef, EnemySpawnerDef, FoliageHabitat, FoliageLayerDef (+22 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "Target"
Cohesion: 0.10
Nodes (8): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, SavingAndLoading.SavableObjects, Enemies, GUIDSystem

### Community 5 - "Utils"
Cohesion: 0.05
Nodes (7): RoleScriptablesEditor, Utils, ScriptablesEditor, SavingAndLoading, SavingAndLoading.Structs, GameResources, World.Generation

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "TechTreeNode"
Cohesion: 0.06
Nodes (24): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+16 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "stream_town_game/src/lib.rs"
Cohesion: 0.03
Nodes (133): AnyResult, App, BackgroundColor, generate_world(), ActionPresentation, ActivePetVisual, actor_combat_visual(), AgentCommand (+125 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 12 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+2 more)

### Community 13 - "PlayerRole"
Cohesion: 0.07
Nodes (15): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+7 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "String"
Cohesion: 0.05
Nodes (73): AnimationClip, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, active_event_text(), ActorAnimationDriver (+65 more)

### Community 16 - "Character"
Cohesion: 0.08
Nodes (19): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+11 more)

### Community 17 - "Result"
Cohesion: 0.18
Nodes (14): DeviceAuthorization, OAuthClient, Client, Formatter, Into, Result, String, Vec (+6 more)

### Community 18 - "Processors"
Cohesion: 0.05
Nodes (14): BuildCostModifier, InputButton, UserInterface.MainMenu, Processors, World, Level, MetaData, Buildings (+6 more)

### Community 19 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 20 - "Targetable"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.05
Nodes (28): int, ChangeTimeStamp, Color, ErrorData, List, GroupErrorData, List, NodeErrorData (+20 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (20): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+12 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.06
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.07
Nodes (15): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, int, STSM_Helper_Attack, STSM_Action_Heal, Action (+7 more)

### Community 25 - "PlayerRoleData"
Cohesion: 0.09
Nodes (11): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+3 more)

### Community 26 - "PoolableObject"
Cohesion: 0.09
Nodes (18): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, Dictionary, float (+10 more)

### Community 27 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "STSM_Idle_Player"
Cohesion: 0.10
Nodes (10): bool, float, Vector3, STSM_Idle_Enemy, bool, float, uint, Vector3 (+2 more)

### Community 31 - "BuildingResourceModelHandler"
Cohesion: 0.09
Nodes (13): BuildingResourceModelHandler, GameObject, BuildingResourceModelHandlerEditor, float, int, Queue, ResourceRateOfChange, UnityEvent (+5 more)

### Community 32 - "RoleDataSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 33 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+9 more)

### Community 34 - "Station"
Cohesion: 0.06
Nodes (25): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+17 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "ContentCatalog"
Cohesion: 0.06
Nodes (94): GameConfig, GameplayConfig, BTreeMap, Default, WorldGenConfig, ContentCatalog, DirtyRegion, GridPos (+86 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.07
Nodes (17): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+9 more)

### Community 39 - ".Log"
Cohesion: 0.07
Nodes (23): HideInCallstack, Object, DebugLogCategory, Action, bool, BoxCollider, CancellationToken, Container (+15 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "BinaryReader"
Cohesion: 0.15
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "legacy.rs"
Cohesion: 0.05
Nodes (110): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+102 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.07
Nodes (21): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+13 more)

### Community 47 - "DebugProcessor"
Cohesion: 0.08
Nodes (9): Container, ContainerBuilder, DebugProcessor, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack (+1 more)

### Community 48 - "NavGrid"
Cohesion: 0.13
Nodes (18): can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, OpenNode, path_routes_around_dynamic_building(), reconstruct_path() (+10 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "ResourceData"
Cohesion: 0.18
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, float, UISettings, ContainerBuilder (+6 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 57 - "StableId"
Cohesion: 0.05
Nodes (79): ObjectiveDef, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips() (+71 more)

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
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

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
Cohesion: 0.12
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "run_transport"
Cohesion: 0.29
Nodes (7): BTreeSet, TwitchConfig, CredentialVault, Self, Sender, run_transport(), UnboundedReceiver

### Community 68 - "Objective"
Cohesion: 0.13
Nodes (4): Action, int, Objective, EnemyType

### Community 69 - "PlayerCommands"
Cohesion: 0.13
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

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

### Community 76 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 83 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "TreeMaterialExtension"
Cohesion: 0.16
Nodes (15): BuildingMaterialExtension, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform, Image, TerrainMaterialExtension, TerrainMaterialUniform, TreeMaterialExtension (+7 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (13): GameObject, int, EnemyWeaponModel, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+5 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "Option"
Cohesion: 0.12
Nodes (47): AssetServer, ArchetypeDef, ArchetypeKind, ArchetypeScene, PresentationCatalog, actor_detail_budget(), animation_property_value(), archetype_by_source() (+39 more)

### Community 91 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 92 - "RaidEvent"
Cohesion: 0.08
Nodes (15): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+7 more)

### Community 93 - "IRuntimeDataScriptable"
Cohesion: 0.11
Nodes (17): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+9 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (101): ArchetypesById, ArchetypeBounds, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset() (+93 more)

### Community 97 - "RoleHandler"
Cohesion: 0.11
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 101 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 102 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

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

### Community 107 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

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
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "SelectedObject"
Cohesion: 0.10
Nodes (6): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 116 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 117 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 118 - "VoteEvent"
Cohesion: 0.18
Nodes (10): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+2 more)

### Community 119 - "Resource"
Cohesion: 0.05
Nodes (21): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, Container (+13 more)

### Community 120 - "DebugSettings"
Cohesion: 0.29
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 121 - "generate_and_spawn_world"
Cohesion: 0.06
Nodes (151): Added, AmbientLight, AppExit, Assets, actor_material(), actor_scene_budget(), Agent, agent_is_moving() (+143 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "Enemy"
Cohesion: 0.11
Nodes (14): Action, float, Enemy, uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp (+6 more)

### Community 126 - "BuildingBase"
Cohesion: 0.09
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 127 - "CommandDictionary"
Cohesion: 0.27
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "IProcessor"
Cohesion: 0.09
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 133 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

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
Cohesion: 0.12
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 139 - "PlayerInventory"
Cohesion: 0.14
Nodes (6): PlayerInventory, Dictionary, ResourceInventory, bool, int, Dictionary

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
Cohesion: 0.11
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

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

### Community 153 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 154 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 155 - "UserInterface"
Cohesion: 0.07
Nodes (10): InputButton, SharedTypes, TownGoal.Data, StreamTown.EditorTools, TownGoal, UserInterface, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.11
Nodes (17): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+9 more)

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

### Community 165 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 166 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 167 - "TechTreeSearchWindow"
Cohesion: 0.32
Nodes (6): List, Texture2D, TechTreeSearchWindow, ISearchWindowProvider, SearchTreeEntry, SearchWindowContext

### Community 168 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 169 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 170 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - "ResourceGenerationSettings"
Cohesion: 0.12
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 173 - "FoliageGroupSaveData"
Cohesion: 0.38
Nodes (6): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "ResourceDataSettings"
Cohesion: 0.13
Nodes (11): ContainerBuilder, ResourceDataSettingsInstaller, Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, bool (+3 more)

### Community 177 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 178 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 179 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ResourceHolder"
Cohesion: 0.13
Nodes (7): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, SelectedResource

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 185 - "SensorSettings"
Cohesion: 0.33
Nodes (4): float, SensorSettings, ContainerBuilder, SensorSettingsInstaller

### Community 186 - "GameStateProcessor"
Cohesion: 0.15
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 187 - "TownGoalSettings"
Cohesion: 0.33
Nodes (4): int, TownGoalSettings, ContainerBuilder, TownGoalSettingsInstaller

### Community 188 - "TargetSettings"
Cohesion: 0.50
Nodes (3): TargetSettings, ContainerBuilder, TargetSettingsInstaller

### Community 189 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 190 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 191 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 192 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 193 - "Sensors"
Cohesion: 0.07
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 194 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 195 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 196 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

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
Cohesion: 0.18
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "WorldGenLayerSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenLayerSettingsInstaller, LayerMask, WorldGenLayerSettings

### Community 209 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

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

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 225 - "WorldSaveData"
Cohesion: 0.13
Nodes (17): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+9 more)

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
Nodes (22): Action, CancellationToken, Component, Container, float, List, Material, materials (+14 more)

### Community 248 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 249 - "twitch.rs"
Cohesion: 0.14
Nodes (16): channel_point_reward_tag_survives_privmsg_conversion(), envelope_from_privmsg(), OAuthErrorResponse, Arc, Mutex, Option, Receiver, token_response_keeps_rotated_refresh_token() (+8 more)

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 263 - "Editor"
Cohesion: 0.18
Nodes (6): GameObject, List, EquipmentHandlerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 275 - "ResourceProcessor"
Cohesion: 0.14
Nodes (3): Container, ContainerBuilder, ResourceProcessor

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 277 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 279 - "IInstaller"
Cohesion: 0.03
Nodes (39): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, Volume (+31 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (67): ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller (+59 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 285 - "SaveDataMapper"
Cohesion: 0.07
Nodes (23): Mesh, Transform, Vector3, SaveDataMapper, int, List, string, uint (+15 more)

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **19 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `MonoBehaviour`, `IProcessor`, `BuildingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `PlayerRole`, `Character`, `ResourceProcessor`, `PlayerProcessor`, `ScriptableObject`, `IInstaller`, `PoolableObject`, `ResourceDataSaveData`, `SaveDataMapper`, `GameEventProcessor`, `SaveFileData`, `.Log`, `TechTreeProcessor`, `TimeProcessor`, `ResourceGenerationSettings`, `SeasonProcessor`, `DebugProcessor`, `StreamTownSessionBridge`, `PlayerCommands`, `.InjectRuntimeData`, `FoliageProcessor`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `Resource`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `MonoBehaviour`, `IProcessor`, `BuildingProcessor`, `Target`, `TwitchChatProcessor`, `Player`, `ResourceProcessor`, `PlayerProcessor`, `ScriptableObject`, `IInstaller`, `PoolableObject`, `ResourceDataSaveData`, `.GenerateFromSettings`, `UserInterface_Debug`, `.Log`, `ResourceGenerationSettings`, `DebugProcessor`, `TwitchClientProcessor`, `WorldGenRuntimeData`, `GameStateProcessor`, `WorldGenDebugSettings`, `ProjectCamera`, `GridProcessor`, `WorldGenLayerSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `Coordinator`, `CampGenerationSettings`, `EnemySpawner`, `SaveProcessor`, `AIPath`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `Access_Slider`, `GraphicsProcessor`, `IProcessor`, `MonoBehaviour`, `SettingsData`, `MainMenuManager`, `ProjectCamera`, `Autosave`, `Access_TextInput`, `SaveProcessor`, `ScriptablesProcessorInfrastructure`, `Access_Text`, `IInstaller`, `Access_Toggle`, `Access_GOList`, `Access_Dropdown`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MonoBehaviour` be split into smaller, more focused modules?**
  _Cohesion score 0.023335621139327384 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.061016949152542375 - nodes in this community are weakly interconnected._
- **Should `stream_town_domain/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13445378151260504 - nodes in this community are weakly interconnected._