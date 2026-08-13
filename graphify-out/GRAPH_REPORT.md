# Graph Report - Stream-Town-Bevy  (2026-08-14)

## Corpus Check
- 629 files · ~1,634,521 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7460 nodes · 20212 edges · 290 communities (265 shown, 25 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1010 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b5f2b416`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Target
- BuildingProcessor
- Res
- stream_town_migrate/src/presentation.rs
- Transform
- WorldGenProcessor
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- StableId
- SettingsProcessor
- .GenerateFromSettings
- GUIDComponent
- Option
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- SelectedBuilding
- GeneratedWorld
- .CreateEnumField
- UserInterface
- BuildingPlacer
- PlayerProcessor
- UnitHealthBar
- STSM_Action_GatherResource
- Query
- TechTreeGraphView
- SaveFileData
- Station
- stream_town_game/src/lib.rs
- StationProcessor
- SavingAndLoading.Structs
- GameEventProcessor
- technology_tab
- UserInterface_Debug
- CommandDictionary
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- Editor
- AnimationControllerDef
- ResourceProcessor
- DebugProcessor
- legacy.rs
- ResourceRuntimeData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- Utils
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Result
- BuildingBase
- STSM_Idle_Player
- TechTreeEditorWindow
- Result
- RenderAssets
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- settings.rs
- simulation.rs
- models.rs
- Tiler
- ScriptablesEditor
- StringUtils
- UserInterface_ObjectSelection
- TimeProcessor
- .RenderResourceType
- STSM_StateAction
- TwitchBotSetupWindow
- UnitTextDisplay
- WorldUtils
- SelectedObject
- Vec
- Access_Text
- .SetGeneratedResources
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- runtime_console.rs
- Targetable
- command.rs
- Goal
- convert_fbx_to_glb.py
- String
- RoleSlot
- TransformSaveData
- Coordinator
- stream_town_domain/src/presentation.rs
- .default
- StateMachine
- .GetResourceTarget
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- BuildingDataSettings
- UserInterface_TownGoal
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- Processors
- IProcessor
- NavGrid
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- RoleData
- TechTreeNode
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- ScriptablesProcessorInfrastructure
- STStateMachine.States
- EditorUtils
- UpdateGraphBounds
- GlobalAudioController
- WindController
- ErrorData
- PlayerRoleData
- SensorProcessor
- ConfirmCheck
- .InjectRuntimeData
- ToolState
- GateController
- UserInterface_GameMenu
- .AddEvent
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- stream_town_migrate/src/content.rs
- Pet
- add_file
- GameStateProcessor
- MiscCommands
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- UIElementWrapper
- SelectedPlayer
- Access_Toggle
- ResourceStorageModifier
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- IRuntimeDataScriptable
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- DontDestroyOnLoad
- ResourceHolder
- NodeUnlockData
- GridProcessor
- config.rs
- xtask/src/main.rs
- PlayerCommands
- CommonEnums.cs
- FoliageGenerationSettings
- UserInterface_RulerVote
- KeepKingVote
- EditorHelpers
- RoleHandler
- SelectedEnemy
- TechTree.Elements
- AnimationHandler
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- ResourceDataSettings
- AllBuildingDataSettings
- ResourceDataSaveData
- DayAndNightProcessor
- LabelDisplayProcessor
- NewKingVote
- VFXArrowPointer
- Globals
- Player
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SelectableObject
- MeshData
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WeatherProcessor
- Common Patterns
- Easings
- FPSDisplay.cs
- Requirement
- UI_TechOption
- VfxAnimationController
- import_save
- Key Rules
- World.Generation.Settings
- RuntimeData Template
- IProcessor.cs
- VfxParticlePosition
- ScriptKeywordProcessor
- IntWrapper
- SimpleScreenShot
- Processor Template
- Common Patterns
- CombatVisualKind
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- PostProcessingInstaller
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- WorldSaveData
- CreateProjectScopeProcessors.cs
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- RotationHandler
- DayAndNightSettings
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- RenderPipelineInstaller
- extraction-spec.md
- PlayerSaveData
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- StatusBar
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- ObjectiveDef
- .AddGoalFollowed
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- VFX
- Q: If there is more to do, keep going.
- PassiveResourceIncrementer
- .RefreshSceneData
- EventProcessor
- SimpleDisableAfterTime
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Character
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- MonoBehaviour
- VideoSettingsPresetsInstaller
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- CampGenerationSettings
- Autosave
- parse_transform_tracks
- ForwardRendererInstaller
- BuildPlacerData
- PlacementProbeHandler
- convert
- setup_camera

## God Nodes (most connected - your core abstractions)
1. `StableId` - 289 edges
2. `Utils` - 158 edges
3. `WorldSimulation` - 157 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 116 edges
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

## Communities (290 total, 25 thin omitted)

### Community 0 - "Target"
Cohesion: 0.10
Nodes (11): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Sensors, Pets, GridSystem.Partitioning, Combat (+3 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "Res"
Cohesion: 0.04
Nodes (122): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, BackgroundColor, ActorNameOverlay, AgentCommandQueue, animation_property_value(), apply_player_settings() (+114 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.13
Nodes (39): animation_take_name(), animator_component(), animator_reference_path(), array_index(), color_value(), convert_embedded_model_clips(), convert_prefab_bindings(), embedded_clip_id() (+31 more)

### Community 4 - "Transform"
Cohesion: 0.06
Nodes (61): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, ActivePetVisual, add_animation_layer_branch(), add_rotation_curve() (+53 more)

### Community 5 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "StableId"
Cohesion: 0.07
Nodes (42): FromStr, StableId, complete_gameplay_scenario_round_trips(), ObjectiveProgress, BTreeMap, Option, Result, Vec (+34 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - ".GenerateFromSettings"
Cohesion: 0.07
Nodes (32): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+24 more)

### Community 12 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 13 - "Option"
Cohesion: 0.05
Nodes (56): actor_detail_budget(), animation_root_name(), BoundsMaterialExtension, BoundsMaterialUniform, building_effect_material(), building_health_fraction(), BuildingMaterialExtension, BuildingMaterialUniform (+48 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (16): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy, STSM_Action_Heal (+8 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (46): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+38 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 19 - "GeneratedWorld"
Cohesion: 0.13
Nodes (35): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise(), generate_foliage(), generate_shoreline_fish() (+27 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "UserInterface"
Cohesion: 0.06
Nodes (14): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, TownGoal (+6 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.07
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "STSM_Action_GatherResource"
Cohesion: 0.13
Nodes (4): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 26 - "Query"
Cohesion: 0.08
Nodes (75): Added, AnimationGraphHandle, AnimationTransitions, Assets, Agent, animate_building_effects(), animate_ping_pointers(), apply_building_commands() (+67 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Station"
Cohesion: 0.09
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (211): AnyResult, AudioSink, active_event_text(), ActorAnimationDriver, adjust_settings_menu(), agent_action_animation(), agent_is_moving(), AgentEnemyModelPresentation (+203 more)

### Community 31 - "StationProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 32 - "SavingAndLoading.Structs"
Cohesion: 0.10
Nodes (3): SavingAndLoading, SavingAndLoading.Structs, World.Generation

### Community 33 - "GameEventProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, bool, ParticleSystem (+6 more)

### Community 34 - "technology_tab"
Cohesion: 0.31
Nodes (16): apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node(), refresh_technology_draft(), Option (+8 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.27
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.09
Nodes (12): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+4 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.07
Nodes (85): GameConfig, ContentCatalog, TargetingScoreDef, GridPos, ActorState, BuildingState, String, action_animation_speed() (+77 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "Editor"
Cohesion: 0.18
Nodes (6): GameObject, List, EquipmentHandlerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "ResourceProcessor"
Cohesion: 0.17
Nodes (11): Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int, Matrix4x4 (+3 more)

### Community 46 - "DebugProcessor"
Cohesion: 0.10
Nodes (7): Dictionary, DebugSettings, Container, ContainerBuilder, DebugLogCategory, DebugProcessor, SerializedScriptableObject

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (37): ActorCustomization, StreamUserType, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies() (+29 more)

### Community 48 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "Utils"
Cohesion: 0.06
Nodes (7): BuildCostModifier, RoleScriptablesEditor, Utils, Level, ScriptablesEditor, Buildings, GameResources

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, float, UISettings, ContainerBuilder (+6 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Result"
Cohesion: 0.13
Nodes (37): MaterialDef, assign_clip_rigs_and_reference_poses(), avatar_mask_id(), clip_id(), collect_prefab_dependencies(), controller_id(), convert(), convert_avatar_masks() (+29 more)

### Community 57 - "BuildingBase"
Cohesion: 0.09
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 58 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (21): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+13 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.31
Nodes (4): BinaryParser<'a>, decode_binary(), Result, LegacyWorldState

### Community 61 - "RenderAssets"
Cohesion: 0.09
Nodes (101): AssetServer, ArchetypeScene, PresentationCatalog, actor_material(), actor_scene_budget(), AgentAnimation, animate_agents(), apply_agent_commands() (+93 more)

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
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "twitch.rs"
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.14
Nodes (4): Action, int, Objective, EnemyType

### Community 69 - "settings.rs"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 70 - "simulation.rs"
Cohesion: 0.08
Nodes (31): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), EnemyCampState (+23 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "TimeProcessor"
Cohesion: 0.38
Nodes (3): Container, ContainerBuilder, TimeProcessor

### Community 77 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 78 - "STSM_StateAction"
Cohesion: 0.06
Nodes (17): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, int, STSM_Helper_Attack, int (+9 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

### Community 83 - "Vec"
Cohesion: 0.16
Nodes (19): binary_fixture(), BinaryParser, legacy_objective_matches(), LegacyDecodedSave, LegacyGoal, LegacyObjective, objective_target_matches(), put_f32() (+11 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - ".SetGeneratedResources"
Cohesion: 0.40
Nodes (5): List, Material, materials, Mesh, meshes

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.11
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 91 - "Targetable"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+7 more)

### Community 92 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 93 - "Goal"
Cohesion: 0.11
Nodes (10): bool, Dictionary, float, int, TechTreeRuntimeData, List, TownGoalRuntimeData, Action (+2 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "String"
Cohesion: 0.13
Nodes (36): AnimationParameterDef, animation_state_id(), animation_state_machine_id(), infer_missing_parameters(), inline_file_id(), normalized_path(), parse_avatar_mask(), parse_blend_tree() (+28 more)

### Community 96 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 97 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (44): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+36 more)

### Community 100 - ".default"
Cohesion: 0.03
Nodes (131): AmbientLight, App, generate_world(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms(), authored_building_nodes_follow_construction_age_and_storage_fill() (+123 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - ".GetResourceTarget"
Cohesion: 0.18
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.05
Nodes (23): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+15 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 108 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

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

### Community 113 - "Processors"
Cohesion: 0.06
Nodes (14): GridProcessorEditor, ObjectSelectionProcessor, InputButton, GridSystem.Utils, UserInterface.MainMenu, Processors, World, Processors.Editor (+6 more)

### Community 114 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 115 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (31): DepositResources, PlayerInventory, Dictionary, ResourceInventory, bool, int, int, ActiveResourceIncrementer (+23 more)

### Community 120 - "RoleData"
Cohesion: 0.08
Nodes (20): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+12 more)

### Community 121 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 126 - "STStateMachine.States"
Cohesion: 0.07
Nodes (12): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+4 more)

### Community 127 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 132 - "PlayerRoleData"
Cohesion: 0.09
Nodes (11): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+3 more)

### Community 133 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (46): bounded_ui_index(), content_tab(), default_catalog_path(), draw_world_preview(), inject_runtime_command(), inspector_tab(), launch_runtime_game(), main() (+38 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

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
Nodes (39): Container, ContainerBuilder, GUIDProcessor, HideInCallstack, Object, Action, bool, BoxCollider (+31 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (115): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+107 more)

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 150 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 155 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 156 - "ResourceStorageModifier"
Cohesion: 0.11
Nodes (8): BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, BuildingResourceModelHandlerEditor, UnityEvent, StorageStatus

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 159 - "IRuntimeDataScriptable"
Cohesion: 0.10
Nodes (17): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+9 more)

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

### Community 165 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 167 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "config.rs"
Cohesion: 0.14
Nodes (17): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+9 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (19): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+11 more)

### Community 170 - "PlayerCommands"
Cohesion: 0.16
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 171 - "CommonEnums.cs"
Cohesion: 0.20
Nodes (9): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, Seasons, TimeOfDay, WallType (+1 more)

### Community 172 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "RoleHandler"
Cohesion: 0.05
Nodes (17): RoleSlotModifier, int, RoleHandler, bool, Dictionary, UnityEvent, Dictionary, int (+9 more)

### Community 178 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 179 - "AnimationHandler"
Cohesion: 0.08
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+7 more)

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

### Community 184 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 185 - "AllBuildingDataSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingDataContainerInstaller, AllBuildingDataSettings

### Community 186 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 187 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 188 - "LabelDisplayProcessor"
Cohesion: 0.32
Nodes (3): Container, ContainerBuilder, LabelDisplayProcessor

### Community 189 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 190 - "VFXArrowPointer"
Cohesion: 0.22
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 192 - "Player"
Cohesion: 0.08
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+1 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

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

### Community 200 - "BuildingModelHandler"
Cohesion: 0.18
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

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

### Community 206 - "WeatherProcessor"
Cohesion: 0.11
Nodes (13): Container, ContainerBuilder, WeatherProcessor, Color, float, int, VisualEffect, SeasonDataSettings (+5 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 212 - "VfxAnimationController"
Cohesion: 0.04
Nodes (21): Api, PersistentScoped, TL_API, Image, TextMeshProUGUI, UIRoleDisplay, float, GameObject (+13 more)

### Community 213 - "import_save"
Cohesion: 0.43
Nodes (7): absolute_path(), backup_candidate(), import_preserves_source_and_recovers_named_backup(), import_save(), ImportReport, Path, PathBuf

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 218 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 221 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "CombatVisualKind"
Cohesion: 0.47
Nodes (6): ActionPresentation, actor_combat_visual(), CombatProjectile, CombatVisualKind, ProjectileSource, ProjectileSpawn

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

### Community 234 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "WorldSaveData"
Cohesion: 0.10
Nodes (21): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+13 more)

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 240 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 243 - "SaveProcessor"
Cohesion: 0.08
Nodes (21): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+13 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 248 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 253 - "PlayerSaveData"
Cohesion: 0.05
Nodes (28): Component, Dictionary, Mesh, Transform, Vector3, SaveDataMapper, bool, int (+20 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "ObjectiveDef"
Cohesion: 0.60
Nodes (4): ObjectiveDef, ObjectiveKind, objective_increment(), ObjectiveEvent

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 269 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 270 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "Character"
Cohesion: 0.07
Nodes (18): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+10 more)

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "MonoBehaviour"
Cohesion: 0.02
Nodes (86): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, Camera (+78 more)

### Community 277 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (71): int, AudioSettings, List, CampGenSettings, List, FoliageGenSettings, int, Vector2 (+63 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 284 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 293 - "parse_transform_tracks"
Cohesion: 0.33
Nodes (6): append_vec3_keys(), parse_inline_array(), parse_transform_tracks(), parses_unity_transform_curves_without_editor_types(), Item, Iterator

### Community 295 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 296 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 298 - "convert"
Cohesion: 0.17
Nodes (17): ActorKind, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert() (+9 more)

## Knowledge Gaps
- **283 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+278 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `Target`, `BuildingProcessor`, `WorldGenProcessor`, `SettingsProcessor`, `.AddEvent`, `UserInterface_GameMenu`, `ObjectPoolingProcessor`, `MonoBehaviour`, `PlayerProcessor`, `ScriptableObject`, `SaveFileData`, `GameEventProcessor`, `SeasonProcessor`, `TechTreeProcessor`, `PlayerCommands`, `ResourceProcessor`, `DebugProcessor`, `RoleHandler`, `StreamTownSessionBridge`, `ResourceDataSaveData`, `TimeProcessor`, `FoliageProcessor`, `TownGoalProcessor`, `MainMenuManager`, `WorldSaveData`, `IProcessor`, `Resource`, `PlayerSaveData`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `Target`, `BuildingProcessor`, `TwitchChatProcessor`, `.GenerateFromSettings`, `ObjectPoolingProcessor`, `MonoBehaviour`, `GameStateProcessor`, `PlayerProcessor`, `ScriptableObject`, `WorldGenRuntimeData`, `UserInterface_Debug`, `GridProcessor`, `ResourceProcessor`, `DebugProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `Player`, `CellSpacePartitioning`, `FoliageProcessor`, `Coordinator`, `GameEvent`, `EnemySpawner`, `IProcessor`, `SaveProcessor`, `AIPath`, `PlayerSaveData`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `Target`, `BuildingProcessor`, `TwitchChatProcessor`, `.AddEvent`, `GUIDComponent`, `HealthHandler`, `ObjectPoolingProcessor`, `Character`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `MiscCommands`, `StationProcessor`, `UserInterface_Debug`, `CommandDictionary`, `CharacterModelHandler`, `PlayerCommands`, `RoleHandler`, `BuildingBase`, `VFXArrowPointer`, `UserInterface_DisplayUsernames`, `UnitTextDisplay`, `TargetSensor`, `SaveProcessor`, `VoteEvent`, `RoleData`, `.SetTargetType`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _283 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Target` be split into smaller, more focused modules?**
  _Cohesion score 0.09653092006033183 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.058699101004759384 - nodes in this community are weakly interconnected._
- **Should `Res` be split into smaller, more focused modules?**
  _Cohesion score 0.04283870967741935 - nodes in this community are weakly interconnected._