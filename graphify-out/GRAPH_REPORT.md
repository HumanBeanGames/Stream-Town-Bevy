# Graph Report - Stream-Town-Bevy  (2026-08-20)

## Corpus Check
- 636 files · ~1,659,088 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7806 nodes · 21694 edges · 288 communities (265 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1018 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2cf64019`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- MonoBehaviour
- BuildingProcessor
- ResourceTarget
- GridPos
- IDataScriptable
- PresentationCatalog
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- ShaderRef
- SettingsProcessor
- .LoadGameAsync
- DayAndNightProcessor
- Commands
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- config.rs
- Station
- .CreateEnumField
- World.Generation
- BuildingPlacer
- MenuRuntime
- UnitHealthBar
- StableId
- Res
- TechTreeNode
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
- PlayerRole
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- .default
- String
- legacy.rs
- WorldInstanceDeterminism
- .Draw
- AudioHandler
- StreamTownSessionBridge
- String
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Targetable
- Option
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- GameEvent
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- command.rs
- IInstaller
- models.rs
- Tiler
- ScriptablesEditor
- runtime_console.rs
- UserInterface_ObjectSelection
- world.rs
- MiscCommands
- AnimationHandler
- TwitchBotSetupWindow
- EnemyModelHandler
- WorldUtils
- BuildingBase
- Vec
- Access_Text
- PlayerRoleData
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- bottom_bar_entries
- Access_Toggle
- RoleDataSettings
- .Log
- convert_fbx_to_glb.py
- stream_town_migrate/src/presentation.rs
- PoolableObject
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- SensorProcessor
- TownGoalProcessor
- MainMenuManager
- RaidEvent
- LoadingManager
- Access_Dropdown
- PlayerInventory
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- Access_TextInput
- ResourceDataSaveData
- convert
- Utils
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- RoleData
- unity_color_filter
- RoleSlot
- SnapToGridMouseMovement
- AIPath
- PlayerInputRuntimeData
- ResourceData
- STSM_StateAction
- UpdateGraphBounds
- GlobalAudioController
- WindController
- TargetProcessor
- .EnsureValidCredentials
- TradeProcessor
- ConfirmCheck
- ResourceRuntimeData
- ToolState
- GateController
- RoleProcessor
- DebugSettings
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- SelectedObject
- Pet
- add_file
- .Update
- UserInterface_GameMenu
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- Option
- SelectedBuilding
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- SelectedResource
- IProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- TimeProcessor
- technology_tab
- TL_Secrets
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- InventorySaveData
- .UserIsSubscribed
- TL_API
- DontDestroyOnLoad
- ResourceStorageModifier
- EditorHelpers
- .RenderResourceType
- SelectedEnemy
- TechTree.Elements
- EditorUtils
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ProjectCamera
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SelectableObject
- ScriptablesProcessorInfrastructure
- ResourceHolder
- UserInterface_RulerVote
- WeatherProcessor
- TownResourceRuntimeData
- Target
- StringUtils
- BuildingDamageMaterialHandler
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- DayAndNightSettings
- UserInterface
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- GridSettings
- SimpleScreenShot
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- ResourceGenerationSettings
- GridProcessor.cs
- ScriptableObject
- BuildingConfigSettings
- setup_camera
- Key Rules
- Processors
- RuntimeData Template
- GameStateProcessor
- ScriptKeywordProcessor
- FPSDisplay
- SimpleDisableAfterTime
- Processor Template
- Common Patterns
- TransformSaveData
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
- UI_TechOption
- CommonEnums.cs
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Access_GOList
- UISettings
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxParticlePosition
- extraction-spec.md
- KeepKingVote
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- EventProcessor
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- AutosaveIntervalsInstaller
- Easings
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- NewKingVote
- Q: If there is more to do, keep going.
- FoliageGenerationSettings.cs
- RenderPipelineInstaller
- stream_town_domain
- import_save
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- BuildPlacerData
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- UnitTravelToPosition
- ObjectSelectionProcessor.Editor.cs
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- .InjectRuntimeData
- .RefreshSceneData
- TwitchClientRuntimeData
- RandomEnabler
- PlayerCustomizationSaveData
- .InjectRuntimeData

## God Nodes (most connected - your core abstractions)
1. `StableId` - 324 edges
2. `WorldSimulation` - 162 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 124 edges
8. `WorldGenProcessor` - 114 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `falling_fish_uses_authored_gravity_terrain_bounce_and_lifetime_loss()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (288 total, 23 thin omitted)

### Community 0 - "MonoBehaviour"
Cohesion: 0.02
Nodes (57): CameraProcessor, PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+49 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (28): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingSettingsInstaller (+20 more)

### Community 2 - "ResourceTarget"
Cohesion: 0.24
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 3 - "GridPos"
Cohesion: 0.08
Nodes (43): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError (+35 more)

### Community 4 - "IDataScriptable"
Cohesion: 0.02
Nodes (64): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller, ContainerBuilder, TimeDataSettingsInstaller (+56 more)

### Community 5 - "PresentationCatalog"
Cohesion: 0.05
Nodes (73): Assets, PresentationCatalog, animate_healing_effects(), animation_property_value(), apply_material_overrides(), character_material(), character_material_from_standard(), character_material_preserves_authored_albedo_and_cosmetic_contract() (+65 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ShaderRef"
Cohesion: 0.08
Nodes (27): BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension, CloudMaterialUniform (+19 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - ".LoadGameAsync"
Cohesion: 0.10
Nodes (22): Action, CancellationToken, List, Task, Transform, int, string, uint (+14 more)

### Community 12 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 13 - "Commands"
Cohesion: 0.07
Nodes (100): actor_material(), actor_scene_budget(), advance_falling_fish(), AgentAnimation, animate_agents(), animate_chimney_smoke_particles(), apply_agent_commands(), bottom_bar_texture() (+92 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (12): Func, List, Action, float, Enemy, Action, bool, float (+4 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (49): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+41 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "config.rs"
Cohesion: 0.14
Nodes (17): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+9 more)

### Community 19 - "Station"
Cohesion: 0.06
Nodes (25): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+17 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.09
Nodes (19): List, Vector2, NodeSaveData, NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button (+11 more)

### Community 21 - "World.Generation"
Cohesion: 0.06
Nodes (17): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, CampGenSettings, SaveSettings, List (+9 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (20): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+12 more)

### Community 23 - "MenuRuntime"
Cohesion: 0.05
Nodes (97): AppExit, BackgroundColor, PlayerSettings, Default, adjust_settings_menu(), apply_settings_draft(), bottom_bar_action_buttons(), bottom_bar_main_buttons() (+89 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "StableId"
Cohesion: 0.05
Nodes (58): ObjectiveDef, ObjectiveKind, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips() (+50 more)

### Community 26 - "Res"
Cohesion: 0.05
Nodes (175): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, AnimationGraphHandle, AnimationTransitions, AudioSink, ActorNameOverlay (+167 more)

### Community 27 - "TechTreeNode"
Cohesion: 0.05
Nodes (31): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+23 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.07
Nodes (12): Player, Dictionary, GameObject, Vector3, List, GameSettings, GameMasterCommands, TwitchClientProcessor (+4 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (191): AnyResult, actor_combat_visual(), ActorHealthFill, ActorHealthOverlay, advance_loading_phase(), advance_loading_runtime(), agent_action_animation(), agent_is_moving() (+183 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (23): HashSet, Action, bool, BoxCollider, Container, Func, GameObject, HashSet (+15 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.07
Nodes (31): AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings, Action, IEnumerator (+23 more)

### Community 35 - "RoleHandler"
Cohesion: 0.10
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 36 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.07
Nodes (21): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+13 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.10
Nodes (65): GameConfig, ContentCatalog, ActorState, String, action_animation_speed(), action_cooldown(), actor_accepts_resource(), actor_archetype() (+57 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.05
Nodes (19): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+11 more)

### Community 42 - "PlayerRole"
Cohesion: 0.10
Nodes (15): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+7 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (124): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+116 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (24): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+16 more)

### Community 45 - ".default"
Cohesion: 0.04
Nodes (128): generate_world(), generate_world_with_content(), GeneratedWorld, agent_facing_matches_unity_rotation_and_action_targets(), append_terrain_skirt(), archetype_by_source(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms() (+120 more)

### Community 46 - "String"
Cohesion: 0.11
Nodes (50): animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_controllers(), convert_embedded_model_clips() (+42 more)

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (39): binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), conversion_rejects_malformed_retained_mesh(), decode_json(), decode_legacy(), json_active_goal(), json_buildings() (+31 more)

### Community 48 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (11): bool, double, float, int, long, MenuItem, string, FrameCapture (+3 more)

### Community 52 - "String"
Cohesion: 0.04
Nodes (73): AnimationClip, AnimationGraph, AnimationTargetId, active_event_text(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve() (+65 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Targetable"
Cohesion: 0.09
Nodes (17): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+9 more)

### Community 57 - "Option"
Cohesion: 0.07
Nodes (47): DisplayMode, actor_detail_budget(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), building_definition_id(), CommandOrigin, complete_converted_animation_spec(), ConvertedAnimationSpec, debug_building_health() (+39 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.25
Nodes (7): BinaryParser<'a>, decode_binary(), FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

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
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "twitch.rs"
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.06
Nodes (20): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData, Slider, TextMeshProUGUI (+12 more)

### Community 69 - "command.rs"
Cohesion: 0.06
Nodes (62): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+54 more)

### Community 70 - "IInstaller"
Cohesion: 0.03
Nodes (38): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, Volume (+30 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.12
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "world.rs"
Cohesion: 0.15
Nodes (28): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise(), generate_foliage(), generate_shoreline_fish() (+20 more)

### Community 77 - "MiscCommands"
Cohesion: 0.16
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 78 - "AnimationHandler"
Cohesion: 0.09
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+7 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "BuildingBase"
Cohesion: 0.07
Nodes (13): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, TilerBuilding (+5 more)

### Community 83 - "Vec"
Cohesion: 0.06
Nodes (55): AnimationNodeIndex, AnimationPlayer, ActivePetVisual, ActorAnimationDriver, advance_animation_crossfade(), animation_event_occurrences(), animation_nodes_for_selection(), animation_selection_duration() (+47 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRoleData"
Cohesion: 0.11
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, StatType

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.12
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

### Community 90 - "bottom_bar_entries"
Cohesion: 0.14
Nodes (18): assign_group_role(), bottom_bar_entries(), building_construction_cost(), building_cost_reduction_percent(), building_icon_path(), building_is_unlocked(), building_upgrade_affordability(), building_upgrade_cost() (+10 more)

### Community 91 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 92 - "RoleDataSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 93 - ".Log"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor, int, STSM_Helper_Build (+9 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (75): animation_state_id(), animation_state_machine_id(), animation_take_name(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert_avatar_masks(), convert_chimney_smoke() (+67 more)

### Community 96 - "PoolableObject"
Cohesion: 0.07
Nodes (23): Container, ContainerBuilder, GUIDProcessor, SaveablFoliage, CreditsRuntimeData, bool, GameStateRuntimeData, bool (+15 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.05
Nodes (32): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, Component, Container, ContainerBuilder, Dictionary (+24 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (18): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+10 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (78): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef (+70 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.07
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 102 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "RaidEvent"
Cohesion: 0.07
Nodes (19): Transform, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+11 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 108 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 109 - "CustomLogHandler"
Cohesion: 0.18
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Type, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.11
Nodes (10): float, int, List, Transform, EnemySpawner, float, ChanceObject, float (+2 more)

### Community 113 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 114 - "ResourceDataSaveData"
Cohesion: 0.11
Nodes (19): bool, int, MeshSaveData, bool, float, int, List, string (+11 more)

### Community 115 - "convert"
Cohesion: 0.10
Nodes (31): ActorKind, ActorCustomization, StreamUserType, actor_prefix(), BinaryParser, clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+23 more)

### Community 116 - "Utils"
Cohesion: 0.04
Nodes (17): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Utils (+9 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.09
Nodes (9): DepositResources, int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor (+1 more)

### Community 120 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.11
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 125 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 126 - "ResourceData"
Cohesion: 0.21
Nodes (6): bool, int, Matrix4x4, uint, Vector3, ResourceData

### Community 127 - "STSM_StateAction"
Cohesion: 0.07
Nodes (11): int, STSM_Helper_Attack, int, STSM_Action_Attack, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool (+3 more)

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
Cohesion: 0.12
Nodes (10): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, List, TargetProcessor, Dictionary (+2 more)

### Community 132 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 133 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (46): bounded_ui_index(), content_tab(), default_catalog_path(), draw_world_preview(), inject_runtime_command(), inspector_tab(), launch_runtime_game(), main() (+38 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "RoleProcessor"
Cohesion: 0.08
Nodes (7): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor

### Community 139 - "DebugSettings"
Cohesion: 0.24
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

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
Nodes (23): bool, List, ObjectPoolingSettings, Action, bool, BoxCollider, CancellationToken, Container (+15 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "SelectedObject"
Cohesion: 0.10
Nodes (6): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 154 - "Option"
Cohesion: 0.10
Nodes (46): clip_id(), color_value(), convert_clips(), convert_post_process(), extracts_indexed_material_properties(), field_bool(), field_f32(), field_str() (+38 more)

### Community 155 - "SelectedBuilding"
Cohesion: 0.13
Nodes (4): bool, float, PassiveResourceIncrementer, SelectedBuilding

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - "IProcessor"
Cohesion: 0.09
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

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

### Community 164 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 165 - "technology_tab"
Cohesion: 0.31
Nodes (16): apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node(), refresh_technology_draft(), Option (+8 more)

### Community 166 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 167 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.22
Nodes (5): Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 170 - "InventorySaveData"
Cohesion: 0.22
Nodes (7): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData

### Community 171 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 173 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 174 - "ResourceStorageModifier"
Cohesion: 0.27
Nodes (3): ResourceStorageModifier, float, int

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 178 - "TechTree.Elements"
Cohesion: 0.08
Nodes (18): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+10 more)

### Community 179 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 185 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 186 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 187 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 188 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 189 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 190 - "Target"
Cohesion: 0.12
Nodes (8): Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies, GUIDSystem

### Community 192 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 195 - "UserInterface"
Cohesion: 0.04
Nodes (32): ActivityStatus, InputButton, SharedTypes, int, ChangeTimeStamp, bool, float, string (+24 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "GridSettings"
Cohesion: 0.25
Nodes (6): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller

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
Cohesion: 0.09
Nodes (20): int, string, ObjectiveSaveData, List, SaveGameData, bool, float, List (+12 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "ResourceGenerationSettings"
Cohesion: 0.25
Nodes (7): List, ResourceGenSettings, AnimationCurve, bool, int, List, ResourceGenerationSettings

### Community 209 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 210 - "ScriptableObject"
Cohesion: 0.11
Nodes (14): int, TechTreeSettings, ContainerBuilder, TechTreeSettingsInstaller, RequirementType, object, Requirement, List (+6 more)

### Community 211 - "BuildingConfigSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "Processors"
Cohesion: 0.05
Nodes (15): BuildCostModifier, InputButton, UserInterface.MainMenu, PlayerControls.ObjectSelection, Processors, Core, Level, MetaData (+7 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "TransformSaveData"
Cohesion: 0.10
Nodes (18): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+10 more)

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
Cohesion: 0.11
Nodes (8): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, RoleScriptablesEditor, WindControllerEditor, GridSystemEditor, Globals, ScriptablesEditor, Editor

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

### Community 239 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 240 - "CommonEnums.cs"
Cohesion: 0.12
Nodes (13): BuildingResourceModelHandler, GameObject, UnityEvent, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+5 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 248 - "UISettings"
Cohesion: 0.33
Nodes (4): float, UISettings, ContainerBuilder, UISettingsInstaller

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 268 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 270 - "import_save"
Cohesion: 0.43
Nodes (7): absolute_path(), backup_candidate(), import_preserves_source_and_recovers_named_backup(), import_save(), ImportReport, Path, PathBuf

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

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
- **286 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+281 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `MonoBehaviour`, `BuildingProcessor`, `RoleProcessor`, `.LoadGameAsync`, `SettingsProcessor`, `ObjectPoolingProcessor`, `World.Generation`, `UserInterface_GameMenu`, `SaveFileData`, `Player`, `IProcessor`, `WorldGenProcessor`, `GameEventProcessor`, `TimeProcessor`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `StreamTownSessionBridge`, `UserInterface`, `IInstaller`, `WorldSaveData`, `ResourceGenerationSettings`, `.PrepareRuntimeForLoad`, `FoliageProcessor`, `.Log`, `PoolableObject`, `PlayerProcessor`, `TownGoalProcessor`, `MainMenuManager`, `Resource`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `MonoBehaviour`, `BuildingProcessor`, `IDataScriptable`, `TwitchChatProcessor`, `ObjectPoolingProcessor`, `World.Generation`, `WorldGenRuntimeData`, `.InjectRuntimeData`, `Player`, `IProcessor`, `UserInterface_Debug`, `GenerationSettings`, `GridProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `ProjectCamera`, `Target`, `IInstaller`, `ResourceGenerationSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `GameStateProcessor`, `.Log`, `PoolableObject`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `RaidEvent`, `EnemySpawner`, `AIPath`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `GridPos`, `PresentationCatalog`, `ToolState`, `Commands`, `stream_town_domain/src/content.rs`, `save.rs`, `config.rs`, `MenuRuntime`, `Res`, `Option`, `stream_town_game/src/lib.rs`, `technology_tab`, `ContentCatalog`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `.default`, `String`, `String`, `Option`, `twitch.rs`, `command.rs`, `runtime_console.rs`, `world.rs`, `Vec`, `bottom_bar_entries`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/presentation.rs`, `convert`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _286 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MonoBehaviour` be split into smaller, more focused modules?**
  _Cohesion score 0.02446941323345818 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.03715635738831615 - nodes in this community are weakly interconnected._
- **Should `GridPos` be split into smaller, more focused modules?**
  _Cohesion score 0.07622504537205081 - nodes in this community are weakly interconnected._