# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 609 files · ~1,604,057 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7013 nodes · 18397 edges · 264 communities (241 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 994 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `294ddd9b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- world.rs
- BuildingProcessor
- stream_town_domain/src/content.rs
- String
- TargetSettings
- EnemyModelHandler
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- settings.rs
- SettingsProcessor
- WorldGenProcessor
- Player
- RoleData
- TechTreeIOUtility
- MonoBehaviour
- Character
- Utils
- World.Generation.Settings
- legacy.rs
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- PlayerRoleData
- Goal
- Result
- SaveFileData
- GameEventProcessor
- ContentCatalog
- ResourceStorageModifier
- World.Generation
- ResourceProcessor
- Station
- UserInterface_Debug
- MiscCommands
- SettingsData
- AnimationHandler
- LegacyEntity
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BinaryWriter
- AnimationControllerDef
- save.rs
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
- RoleHandler
- StableId
- STSM_Idle_Player
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
- stream_town_migrate/src/presentation.rs
- UserInterface_ObjectSelection
- .LogWarning
- ProjectCamera
- GridProcessor
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- GameEvent
- TreeMaterialExtension
- Access_Text
- stream_town_game/src/lib.rs
- CellSpacePartitioning
- UserInterface_TownVote
- Targetable
- FoliageProcessor
- Option
- PlayerRole
- RaidEvent
- Access_Dropdown
- convert_fbx_to_glb.py
- convert
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
- UserInterface_TownGoal
- RotationHandler
- IRuntimeDataScriptable
- VoteEvent
- Resource
- PlayerInventory
- Res
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- PoolableObject
- BuildingBase
- CommandDictionary
- UpdateGraphBounds
- SelectableObject
- stream_town_migrate/src/main.rs
- WorldInstanceDeterminism
- .RenderResourceType
- GameStateProcessor
- ConfirmCheck
- TownResourceRuntimeData
- ToolState
- LabelDisplayProcessor
- SensorProcessor
- BuildingSettings
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
- SelectedPlayerGroup
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- WeatherProcessor
- UserInterface
- .CaptureFoliageGroups
- Stream Town Reloaded - Architecture Documentation
- WindController
- TechTreeNode
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- FoliageGenerationSettings
- CommonEnums.cs
- CampGenerationSettings
- Easings
- SelectedResource
- NodeUnlockData
- TimeProcessor
- UserInterface_GameMenu
- UIElementWrapper
- SelectedBuilding
- EditorHelpers
- EventProcessor
- IProcessor
- PlayerSaveData
- TerrainGenSettings
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- GUIDProcessor
- WorldGenRuntimeData
- DayAndNightSettings
- CreditsProcessor
- PassiveResourceIncrementer
- KeepKingVote
- WorldGenDebugSettings
- BuildingDataSettings
- command.rs
- EventCommands
- Target
- TL_API
- NewKingVote
- SelectedEnemyCamp
- UILineRenderer
- UserInterface_DisplayUsernames
- ResourceDataSettings
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- GameEventSettings
- ObjectSelectionProcessor.Editor.cs
- Key Rules
- SelectedEnemy
- Common Patterns
- Processors
- BuildingConfigSettings
- TimeSettings
- FPSDisplay.cs
- WorldGenScaleSettings
- GridSettings
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- WeatherSettings
- VfxParticlePosition
- ScriptKeywordProcessor
- IntWrapper
- xtask/src/main.rs
- Processor Template
- Common Patterns
- CombatVisualKind
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- PlacementProbeHandler
- Twitch setup
- graphify reference: add a URL and watch a folder
- StatusBar
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- ObjectiveSaveData
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- .AddGoalFollowed
- CreateProjectScopeProcessors.cs
- BuildPlacerData
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- append_vec3_keys
- AGENTS.md
- TradeProcessor
- ResourceGenerationSettings
- CustomLogger
- extraction-spec.md
- RoleDataSettings
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- BuildingRuntimeData
- EquipmentHandlerEditor
- ScriptablesProcessorInfrastructure
- IInstaller
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
- `embedded_config_supports_vertical_slice_scale()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `rotated_footprints_and_building_moves_are_deterministic()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (264 total, 23 thin omitted)

