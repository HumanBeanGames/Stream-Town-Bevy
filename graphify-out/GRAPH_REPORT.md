# Graph Report - Stream-Town-Bevy  (2026-08-14)

## Corpus Check
- 630 files · ~1,643,871 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7616 nodes · 20824 edges · 264 communities (246 shown, 18 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1012 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c5027ac9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- recruit_group_selection_input
- BuildingProcessor
- .default
- stream_town_migrate/src/content.rs
- ScriptableObject
- WorldGenProcessor
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- simulation.rs
- SettingsProcessor
- ResMut
- GUIDComponent
- Option
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- SelectedObject
- StationProcessor
- .CreateEnumField
- stream_town_migrate/src/presentation.rs
- BuildingPlacer
- PlayerProcessor
- UnitHealthBar
- Targetable
- Query
- TechTreeGraphView
- SaveFileData
- Player
- stream_town_game/src/lib.rs
- runtime_console.rs
- UserInterface_Debug
- GameEventProcessor
- MeshData
- .LogWarning
- CommandDictionary
- SettingsData
- SeasonProcessor
- StableId
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- world.rs
- AnimationControllerDef
- parse_transform_tracks
- config.rs
- legacy.rs
- group_selection_action_buttons
- .Draw
- AudioHandler
- StreamTownSessionBridge
- Utils
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- RoleData
- TargetProcessor
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- Res
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- settings.rs
- MonoBehaviour
- models.rs
- Tiler
- ScriptablesEditor
- LegacyEntity
- UserInterface_ObjectSelection
- TimeProcessor
- Goal
- AnimationHandler
- TwitchBotSetupWindow
- MiscCommands
- WorldUtils
- PlayerInventory
- .new
- Access_Text
- UserInterface_GameMenu.cs
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- PlayerRoleData
- PoolableObject
- command.rs
- technology_tab
- convert_fbx_to_glb.py
- Result
- CommonEnums.cs
- .LoadGameAsync
- Coordinator
- stream_town_domain/src/presentation.rs
- IProcessor
- StateMachine
- BuildingBase
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- SensorProcessor
- UserInterface_TownGoal
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- GameStateProcessor
- UIElementWrapper
- PassiveResourceIncrementer
- NavGrid
- GridProcessor.cs
- stream_town_migrate/src/main.rs
- VoteEvent
- IInstaller
- .GenerateFromSettings
- unity_color_filter
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- VFXArrowPointer
- PlayerInputRuntimeData
- EditorUtils
- UpdateGraphBounds
- GlobalAudioController
- WindController
- Station
- Access_Toggle
- BuildingSettings
- ConfirmCheck
- UnitTravelToPosition
- ToolState
- GateController
- PlayerRole
- StatusBar
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- .Log
- RuntimeData Template
- Key Rules
- SelectedBuilding
- Pet
- add_file
- Enemy
- UserInterface_GameMenu
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- Vec
- RoleHandler
- setup_camera
- ResourceHolder
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- LabelDisplayProcessor
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- Audio
- UserInterface_RulerVote
- EditorHelpers
- SelectedEnemy
- TechTree.Elements
- NodeUnlockData
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SelectableObject
- FoliageGenerationSettings.cs
- DayAndNightProcessor
- GameEventSystem
- stream_town_domain/src/lib.rs
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- DebugProcessor
- UI_TechOption
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- SaveDataMapper
- Editor
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- Common Patterns
- Requirement
- KeepKingVote
- EventProcessor
- Key Rules
- ScriptablesProcessorInfrastructure
- RuntimeData Template
- BuildingModelHandler
- ScriptKeywordProcessor
- FPSDisplay.cs
- Processor Template
- Common Patterns
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- EquipmentHandlerEditor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- WorldSaveData
- CreateDefaultSettingsAssets.cs
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- ResourceDataSaveData
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- RotationHandler
- SimpleScreenShot
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- WeatherProcessor
- extraction-spec.md
- SaveProcessor
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- IntWrapper
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Easings
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Q: If there is more to do, keep going.
- StringUtils
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- TechTreeProcessor.cs
- Q: role level experience progression station equipment inventory skill upgrade
- NewKingVote
- BuildPlacerData
- RandomEnabler
- ScriptableObjectAssetData
- Autosave
- TechTreeNode
- OpenNode
- ObjectSelectionProcessor.Editor.cs
- GameSettings

## God Nodes (most connected - your core abstractions)
1. `StableId` - 304 edges
2. `WorldSimulation` - 162 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 126 edges
8. `WorldGenProcessor` - 110 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (264 total, 18 thin omitted)

### Community 0 - "recruit_group_selection_input"
Cohesion: 0.08
Nodes (49): AccumulatedMouseMotion, AccumulatedMouseScroll, ActorHealthFill, ActorHealthOverlay, ActorNameOverlay, apply_player_settings(), building_health_color(), building_health_fraction() (+41 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (12): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, STSM_HelperDeposit, BuildingType, foodCost (+4 more)

### Community 2 - ".default"
Cohesion: 0.05
Nodes (91): generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats(), authored_target_sizes_drive_unity_action_reach_formulas() (+83 more)

### Community 3 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (125): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+117 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.02
Nodes (81): ContainerBuilder, ResourceGenSettingsInstaller, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+73 more)

### Community 5 - "WorldGenProcessor"
Cohesion: 0.08
Nodes (14): bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int, IReadOnlyList (+6 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "simulation.rs"
Cohesion: 0.07
Nodes (31): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+23 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "ResMut"
Cohesion: 0.06
Nodes (75): AppExit, advance_loading_phase(), AgentCommand, AgentCommandQueue, authored_color_grading(), autosave_game(), bottom_bar_action_buttons(), bottom_bar_input() (+67 more)

### Community 12 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 13 - "Option"
Cohesion: 0.03
Nodes (124): AmbientLight, BackgroundColor, PresentationCatalog, actor_detail_budget(), animation_property_value(), animation_root_name(), authored_post_process_stack(), bottom_bar_texture() (+116 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (14): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, int, STSM_Helper_Attack, Action, bool (+6 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.06
Nodes (64): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+56 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "SelectedObject"
Cohesion: 0.08
Nodes (7): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup, SelectedResource

### Community 19 - "StationProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (85): PrefabPresentationBinding, animation_take_name(), animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), clip_id(), collect_prefab_dependencies() (+77 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (20): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+12 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.08
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "Targetable"
Cohesion: 0.12
Nodes (8): bool, BoxCollider, float, int, Transform, Vector3, Targetable, IPooledObjectReset

### Community 26 - "Query"
Cohesion: 0.04
Nodes (122): Added, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, Assets, active_event_text(), ActivePetVisual (+114 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (13): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, OnChatCommandReceivedArgs (+5 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (162): AnimationClip, AnimationTargetId, AnyResult, AudioSink, add_rotation_curve(), add_scale_curve(), add_translation_curve(), adjust_settings_menu() (+154 more)

### Community 31 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 35 - ".LogWarning"
Cohesion: 0.05
Nodes (18): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, int, STSM_Helper_Build (+10 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.07
Nodes (21): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+13 more)

### Community 39 - "StableId"
Cohesion: 0.03
Nodes (197): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, EnemyDef, ObjectiveDef, ObjectiveKind, ResourceReward (+189 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "world.rs"
Cohesion: 0.16
Nodes (26): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise(), generate_foliage(), generate_shoreline_fish() (+18 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+18 more)

### Community 45 - "parse_transform_tracks"
Cohesion: 0.33
Nodes (6): append_vec3_keys(), parse_inline_array(), parse_transform_tracks(), parses_unity_transform_curves_without_editor_types(), Item, Iterator

### Community 46 - "config.rs"
Cohesion: 0.16
Nodes (15): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Default, Result, Self, String (+7 more)

### Community 47 - "legacy.rs"
Cohesion: 0.23
Nodes (29): clamped_cell(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps(), json_f32_default() (+21 more)

### Community 48 - "group_selection_action_buttons"
Cohesion: 0.13
Nodes (20): BuildingLevelEnabled, BuildingRemoveLabel, group_selection_action_buttons(), group_selection_panel_signature(), GroupDismissLabel, GroupSelectionAction, GroupSelectionActions, GroupSelectionPanel (+12 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "Utils"
Cohesion: 0.04
Nodes (31): BuildCostModifier, InputButton, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Utils, Processors (+23 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (13): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Slider, TextMeshProUGUI, UIRuntimeData (+5 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 57 - "TargetProcessor"
Cohesion: 0.13
Nodes (10): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, List, TargetProcessor, Dictionary (+2 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.09
Nodes (13): bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation, bool (+5 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 61 - "Res"
Cohesion: 0.08
Nodes (116): AnimationTransitions, actor_material(), actor_scene_budget(), agent_is_moving(), AgentAnimation, animate_agents(), animate_building_effects(), animate_combat_effects() (+108 more)

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
Cohesion: 0.14
Nodes (5): Action, int, Objective, ObjectiveType, ObjectiveData

### Community 69 - "settings.rs"
Cohesion: 0.10
Nodes (34): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+26 more)

### Community 70 - "MonoBehaviour"
Cohesion: 0.01
Nodes (88): Api, CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, PersistentScoped, ContainerBuilder (+80 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (11): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+3 more)

### Community 74 - "LegacyEntity"
Cohesion: 0.16
Nodes (18): ActorCustomization, StreamUserType, should_show_actor_name(), ImportReport, legacy_objective_matches(), legacy_user_type(), LegacyDecodedSave, LegacyEntity (+10 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "TimeProcessor"
Cohesion: 0.18
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 77 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.05
Nodes (24): AnimationHandler, Animator, bool, Dictionary, float, int, bool, int (+16 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "PlayerInventory"
Cohesion: 0.17
Nodes (4): PlayerInventory, Dictionary, float, STSM_Action_DepositResource

### Community 83 - ".new"
Cohesion: 0.15
Nodes (22): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+14 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "UserInterface_GameMenu.cs"
Cohesion: 0.25
Nodes (3): ContainerBuilder, MetaDataInstaller, MetaData

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
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "PlayerRoleData"
Cohesion: 0.09
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 91 - "PoolableObject"
Cohesion: 0.06
Nodes (21): Container, ContainerBuilder, GUIDProcessor, CreditsRuntimeData, bool, GameStateRuntimeData, bool, Dictionary (+13 more)

### Community 92 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 93 - "technology_tab"
Cohesion: 0.31
Nodes (16): apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node(), refresh_technology_draft(), Option (+8 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "Result"
Cohesion: 0.10
Nodes (51): AvatarMaskDef, animation_state_id(), animation_state_machine_id(), avatar_mask_id(), controller_id(), convert_avatar_masks(), convert_fireworks(), f32_to_u16() (+43 more)

### Community 96 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (21): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+13 more)

### Community 97 - ".LoadGameAsync"
Cohesion: 0.07
Nodes (33): Action, CancellationToken, Task, Transform, int, List, string, uint (+25 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.06
Nodes (61): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef (+53 more)

### Community 100 - "IProcessor"
Cohesion: 0.08
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 102 - "BuildingBase"
Cohesion: 0.09
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.05
Nodes (23): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+15 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 108 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.11
Nodes (13): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+5 more)

### Community 112 - "GameStateProcessor"
Cohesion: 0.07
Nodes (14): Transform, float, int, List, Transform, EnemySpawner, Container, ContainerBuilder (+6 more)

### Community 113 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 114 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 115 - "NavGrid"
Cohesion: 0.17
Nodes (16): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building() (+8 more)

### Community 116 - "GridProcessor.cs"
Cohesion: 0.28
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "IInstaller"
Cohesion: 0.04
Nodes (34): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer (+26 more)

### Community 120 - ".GenerateFromSettings"
Cohesion: 0.08
Nodes (27): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+19 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 126 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 127 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "Station"
Cohesion: 0.09
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 132 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 133 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (46): bounded_ui_index(), content_tab(), default_catalog_path(), draw_world_preview(), inject_runtime_command(), inspector_tab(), launch_runtime_game(), main() (+38 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "PlayerRole"
Cohesion: 0.06
Nodes (15): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+7 more)

### Community 139 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

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
Nodes (23): Action, HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container (+15 more)

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

### Community 149 - "Enemy"
Cohesion: 0.21
Nodes (4): Action, float, Enemy, EnemyType

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.07
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Vec"
Cohesion: 0.24
Nodes (11): binary_fixture(), BinaryParser, put_f32(), put_i32(), put_string(), put_u32(), FnMut, Self (+3 more)

### Community 154 - "RoleHandler"
Cohesion: 0.09
Nodes (9): PlayerDeathHandler, bool, float, Vector3, RoleHandler, bool, Dictionary, UnityEvent (+1 more)

### Community 156 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

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

### Community 164 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+6 more)

### Community 167 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (19): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+11 more)

### Community 170 - "Audio"
Cohesion: 0.12
Nodes (3): RoleScriptablesEditor, ScriptablesEditor, Audio

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

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
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 186 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 187 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 190 - "GameEventSystem"
Cohesion: 0.07
Nodes (19): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+11 more)

### Community 191 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "DebugProcessor"
Cohesion: 0.13
Nodes (9): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, DebugLogCategory, DebugProcessor (+1 more)

### Community 195 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "SaveDataMapper"
Cohesion: 0.06
Nodes (25): ResourceInventory, bool, int, Dictionary, List, Mesh, Vector3, SaveDataMapper (+17 more)

### Community 200 - "Editor"
Cohesion: 0.12
Nodes (7): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "UnityGraphics"
Cohesion: 0.40
Nodes (4): Vector3, UnityGraphics, FieldInfo, ShadowResolution

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 213 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (76): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, AllRoleDataSettingsInstaller, ContainerBuilder, AudioSettingsInstaller, ContainerBuilder, BuildingConfigSettingsInstaller (+68 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 218 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

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

### Community 234 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 238 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 243 - "ResourceDataSaveData"
Cohesion: 0.13
Nodes (16): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+8 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 248 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 253 - "SaveProcessor"
Cohesion: 0.06
Nodes (26): Component, Container, ContainerBuilder, Dictionary, float, List, Material, materialIndex (+18 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

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

### Community 280 - "TechTreeProcessor.cs"
Cohesion: 0.06
Nodes (16): int, TechTreeSettings, InputButton, SharedTypes, int, ChangeTimeStamp, NodeGroup_SO, List (+8 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 284 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 286 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 289 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 290 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 299 - "GameSettings"
Cohesion: 0.40
Nodes (4): ContainerBuilder, GameSettingsInstaller, List, GameSettings

## Knowledge Gaps
- **284 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+279 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `WorldGenProcessor`, `PlayerRole`, `SettingsProcessor`, `.Log`, `UserInterface_GameMenu`, `PlayerProcessor`, `SaveFileData`, `GameEventProcessor`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `GameSettings`, `StreamTownSessionBridge`, `Utils`, `DebugProcessor`, `MonoBehaviour`, `SaveDataMapper`, `TimeProcessor`, `FoliageProcessor`, `PoolableObject`, `.LoadGameAsync`, `IProcessor`, `TownGoalProcessor`, `MainMenuManager`, `WorldSaveData`, `IInstaller`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `.Log`, `PlayerProcessor`, `Player`, `WorldGenRuntimeData`, `UserInterface_Debug`, `.LogWarning`, `GridProcessor`, `ResourceProcessor`, `Utils`, `TwitchClientProcessor`, `Access_Dropdown`, `DebugProcessor`, `MonoBehaviour`, `CellSpacePartitioning`, `FoliageProcessor`, `PoolableObject`, `Coordinator`, `IProcessor`, `GameEvent`, `GameStateProcessor`, `IInstaller`, `.GenerateFromSettings`, `AIPath`, `SaveProcessor`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `recruit_group_selection_input`, `.default`, `stream_town_migrate/src/content.rs`, `ToolState`, `simulation.rs`, `ResMut`, `Option`, `stream_town_domain/src/content.rs`, `save.rs`, `stream_town_migrate/src/presentation.rs`, `Query`, `stream_town_game/src/lib.rs`, `runtime_console.rs`, `AnimationControllerDef`, `config.rs`, `group_selection_action_buttons`, `Res`, `stream_town_domain/src/lib.rs`, `twitch.rs`, `LegacyEntity`, `.new`, `command.rs`, `technology_tab`, `Result`, `stream_town_domain/src/presentation.rs`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _284 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `recruit_group_selection_input` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07111756168359942 - nodes in this community are weakly interconnected._
- **Should `.default` be split into smaller, more focused modules?**
  _Cohesion score 0.0493221131369799 - nodes in this community are weakly interconnected._