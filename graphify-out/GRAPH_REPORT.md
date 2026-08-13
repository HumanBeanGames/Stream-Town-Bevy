# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 610 files · ~1,614,733 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7166 nodes · 19062 edges · 267 communities (247 shown, 20 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 995 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0434a6cb`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- xtask/src/main.rs
- BuildingProcessor
- generate_and_spawn_world
- stream_town_migrate/src/presentation.rs
- drive_converted_animations
- STSM_StateAction
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- Target
- SettingsProcessor
- .Log
- legacy.rs
- UserInterface
- TechTreeIOUtility
- STSM_Idle_Player
- process_injected_commands
- SensorProcessor
- WorldGenProcessor
- IProcessor.cs
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- RoleHandler
- Option
- menu_input
- SaveFileData
- GameEventProcessor
- StableId
- PlayerRoleData
- STSM_Action_PlayerBase
- RoleDataSettings
- save.rs
- UserInterface_Debug
- CommandDictionary
- SettingsData
- AnimationHandler
- Result
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- WorldSimulation
- AnimationControllerDef
- BTreeMap
- SeasonProcessor
- GUIDProcessor
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Access_Dropdown
- simulation.rs
- STSM_GoToLocation
- TechTreeEditorWindow
- BuildingScriptablesEditor.cs
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- BinaryWriter
- SimulationError
- models.rs
- Tiler
- ScriptablesEditor
- GridProcessor.cs
- UserInterface_ObjectSelection
- SelectedPlayer
- ProjectCamera
- BinarySaveCodec
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- ResourceData
- convert
- Access_Text
- stream_town_game/src/lib.rs
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- DebugProcessor
- BuildingBase
- PlayerCommands
- GameEvent
- convert_fbx_to_glb.py
- command.rs
- stream_town_migrate/src/content.rs
- EditorUtils
- Coordinator
- stream_town_domain/src/presentation.rs
- Globals
- ObjectPoolingSettings
- PlayerInventory
- TownGoalProcessor
- MainMenuManager
- UnitHealthBar
- LoadingManager
- PlayerSaveData
- TechTreeNode
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- EnemyModelHandler
- Targetable
- LabelDisplayProcessor
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- SelectableObject
- RaidEvent
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- StationProcessor
- UserInterface_GameMenu
- TransformSaveData
- UpdateGraphBounds
- ResourceRuntimeData
- .EnsureValidCredentials
- Res
- BuildingSettings
- EventProcessor
- ConfirmCheck
- SaveProcessor.cs
- ToolState
- UserInterface.MainMenu
- Goal
- stream_town_domain/src/lib.rs
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- RoleSlot
- Requirement
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- SeasonDataSettings
- IProcessor
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- WindController
- Editor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- MiscCommands
- WorldGenSaveData
- GridProcessor
- Easings
- UIElementWrapper
- RoleProcessor
- TimeProcessor
- TL_Secrets
- UserInterface_RulerVote
- SelectedBuilding
- EditorHelpers
- BuildingResourceModelHandler
- SelectedEnemy
- .UserIsSubscribed
- UserInterface_TownGoal
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Enemy
- GameStateProcessor
- .RenderResourceType
- STSM_Helper_Attack
- IRuntimeDataScriptable
- CommonEnums.cs
- PlacementProbeHandler
- .tick
- .ValidateTokenAsync
- settings.rs
- Player
- BuildingDamageMaterialHandler
- SimpleDisableAfterTime
- GenerationSettings
- PlayerInputRuntimeData
- UILineRenderer
- UserInterface_DisplayUsernames
- PostProcessingInstaller
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- WeatherProcessor
- MonoBehaviour
- Key Rules
- StringUtils
- Common Patterns
- PlayerDeathHandler
- IntWrapper
- AudioMixerInstaller
- UI_TechOption
- ScriptablesProcessorInfrastructure
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- AutosaveIntervalsInstaller
- VfxParticlePosition
- ScriptKeywordProcessor
- ForwardRendererInstaller
- FPSDisplay.cs
- Processor Template
- Common Patterns
- RenderPipelineInstaller
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- STSM_HelperBase
- Twitch setup
- graphify reference: add a URL and watch a folder
- VideoSettingsPresetsInstaller
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Utils
- ScriptableObjectAssetData
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- SimpleRotateOnAxis
- CreateDefaultSettingsAssets.cs
- KeepKingVote
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- CellSpacePartitioningInstaller
- RotationHandler
- TwitchClientRuntimeData
- CustomLogger
- NewKingVote
- extraction-spec.md
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- ObjectiveSaveData
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- EquipmentHandlerEditor
- FoliageGenerationSettings.cs
- UnityGraphics
- TL_API
- ObjectSelectionProcessor.Editor.cs
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- Autosave