### Community 0 - "world.rs"
Cohesion: 0.17
Nodes (22): WorldGenConfig, FoliageHabitat, FoliageLayerDef, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise() (+14 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (44): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+36 more)

### Community 3 - "String"
Cohesion: 0.15
Nodes (36): animation_state_id(), infer_missing_parameters(), inline_file_id(), inline_mapping_value(), parse_animation_events(), parse_blend_tree(), parse_child_references(), parse_conditions() (+28 more)

### Community 4 - "TargetSettings"
Cohesion: 0.29
Nodes (4): TargetSettings, ContainerBuilder, TargetSettingsInstaller, TargetableData

### Community 5 - "EnemyModelHandler"
Cohesion: 0.08
Nodes (11): bool, int, List, EnemyModelHandler, int, STSM_Action_Attack, bool, float (+3 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "settings.rs"
Cohesion: 0.11
Nodes (27): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+19 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (27): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+19 more)

### Community 12 - "Player"
Cohesion: 0.05
Nodes (14): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+6 more)

### Community 13 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "MonoBehaviour"
Cohesion: 0.03
Nodes (40): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, MetaDataInstaller, PersistentScoped, UnitTravelToPosition, Vector3 (+32 more)

### Community 16 - "Character"
Cohesion: 0.06
Nodes (23): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+15 more)

### Community 17 - "Utils"
Cohesion: 0.06
Nodes (8): BuildCostModifier, RoleScriptablesEditor, Utils, Level, ScriptablesEditor, Buildings, SavingAndLoading, SavingAndLoading.Structs

