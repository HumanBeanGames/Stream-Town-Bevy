# Graph Report - Stream-Town-Bevy  (2026-08-14)

## Corpus Check
- 630 files · ~1,655,979 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7769 nodes · 21595 edges · 294 communities (266 shown, 28 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1015 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `029e8d46`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- World.Generation.Settings
- BuildingProcessor
- STSM_Idle_Player
- NavGrid
- ScriptableObject
- IntWrapper
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- ShaderRef
- SettingsProcessor
- STSM_Action_PlayerBase
- Targetable
- Option
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- BinaryReader
- StationProcessor
- .CreateEnumField
- World.Generation
- BuildingPlacer
- ResMut
- UnitHealthBar
- StableId
- Res
- TechTreeGraphView
- SaveFileData
- Player
- stream_town_game/src/lib.rs
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- RoleHandler
- CommandDictionary
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- .default
- String
- legacy.rs
- WorldInstanceDeterminism
- .Draw
- AudioHandler
- StreamTownSessionBridge
- Vec
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- GUIDComponent
- settings.rs
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- TechTreeNode
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- command.rs
- MonoBehaviour
- models.rs
- Tiler
- ScriptablesEditor
- runtime_console.rs
- UserInterface_ObjectSelection
- world.rs
- Goal
- AnimationHandler
- TwitchBotSetupWindow
- EnemyModelHandler
- WorldUtils
- BuildingBase
- String
- Access_Text
- PlayerRoleData
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- IProcessor.cs
- Access_Toggle
- RoleDataSettings
- PlayerInventory
- convert_fbx_to_glb.py
- stream_town_migrate/src/presentation.rs
- CommonEnums.cs
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- SensorProcessor
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- config.rs
- UserInterface_TownGoal
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- Enemy
- UIElementWrapper
- WorldGenSaveData
- .new
- UserInterface
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- RoleData
- unity_color_filter
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- PlayerInputRuntimeData
- Season
- STSM_StateAction
- UpdateGraphBounds
- GlobalAudioController
- WindController
- TargetProcessor
- MiscCommands
- ResourceGenerationSettings
- ConfirmCheck
- ResourceRuntimeData
- ToolState
- GateController
- PlayerRole
- stream_town_domain/src/lib.rs
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- SelectedBuilding
- Pet
- add_file
- .SetGeneratedResources
- UserInterface_GameMenu
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- convert_post_process
- TradeProcessor
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- SelectedResource
- IProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- IInstaller
- STSM_Helper_Build
- PlayerDeathHandler
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- InventoryEntrySaveData
- SelectedPlayerGroup
- TL_API
- ResourceStorageModifier
- BuildingSettings
- EditorHelpers
- .RenderResourceType
- SelectedEnemy
- TechTree.Elements
- DontDestroyOnLoad
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SelectableObject
- ScriptablesProcessorInfrastructure
- ResourceHolder
- DayAndNightProcessor
- WeatherProcessor
- HealthModifier
- Utils
- BuildingDamageMaterialHandler
- TownResourceRuntimeData
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- DebugSettings
- Character
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- SeasonDataSettings
- VfxParticlePosition
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- TerrainGenSettings
- Common Patterns
- SimpleScreenShot
- ObjectiveSaveData
- Requirement
- ParallelProgressReporter
- setup_camera
- WorldSaveData
- Key Rules
- UserInterface.MainMenu
- RuntimeData Template
- IRuntimeDataScriptable
- PassiveResourceIncrementer
- ScriptKeywordProcessor
- FPSDisplay.cs
- SimpleDisableAfterTime
- Processor Template
- Common Patterns
- EnemyCampSaveData
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Editor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- BuildingModelHandler
- CreateProjectScopeProcessors.cs
- OpenNode
- Station
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- ResourceDataSaveData
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- PostProcessingInstaller
- EquipmentHandlerEditor
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- AudioMixerInstaller
- extraction-spec.md
- AutosaveIntervalsInstaller
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- ObjectSelectionProcessor.Editor.cs
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- ForwardRendererInstaller
- Easings
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- RenderPipelineInstaller
- Q: If there is more to do, keep going.
- FoliageGenerationSettings
- RotationHandler
- VideoSettingsPresetsInstaller
- StatusBar
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- PlacementProbeHandler
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- .RefreshSceneBindingsAndTryGenerate
- InstantiationBarrier
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- TownGoal.Data
- Q: role level experience progression station equipment inventory skill upgrade
- ChannelDataInstaller
- SaveStateInstaller
- UnitTravelToPosition
- RandomEnabler
- PlayerCustomizationSaveData
- FloatWrapper
- .InjectRuntimeData
- .InjectRuntimeData
- PlayerRoleSaveData
- Autosave
- FoliageSaveData
- BuildPlacerData

## God Nodes (most connected - your core abstractions)
1. `StableId` - 322 edges
2. `WorldSimulation` - 162 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 124 edges
8. `WorldGenProcessor` - 110 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
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

## Communities (294 total, 28 thin omitted)

### Community 0 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (13): Dictionary, BuildingDataContainer, Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType (+5 more)

### Community 2 - "STSM_Idle_Player"
Cohesion: 0.09
Nodes (12): float, int, Resource, uint, Vector3, ResourceTarget, STSM_Action_GatherResource, bool (+4 more)

### Community 3 - "NavGrid"
Cohesion: 0.17
Nodes (15): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building(), reconstruct_path() (+7 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.02
Nodes (100): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller (+92 more)

### Community 5 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.15
Nodes (6): Action, int, List, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ShaderRef"
Cohesion: 0.07
Nodes (28): BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension, CloudMaterialUniform (+20 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "STSM_Action_PlayerBase"
Cohesion: 0.14
Nodes (4): AttackUnit, STSM_Action_Build, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 12 - "Targetable"
Cohesion: 0.11
Nodes (8): bool, BoxCollider, float, int, Transform, Vector3, Targetable, IPooledObjectReset

### Community 13 - "Option"
Cohesion: 0.05
Nodes (161): PresentationCatalog, actor_detail_budget(), actor_material(), actor_scene_budget(), AgentAnimation, animate_agents(), animate_chimney_smoke_particles(), animate_healing_effects() (+153 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.16
Nodes (7): STSM_Action_Heal, Action, bool, float, int, UnityEvent, HealthHandler

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.06
Nodes (64): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+56 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "BinaryReader"
Cohesion: 0.22
Nodes (3): CancellationToken, Func, BinaryReader

### Community 19 - "StationProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "World.Generation"
Cohesion: 0.07
Nodes (13): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, SaveSettings, ContainerBuilder, SaveSettingsInstaller, float (+5 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (20): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+12 more)

### Community 23 - "ResMut"
Cohesion: 0.05
Nodes (95): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, PlayerSettings, Default, advance_loading_phase(), apply_settings_draft(), authored_color_grading() (+87 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "StableId"
Cohesion: 0.04
Nodes (97): ObjectiveDef, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips() (+89 more)

### Community 26 - "Res"
Cohesion: 0.05
Nodes (173): Added, AmbientLight, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, Assets, AudioSink, BackgroundColor (+165 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.05
Nodes (13): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, OnChatCommandReceivedArgs (+5 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (252): AnyResult, ActionPresentation, active_event_text(), actor_combat_visual(), ActorAnimationDriver, ActorHealthFill, ActorHealthOverlay, adjust_settings_menu() (+244 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (27): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+19 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 36 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.14
Nodes (4): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 39 - "ContentCatalog"
Cohesion: 0.06
Nodes (115): GameConfig, ContentCatalog, DirtyRegion, GridPos, ActorState, String, GeneratedFoliage, GeneratedWorld (+107 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.12
Nodes (9): Camera, Container, InputButton, List, UnityAction, Vector2, Vector3, ObjectSelectionProcessor (+1 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (125): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+117 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (30): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+22 more)

### Community 45 - ".default"
Cohesion: 0.04
Nodes (102): generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms(), authored_building_nodes_follow_construction_age_and_storage_fill(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats() (+94 more)

### Community 46 - "String"
Cohesion: 0.08
Nodes (67): MaterialDef, PrefabPresentationBinding, animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value() (+59 more)

### Community 47 - "legacy.rs"
Cohesion: 0.16
Nodes (39): ActorCustomization, StreamUserType, binary_fixture(), BinaryParser, clamped_cell(), decode_json(), json_active_goal(), json_buildings() (+31 more)

### Community 48 - "WorldInstanceDeterminism"
Cohesion: 0.31
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "Vec"
Cohesion: 0.05
Nodes (67): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve() (+59 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 57 - "settings.rs"
Cohesion: 0.10
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 61 - "TechTreeNode"
Cohesion: 0.13
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

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
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "twitch.rs"
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 70 - "MonoBehaviour"
Cohesion: 0.03
Nodes (35): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, MetaDataInstaller, PersistentScoped, Transform, PlayerSpawnPoint (+27 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.06
Nodes (20): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+12 more)

### Community 74 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "world.rs"
Cohesion: 0.15
Nodes (28): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise(), generate_foliage(), generate_shoreline_fish() (+20 more)

### Community 77 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.10
Nodes (11): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+3 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "EnemyModelHandler"
Cohesion: 0.14
Nodes (8): bool, int, List, EnemyModelHandler, bool, float, Vector3, STSM_Action_EnemyAttack

### Community 81 - "WorldUtils"
Cohesion: 0.19
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "BuildingBase"
Cohesion: 0.08
Nodes (16): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, int (+8 more)

### Community 83 - "String"
Cohesion: 0.27
Nodes (11): ActorKind, actor_prefix(), entity_id(), json_pet_name(), json_role_name(), legacy_pet_name(), legacy_role_name(), resolve_legacy_archetype() (+3 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRoleData"
Cohesion: 0.11
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, StatType

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.08
Nodes (18): Dictionary, TechVote, Button, GameObject, Image, Slider, TextMeshProUGUI, Transform (+10 more)

### Community 88 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "IProcessor.cs"
Cohesion: 0.15
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 91 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 92 - "RoleDataSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 93 - "PlayerInventory"
Cohesion: 0.12
Nodes (8): DepositResources, PlayerInventory, Dictionary, ResourceInventory, bool, int, float, STSM_Action_DepositResource

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (91): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id(), convert(), convert_avatar_masks() (+83 more)

### Community 96 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (14): BuildingResourceModelHandler, GameObject, UnityEvent, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+6 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.04
Nodes (50): Action, CancellationToken, Component, Container, ContainerBuilder, Dictionary, float, List (+42 more)

### Community 98 - "Coordinator"
Cohesion: 0.10
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (72): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+64 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.07
Nodes (16): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+8 more)

### Community 101 - "StateMachine"
Cohesion: 0.12
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.11
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.05
Nodes (23): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+15 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "config.rs"
Cohesion: 0.14
Nodes (17): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+9 more)

### Community 108 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.17
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "Enemy"
Cohesion: 0.07
Nodes (14): Transform, Action, float, Enemy, float, int, List, Transform (+6 more)

### Community 113 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 114 - "WorldGenSaveData"
Cohesion: 0.10
Nodes (17): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+9 more)

### Community 115 - ".new"
Cohesion: 0.11
Nodes (29): absolute_path(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), conversion_rejects_malformed_retained_mesh(), convert() (+21 more)

### Community 116 - "UserInterface"
Cohesion: 0.06
Nodes (17): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors (+9 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.06
Nodes (21): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, float (+13 more)

### Community 119 - "Resource"
Cohesion: 0.07
Nodes (15): int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor, bool (+7 more)

### Community 120 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 126 - "Season"
Cohesion: 0.15
Nodes (11): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, bool, float (+3 more)

### Community 127 - "STSM_StateAction"
Cohesion: 0.12
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "TargetProcessor"
Cohesion: 0.13
Nodes (10): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, List, TargetProcessor, Dictionary (+2 more)

### Community 132 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 133 - "ResourceGenerationSettings"
Cohesion: 0.12
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 136 - "ToolState"
Cohesion: 0.10
Nodes (63): apply_technology_draft(), bounded_ui_index(), commit_catalog_candidate(), content_tab(), create_technology_group(), create_technology_node(), default_catalog_path(), delete_selected_technology_group() (+55 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "PlayerRole"
Cohesion: 0.06
Nodes (14): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+6 more)

### Community 139 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

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
Cohesion: 0.04
Nodes (34): Container, ContainerBuilder, GUIDProcessor, Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor (+26 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "SelectedBuilding"
Cohesion: 0.08
Nodes (5): SelectedBuilding, SelectedEnemyCamp, object, UnityAction, SelectedObject

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - ".SetGeneratedResources"
Cohesion: 0.40
Nodes (5): List, Material, materials, Mesh, meshes

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.17
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 154 - "convert_post_process"
Cohesion: 0.24
Nodes (14): animation_take_name(), convert_embedded_model_clips(), convert_post_process(), field_bool(), field_f32(), field_str(), field_u64(), field_value() (+6 more)

### Community 155 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - "IProcessor"
Cohesion: 0.23
Nodes (4): CancellationToken, Task, Container, IProcessor

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

### Community 164 - "IInstaller"
Cohesion: 0.08
Nodes (15): ContainerBuilder, GameSettingsInstaller, List, GameSettings, Action, Container, ContainerBuilder, EventProcessor (+7 more)

### Community 165 - "STSM_Helper_Build"
Cohesion: 0.18
Nodes (5): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase

### Community 166 - "PlayerDeathHandler"
Cohesion: 0.22
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 167 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.14
Nodes (12): Container, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int (+4 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (19): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+11 more)

### Community 170 - "InventoryEntrySaveData"
Cohesion: 0.33
Nodes (4): bool, int, string, InventoryEntrySaveData

### Community 171 - "SelectedPlayerGroup"
Cohesion: 0.21
Nodes (3): List, List, SelectedPlayerGroup

### Community 173 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 174 - "BuildingSettings"
Cohesion: 0.20
Nodes (6): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 179 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 185 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 186 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 187 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 188 - "WeatherProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 189 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 190 - "Utils"
Cohesion: 0.05
Nodes (12): BuildCostModifier, WindControllerEditor, InputButton, Utils, Processors, World, Level, Buildings (+4 more)

### Community 191 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 192 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "DebugSettings"
Cohesion: 0.29
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 195 - "Character"
Cohesion: 0.06
Nodes (25): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+17 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 200 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 209 - "ObjectiveSaveData"
Cohesion: 0.40
Nodes (3): int, string, ObjectiveSaveData

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 213 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "UserInterface.MainMenu"
Cohesion: 0.18
Nodes (3): UserInterface.MainMenu, MetaData, Settings

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "IRuntimeDataScriptable"
Cohesion: 0.09
Nodes (16): Container, ContainerBuilder, GameStateProcessor, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData (+8 more)

### Community 218 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 221 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "EnemyCampSaveData"
Cohesion: 0.40
Nodes (3): int, uint, EnemyCampSaveData

### Community 225 - "Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?, Source Nodes

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

### Community 234 - "Editor"
Cohesion: 0.06
Nodes (13): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, RoleScriptablesEditor, CellPartitioningEditor, GridProcessorEditor, GridSystemEditor (+5 more)

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 240 - "Station"
Cohesion: 0.10
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 243 - "ResourceDataSaveData"
Cohesion: 0.27
Nodes (10): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+2 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 248 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 253 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "FoliageGenerationSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, List, FoliageGenSettings, List, WaterFoliageGenSettings (+7 more)

### Community 268 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 269 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 270 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "TownGoal.Data"
Cohesion: 0.09
Nodes (10): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 293 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

## Knowledge Gaps
- **284 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+279 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **28 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ResourceGenerationSettings`, `PlayerRole`, `FoliageGenerationSettings`, `SettingsProcessor`, `ObjectPoolingProcessor`, `UserInterface_GameMenu`, `SaveFileData`, `IProcessor`, `WorldGenProcessor`, `GameEventProcessor`, `IInstaller`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `StreamTownSessionBridge`, `Character`, `MonoBehaviour`, `FoliageProcessor`, `PlayerProcessor`, `TownGoalProcessor`, `MainMenuManager`, `Resource`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `ToolState`, `stream_town_domain/src/lib.rs`, `Option`, `stream_town_domain/src/content.rs`, `save.rs`, `Res`, `convert_post_process`, `stream_town_game/src/lib.rs`, `ContentCatalog`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `.default`, `String`, `Vec`, `twitch.rs`, `command.rs`, `runtime_console.rs`, `world.rs`, `String`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/presentation.rs`, `config.rs`, `.new`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `ResourceGenerationSettings`, `TwitchChatProcessor`, `FoliageGenerationSettings`, `ObjectPoolingProcessor`, `World.Generation`, `WorldGenRuntimeData`, `Player`, `IProcessor`, `UserInterface_Debug`, `IInstaller`, `GridProcessor`, `ResourceProcessor`, `WorldInstanceDeterminism`, `TwitchClientProcessor`, `Access_Dropdown`, `MonoBehaviour`, `TerrainGenSettings`, `WorldUtils`, `CellSpacePartitioning`, `FoliageProcessor`, `IRuntimeDataScriptable`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `GameEvent`, `Enemy`, `UserInterface`, `AIPath`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _284 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World.Generation.Settings` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.061016949152542375 - nodes in this community are weakly interconnected._
- **Should `STSM_Idle_Player` be split into smaller, more focused modules?**
  _Cohesion score 0.08819345661450925 - nodes in this community are weakly interconnected._