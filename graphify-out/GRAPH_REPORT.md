# Graph Report - Stream-Town-Bevy  (2026-08-21)

## Corpus Check
- 644 files · ~1,678,328 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8039 nodes · 22523 edges · 303 communities (278 shown, 25 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1021 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a73cdebf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- World.Generation.Settings
- BuildingProcessor
- GeneratedWorld
- simulation.rs
- ScriptableObject
- UnityAsset
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- Commands
- SettingsProcessor
- Station
- Target
- Option
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- Access_Dropdown
- BinaryReader
- Option
- ObjectPoolingProcessor
- BuildingPlacer
- StableId
- UnitHealthBar
- Processors
- Query
- TechTreeGraphView
- SaveFileData
- Player
- VfxAnimationController
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- component_field_value
- ResMut
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- stream_town_game/src/lib.rs
- stream_town_migrate/src/presentation.rs
- legacy.rs
- ResourceDataSaveData
- .Draw
- IRuntimeDataScriptable
- StreamTownSessionBridge
- settings.rs
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- GridPos
- UserInterface_RulerVote
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- Character
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- command.rs
- .default
- models.rs
- Tiler
- ScriptablesEditor
- Units
- UserInterface_ObjectSelection
- PlayerRoleData
- Goal
- Vec
- TwitchBotSetupWindow
- MiscCommands
- WorldUtils
- GameConfig
- String
- Access_Text
- PlayerRole
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- RoleHandler
- RaidEvent
- ResourceRuntimeData
- STSM_Idle_Player
- convert_fbx_to_glb.py
- tools_ui
- SelectedBuilding
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- update_environment_presentation
- TownGoalProcessor
- MainMenuManager
- SensorProcessor
- LoadingManager
- Access_Toggle
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- VoteEvent
- ResourceTarget
- String
- GridProcessor
- stream_town_migrate/src/main.rs
- WorldGenSaveData
- Resource
- AnimationHandler
- unity_color_filter
- TechTreeNode
- SnapToGridMouseMovement
- AIPath
- PendingChatCommand
- PlayerInputProcessor
- .RestoreWorldState
- UpdateGraphBounds
- CommandDictionary
- convert_archetypes
- .SetGeneratedResources
- WeatherProcessor
- twitch_tab
- ConfirmCheck
- SimpleDisableAfterTime
- runtime_console.rs
- GateController
- SelectableObject
- .GetMissingDataScriptableDependencies
- .RenderResourceType
- What You Must Do When Invoked
- RuntimeData Template
- convert
- RuntimeData Template
- Key Rules
- GridProcessor.cs
- Pet
- add_file
- SelectedObject
- .default
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- stream_town_tools/src/main.rs
- BuildingResourceModelHandler
- .StartMusic
- UnitTextDisplay
- Stream Town Reloaded - Architecture Documentation
- WindController
- .InitializeAndActivateProcessorsAsync
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- .CreateEnumField
- UserInterface_TownGoal
- .SetTargetType
- SimpleMusicController
- WorldInstanceDeterminism
- ResourceProcessor
- xtask/src/main.rs
- SelectedPlayerGroup
- PlayerSaveData
- TechNodeData
- SeasonDataSettings
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- SelectedResource
- SelectedEnemy
- TechTree.Elements
- UserInterface
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ProjectCamera
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- List
- MonoBehaviour
- WorldGenerationReferenceExporter
- EditorUtils
- import_save
- UserInterface_GameMenu
- Easings
- stream_town_migrate/src/menu_scene.rs
- Targetable
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- .RestoreObjectiveProgress
- .DrawDataFieldAndLabel
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- IProcessor.cs
- SimpleScreenShot
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- BuildingModelHandler
- FoliageGenerationSettings.cs
- Requirement
- IProcessor
- TerrainGenSettings
- GameStateProcessor
- Key Rules
- ScriptablesProcessorInfrastructure
- RuntimeData Template
- DayAndNightSettings
- TargetProcessor
- ScriptKeywordProcessor
- FPSDisplay
- BevyMigrationExporter
- Processor Template
- Common Patterns
- SaveDataMapper
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
- DebugProcessor
- CreateProjectScopeProcessors.cs
- VFXArrowPointer
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- WorldGenRuntimeData
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxSeagullSpawner
- extraction-spec.md
- PassiveResourceIncrementer
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- UI_TechOption
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Utils
- TL_API
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- PlacementProbeHandler
- Q: If there is more to do, keep going.
- ObjectSelectionProcessor.Editor.cs
- KeepKingVote
- stream_town_domain
- StringUtils
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- stream_town_domain/src/lib.rs
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- ResourceCostData
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- Access_GOList
- NewKingVote
- MainMenuSceneReference
- RandomEnabler
- animate_healing_effects
- StreamTown.Migration
- ResourceDataSettings
- .ExportModification
- DontDestroyOnLoad
- SelectedEnemyCamp
- NodeGroup_SO
- VfxParticlePosition
- IntWrapper
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- .StartupSequence
- ScriptableObjectAssetData
- .InjectRuntimeData
- preview_lerp_color

## God Nodes (most connected - your core abstractions)
1. `StableId` - 335 edges
2. `WorldSimulation` - 164 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 132 edges
8. `WorldGenProcessor` - 114 edges
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

## Communities (303 total, 25 thin omitted)

### Community 0 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (34): BuildingBase, bool, float, int, List, UnityEvent, TilerBuilding, bool (+26 more)

### Community 2 - "GeneratedWorld"
Cohesion: 0.09
Nodes (55): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash() (+47 more)

### Community 3 - "simulation.rs"
Cohesion: 0.08
Nodes (30): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), EnemyCampState (+22 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.03
Nodes (62): int, AudioSettings, List, CampGenSettings, List, FoliageGenSettings, bool, ParticleSystem (+54 more)

### Community 5 - "UnityAsset"
Cohesion: 0.19
Nodes (43): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), field(), field_value(), foliage_layers(), generated_record_ids() (+35 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.07
Nodes (19): Container, ContainerBuilder, TimeProcessor, bool, float, Func, int, PlayerExistsByIDDelegate (+11 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Commands"
Cohesion: 0.08
Nodes (106): actor_material(), actor_scene_budget(), animate_chimney_smoke_particles(), authored_main_menu_mesh(), bottom_bar_texture(), building_age(), building_effect_material(), BuildingEffectKind (+98 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "Station"
Cohesion: 0.04
Nodes (45): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+37 more)

### Community 12 - "Target"
Cohesion: 0.11
Nodes (8): Target, Utils.Pooling, GridSystem.Partitioning, Combat, Environment, SavingAndLoading.SavableObjects, Enemies, GUIDSystem

### Community 13 - "Option"
Cohesion: 0.04
Nodes (74): ArchetypeDef, ArchetypeKind, ArchetypeScene, actor_detail_budget(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_scene_for_age(), authored_color_grading() (+66 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (16): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy (+8 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (39): AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, EnemyModelSetDef, EnemyRunAnimation, EnemyWeaponModelDef (+31 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "Access_Dropdown"
Cohesion: 0.05
Nodes (20): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_ChannelNameInput, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown (+12 more)

### Community 19 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 20 - "Option"
Cohesion: 0.12
Nodes (31): animator_component(), animator_reference_path(), color_value(), convert_post_process(), extracts_indexed_material_properties(), field_array(), field_bool(), field_f32() (+23 more)

### Community 21 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (38): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+30 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "StableId"
Cohesion: 0.05
Nodes (67): ObjectiveDef, FromStr, StableId, complete_gameplay_scenario_round_trips(), objective_increment(), ObjectiveEvent, ObjectiveProgress, BTreeMap (+59 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "Processors"
Cohesion: 0.05
Nodes (12): BuildCostModifier, InputButton, UserInterface.MainMenu, PlayerControls.ObjectSelection, Processors, World, Level, MetaData (+4 more)

### Community 26 - "Query"
Cohesion: 0.06
Nodes (151): Aabb, AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, AnimationGraphHandle, AnimationTransitions, AudioSink (+143 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.05
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, ModeratorCommands (+4 more)

### Community 30 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.05
Nodes (18): SortBuildingByLowerLevel, Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor (+10 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - "component_field_value"
Cohesion: 0.21
Nodes (25): building_model_definitions(), building_node_age(), component(), component_at(), component_field_value(), component_reference_name(), component_reference_names(), component_type() (+17 more)

### Community 36 - "ResMut"
Cohesion: 0.03
Nodes (133): AnyResult, AppExit, PlayerSettings, Default, adjust_settings_menu(), advance_loading_phase(), advance_loading_runtime(), AgentCommandQueue (+125 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.11
Nodes (9): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData (+1 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.09
Nodes (70): ContentCatalog, ActorState, RoleProgress, Default, String, action_animation_speed(), action_cooldown(), actor_accepts_resource() (+62 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (47): animation_parameter_name(), archetype_scenes(), asset(), authored_mask(), authored_value(), building_placements(), BuildingPlacement, child_technology_guids() (+39 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (171): ActorHealthFill, ActorHealthOverlay, advance_falling_fish(), AgentEnemyModelPresentation, AgentEquipmentPresentation, animate_loading_icon(), animation_property_value(), append_terrain_skirt() (+163 more)

### Community 46 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (111): animation_state_id(), animation_state_machine_id(), animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), clip_id(), collect_prefab_dependencies() (+103 more)

### Community 47 - "legacy.rs"
Cohesion: 0.15
Nodes (42): ActorCustomization, StreamUserType, pending_stream_user_type(), binary_fixture(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json() (+34 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (30): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+22 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 52 - "settings.rs"
Cohesion: 0.10
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.08
Nodes (15): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, TwitchClientProcessor (+7 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "GridPos"
Cohesion: 0.07
Nodes (44): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError (+36 more)

### Community 57 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "Character"
Cohesion: 0.06
Nodes (20): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+12 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.15
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
Cohesion: 0.08
Nodes (41): channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join(), OAuthClient, OAuthErrorResponse (+33 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 70 - ".default"
Cohesion: 0.03
Nodes (138): generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats(), authored_target_sizes_drive_unity_action_reach_formulas() (+130 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.12
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "Units"
Cohesion: 0.07
Nodes (14): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Units (+6 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "PlayerRoleData"
Cohesion: 0.08
Nodes (16): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+8 more)

### Community 77 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "Vec"
Cohesion: 0.14
Nodes (33): append_vec3_keys(), convert_fireworks(), inline_file_id(), json_f32(), parse_child_references(), parse_reference_list(), parse_state_machine_transition_sources(), reference_id() (+25 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 81 - "WorldUtils"
Cohesion: 0.27
Nodes (5): GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "GameConfig"
Cohesion: 0.09
Nodes (44): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, BTreeMap, BTreeSet (+36 more)

### Community 83 - "String"
Cohesion: 0.03
Nodes (126): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, active_event_text(), ActivePetVisual, ActorAnimationDriver (+118 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRole"
Cohesion: 0.05
Nodes (24): RoleSlotModifier, int, RoleSlot, bool, int, Dictionary, int, RoleDataContainer (+16 more)

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

### Community 90 - "RoleHandler"
Cohesion: 0.15
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 93 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "tools_ui"
Cohesion: 0.19
Nodes (21): content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), preview_grid_point(), redo_catalog_edit(), role_i32(), role_u16() (+13 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (22): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+14 more)

### Community 98 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (79): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+71 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.06
Nodes (15): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnChatCommandReceivedArgs (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "update_environment_presentation"
Cohesion: 0.11
Nodes (29): Assets, building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced, BuildingMaterialInstances, debug_weather_override() (+21 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 108 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 113 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 114 - "ResourceTarget"
Cohesion: 0.24
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 115 - "String"
Cohesion: 0.23
Nodes (15): ImportReport, json_pet_name(), legacy_objective_matches(), legacy_pet_name(), LegacyGoal, LegacyObjective, objective_target_matches(), restore_legacy_goal() (+7 more)

### Community 116 - "GridProcessor"
Cohesion: 0.07
Nodes (19): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, int, List (+11 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "WorldGenSaveData"
Cohesion: 0.14
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (26): DepositResources, ResourceStorageModifier, float, int, PlayerInventory, Dictionary, int, ActiveResourceIncrementer (+18 more)

### Community 120 - "AnimationHandler"
Cohesion: 0.04
Nodes (32): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+24 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "PendingChatCommand"
Cohesion: 0.23
Nodes (12): building_definition_id(), CommandOrigin, local_ui_command(), local_ui_vote_command(), local_ui_voter(), local_vote_falls_back_to_a_live_non_enemy_actor(), PendingChatCommand, require_game_master() (+4 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - ".RestoreWorldState"
Cohesion: 0.22
Nodes (4): float, int, TimeRuntimeData, IEnumerable

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "CommandDictionary"
Cohesion: 0.13
Nodes (9): Action, Dictionary, IReadOnlyList, List, CommandDictionary, OnChatCommandReceivedArgs, OnMessageReceivedArgs, OnChatCommandReceivedArgs (+1 more)

### Community 130 - "convert_archetypes"
Cohesion: 0.15
Nodes (15): ArchetypesById, ArchetypeBounds, EnemyDef, EnemySpawnerDef, ResourceReward, WeightedEnemySpawn, archetype_bounds(), archetype_kind() (+7 more)

### Community 131 - ".SetGeneratedResources"
Cohesion: 0.40
Nodes (5): List, Material, materials, Mesh, meshes

### Community 132 - "WeatherProcessor"
Cohesion: 0.13
Nodes (10): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, IMainThreadInitializableProcessor, Container (+2 more)

### Community 133 - "twitch_tab"
Cohesion: 0.29
Nodes (11): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+3 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 136 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 139 - ".GetMissingDataScriptableDependencies"
Cohesion: 0.22
Nodes (3): Container, IEnumerable, Type

### Community 140 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "convert"
Cohesion: 0.18
Nodes (17): ActorKind, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), decode_binary() (+9 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "GridProcessor.cs"
Cohesion: 0.32
Nodes (3): GridProcessorEditor, GridSystem.Utils, GridSystem

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 150 - ".default"
Cohesion: 0.18
Nodes (17): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_game_config(), main() (+9 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "stream_town_tools/src/main.rs"
Cohesion: 0.10
Nodes (59): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), bounded_ui_index(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role() (+51 more)

### Community 154 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 155 - ".StartMusic"
Cohesion: 0.49
Nodes (3): SeasonAudioData, AudioClip, List

### Community 156 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.21
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

### Community 163 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 164 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 165 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 168 - "ResourceProcessor"
Cohesion: 0.14
Nodes (12): Container, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int (+4 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 171 - "PlayerSaveData"
Cohesion: 0.06
Nodes (27): Component, Transform, int, List, string, uint, BuildingSaveData, int (+19 more)

### Community 172 - "TechNodeData"
Cohesion: 0.39
Nodes (3): List, Node_SO, TechNodeData

### Community 173 - "SeasonDataSettings"
Cohesion: 0.29
Nodes (6): Color, float, int, VisualEffect, SeasonDataSettings, Gradient

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (22): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+14 more)

### Community 179 - "UserInterface"
Cohesion: 0.07
Nodes (12): InputButton, SharedTypes, TownGoal.Data, StreamTown.EditorTools, TownGoal, UserInterface, GameEventSystem, GameEventSystem.Events (+4 more)

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

### Community 184 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 185 - "MonoBehaviour"
Cohesion: 0.02
Nodes (96): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder (+88 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 188 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.27
Nodes (17): ConversionReport, convert(), finite(), quat(), Path, Result, String, Vec (+9 more)

### Community 192 - "Targetable"
Cohesion: 0.06
Nodes (25): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent (+17 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 195 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 200 - "SimpleScreenShot"
Cohesion: 0.07
Nodes (12): PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, float, Vector3 (+4 more)

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
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 209 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "IProcessor"
Cohesion: 0.07
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Container, IProcessor, Container, ContainerBuilder, CreditsProcessor (+7 more)

### Community 212 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 213 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 218 - "TargetProcessor"
Cohesion: 0.15
Nodes (9): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor, Dictionary, List (+1 more)

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "SaveDataMapper"
Cohesion: 0.08
Nodes (20): ResourceInventory, bool, int, Dictionary, Mesh, Vector3, SaveDataMapper, bool (+12 more)

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
Cohesion: 0.07
Nodes (24): Audio provenance, Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows, Binaries, Commands (+16 more)

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
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "DebugProcessor"
Cohesion: 0.13
Nodes (9): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+1 more)

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 243 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

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

### Community 253 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Utils"
Cohesion: 0.05
Nodes (9): int, ChangeTimeStamp, DataStructures, TownGoal.Enumerations, Utils, GameResources, World.Generation, TownGoal.Data.Save (+1 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "PlacementProbeHandler"
Cohesion: 0.29
Nodes (4): PlacementProbe, float, PlacementProbeHandler, SurfaceType

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

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

### Community 282 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 283 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 284 - "MainMenuSceneReference"
Cohesion: 0.57
Nodes (6): MainMenuCameraReference, MainMenuEmbeddedMesh, MainMenuModelInstance, MainMenuSceneReference, String, Vec

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 286 - "animate_healing_effects"
Cohesion: 0.29
Nodes (13): animate_healing_effects(), f32_to_u16_saturating(), gradient_material(), healing_burst_effect(), healing_channel_effect(), healing_effect_curves_preserve_authored_lifetimes_and_channel_keys(), healing_effect_duration(), healing_effect_sample() (+5 more)

### Community 288 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 299 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 300 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 302 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 303 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 304 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 305 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 306 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 307 - ".StartupSequence"
Cohesion: 0.20
Nodes (3): IEnumerator, LoadSceneMode, Scene

### Community 308 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 313 - "preview_lerp_color"
Cohesion: 1.00
Nodes (3): preview_lerp_color(), Color32, terrain_preview_color()

## Knowledge Gaps
- **297 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+292 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `UpdateGraphBounds`, `SimpleDisableAfterTime`, `Station`, `Target`, `StringUtils`, `BuildingPlacer`, `Processors`, `UnitTextDisplay`, `RandomEnabler`, `GenerationSettings`, `TechTree.Elements`, `UserInterface`, `MonoBehaviour`, `Character`, `Easings`, `.DrawDataFieldAndLabel`, `SimpleScreenShot`, `Units`, `ScriptablesProcessorInfrastructure`, `FPSDisplay`, `DayAndNightProcessor`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `Target`, `ObjectPoolingProcessor`, `Player`, `UserInterface_Debug`, `ResourceProcessor`, `PlayerSaveData`, `.StartupSequence`, `TwitchClientProcessor`, `ProjectCamera`, `MonoBehaviour`, `IProcessor`, `TerrainGenSettings`, `GameStateProcessor`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `SaveProcessor`, `PlayerProcessor`, `DebugProcessor`, `DayAndNightProcessor`, `WorldGenRuntimeData`, `GridProcessor`, `AIPath`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `HealthHandler`, `ObjectPoolingProcessor`, `BuildingPlacer`, `NewKingVote`, `Player`, `WorldGenProcessor`, `UserInterface_Debug`, `GameEventProcessor`, `TechTreeProcessor`, `SelectedPlayerGroup`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `Character`, `PlayerRoleData`, `IProcessor`, `PlayerRole`, `UserInterface_TownVote`, `RoleHandler`, `RaidEvent`, `SaveProcessor`, `DayAndNightProcessor`, `Resource`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _297 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World.Generation.Settings` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.03720693170234455 - nodes in this community are weakly interconnected._
- **Should `GeneratedWorld` be split into smaller, more focused modules?**
  _Cohesion score 0.09234972677595628 - nodes in this community are weakly interconnected._