### Community 18 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 19 - "legacy.rs"
Cohesion: 0.18
Nodes (36): binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal(), json_buildings() (+28 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.05
Nodes (28): int, ChangeTimeStamp, Color, ErrorData, List, GroupErrorData, List, NodeErrorData (+20 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "HealthHandler"
Cohesion: 0.07
Nodes (19): HealthModifier, bool, float, GameObject, HealUnit, BuildingDamageMaterialHandler, bool, IEnumerator (+11 more)

### Community 25 - "PlayerRoleData"
Cohesion: 0.11
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, StatType

### Community 26 - "Goal"
Cohesion: 0.13
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 27 - "Result"
Cohesion: 0.25
Nodes (7): BinaryParser<'a>, decode_binary(), FnMut, Result, Self, T, LegacyWorldState

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "ContentCatalog"
Cohesion: 0.06
Nodes (86): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, ActorState, RoleProgress, Default, String (+78 more)

### Community 31 - "ResourceStorageModifier"
Cohesion: 0.12
Nodes (7): BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, UnityEvent, StorageStatus

### Community 33 - "ResourceProcessor"
Cohesion: 0.11
Nodes (14): Container, ContainerBuilder, float, int, List, Material, materials, Mesh (+6 more)

### Community 34 - "Station"
Cohesion: 0.06
Nodes (19): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+11 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.07
Nodes (18): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+10 more)

### Community 39 - "LegacyEntity"
Cohesion: 0.14
Nodes (21): ActorCustomization, StreamUserType, ImportReport, json_pet_name(), json_role_name(), legacy_objective_matches(), legacy_pet_name(), legacy_role_name() (+13 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (28): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+20 more)

### Community 45 - "save.rs"
Cohesion: 0.15
Nodes (31): detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native(), native_save_is_atomic_and_keeps_backup() (+23 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.07
Nodes (21): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+13 more)

### Community 47 - "DebugProcessor"
Cohesion: 0.13
Nodes (9): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, DebugLogCategory, DebugProcessor (+1 more)

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
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "ResourceData"
Cohesion: 0.18
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 57 - "StableId"
Cohesion: 0.04
Nodes (85): ObjectiveDef, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips() (+77 more)

### Community 58 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (21): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+13 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
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

### Community 67 - "twitch.rs"
Cohesion: 0.06
Nodes (52): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet, Default, Result, Self (+44 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

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
Cohesion: 0.06
Nodes (21): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+13 more)

### Community 74 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.09
Nodes (65): AvatarMaskDef, TextureDef, animation_state_machine_id(), array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), clip_id(), collect_prefab_dependencies() (+57 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - ".LogWarning"
Cohesion: 0.08
Nodes (7): AttackUnit, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

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
Cohesion: 0.19
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 83 - "TreeMaterialExtension"
Cohesion: 0.15
Nodes (16): BuildingMaterialExtension, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform, Image, TerrainMaterialExtension, TerrainMaterialUniform, tree_season_controls() (+8 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (194): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, AnyResult, AnimationTransformTrack, PlayerSettings (+186 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "Targetable"
Cohesion: 0.06
Nodes (23): ProjectileShooter, float, int, string, Container, ContainerBuilder, List, TargetProcessor (+15 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "Option"
Cohesion: 0.06
Nodes (110): AssetServer, GridPos, PresentationCatalog, GeneratedWorld, actor_material(), animation_property_value(), animation_selection_duration(), archetype_id_by_source() (+102 more)

### Community 91 - "PlayerRole"
Cohesion: 0.08
Nodes (8): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, PlayerRole

### Community 92 - "RaidEvent"
Cohesion: 0.07
Nodes (19): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+11 more)

### Community 93 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "convert"
Cohesion: 0.16
Nodes (20): ActorKind, SavedActor, absolute_path(), actor_prefix(), backup_candidate(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+12 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 97 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (43): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+35 more)

### Community 100 - "Editor"
Cohesion: 0.07
Nodes (11): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, WindControllerEditor, GridProcessorEditor, GridSystemEditor, GridSystem.Utils (+3 more)

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
Cohesion: 0.10
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.08
Nodes (15): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+7 more)

### Community 107 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 108 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.11
Nodes (13): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+5 more)

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 115 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 116 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 117 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (14): Queue, AudioRuntimeData, CreditsRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+6 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.08
Nodes (16): DepositResources, int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor (+8 more)

### Community 120 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 121 - "Res"
Cohesion: 0.05
Nodes (163): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, AnimationGraphHandle, App, AppExit, Assets (+155 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "PoolableObject"
Cohesion: 0.07
Nodes (22): Action, float, Enemy, uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp (+14 more)

### Community 126 - "BuildingBase"
Cohesion: 0.11
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 127 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - "WorldInstanceDeterminism"
Cohesion: 0.31
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 132 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 133 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (77): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+69 more)

### Community 137 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 138 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 139 - "BuildingSettings"
Cohesion: 0.20
Nodes (6): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller

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
Cohesion: 0.06
Nodes (29): HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder (+21 more)

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
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 154 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 155 - "UserInterface"
Cohesion: 0.08
Nodes (8): InputButton, SharedTypes, TownGoal.Data, StreamTown.EditorTools, UserInterface, TechTree.Data, Data, GameEventSystem.Events.Voting

### Community 156 - ".CaptureFoliageGroups"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "TechTreeNode"
Cohesion: 0.13
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

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

### Community 166 - "CommonEnums.cs"
Cohesion: 0.13
Nodes (13): Dictionary, List, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, Seasons (+5 more)

### Community 167 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 177 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 178 - "PlayerSaveData"
Cohesion: 0.05
Nodes (29): Dictionary, List, Mesh, Vector3, SaveDataMapper, bool, int, List (+21 more)

### Community 179 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 184 - "DayAndNightSettings"
Cohesion: 0.12
Nodes (11): float, Material, Volume, DayAndNightSettings, float, UISettings, ContainerBuilder, DayAndNightSettingsInstaller (+3 more)

### Community 185 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 187 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 189 - "WorldGenDebugSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, WorldGenDebugSettingsInstaller, bool, float, WorldGenDebugSettings

### Community 190 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 191 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 193 - "Target"
Cohesion: 0.06
Nodes (16): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors (+8 more)

### Community 195 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "Processors"
Cohesion: 0.06
Nodes (9): InputButton, UserInterface.MainMenu, Processors, MetaData, Audio, Settings, Environment, GameResources (+1 more)

### Community 209 - "BuildingConfigSettings"
Cohesion: 0.33
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 210 - "TimeSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, TimeDataSettingsInstaller, int, TimeSettings

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 212 - "WorldGenScaleSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, WorldGenScaleSettingsInstaller, float, WorldGenScaleSettings

### Community 213 - "GridSettings"
Cohesion: 0.29
Nodes (5): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "WeatherSettings"
Cohesion: 0.33
Nodes (4): VisualEffect, WeatherSettings, ContainerBuilder, WeatherSettingsInstaller

### Community 218 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 221 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "CombatVisualKind"
Cohesion: 0.47
Nodes (6): ActionPresentation, actor_combat_visual(), CombatProjectile, CombatVisualKind, ProjectileSource, ProjectileSpawn

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

### Community 231 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 235 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.08
Nodes (16): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+8 more)