## God Nodes (most connected - your core abstractions)
1. `StableId` - 258 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldSimulation` - 138 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `ContentCatalog` - 90 edges

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

## Communities (267 total, 20 thin omitted)

### Community 0 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (16): BuildPlacerData, GameObject, Renderer, string, Vector2, Container, ContainerBuilder, Dictionary (+8 more)

### Community 2 - "generate_and_spawn_world"
Cohesion: 0.07
Nodes (84): GameConfig, GameplayConfig, BTreeMap, WorldGenConfig, FoliageHabitat, FoliageLayerDef, GridPos, authored_foliage_is_deterministic_and_respects_habitat_and_resources() (+76 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "drive_converted_animations"
Cohesion: 0.06
Nodes (62): AnimationClip, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, add_animation_layer_branch(), add_rotation_curve() (+54 more)

### Community 5 - "STSM_StateAction"
Cohesion: 0.18
Nodes (5): bool, float, int, STSM_StateAction, AnimationName

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Target"
Cohesion: 0.08
Nodes (16): STStateMachine.States, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors, GridSystem.Partitioning (+8 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - ".Log"
Cohesion: 0.11
Nodes (7): Action, GameObject, IEnumerable, HideInCallstack, Object, DebugLogCategory, Vector2Int

### Community 12 - "legacy.rs"
Cohesion: 0.13
Nodes (49): ActorCustomization, StreamUserType, binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal() (+41 more)

### Community 13 - "UserInterface"
Cohesion: 0.04
Nodes (21): int, TechTreeSettings, InputButton, SharedTypes, int, ChangeTimeStamp, NodeGroup_SO, List (+13 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "STSM_Idle_Player"
Cohesion: 0.08
Nodes (11): STSM_Action_GatherResource, bool, float, Vector3, STSM_Idle_Enemy, bool, float, uint (+3 more)

### Community 16 - "process_injected_commands"
Cohesion: 0.08
Nodes (45): active_event_text(), AuthoredCreditsElement, bottom_bar_authored_order(), bottom_bar_entries(), BottomBarAction, building_definition_id(), building_icon_path(), building_model_node_names() (+37 more)

### Community 17 - "SensorProcessor"
Cohesion: 0.12
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 18 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (28): HashSet, bool, BoxCollider, Container, ContainerBuilder, Func, HashSet, int (+20 more)

### Community 19 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.06
Nodes (24): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+16 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.10
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 24 - "HealthHandler"
Cohesion: 0.14
Nodes (6): Action, bool, float, int, UnityEvent, HealthHandler

### Community 25 - "RoleHandler"
Cohesion: 0.07
Nodes (18): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+10 more)

### Community 26 - "Option"
Cohesion: 0.04
Nodes (115): AssetServer, BackgroundColor, PresentationCatalog, actor_material(), animation_property_value(), apply_material_overrides(), bottom_bar_texture(), building_effect_material() (+107 more)

### Community 27 - "menu_input"
Cohesion: 0.08
Nodes (48): AppExit, bottom_bar_action_buttons(), bottom_bar_input(), bottom_bar_main_buttons(), bottom_bar_scroll_buttons(), BottomBarContext, BottomBarMainButton, BottomBarRuntime (+40 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "StableId"
Cohesion: 0.04
Nodes (156): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+148 more)

### Community 31 - "PlayerRoleData"
Cohesion: 0.10
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 32 - "STSM_Action_PlayerBase"
Cohesion: 0.09
Nodes (10): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_Heal (+2 more)

### Community 33 - "RoleDataSettings"
Cohesion: 0.16
Nodes (11): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+3 more)

### Community 34 - "save.rs"
Cohesion: 0.15
Nodes (31): detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native(), native_save_is_atomic_and_keeps_backup() (+23 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 39 - "Result"
Cohesion: 0.25
Nodes (7): BinaryParser<'a>, decode_binary(), FnMut, Result, Self, T, LegacyWorldState

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "WorldSimulation"
Cohesion: 0.11
Nodes (16): EnemyCampState, objective_increment(), ObjectiveEvent, RaidState, BTreeMap, BTreeSet, Option, Vec (+8 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.08
Nodes (14): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+6 more)

### Community 47 - "GUIDProcessor"
Cohesion: 0.11
Nodes (6): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData

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
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.11
Nodes (14): Container, ContainerBuilder, float, int, List, Material, materials, Mesh (+6 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 57 - "simulation.rs"
Cohesion: 0.08
Nodes (26): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+18 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 61 - "StateMachine"
Cohesion: 0.13
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
Nodes (52): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet, Default, Result, Self (+44 more)

### Community 68 - "Objective"
Cohesion: 0.14
Nodes (5): Action, int, Objective, ObjectiveType, ObjectiveData

### Community 69 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 70 - "SimulationError"
Cohesion: 0.17
Nodes (6): capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), Result, SimulationError, technology_vote_starts_persistent_goal_and_unlocks_after_all_objectives(), validate_trade_resource()

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "GridProcessor.cs"
Cohesion: 0.28
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.10
Nodes (14): SelectedEnemyCamp, BoxCollider, Button, GameObject, Image, List, object, Slider (+6 more)

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.14
Nodes (13): bool, CancellationTokenSource, int, long, MenuItem, string, DeviceCodeResponse, ErrorResponse (+5 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "ResourceData"
Cohesion: 0.17
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 83 - "convert"
Cohesion: 0.11
Nodes (28): ActorKind, SavedActor, absolute_path(), actor_prefix(), backup_candidate(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id() (+20 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (153): AnyResult, actor_detail_budget(), actor_scene_budget(), adjust_settings_menu(), AgentCommand, AgentCommandQueue, AgentEquipmentPresentation, AmbienceAudio (+145 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.14
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "DebugProcessor"
Cohesion: 0.12
Nodes (10): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, DebugProcessor, int (+2 more)

### Community 91 - "BuildingBase"
Cohesion: 0.12
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 92 - "PlayerCommands"
Cohesion: 0.13
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 93 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (106): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+98 more)

### Community 97 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "Globals"
Cohesion: 0.18
Nodes (3): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, Globals

### Community 101 - "ObjectPoolingSettings"
Cohesion: 0.18
Nodes (9): bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller, GameObject, int, string (+1 more)

### Community 102 - "PlayerInventory"
Cohesion: 0.22
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.11
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "PlayerSaveData"
Cohesion: 0.06
Nodes (29): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, bool (+21 more)

### Community 108 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

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
Cohesion: 0.11
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "SelectedObject"
Cohesion: 0.12
Nodes (4): object, UnityAction, SelectedObject, SelectedResource

### Community 114 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 115 - "Targetable"
Cohesion: 0.05
Nodes (27): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+19 more)

### Community 116 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.05
Nodes (23): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, float (+15 more)

### Community 120 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 121 - "RaidEvent"
Cohesion: 0.07
Nodes (19): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+11 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "StationProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 127 - "TransformSaveData"
Cohesion: 0.08
Nodes (18): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+10 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 130 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 131 - "Res"
Cohesion: 0.06
Nodes (141): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, App, Assets, AudioSink, ActorAnimationDriver (+133 more)

### Community 132 - "BuildingSettings"
Cohesion: 0.11
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 133 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "SaveProcessor.cs"
Cohesion: 0.07
Nodes (20): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+12 more)

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (86): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+78 more)

### Community 137 - "UserInterface.MainMenu"
Cohesion: 0.14
Nodes (5): ContainerBuilder, MetaDataInstaller, UserInterface.MainMenu, MetaData, Settings

### Community 138 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

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
Cohesion: 0.06
Nodes (28): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+20 more)

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

### Community 149 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 150 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 154 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 155 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "Editor"
Cohesion: 0.20
Nodes (4): WindControllerEditor, CellPartitioningEditor, GridSystemEditor, Editor

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

### Community 165 - "MiscCommands"
Cohesion: 0.11
Nodes (11): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData, Dictionary, MiscCommands (+3 more)

### Community 166 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 167 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 169 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 170 - "RoleProcessor"
Cohesion: 0.07
Nodes (9): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, List (+1 more)

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 174 - "SelectedBuilding"
Cohesion: 0.12
Nodes (4): bool, float, PassiveResourceIncrementer, SelectedBuilding

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

### Community 178 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 179 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Enemy"
Cohesion: 0.07
Nodes (20): CollectResource, Action, float, Enemy, AnimationCurve, bool, int, object (+12 more)

### Community 183 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 184 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 185 - "STSM_Helper_Attack"
Cohesion: 0.18
Nodes (4): int, STSM_Helper_Attack, int, STSM_Action_Attack

### Community 186 - "IRuntimeDataScriptable"
Cohesion: 0.10
Nodes (18): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+10 more)

### Community 187 - "CommonEnums.cs"
Cohesion: 0.18
Nodes (10): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, ReviveType, Seasons, TimeOfDay (+2 more)

### Community 189 - ".tick"
Cohesion: 0.29
Nodes (3): healing_and_food_revives_preserve_health_invariants(), ruler_vote_rejects_duplicates_and_invalid_candidates(), scheduled_ruler_elections_pause_resolve_and_restore_roles()

### Community 190 - ".ValidateTokenAsync"
Cohesion: 0.33
Nodes (6): CancellationToken, Dictionary, Task, UnityWebRequest, TokenValidationResponse, WebResponse

### Community 191 - "settings.rs"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 192 - "Player"
Cohesion: 0.07
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+1 more)

### Community 193 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 194 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 195 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 196 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 200 - "BuildingModelHandler"
Cohesion: 0.18
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 204 - "MonoBehaviour"
Cohesion: 0.03
Nodes (54): CameraProcessor, PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+46 more)

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "PlayerDeathHandler"
Cohesion: 0.25
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 209 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 211 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 212 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 213 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (82): ContainerBuilder, InstantiationBarrier, ChannelDataInstaller, ContainerBuilder, ContainerBuilder, SaveStateInstaller, ContainerBuilder, AllBuildingDataSettingsInstaller (+74 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 218 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 221 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 225 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "Utils"
Cohesion: 0.05
Nodes (13): BuildCostModifier, InputButton, PlayerControls.ObjectSelection, Utils, Processors, World, Level, Buildings (+5 more)

### Community 235 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

### Community 238 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.08
Nodes (22): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+14 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 247 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 251 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 262 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 264 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 280 - "ScriptableObject"
Cohesion: 0.02
Nodes (83): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+75 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

## Knowledge Gaps
- **232 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+227 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **20 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `SaveProcessor.cs`, `SettingsProcessor`, `ObjectPoolingProcessor`, `WorldGenProcessor`, `PlayerProcessor`, `ScriptableObject`, `IProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `TechTreeProcessor`, `RoleProcessor`, `TimeProcessor`, `SeasonProcessor`, `GUIDProcessor`, `StreamTownSessionBridge`, `ResourceProcessor`, `MonoBehaviour`, `ScriptablesProcessorInfrastructure`, `FoliageProcessor`, `DebugProcessor`, `PlayerCommands`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `PlayerSaveData`, `Resource`, `UserInterface_GameMenu`, `TransformSaveData`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `Target`, `.Log`, `ObjectPoolingProcessor`, `PlayerProcessor`, `ScriptableObject`, `IProcessor`, `UserInterface_Debug`, `GridProcessor`, `GUIDProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `GameStateProcessor`, `IRuntimeDataScriptable`, `Player`, `MonoBehaviour`, `ProjectCamera`, `ResourceData`, `ScriptablesProcessorInfrastructure`, `CellSpacePartitioning`, `FoliageProcessor`, `DebugProcessor`, `PlayerSaveData`, `EnemySpawner`, `SaveProcessor`, `RaidEvent`, `AIPath`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `BuildingProcessor`, `TwitchChatProcessor`, `SaveProcessor.cs`, `Target`, `ObjectPoolingProcessor`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `RoleHandler`, `GameEventProcessor`, `PlayerRoleData`, `UserInterface_Debug`, `CommandDictionary`, `MiscCommands`, `CharacterModelHandler`, `Enemy`, `IRuntimeDataScriptable`, `UserInterface_DisplayUsernames`, `TargetSensor`, `PlayerCommands`, `SaveProcessor`, `LabelDisplayProcessor`, `VoteEvent`, `.SetTargetType`, `StationProcessor`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _232 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.060451977401129946 - nodes in this community are weakly interconnected._
- **Should `generate_and_spawn_world` be split into smaller, more focused modules?**
  _Cohesion score 0.06862745098039216 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/presentation.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10367063492063493 - nodes in this community are weakly interconnected._