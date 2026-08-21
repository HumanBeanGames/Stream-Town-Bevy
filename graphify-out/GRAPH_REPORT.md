# Graph Report - Stream-Town-Bevy  (2026-08-21)

## Corpus Check
- 644 files · ~1,679,188 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8046 nodes · 22550 edges · 306 communities (278 shown, 28 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1021 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `64ef24ca`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- World.Generation.Settings
- BuildingType
- world.rs
- simulation.rs
- IDataScriptable
- Station
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- Option
- SettingsProcessor
- Targetable
- Target
- RenderAssets
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- UIElementWrapper
- BuildingProcessor
- BuildingBase
- ObjectPoolingProcessor
- BuildingPlacer
- StableId
- UnitHealthBar
- Utils
- Query
- TechTreeGraphView
- SaveFileData
- Player
- VfxAnimationController
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- DayAndNightProcessor
- ResMut
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- PlayerRole
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- stream_town_game/src/lib.rs
- stream_town_migrate/src/presentation.rs
- Value
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- command.rs
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- PlayerCommands
- UserInterface_RulerVote
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- SavingAndLoading.Structs
- CameraController
- ScriptableObject
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- EnemyModelHandler
- .default
- models.rs
- Tiler
- ScriptablesEditor
- HealthModifier
- UserInterface_ObjectSelection
- RoleHandler
- Goal
- Result
- TwitchBotSetupWindow
- MiscCommands
- WorldUtils
- config.rs
- String
- Access_Text
- RoleProcessor
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- SelectedPlayer
- RaidEvent
- ResourceRuntimeData
- .Log
- convert_fbx_to_glb.py
- preview_grid_point
- SelectedBuilding
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- setup_rendering
- TownGoalProcessor
- MainMenuManager
- Result
- LoadingManager
- Access_Toggle
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- GameStateProcessor
- VoteEvent
- ResourceTarget
- ResourceInventory
- GridProcessor
- stream_town_migrate/src/main.rs
- GUIDProcessor
- Resource
- STSM_StateAction
- unity_color_filter
- TechTreeNode
- SnapToGridMouseMovement
- AIPath
- process_injected_commands
- PlayerInputProcessor
- .RestoreWorldState
- UpdateGraphBounds
- CommandDictionary
- AnimationHandler
- PlayerInventory
- WeatherProcessor
- twitch_tab
- ConfirmCheck
- Dictionary
- runtime_console.rs
- GateController
- SelectableObject
- IRuntimeDataScriptable
- .Update
- What You Must Do When Invoked
- RuntimeData Template
- legacy.rs
- RuntimeData Template
- Key Rules
- GridProcessor.cs
- Pet
- add_file
- TradeProcessor
- .default
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- stream_town_tools/src/main.rs
- BuildingResourceModelHandler
- parse_controller
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WindController
- IProcessor
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
- ResourceStorageModifier
- PlayerSaveData
- group_selection_action_buttons
- TechnologyTreeGroup
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- SelectedResource
- SelectedEnemy
- TechTree.Elements
- TechTreeProcessor.cs
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- List
- ScriptablesProcessorInfrastructure
- WorldGenerationReferenceExporter
- EditorUtils
- UserInterface_Roles
- UserInterface_GameMenu
- Easings
- stream_town_migrate/src/menu_scene.rs
- ResourceHolder
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- EnemyWeaponModel
- .DrawDataFieldAndLabel
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- IProcessor.cs
- MonoBehaviour
- graphify reference: extra exports and benchmark
- Key Rules
- PlayerControls
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- BuildingModelHandler
- BuildingDamageMaterialHandler
- Requirement
- EventProcessor
- TimeProcessor
- STSM_HelperBase
- Key Rules
- AudioSettings
- RuntimeData Template
- load_player_settings
- RotationHandler
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
- DebugSettings
- CreateDefaultSettingsAssets.cs
- parse_property_curves
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
- PlayerDeathHandler
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
- PlayerInputRuntimeData
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- SimpleScreenShot
- PostProcessingInstaller
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- BuildPlacerData
- NewKingVote
- MainMenuSceneReference
- RandomEnabler
- PlayerSpawnPoint
- StreamTown.Migration
- StatusBar
- CellSpacePartitioningInstaller
- .ExportModification
- ChannelDataInstaller
- SaveStateInstaller
- DontDestroyOnLoad
- UserInterface_GameMenuInstaller
- FloatWrapper
- player_window_mode
- setup_camera
- VfxParticlePosition
- IntWrapper
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- ScriptableObjectAssetData

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
- `falling_fish_uses_authored_gravity_terrain_bounce_and_lifetime_loss()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `food_roles_only_select_their_authored_target_types()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `full_town_storage_pauses_gathering_and_preserves_carried_overflow()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (306 total, 28 thin omitted)

### Community 0 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 1 - "BuildingType"
Cohesion: 0.05
Nodes (23): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+15 more)

### Community 2 - "world.rs"
Cohesion: 0.06
Nodes (68): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+60 more)

### Community 3 - "simulation.rs"
Cohesion: 0.07
Nodes (36): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather() (+28 more)

### Community 4 - "IDataScriptable"
Cohesion: 0.02
Nodes (78): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+70 more)

### Community 5 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (15): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, bool (+7 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Option"
Cohesion: 0.05
Nodes (136): ArchetypeScene, ChimneySmokeDef, PresentationCatalog, GeneratedWorld, actor_detail_budget(), actor_material(), actor_scene_budget(), advance_falling_fish() (+128 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "Targetable"
Cohesion: 0.04
Nodes (46): List, Vector3, RoleData, AudioClip, bool, float, int, Sprite (+38 more)

### Community 12 - "Target"
Cohesion: 0.07
Nodes (16): STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Behaviours, Target, Utils.Pooling, Sensors (+8 more)

### Community 13 - "RenderAssets"
Cohesion: 0.05
Nodes (63): BackgroundColor, bottom_bar_texture(), BottomBarContext, BottomBarMainButton, BoundsMaterialExtension, BoundsMaterialUniform, building_effect_material(), BuildingMaterialExtension (+55 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.08
Nodes (20): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+12 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (11): Action, float, Enemy, STSM_Action_Heal, Action, bool, float, int (+3 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (45): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+37 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 19 - "BuildingProcessor"
Cohesion: 0.09
Nodes (5): Container, ContainerBuilder, Dictionary, List, BuildingProcessor

### Community 20 - "BuildingBase"
Cohesion: 0.09
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 21 - "ObjectPoolingProcessor"
Cohesion: 0.06
Nodes (29): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+21 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "StableId"
Cohesion: 0.08
Nodes (37): FromStr, StableId, complete_gameplay_scenario_round_trips(), FishGodState, RaidState, BTreeMap, BTreeSet, Option (+29 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "Utils"
Cohesion: 0.04
Nodes (16): BuildCostModifier, Utils, Processors, StreamTown.EditorTools, Animation, World, Level, STStateMachine (+8 more)

### Community 26 - "Query"
Cohesion: 0.06
Nodes (165): Aabb, AmbientLight, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver (+157 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.09
Nodes (16): Vector2, int, List, Port, Vector2, TechTreeGraphView, List, Texture2D (+8 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, ModeratorCommands (+2 more)

### Community 30 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (23): Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject, HashSet (+15 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 33 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 36 - "ResMut"
Cohesion: 0.04
Nodes (116): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, PlayerSettings, Default, adjust_settings_menu(), advance_loading_phase(), AgentCommandQueue (+108 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.09
Nodes (12): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+4 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.06
Nodes (110): GameConfig, ContentCatalog, RoleDef, BTreeSet, StationDef, GridPos, ActorState, BuildingState (+102 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "PlayerRole"
Cohesion: 0.08
Nodes (15): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+7 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (125): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+117 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (30): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+22 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (221): ActorHealthFill, ActorHealthOverlay, advance_loading_runtime(), AgentEnemyModelPresentation, AgentEquipmentPresentation, AmbienceAudio, animate_loading_icon(), animation_selection_duration() (+213 more)

### Community 46 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (95): animation_take_name(), animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert() (+87 more)

### Community 47 - "Value"
Cohesion: 0.20
Nodes (27): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+19 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "command.rs"
Cohesion: 0.06
Nodes (62): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+54 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "PlayerCommands"
Cohesion: 0.13
Nodes (6): List, GameSettings, OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 57 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.09
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.14
Nodes (7): bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "SavingAndLoading.Structs"
Cohesion: 0.09
Nodes (18): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+10 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "ScriptableObject"
Cohesion: 0.11
Nodes (16): int, TechTreeSettings, Action, bool, Dictionary, IEnumerable, List, TechnologyTree (+8 more)

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
Cohesion: 0.08
Nodes (43): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join() (+35 more)

### Community 68 - "Objective"
Cohesion: 0.13
Nodes (4): Action, int, Objective, EnemyType

### Community 69 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 70 - ".default"
Cohesion: 0.05
Nodes (91): generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), apply_authored_local_rotation(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats() (+83 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "HealthModifier"
Cohesion: 0.22
Nodes (6): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "RoleHandler"
Cohesion: 0.08
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, RoleHandler, bool, Dictionary (+6 more)

### Community 77 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "Result"
Cohesion: 0.08
Nodes (63): AnimationFloatKeyframe, append_vec3_keys(), avatar_mask_id(), controller_id(), convert_chimney_smoke(), convert_fireworks(), convert_healing_vfx(), converts_authored_healing_graphs_and_prefab_overrides() (+55 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "config.rs"
Cohesion: 0.13
Nodes (23): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Option (+15 more)

### Community 83 - "String"
Cohesion: 0.04
Nodes (99): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, active_event_text(), add_animation_composition(), add_animation_layer_branch(), add_rotation_curve() (+91 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "RoleProcessor"
Cohesion: 0.05
Nodes (19): RoleSlotModifier, int, RoleSlot, bool, int, Dictionary, int, RoleDataContainer (+11 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (13): Bounds, CellPartitioningEditor, bool, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.05
Nodes (42): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+34 more)

### Community 90 - "SelectedPlayer"
Cohesion: 0.08
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedPlayer

### Community 91 - "RaidEvent"
Cohesion: 0.08
Nodes (16): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+8 more)

### Community 92 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 93 - ".Log"
Cohesion: 0.05
Nodes (20): Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor, LoadSceneMode, Scene (+12 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Rect, Pos2

### Community 97 - "SaveProcessor"
Cohesion: 0.07
Nodes (23): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+15 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (75): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve (+67 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.07
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "setup_rendering"
Cohesion: 0.06
Nodes (61): Added, Assets, authored_color_grading(), authored_post_process_stack(), bounds_material(), building_bounds_material_preserves_unity_placement_contract(), building_material(), character_material_from_standard() (+53 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "Result"
Cohesion: 0.26
Nodes (20): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role(), delete_selected_technology_group() (+12 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 108 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "GameStateProcessor"
Cohesion: 0.07
Nodes (14): Transform, float, int, List, Transform, EnemySpawner, Container, ContainerBuilder (+6 more)

### Community 113 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 114 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 115 - "ResourceInventory"
Cohesion: 0.11
Nodes (12): ResourceInventory, bool, int, float, int, Queue, ResourceRateOfChange, Dictionary (+4 more)

### Community 116 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "GUIDProcessor"
Cohesion: 0.14
Nodes (7): Container, ContainerBuilder, HashSet, GUIDProcessor, bool, Dictionary, GUIDRuntimeData

### Community 119 - "Resource"
Cohesion: 0.09
Nodes (14): int, ActiveResourceIncrementer, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor, bool (+6 more)

### Community 120 - "STSM_StateAction"
Cohesion: 0.11
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

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
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "process_injected_commands"
Cohesion: 0.18
Nodes (20): building_definition_id(), CommandOrigin, item_info(), normalized_content_name(), pending_stream_user_type(), PendingChatCommand, prefixed_id(), process_injected_commands() (+12 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - ".RestoreWorldState"
Cohesion: 0.16
Nodes (6): int, string, ObjectiveSaveData, float, int, TimeRuntimeData

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "CommandDictionary"
Cohesion: 0.27
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 130 - "AnimationHandler"
Cohesion: 0.17
Nodes (7): AnimationHandler, Animator, bool, Dictionary, float, int, AnimationName

### Community 131 - "PlayerInventory"
Cohesion: 0.16
Nodes (5): DepositResources, PlayerInventory, Dictionary, float, STSM_Action_DepositResource

### Community 132 - "WeatherProcessor"
Cohesion: 0.11
Nodes (13): Container, ContainerBuilder, WeatherProcessor, Color, float, int, VisualEffect, SeasonDataSettings (+5 more)

### Community 133 - "twitch_tab"
Cohesion: 0.29
Nodes (11): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+3 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "Dictionary"
Cohesion: 0.35
Nodes (4): Dictionary, materialIndex, Matrix4x4, meshIndex

### Community 136 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 139 - "IRuntimeDataScriptable"
Cohesion: 0.13
Nodes (13): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+5 more)

### Community 140 - ".Update"
Cohesion: 0.17
Nodes (16): List, Material, materials, Mesh, meshes, Dictionary, int, List (+8 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "legacy.rs"
Cohesion: 0.10
Nodes (54): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, checked_in_schema_one_fixture_imports_retained_terrain() (+46 more)

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

### Community 149 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 150 - ".default"
Cohesion: 0.13
Nodes (22): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), format_game_master_ids(), game_config_save_is_atomic_validated_and_round_trips(), load_game_config() (+14 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "stream_town_tools/src/main.rs"
Cohesion: 0.11
Nodes (53): bounded_ui_index(), content_tab(), draw_world_preview(), format_runtime_frame_times(), inject_runtime_command(), inspector_tab(), launch_runtime_game(), load_content_catalog() (+45 more)

### Community 154 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

### Community 155 - "parse_controller"
Cohesion: 0.23
Nodes (13): animation_state_id(), animation_state_machine_id(), clip_id(), inline_file_id(), parse_blend_tree(), parse_child_references(), parse_controller(), parse_reference_list() (+5 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

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
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 165 - ".SetTargetType"
Cohesion: 0.18
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "WorldInstanceDeterminism"
Cohesion: 0.31
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 168 - "ResourceProcessor"
Cohesion: 0.11
Nodes (9): Container, ContainerBuilder, ResourceProcessor, bool, int, Matrix4x4, uint, Vector3 (+1 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 170 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 171 - "PlayerSaveData"
Cohesion: 0.14
Nodes (11): List, InventorySaveData, int, PlayerCustomizationSaveData, bool, int, List, string (+3 more)

### Community 172 - "group_selection_action_buttons"
Cohesion: 0.24
Nodes (12): BuildingLevelEnabled, BuildingRemoveLabel, format_passive_resource_rates(), group_selection_action_buttons(), group_selection_panel_signature(), GroupDismissLabel, GroupSelectionAction, GroupSelectionActions (+4 more)

### Community 173 - "TechnologyTreeGroup"
Cohesion: 0.25
Nodes (5): Color, float, string, TechnologyTreeGroup, Group

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (20): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+12 more)

### Community 179 - "TechTreeProcessor.cs"
Cohesion: 0.08
Nodes (11): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TownGoal, TechTree.Data (+3 more)

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

### Community 184 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 185 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (82): ContainerBuilder, InstantiationBarrier, Camera, GameObject, ProjectCameraInstaller, ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder (+74 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 188 - "UserInterface_Roles"
Cohesion: 0.20
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.24
Nodes (19): ConversionReport, convert(), finite(), right_handed_quat(), right_handed_triangles(), right_handed_vec3(), Path, Result (+11 more)

### Community 192 - "ResourceHolder"
Cohesion: 0.09
Nodes (17): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent (+9 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

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
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 200 - "MonoBehaviour"
Cohesion: 0.04
Nodes (27): CameraProcessor, PersistentScoped, UnitTravelToPosition, Vector3, Projectile, Slider, TextMeshProUGUI, UI_Objective (+19 more)

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "PlayerControls"
Cohesion: 0.08
Nodes (13): ContainerBuilder, MetaDataInstaller, InputButton, Vector3, UnityGraphics, ContainerBuilder, SettingsInstaller, URP (+5 more)

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.10
Nodes (21): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+13 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 209 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 212 - "TimeProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, TimeProcessor

### Community 213 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "AudioSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "load_player_settings"
Cohesion: 0.32
Nodes (7): AnyResult, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), player_settings_path(), PathBuf, main()

### Community 218 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

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
Cohesion: 0.07
Nodes (23): Mesh, Transform, Vector3, SaveDataMapper, int, List, string, uint (+15 more)

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

### Community 237 - "DebugSettings"
Cohesion: 0.36
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 238 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "parse_property_curves"
Cohesion: 0.33
Nodes (6): AnimationTangent, parse_animation_events(), parse_property_curves(), parse_tangent(), parses_property_curves_and_animation_events_without_unity_types(), unity_scalar()

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
Cohesion: 0.22
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

### Community 261 - "PlayerDeathHandler"
Cohesion: 0.33
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

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

### Community 273 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 277 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 282 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 283 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 284 - "MainMenuSceneReference"
Cohesion: 0.57
Nodes (6): MainMenuCameraReference, MainMenuEmbeddedMesh, MainMenuModelInstance, MainMenuSceneReference, String, Vec

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 288 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 296 - "player_window_mode"
Cohesion: 0.67
Nodes (4): DisplayMode, player_window_mode(), startup_window_mode(), WindowMode

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

### Community 308 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

## Knowledge Gaps
- **297 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+292 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **28 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `RenderAssets` (4× useful, score=3.481802588) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.66401759) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.54480177)
- `WorldSimulation` (2× useful, score=1.847279109)
- `load_input()` (2× useful, score=1.696288071) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (2× useful, score=1.665761652) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.664595804)
- `MaterialDef` (2× useful, score=1.664127343) _(code changed — re-verify)_
- `BevyMigrationExporter` (2× useful, score=1.63601087)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `UpdateGraphBounds`, `IDataScriptable`, `Targetable`, `Target`, `StringUtils`, `SimpleScreenShot`, `BuildingPlacer`, `LabelDisplayProcessor`, `RandomEnabler`, `GenerationSettings`, `.CreateEnumField`, `TechTree.Elements`, `TechTreeProcessor.cs`, `ScriptablesProcessorInfrastructure`, `SavingAndLoading.Structs`, `Easings`, `.DrawDataFieldAndLabel`, `MonoBehaviour`, `PlayerControls`, `FPSDisplay`, `GameStateProcessor`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `IDataScriptable`, `TwitchChatProcessor`, `Dictionary`, `Target`, `BuildingProcessor`, `ObjectPoolingProcessor`, `Player`, `IProcessor`, `UserInterface_Debug`, `WorldInstanceDeterminism`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `ScriptablesProcessorInfrastructure`, `MonoBehaviour`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `.Log`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `GameStateProcessor`, `WorldGenRuntimeData`, `GridProcessor`, `GUIDProcessor`, `AIPath`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `TwitchChatProcessor`, `BottomBarInterface`, `Target`, `HealthHandler`, `BuildingProcessor`, `ObjectPoolingProcessor`, `BuildingPlacer`, `NewKingVote`, `Player`, `IProcessor`, `WorldGenProcessor`, `GameEventProcessor`, `UserInterface_Debug`, `TechTreeProcessor`, `PlayerRole`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `PlayerCommands`, `ScriptablesProcessorInfrastructure`, `UserInterface_Roles`, `MonoBehaviour`, `RoleHandler`, `TimeProcessor`, `RoleProcessor`, `UserInterface_TownVote`, `SelectedPlayer`, `RaidEvent`, `SaveProcessor`, `GameStateProcessor`, `Resource`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _297 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World.Generation.Settings` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._
- **Should `BuildingType` be split into smaller, more focused modules?**
  _Cohesion score 0.0549645390070922 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.057967313585291114 - nodes in this community are weakly interconnected._