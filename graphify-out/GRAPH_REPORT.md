# Graph Report - Stream-Town-Bevy  (2026-08-21)

## Corpus Check
- 638 files · ~1,667,677 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7880 nodes · 22001 edges · 261 communities (238 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1020 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `62014cf0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- World.Generation.Settings
- BuildingProcessor
- world.rs
- BTreeMap
- ScriptableObject
- .default
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- Option
- SettingsProcessor
- SaveDataMapper
- GameStateProcessor
- Commands
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- stream_town_migrate/src/presentation.rs
- Station
- DayAndNightProcessor
- GUIDProcessor
- BuildingPlacer
- .EnsureValidCredentials
- UnitHealthBar
- MiscCommands
- Res
- TechTreeNode
- SaveFileData
- Player
- simulation.rs
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- MeshData
- RoleProcessor
- CommandDictionary
- SettingsData
- SeasonProcessor
- StableId
- DebugProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- stream_town_game/src/lib.rs
- String
- legacy.rs
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- PlayerSettings
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Utils
- .SetTargetType
- STSM_Idle_Player
- TechTreeEditorWindow
- Result
- UserInterface
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- command.rs
- IntWrapper
- models.rs
- Tiler
- ScriptablesEditor
- StationSensor
- UserInterface_ObjectSelection
- StreamUserType
- TL_Secrets
- AnimationHandler
- TwitchBotSetupWindow
- EnemyModelHandler
- WorldUtils
- .UserIsSubscribed
- String
- Access_Text
- PlayerRoleData
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- CommonEnums.cs
- RaidEvent
- RoleHandler
- STSM_StateAction
- convert_fbx_to_glb.py
- RoleDataContainer
- SelectedObject
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- IProcessor.cs
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- UIElementWrapper
- PlayerInventory
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- UserInterface_RulerVote
- add_rotation_curve
- .new
- Buildings
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- .OnGUI
- unity_color_filter
- SimpleMusicController
- SnapToGridMouseMovement
- AIPath
- BuildingDamageMaterialHandler
- VfxAnimationController
- KeepKingVote
- UpdateGraphBounds
- GlobalAudioController
- WindController
- Enemy
- UI_TechOption
- RoleSlotModifier
- ConfirmCheck
- SimpleDisableAfterTime
- ToolState
- GateController
- SelectedPlayerGroup
- IProcessor
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- .Log
- RuntimeData Template
- Key Rules
- SelectedBuilding
- Pet
- add_file
- SelectedEnemyCamp
- DontDestroyOnLoad
- CampGenerationSettings
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- .RefreshSceneBindingsAndTryGenerate
- DebugSettings
- UnitTextDisplay
- Stream Town Reloaded - Architecture Documentation
- NewKingVote
- .InitializeAndActivateProcessorsAsync
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- AutosaveIntervalsInstaller
- IRuntimeDataScriptable
- Access_Toggle
- ChangeTimeStamp
- GridNode
- ResourceProcessor
- xtask/src/main.rs
- SelectedPlayer
- EnemyCampSaveData
- .DrawDataFieldAndLabel
- TL_API
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- ObjectiveSaveData
- SelectedEnemy
- Age
- FoliageSaveData
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- TwitchClientRuntimeData
- MonoBehaviour
- GridProcessor
- UserInterface_GameMenu
- StringUtils
- Targetable
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SelectedResource
- SavingAndLoading.Structs
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- SimpleScreenShot
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- Requirement
- TerrainGenSettings
- Key Rules
- ScriptablesProcessorInfrastructure
- RuntimeData Template
- ScriptKeywordProcessor
- FPSDisplay
- Processor Template
- Common Patterns
- PlayerSaveData
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
- VfxParticlePosition
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxSeagullSpawner
- extraction-spec.md
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Q: If there is more to do, keep going.
- .GenerateFromSettings
- VFXArrowPointer
- stream_town_domain
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Processors
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- RandomEnabler

## God Nodes (most connected - your core abstractions)
1. `StableId` - 330 edges
2. `WorldSimulation` - 164 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 129 edges
8. `WorldGenProcessor` - 114 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `combat_goal_damages_kills_and_respawns()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `falling_fish_uses_authored_gravity_terrain_bounce_and_lifetime_loss()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `food_roles_only_select_their_authored_target_types()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (261 total, 23 thin omitted)

### Community 0 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (35): BuildingBase, bool, float, int, List, UnityEvent, TilerBuilding, bool (+27 more)

### Community 2 - "world.rs"
Cohesion: 0.06
Nodes (62): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+54 more)

