# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 624 files · ~1,622,916 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7296 nodes · 19425 edges · 271 communities (245 shown, 26 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1003 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `69117fff`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- drive_converted_animations
- BuildingProcessor
- Res
- stream_town_migrate/src/presentation.rs
- stream_town_domain/src/content.rs
- TownGoal.Data
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- BinaryReader
- SettingsProcessor
- WorldGenProcessor
- PoolableObject
- StreamUserType
- TechTreeIOUtility
- GameMasterCommands
- STSM_StateAction
- save.rs
- StableId
- Option
- .CreateEnumField
- Age
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- .RecalculateStats
- Handle
- RoleDataSettings
- SaveFileData
- GameEventProcessor
- stream_town_game/src/lib.rs
- Station
- TechTreeNode
- RoleHandler
- BuildingResourceModelHandler
- UserInterface_Debug
- CommandDictionary
- SettingsData
- GlobalAudioController
- WorldSimulation
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- Editor
- AnimationControllerDef
- BTreeMap
- IRuntimeDataScriptable
- legacy.rs
- world.rs
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- DebugProcessor
- BuildingBase
- STSM_Idle_Player
- TechTreeEditorWindow
- ResourceHolder
- BinarySaveCodec
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- GameEvent
- CommonEnums.cs
- models.rs
- Tiler
- ScriptablesEditor
- StringUtils
- UserInterface_ObjectSelection
- TimeProcessor
- ProjectCamera
- AnimationHandler
- TwitchBotSetupWindow
- SeasonProcessor
- WorldUtils
- SelectedBuilding
- DayAndNightSettings
- Access_Text
- .GenerateFromSettings
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- runtime_console.rs
- GUIDProcessor
- UserInterface_GameMenu
- STSM_Action_GatherResource
- convert_fbx_to_glb.py
- command.rs
- stream_town_migrate/src/content.rs
- WeatherProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- CellSpacePartitioningInstaller
- StateMachine
- ResourceDataSaveData
- TownGoalProcessor
- MainMenuManager
- UnitHealthBar
- LoadingManager
- WorldSaveData
- BuildingDataSettings
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedPlayer
- Result
- Targetable
- UI_TechOption
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- .new
- RaidEvent
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Access_Toggle
- Target
- .OnGUI
- UpdateGraphBounds
- World.Generation
- WindController
- UserInterface.MainMenu
- RoleData
- EventProcessor
- ConfirmCheck
- TwitchUser
- ToolState
- GateController
- technology_tab
- SelectedObject
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- .Log
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- DontDestroyOnLoad
- Requirement
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- Access_Dropdown
- IProcessor.cs
- SelectableObject
- ResourceStorageModifier
- Stream Town Reloaded - Architecture Documentation
- Audio
- BuildingSettings
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GameStateProcessor
- MiscCommands
- .InjectRuntimeData
- GridProcessor
- PassiveResourceIncrementer
- Easings
- String
- ResourceRuntimeData
- Access_GOList
- UserInterface_RulerVote
- PlayerInventory
- EditorHelpers
- GridProcessor.cs
- SelectedEnemy
- SensorProcessor
- LabelDisplayProcessor
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ObjectPoolingRuntimeData
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- HealthModifier
- SelectedResource
- TransformSaveData
- .Update
- Access_TextInput
- TL_API
- BuildPlacerData
- settings.rs
- Player
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SimpleDisableAfterTime
- MeshData
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- VfxAnimationController
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- IProcessor
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- EquipmentHandlerEditor
- Common Patterns
- .RenderResourceType
- FPSDisplay.cs
- ObjectSelectionProcessor.Editor.cs
- SimpleScreenShot
- VFX
- ScriptablesProcessorInfrastructure
- Key Rules
- MonoBehaviour
- RuntimeData Template
- ResourceTarget
- VfxParticlePosition
- ScriptKeywordProcessor
- FoliageGenerationSettings.cs
- Processor Template
- Common Patterns
- UserInterface_GameMenuInstaller
- SelectedEnemyCamp
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- KeepKingVote
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- WorldGenSaveData
- CreateProjectScopeProcessors.cs
- SelectedPlayerGroup
- setup_camera
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- NewKingVote
- extraction-spec.md
- PlayerSaveData
- RandomEnabler
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- UnityGraphics
- Q: If there is more to do, keep going.
- SaveDataMapper
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Utils
- IInstaller
- Q: How does Bevy new-town population now match Unity shipping startup?
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- Autosave