### Community 244 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

### Community 246 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 249 - "ResourceGenerationSettings"
Cohesion: 0.12
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 254 - "RoleDataSettings"
Cohesion: 0.07
Nodes (22): RoleSlot, bool, int, ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer (+14 more)

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 262 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 263 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 279 - "IInstaller"
Cohesion: 0.04
Nodes (32): ContainerBuilder, InstantiationBarrier, ContainerBuilder, Volume, PostProcessingInstaller, AudioMixerInstaller, AudioMixer, ContainerBuilder (+24 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (56): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, ContainerBuilder, WorldGenBehaviorSettingsInstaller (+48 more)

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

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `Player`, `ObjectPoolingProcessor`, `Character`, `MonoBehaviour`, `PlayerProcessor`, `ScriptableObject`, `IInstaller`, `.CaptureFoliageGroups`, `GameEventProcessor`, `SaveFileData`, `ResourceProcessor`, `TechTreeProcessor`, `TimeProcessor`, `UserInterface_GameMenu`, `SeasonProcessor`, `DebugProcessor`, `IProcessor`, `PlayerSaveData`, `StreamTownSessionBridge`, `GUIDProcessor`, `FoliageProcessor`, `PlayerRole`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `Resource`, `ResourceGenerationSettings`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `WorldInstanceDeterminism`, `GameStateProcessor`, `TwitchChatProcessor`, `Player`, `ObjectPoolingProcessor`, `MonoBehaviour`, `PlayerProcessor`, `ScriptableObject`, `IInstaller`, `ResourceProcessor`, `UserInterface_Debug`, `CampGenerationSettings`, `DebugProcessor`, `IProcessor`, `TerrainGenSettings`, `ResourceData`, `TwitchClientProcessor`, `GUIDProcessor`, `WorldGenRuntimeData`, `WorldGenDebugSettings`, `Target`, `.LogWarning`, `ProjectCamera`, `GridProcessor`, `WorldGenScaleSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `LoadingManager`, `EnemySpawner`, `SaveProcessor`, `ResourceGenerationSettings`, `AIPath`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `TwitchChatProcessor`, `BuildingRuntimeData`, `LabelDisplayProcessor`, `RoleData`, `Character`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `GameEventProcessor`, `Station`, `UserInterface_Debug`, `MiscCommands`, `CharacterModelHandler`, `StreamTownSessionBridge`, `RoleHandler`, `EventCommands`, `Target`, `UserInterface_DisplayUsernames`, `Targetable`, `PlayerRole`, `SaveProcessor`, `VoteEvent`, `.SetTargetType`, `PoolableObject`, `CommandDictionary`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06327683615819209 - nodes in this community are weakly interconnected._
- **Should `stream_town_domain/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10034013605442177 - nodes in this community are weakly interconnected._
- **Should `String` be split into smaller, more focused modules?**
  _Cohesion score 0.1492063492063492 - nodes in this community are weakly interconnected._