### Community 3 - "BTreeMap"
Cohesion: 0.11
Nodes (34): animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert_materials(), convert_model_materials() (+26 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.03
Nodes (78): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+70 more)

### Community 5 - ".default"
Cohesion: 0.04
Nodes (88): EnemySpawnerDef, WeightedEnemySpawn, generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), append_terrain_quad(), append_terrain_skirt(), archetype_id_by_source() (+80 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Option"
Cohesion: 0.03
Nodes (114): Assets, ArchetypeDef, ArchetypeKind, ArchetypeScene, HealthDef, RotatingNodeDef, Option, actor_detail_budget() (+106 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "SaveDataMapper"
Cohesion: 0.11
Nodes (13): List, Mesh, Vector3, SaveDataMapper, bool, int, MeshSaveData, int (+5 more)

### Community 12 - "GameStateProcessor"
Cohesion: 0.15
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 13 - "Commands"
Cohesion: 0.06
Nodes (129): BackgroundColor, PresentationCatalog, RainingFishVfxDef, actor_material(), actor_scene_budget(), advance_falling_fish(), AgentAnimation, animate_agents() (+121 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.13
Nodes (8): Func, Action, bool, float, int, UnityEvent, HealthHandler, ReviveType

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (45): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, EnemyDef, EnemyModelSetDef (+37 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (84): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert(), convert_avatar_masks(), convert_chimney_smoke() (+76 more)

### Community 19 - "Station"
Cohesion: 0.08
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 20 - "DayAndNightProcessor"
Cohesion: 0.13
Nodes (7): Container, ContainerBuilder, DayAndNightProcessor, IPostInitializeProcessor, bool, float, DayAndNightRuntimeData

### Community 21 - "GUIDProcessor"
Cohesion: 0.12
Nodes (6): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData

### Community 22 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 26 - "Res"
Cohesion: 0.03
Nodes (281): Aabb, AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, AnimationGraphHandle, AnimationTransitions, AppExit (+273 more)

### Community 27 - "TechTreeNode"
Cohesion: 0.05
Nodes (34): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+26 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, ModeratorCommands (+4 more)

### Community 30 - "simulation.rs"
Cohesion: 0.06
Nodes (41): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+33 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (18): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+10 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, bool, ParticleSystem (+6 more)

### Community 34 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 35 - "RoleProcessor"
Cohesion: 0.15
Nodes (4): Container, ContainerBuilder, int, RoleProcessor

### Community 36 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.06
Nodes (26): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+18 more)

### Community 39 - "StableId"
Cohesion: 0.04
Nodes (192): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, ObjectiveDef, RoleDef, StationDef, FromStr (+184 more)

### Community 40 - "DebugProcessor"
Cohesion: 0.05
Nodes (26): IMainThreadInitializableProcessor, Container, ContainerBuilder, DebugProcessor, Camera, Container, ContainerBuilder, InputButton (+18 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.05
Nodes (19): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+11 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (125): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+117 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+18 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (219): AnyResult, ActorHealthOverlay, adjust_settings_menu(), advance_loading_runtime(), AgentCommandQueue, animate_fish_school(), animate_loading_icon(), apply_loading_icon_rotation() (+211 more)

### Community 46 - "String"
Cohesion: 0.08
Nodes (59): animation_take_name(), clip_id(), convert_clips(), convert_embedded_model_clips(), convert_post_process(), field_bool(), field_f32(), field_str() (+51 more)

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (39): ActorCustomization, binary_fixture(), BinaryParser, clamped_cell(), decode_json(), json_active_goal(), json_buildings(), json_customization() (+31 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.11
Nodes (19): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+11 more)

### Community 49 - ".Draw"
Cohesion: 0.08
Nodes (24): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Port, VisualElement, Button, EnumField (+16 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 52 - "PlayerSettings"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Utils"
Cohesion: 0.06
Nodes (10): STStateMachine.States, Utils, Behaviours, Animation, Sensors, STStateMachine, Pathfinding, STStateMachine.Helpers (+2 more)

### Community 57 - ".SetTargetType"
Cohesion: 0.15
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 58 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (24): int, STSM_Helper_Build, STSM_HelperDeposit, STSM_Action_Build, float, STSM_Action_DepositResource, bool, float (+16 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (8): bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow, EditorWindow

### Community 60 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 61 - "UserInterface"
Cohesion: 0.05
Nodes (18): InputButton, SharedTypes, DataStructures, TownGoal.Data, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Core (+10 more)

### Community 62 - "CameraController"
Cohesion: 0.08
Nodes (12): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+4 more)

### Community 63 - "Node_SO"
Cohesion: 0.12
Nodes (13): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+5 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.09
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "twitch.rs"
Cohesion: 0.06
Nodes (54): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet, Default, Result, Self (+46 more)

### Community 68 - "Objective"
Cohesion: 0.08
Nodes (13): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+5 more)

### Community 69 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 70 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.13
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (11): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+3 more)

### Community 74 - "StationSensor"
Cohesion: 0.12
Nodes (5): List, SensorBase, UnityEvent, StationSensor, StationMask

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "StreamUserType"
Cohesion: 0.50
Nodes (3): StreamUserType, should_show_actor_name(), legacy_user_type()

### Community 77 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 78 - "AnimationHandler"
Cohesion: 0.09
Nodes (14): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+6 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 83 - "String"
Cohesion: 0.04
Nodes (91): AnimationGraph, AnimationNodeIndex, AnimationPlayer, active_event_text(), ActivePetVisual, add_animation_composition(), add_animation_layer_branch(), advance_animation_crossfade() (+83 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRoleData"
Cohesion: 0.09
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.08
Nodes (12): RotationHandler, float, Quaternion, Vector3, ProjectileShooter, float, int, string (+4 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "CommonEnums.cs"
Cohesion: 0.17
Nodes (10): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, RunAnimation, Seasons, TimeOfDay (+2 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (17): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+9 more)

### Community 92 - "RoleHandler"
Cohesion: 0.06
Nodes (25): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+17 more)

### Community 93 - "STSM_StateAction"
Cohesion: 0.05
Nodes (18): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, int, STSM_Helper_Attack (+10 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "RoleDataContainer"
Cohesion: 0.12
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 96 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 97 - "SaveProcessor"
Cohesion: 0.05
Nodes (37): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+29 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (77): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef (+69 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.09
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 102 - "IProcessor.cs"
Cohesion: 0.22
Nodes (8): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport, ProcessorStartupStage

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.07
Nodes (10): SortBuildingByLowerLevel, EventType, Action, bool, double, object, EventType, GameEvent (+2 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

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
Cohesion: 0.11
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 114 - "add_rotation_curve"
Cohesion: 0.31
Nodes (10): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), safe_ratio() (+2 more)

### Community 115 - ".new"
Cohesion: 0.09
Nodes (38): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+30 more)

### Community 116 - "Buildings"
Cohesion: 0.07
Nodes (12): BuildCostModifier, PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Level, GridSystem.Partitioning, Buildings (+4 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.18
Nodes (10): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+2 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (36): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer (+28 more)

### Community 120 - ".OnGUI"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 126 - "VfxAnimationController"
Cohesion: 0.07
Nodes (13): PersistentScoped, Transform, PlayerSpawnPoint, Image, TextMeshProUGUI, UIRoleDisplay, float, Vector3 (+5 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "Enemy"
Cohesion: 0.07
Nodes (20): CollectResource, Action, float, Enemy, AnimationCurve, bool, int, object (+12 more)

### Community 132 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (86): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+78 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectedPlayerGroup"
Cohesion: 0.17
Nodes (3): List, List, SelectedPlayerGroup

### Community 139 - "IProcessor"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, LabelDisplayProcessor, TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, IProcessor (+14 more)

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
Cohesion: 0.05
Nodes (33): HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder (+25 more)

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

### Community 151 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 155 - "DebugSettings"
Cohesion: 0.48
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 156 - "UnitTextDisplay"
Cohesion: 0.11
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 159 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.17
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 163 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 164 - "IRuntimeDataScriptable"
Cohesion: 0.06
Nodes (25): Container, ContainerBuilder, TimeProcessor, CreditsRuntimeData, IRuntimeDataScriptable, bool, Dictionary, InputButton (+17 more)

### Community 165 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 166 - "ChangeTimeStamp"
Cohesion: 0.50
Nodes (3): int, ChangeTimeStamp, DateTime

### Community 167 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 171 - "EnemyCampSaveData"
Cohesion: 0.50
Nodes (3): int, uint, EnemyCampSaveData

### Community 172 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 178 - "Age"
Cohesion: 0.06
Nodes (25): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+17 more)

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

### Community 185 - "MonoBehaviour"
Cohesion: 0.01
Nodes (112): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder (+104 more)

### Community 187 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 192 - "Targetable"
Cohesion: 0.10
Nodes (12): List, Dictionary, List, TargetRuntimeData, bool, BoxCollider, float, int (+4 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "SavingAndLoading.Structs"
Cohesion: 0.06
Nodes (15): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+7 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 200 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

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

### Community 206 - "WorldSaveData"
Cohesion: 0.13
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 213 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "PlayerSaveData"
Cohesion: 0.08
Nodes (19): ResourceInventory, bool, int, Dictionary, bool, int, List, string (+11 more)

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
Cohesion: 0.13
Nodes (6): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "VfxParticlePosition"
Cohesion: 0.22
Nodes (4): Transform, VisualEffect, VfxParticlePosition, VFX

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Q: How are shipping visual and audio parity implemented in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How are shipping visual and audio parity implemented in the Bevy migration?, Source Nodes

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

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

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - ".GenerateFromSettings"
Cohesion: 0.07
Nodes (28): HashSet, Func, HashSet, Vector2, Vector3, Action, IEnumerator, Vector2 (+20 more)

### Community 268 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 277 - "Processors"
Cohesion: 0.06
Nodes (10): ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, Processors.Editor, MetaData, Audio, Settings (+2 more)

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

## Knowledge Gaps
- **293 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+288 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `RenderAssets` (4× useful, score=3.481802588) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.66401759)
- `WorldSnapshot` (3× useful, score=2.54480177)
- `WorldSimulation` (2× useful, score=1.847279109)
- `load_input()` (2× useful, score=1.696288071) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (2× useful, score=1.665761652) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.664595804)
- `MaterialDef` (2× useful, score=1.664127343)
- `BevyMigrationExporter` (2× useful, score=1.63601087)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `SettingsProcessor`, `SaveDataMapper`, `IProcessor`, `.Log`, `GUIDProcessor`, `SaveFileData`, `WorldGenProcessor`, `GameEventProcessor`, `RoleProcessor`, `IRuntimeDataScriptable`, `SeasonProcessor`, `ResourceProcessor`, `DebugProcessor`, `TechTreeProcessor`, `ResourceDataSaveData`, `StreamTownSessionBridge`, `MonoBehaviour`, `UserInterface`, `UserInterface_GameMenu`, `WorldSaveData`, `FoliageProcessor`, `PlayerProcessor`, `TownGoalProcessor`, `MainMenuManager`, `Resource`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `.GenerateFromSettings`, `IProcessor`, `GameStateProcessor`, `.Log`, `GUIDProcessor`, `WorldGenRuntimeData`, `Player`, `UserInterface_Debug`, `ResourceProcessor`, `DebugProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `MonoBehaviour`, `GridProcessor`, `TerrainGenSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `EnemySpawner`, `Buildings`, `AIPath`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `world.rs`, `BTreeMap`, `.default`, `ToolState`, `Option`, `Commands`, `stream_town_domain/src/content.rs`, `save.rs`, `stream_town_migrate/src/presentation.rs`, `Res`, `simulation.rs`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `stream_town_game/src/lib.rs`, `String`, `twitch.rs`, `command.rs`, `String`, `stream_town_domain/src/presentation.rs`, `.new`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _293 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World.Generation.Settings` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.037502379592613745 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05871725383920506 - nodes in this community are weakly interconnected._