## God Nodes (most connected - your core abstractions)
1. `StableId` - 268 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `WorldSimulation` - 143 edges
6. `Player` - 142 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `ContentCatalog` - 99 edges

## Surprising Connections (you probably didn't know these)
- `CreditsRuntimeData` --implements--> `IRuntimeDataScriptable`  [EXTRACTED]
  Assets/Scripts/Scriptables/CreditsRuntimeData.cs → Assets/Scripts/Scriptables/IRuntimeDataScriptable.cs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `embedded_config_matches_shipping_starting_roster()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `native_world_compatibility_accepts_current_and_verified_upgrades_only()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (271 total, 26 thin omitted)

### Community 0 - "drive_converted_animations"
Cohesion: 0.05
Nodes (77): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+69 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (15): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, Dictionary, int, List (+7 more)

### Community 2 - "Res"
Cohesion: 0.05
Nodes (162): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, AnimationGraphHandle, App, AppExit, Assets (+154 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (33): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingModelDef, ContentError, EnemyDef, EnemySpawnerDef, FoliageHabitat (+25 more)

### Community 5 - "TownGoal.Data"
Cohesion: 0.10
Nodes (8): InputButton, SharedTypes, bool, GameStateRuntimeData, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects, Data

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "BinaryReader"
Cohesion: 0.15
Nodes (3): Func, List, BinaryReader

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 12 - "PoolableObject"
Cohesion: 0.08
Nodes (22): Action, float, Enemy, uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp (+14 more)

### Community 13 - "StreamUserType"
Cohesion: 0.33
Nodes (5): StreamUserType, building_health_color(), Color, should_show_actor_name(), stream_user_color()

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "GameMasterCommands"
Cohesion: 0.11
Nodes (5): List, GameSettings, GameMasterCommands, RulerCommands, Vector3

### Community 16 - "STSM_StateAction"
Cohesion: 0.06
Nodes (18): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, int, STSM_Helper_Attack, int (+10 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "StableId"
Cohesion: 0.04
Nodes (65): GameplayConfig, BTreeMap, ObjectiveDef, ObjectiveKind, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds() (+57 more)

### Community 19 - "Option"
Cohesion: 0.06
Nodes (131): AssetServer, ArchetypeDef, ArchetypeKind, ArchetypeScene, PresentationCatalog, actor_detail_budget(), actor_scene_budget(), animated_pets_resolve_their_own_unity_controllers_and_rigs() (+123 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.14
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 21 - "Age"
Cohesion: 0.05
Nodes (30): int, ChangeTimeStamp, Color, ErrorData, List, GroupErrorData, List, NodeErrorData (+22 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.09
Nodes (10): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, ModeratorCommands (+2 more)

### Community 24 - "HealthHandler"
Cohesion: 0.07
Nodes (13): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, STSM_Action_Heal, Action, bool (+5 more)

### Community 25 - ".RecalculateStats"
Cohesion: 0.17
Nodes (8): StatModifiers, Dictionary, Dictionary, List, Queue, Transform, PlayerRuntimeData, StatType

### Community 26 - "Handle"
Cohesion: 0.06
Nodes (46): ActorCustomization, actor_material(), BoundsMaterialExtension, BoundsMaterialUniform, building_effect_material(), BuildingEffectKind, BuildingEffectParticle, BuildingMaterialExtension (+38 more)

### Community 27 - "RoleDataSettings"
Cohesion: 0.08
Nodes (16): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AudioClip, bool (+8 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (205): AnyResult, active_event_text(), adjust_settings_menu(), AgentCommand, AgentCommandQueue, AgentEquipmentPresentation, AmbienceAudio, animation_root_name() (+197 more)

### Community 31 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 32 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 33 - "RoleHandler"
Cohesion: 0.06
Nodes (18): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleHandler (+10 more)

### Community 34 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.23
Nodes (7): IReadOnlyList, List, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 39 - "WorldSimulation"
Cohesion: 0.04
Nodes (161): GameConfig, BuildingDef, ContentCatalog, RoleDef, BTreeSet, StationDef, GridPos, ActorState (+153 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.05
Nodes (19): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+11 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 46 - "IRuntimeDataScriptable"
Cohesion: 0.08
Nodes (23): Queue, AudioRuntimeData, UnityEvent, DebugRuntimeData, bool, float, ParticleSystem, SortedSet (+15 more)

### Community 47 - "legacy.rs"
Cohesion: 0.16
Nodes (43): binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, checked_in_schema_one_fixture_imports_retained_terrain(), decode_binary(), decode_json(), decode_legacy(), json_active_goal() (+35 more)

### Community 48 - "world.rs"
Cohesion: 0.06
Nodes (57): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+49 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (11): bool, double, float, int, long, MenuItem, string, FrameCapture (+3 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.12
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

### Community 56 - "DebugProcessor"
Cohesion: 0.13
Nodes (9): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, DebugLogCategory, DebugProcessor (+1 more)

### Community 57 - "BuildingBase"
Cohesion: 0.10
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 58 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (21): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+13 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (7): bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 61 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (6): Action, CancellationToken, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.13
Nodes (12): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+4 more)

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
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.09
Nodes (10): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 69 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 70 - "CommonEnums.cs"
Cohesion: 0.14
Nodes (13): Dictionary, List, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, ReviveType (+5 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (11): bool, Color, Dictionary, int, MenuItem, ScriptableObject, string, Vector2 (+3 more)

### Community 74 - "StringUtils"
Cohesion: 0.14
Nodes (4): RoleScriptablesEditor, Utils, StringUtils, ScriptablesEditor

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "TimeProcessor"
Cohesion: 0.16
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.09
Nodes (14): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+6 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "SeasonProcessor"
Cohesion: 0.07
Nodes (21): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 83 - "DayAndNightSettings"
Cohesion: 0.10
Nodes (12): ContainerBuilder, AllRoleDataSettingsInstaller, float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller (+4 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - ".GenerateFromSettings"
Cohesion: 0.08
Nodes (27): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+19 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.09
Nodes (12): RotationHandler, float, Quaternion, Vector3, ProjectileShooter, float, int, string (+4 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 91 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 93 - "STSM_Action_GatherResource"
Cohesion: 0.11
Nodes (5): AttackUnit, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "command.rs"
Cohesion: 0.07
Nodes (51): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+43 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 97 - "WeatherProcessor"
Cohesion: 0.17
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

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
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 108 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.07
Nodes (20): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+12 more)

### Community 114 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 115 - "Targetable"
Cohesion: 0.07
Nodes (19): List, Vector3, Container, ContainerBuilder, List, TargetProcessor, Dictionary, List (+11 more)

### Community 116 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.07
Nodes (12): DepositResources, int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor (+4 more)

### Community 120 - ".new"
Cohesion: 0.13
Nodes (23): absolute_path(), backup_candidate(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), conversion_rejects_malformed_retained_mesh(), convert(), duration_days() (+15 more)

### Community 121 - "RaidEvent"
Cohesion: 0.07
Nodes (19): Transform, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+11 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.15
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 126 - "Target"
Cohesion: 0.06
Nodes (16): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors (+8 more)

### Community 127 - ".OnGUI"
Cohesion: 0.13
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "World.Generation"
Cohesion: 0.03
Nodes (42): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller (+34 more)

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "UserInterface.MainMenu"
Cohesion: 0.13
Nodes (6): CameraProcessor, ContainerBuilder, MetaDataInstaller, UserInterface.MainMenu, MetaData, Settings

### Community 132 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 133 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (46): bounded_ui_index(), content_tab(), default_catalog_path(), draw_world_preview(), inject_runtime_command(), inspector_tab(), launch_runtime_game(), main() (+38 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "technology_tab"
Cohesion: 0.31
Nodes (16): apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node(), refresh_technology_draft(), Option (+8 more)

### Community 139 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - ".Log"
Cohesion: 0.07
Nodes (22): HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder (+14 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "DayAndNightProcessor"
Cohesion: 0.14
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

### Community 150 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 154 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 155 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 156 - "ResourceStorageModifier"
Cohesion: 0.27
Nodes (3): ResourceStorageModifier, float, int

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - "BuildingSettings"
Cohesion: 0.20
Nodes (6): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller

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

### Community 164 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 165 - "MiscCommands"
Cohesion: 0.16
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 167 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 170 - "String"
Cohesion: 0.23
Nodes (13): ActorKind, actor_prefix(), entity_id(), json_pet_name(), json_role_name(), legacy_pet_name(), legacy_role_name(), LegacyGoal (+5 more)

### Community 171 - "ResourceRuntimeData"
Cohesion: 0.29
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 172 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 178 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 179 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 186 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 187 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 188 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 190 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 191 - "settings.rs"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 192 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, OnChatCommandReceivedArgs, TwitchClientProcessor (+2 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 195 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 209 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 211 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 213 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (9): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, BuildingScriptablesEditor, CreditsRuntimeData, Reflex.Core, Data.Containers (+1 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "MonoBehaviour"
Cohesion: 0.03
Nodes (52): PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI, UI_VoteObjectiveRow (+44 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "ResourceTarget"
Cohesion: 0.26
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 218 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed, Source Nodes

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "Q: shader material giraffe pet skinning prefab reachable shipping presentation"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: shader material giraffe pet skinning prefab reachable shipping presentation, Source Nodes

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "WorldGenSaveData"
Cohesion: 0.14
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 243 - "SaveProcessor"
Cohesion: 0.07
Nodes (25): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+17 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 253 - "PlayerSaveData"
Cohesion: 0.10
Nodes (17): ResourceInventory, bool, int, Dictionary, bool, int, List, string (+9 more)

### Community 254 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "SaveDataMapper"
Cohesion: 0.08
Nodes (17): Component, List, Mesh, Transform, Vector3, SaveDataMapper, bool, int (+9 more)

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "Utils"
Cohesion: 0.05
Nodes (24): BuildCostModifier, InputButton, Utils, Processors, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Character.Enumerations (+16 more)

### Community 276 - "IInstaller"
Cohesion: 0.04
Nodes (32): ContainerBuilder, InstantiationBarrier, ContainerBuilder, Volume, PostProcessingInstaller, AudioMixerInstaller, AudioMixer, ContainerBuilder (+24 more)

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 280 - "ScriptableObject"
Cohesion: 0.02
Nodes (101): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller (+93 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

## Knowledge Gaps
- **270 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+265 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `World.Generation`, `BuildingProcessor`, `SettingsProcessor`, `SaveDataMapper`, `WorldGenProcessor`, `GameMasterCommands`, `.Log`, `Utils`, `IInstaller`, `PlayerProcessor`, `SaveFileData`, `GameEventProcessor`, `RoleHandler`, `TechTreeProcessor`, `StreamTownSessionBridge`, `ResourceProcessor`, `DebugProcessor`, `IProcessor`, `TimeProcessor`, `SeasonProcessor`, `MonoBehaviour`, `FoliageProcessor`, `GUIDProcessor`, `UserInterface_GameMenu`, `ResourceDataSaveData`, `TownGoalProcessor`, `MainMenuManager`, `WorldSaveData`, `WorldGenSaveData`, `Resource`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `World.Generation`, `BuildingProcessor`, `TwitchChatProcessor`, `SaveDataMapper`, `.Log`, `GameMasterCommands`, `IInstaller`, `PlayerProcessor`, `ScriptableObject`, `UserInterface_Debug`, `GameStateProcessor`, `GridProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `DebugProcessor`, `IProcessor`, `ProjectCamera`, `.GenerateFromSettings`, `CellSpacePartitioning`, `MonoBehaviour`, `FoliageProcessor`, `GUIDProcessor`, `Coordinator`, `EnemySpawner`, `SaveProcessor`, `RaidEvent`, `AIPath`, `Target`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `RoleData`, `TwitchChatProcessor`, `TwitchUser`, `PoolableObject`, `GameMasterCommands`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `.RecalculateStats`, `GameEventProcessor`, `Station`, `RoleHandler`, `UserInterface_Debug`, `CommandDictionary`, `MiscCommands`, `CharacterModelHandler`, `LabelDisplayProcessor`, `UserInterface_DisplayUsernames`, `TargetSensor`, `SaveProcessor`, `VoteEvent`, `.SetTargetType`, `Target`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _270 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `drive_converted_animations` be split into smaller, more focused modules?**
  _Cohesion score 0.047163362952836636 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06980392156862746 - nodes in this community are weakly interconnected._
- **Should `Res` be split into smaller, more focused modules?**
  _Cohesion score 0.047097480832420595 - nodes in this community are weakly interconnected._