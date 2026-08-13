# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 610 files · ~1,615,401 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7175 nodes · 19110 edges · 258 communities (235 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 995 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2fc081a6`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- xtask/src/main.rs
- BuildingProcessor
- RenderAssets
- stream_town_migrate/src/presentation.rs
- drive_converted_animations
- STSM_StateAction
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- runtime_console.rs
- SettingsProcessor
- WorldGenProcessor
- legacy.rs
- IRuntimeDataScriptable
- TechTreeIOUtility
- STSM_Idle
- process_injected_commands
- SensorProcessor
- .GenerateFromSettings
- IProcessor.cs
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- RoleHandler
- Option
- BottomBarRuntime
- SaveFileData
- GameEventProcessor
- ContentCatalog
- StatModifiers
- STSM_Idle_Player
- RoleData
- UnityAsset
- UserInterface_Debug
- CommandDictionary
- SettingsData
- AnimationHandler
- stream_town_domain/src/content.rs
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- ObjectiveDef
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
- config.rs
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- world.rs
- StableId
- models.rs
- Tiler
- ScriptablesEditor
- PlayerSettings
- UserInterface_ObjectSelection
- SelectedPlayer
- ProjectCamera
- BinarySaveCodec
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- String
- UnitTextDisplay
- Access_Text
- stream_town_game/src/lib.rs
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- DebugProcessor
- BuildingBase
- SelectedPlayerGroup
- GameEvent
- convert_fbx_to_glb.py
- command.rs
- stream_town_migrate/src/content.rs
- UserInterface_Roles
- Coordinator
- stream_town_domain/src/presentation.rs
- convert_archetypes
- technology_tab
- MeshSaveData
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
- Enemy
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Station
- UserInterface_GameMenu
- TransformSaveData
- UpdateGraphBounds
- TargetProcessor
- .EnsureValidCredentials
- Res
- BuildingDataSettings
- EventProcessor
- ConfirmCheck
- TwitchUser
- ToolState
- ResourceHolder
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
- BuildingSettings
- Requirement
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- SeasonDataSettings
- IProcessor
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- ErrorData
- SimpleCancelBuildingPlacer
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
- PoolableObject
- GameStateProcessor
- PassiveResourceIncrementer
- SelectedResource
- WorldGenRuntimeData
- CommonEnums.cs
- BuildPlacerData
- CreditsProcessor
- SelectedEnemyCamp
- settings.rs
- Player
- BuildingDamageMaterialHandler
- PlayerRoleSaveData
- MeshData
- PlayerInputRuntimeData
- UILineRenderer
- UserInterface_DisplayUsernames
- UnitTravelToPosition
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- WeatherProcessor
- MonoBehaviour
- Key Rules
- BuildingRuntimeData
- Common Patterns
- BuildCostModifier.cs
- InputButton.cs
- setup_camera
- UI_TechOption
- ScriptablesProcessorInfrastructure
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- VfxAnimationController
- ScriptKeywordProcessor
- Processor Template
- Common Patterns
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- STSM_HelperBase
- Twitch setup
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Utils
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- CreateProjectScopeProcessors.cs
- KeepKingVote
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
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
- World.Generation
- UnityGraphics
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
10. `ContentCatalog` - 91 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `stress()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/xtask/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (258 total, 23 thin omitted)

### Community 0 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 1 - "BuildingProcessor"
Cohesion: 0.08
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "RenderAssets"
Cohesion: 0.09
Nodes (79): BackgroundColor, GameConfig, GeneratedResource, actor_material(), bottom_bar_texture(), building_effect_material(), BuildingEffectKind, BuildingEffectParticle (+71 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "drive_converted_animations"
Cohesion: 0.07
Nodes (58): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, ActorAnimationDriver, add_animation_layer_branch(), add_rotation_curve() (+50 more)

### Community 5 - "STSM_StateAction"
Cohesion: 0.11
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (26): List, GameSettings, MetaData, bool, float, Func, int, PlayerExistsByIDDelegate (+18 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (16): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+8 more)

### Community 12 - "legacy.rs"
Cohesion: 0.05
Nodes (112): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+104 more)

### Community 13 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (23): ContainerBuilder, MetaDataInstaller, InputButton, SharedTypes, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool (+15 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 16 - "process_injected_commands"
Cohesion: 0.05
Nodes (61): StreamUserType, active_event_text(), AuthoredCreditsElement, bottom_bar_authored_order(), bottom_bar_entries(), BottomBarAction, building_definition_id(), building_icon_path() (+53 more)

### Community 17 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 18 - ".GenerateFromSettings"
Cohesion: 0.08
Nodes (27): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+19 more)

### Community 19 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.07
Nodes (21): int, ChangeTimeStamp, ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData (+13 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.09
Nodes (14): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+6 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.09
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 24 - "HealthHandler"
Cohesion: 0.10
Nodes (11): PlayerDeathHandler, bool, float, Vector3, Action, bool, float, int (+3 more)

### Community 25 - "RoleHandler"
Cohesion: 0.09
Nodes (10): PlayerRoleData, AudioClip, bool, float, int, RoleHandler, bool, Dictionary (+2 more)

### Community 26 - "Option"
Cohesion: 0.05
Nodes (82): AmbientLight, AssetServer, PresentationCatalog, animation_property_value(), animation_root_name(), apply_material_overrides(), building_material(), BuildingMaterialExtension (+74 more)

### Community 27 - "BottomBarRuntime"
Cohesion: 0.30
Nodes (12): bottom_bar_action_buttons(), bottom_bar_main_buttons(), bottom_bar_scroll_buttons(), BottomBarContext, BottomBarMainButton, BottomBarRuntime, BottomBarScrollButton, toggle_bottom_bar_context() (+4 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "ContentCatalog"
Cohesion: 0.04
Nodes (149): ArchetypeDef, ArchetypeKind, ArchetypeScene, BuildingDef, BuildingModelDef, ContentCatalog, EnemySpawnerDef, HealthDef (+141 more)

### Community 31 - "StatModifiers"
Cohesion: 0.21
Nodes (8): StatModifiers, Dictionary, Dictionary, List, Queue, Transform, PlayerRuntimeData, StatType

### Community 32 - "STSM_Idle_Player"
Cohesion: 0.06
Nodes (16): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_GatherResource (+8 more)

### Community 33 - "RoleData"
Cohesion: 0.09
Nodes (18): RoleData, AudioClip, bool, float, int, Sprite, string, Dictionary (+10 more)

### Community 34 - "UnityAsset"
Cohesion: 0.24
Nodes (34): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), field_value(), foliage_layers(), generated_record_ids(), objective_definitions() (+26 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 39 - "stream_town_domain/src/content.rs"
Cohesion: 0.14
Nodes (20): ArchetypeBounds, AuthoredRecord, AuthoredValue, ContentError, FoliageVariantDef, ObjectiveKind, PassiveResourceContribution, ProjectileShooterDef (+12 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "ObjectiveDef"
Cohesion: 0.32
Nodes (6): ObjectiveDef, objective_increment(), ObjectiveEvent, ObjectiveProgress, Vec, TownGoalState

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.09
Nodes (12): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+4 more)

### Community 47 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 48 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (16): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+8 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

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
Cohesion: 0.07
Nodes (30): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+22 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "config.rs"
Cohesion: 0.14
Nodes (17): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+9 more)

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
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "world.rs"
Cohesion: 0.18
Nodes (19): WorldGenConfig, FoliageHabitat, FoliageLayerDef, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise() (+11 more)

### Community 70 - "StableId"
Cohesion: 0.11
Nodes (19): FromStr, StableId, BuildingState, complete_gameplay_scenario_round_trips(), EnemyCampState, RaidState, BTreeMap, BTreeSet (+11 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.05
Nodes (23): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, RoleScriptablesEditor, bool (+15 more)

### Community 74 - "PlayerSettings"
Cohesion: 0.13
Nodes (17): PlayerSettings, Default, CreditsTimeline, generate_connect_code(), MenuPage, MenuRuntime, open_settings_menu(), player_msaa() (+9 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.17
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "String"
Cohesion: 0.15
Nodes (20): animation_parameter_name(), authored_mask(), child_technology_guids(), ContentConversionReport, convert(), decomposes_combined_unity_flag_values(), mask_ids(), normalized_path() (+12 more)

### Community 83 - "UnitTextDisplay"
Cohesion: 0.12
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (164): AnyResult, AudioSink, ActionPresentation, ActivePetVisual, actor_combat_visual(), actor_detail_budget(), actor_scene_budget(), adjust_settings_menu() (+156 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "DebugProcessor"
Cohesion: 0.10
Nodes (11): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+3 more)

### Community 91 - "BuildingBase"
Cohesion: 0.12
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 92 - "SelectedPlayerGroup"
Cohesion: 0.16
Nodes (3): List, List, SelectedPlayerGroup

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
Cohesion: 0.15
Nodes (37): asset(), authored_value(), building_model_definitions(), building_node_age(), component(), component_field_value(), component_reference_name(), component_reference_names() (+29 more)

### Community 97 - "UserInterface_Roles"
Cohesion: 0.14
Nodes (6): bool, Dictionary, GameObject, Transform, UserInterface_Roles, Color32

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (20): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+12 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "convert_archetypes"
Cohesion: 0.17
Nodes (16): ArchetypesById, EnemyDef, ResourceReward, archetype_kind(), archetype_scenes(), building_placements(), BuildingPlacement, collect_model_dependencies() (+8 more)

### Community 101 - "technology_tab"
Cohesion: 0.31
Nodes (16): apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node(), refresh_technology_draft(), Option (+8 more)

### Community 102 - "MeshSaveData"
Cohesion: 0.14
Nodes (9): Mesh, Vector3, bool, int, MeshSaveData, float, Vector2SaveData, float (+1 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.15
Nodes (6): Button, GameObject, IEnumerator, int, MainMenuManager, Inject

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.08
Nodes (15): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+7 more)

### Community 107 - "PlayerSaveData"
Cohesion: 0.08
Nodes (18): Component, Dictionary, Transform, bool, int, List, string, InventoryEntrySaveData (+10 more)

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
Cohesion: 0.13
Nodes (10): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+2 more)

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 114 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (10): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+2 more)

### Community 115 - "Targetable"
Cohesion: 0.12
Nodes (8): bool, BoxCollider, float, int, Transform, Vector3, Targetable, IPooledObjectReset

### Community 116 - "LabelDisplayProcessor"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller (+6 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.16
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (34): DepositResources, ResourceStorageModifier, float, int, PlayerInventory, Dictionary, ResourceInventory, bool (+26 more)

### Community 120 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 121 - "Enemy"
Cohesion: 0.06
Nodes (19): Action, float, Enemy, Animator, GameObject, IEnumerator, int, FishGodEvent (+11 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.11
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 125 - "Station"
Cohesion: 0.07
Nodes (18): Station, Dictionary, float, int, List, Queue, Transform, Container (+10 more)

### Community 127 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 130 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 131 - "Res"
Cohesion: 0.05
Nodes (148): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimationGraphHandle, App, AppExit, Assets, ActorNameOverlay (+140 more)

### Community 132 - "BuildingDataSettings"
Cohesion: 0.17
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

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

### Community 137 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

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
Nodes (27): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+19 more)

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
Cohesion: 0.10
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "BuildingSettings"
Cohesion: 0.18
Nodes (5): bool, Dictionary, int, BuildingSettings, Age

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
Cohesion: 0.15
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

### Community 158 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

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
Cohesion: 0.08
Nodes (15): GateController, Animator, bool, int, List, float, Material, Vector2 (+7 more)

### Community 165 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 166 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 167 - "GridProcessor"
Cohesion: 0.04
Nodes (28): int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor (+20 more)

### Community 169 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 170 - "RoleProcessor"
Cohesion: 0.08
Nodes (13): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+5 more)

### Community 171 - "TimeProcessor"
Cohesion: 0.16
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 172 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

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

### Community 182 - "PoolableObject"
Cohesion: 0.09
Nodes (18): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+10 more)

### Community 183 - "GameStateProcessor"
Cohesion: 0.19
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 184 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 186 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 187 - "CommonEnums.cs"
Cohesion: 0.12
Nodes (16): Vector3, TargetableData, Dictionary, List, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus (+8 more)

### Community 188 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 189 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 191 - "settings.rs"
Cohesion: 0.11
Nodes (27): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+19 more)

### Community 192 - "Player"
Cohesion: 0.06
Nodes (13): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, OnChatCommandReceivedArgs (+5 more)

### Community 193 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 194 - "PlayerRoleSaveData"
Cohesion: 0.33
Nodes (3): List, int, PlayerRoleSaveData

### Community 195 - "MeshData"
Cohesion: 0.16
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 196 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

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
Cohesion: 0.01
Nodes (112): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, PersistentScoped, ContainerBuilder, Volume (+104 more)

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 212 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 213 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (9): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, BuildingScriptablesEditor, Reflex.Core, Data.Containers, Settings (+1 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.12
Nodes (9): Api, TL_API, Image, TextMeshProUGUI, StatusBar, int, string, SimpleScreenShot (+1 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 218 - "VfxAnimationController"
Cohesion: 0.13
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

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

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "Utils"
Cohesion: 0.04
Nodes (39): STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Utils, Processors, Pets.Enumerations, StreamTown.EditorTools (+31 more)

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.07
Nodes (23): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+15 more)

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

### Community 262 - "World.Generation"
Cohesion: 0.03
Nodes (41): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller (+33 more)

### Community 264 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 280 - "ScriptableObject"
Cohesion: 0.02
Nodes (97): ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller (+89 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

## Knowledge Gaps
- **232 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+227 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `World.Generation`, `SettingsProcessor`, `WorldGenProcessor`, `ObjectPoolingProcessor`, `PlayerProcessor`, `ScriptableObject`, `IProcessor`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `TechTreeProcessor`, `RoleProcessor`, `TimeProcessor`, `SeasonProcessor`, `GUIDProcessor`, `StreamTownSessionBridge`, `ResourceProcessor`, `MonoBehaviour`, `FoliageProcessor`, `DebugProcessor`, `WorldSaveData`, `MeshSaveData`, `TownGoalProcessor`, `MainMenuManager`, `Utils`, `PlayerSaveData`, `Resource`, `UserInterface_GameMenu`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `World.Generation`, `TwitchChatProcessor`, `ObjectPoolingProcessor`, `.GenerateFromSettings`, `PlayerProcessor`, `ScriptableObject`, `IProcessor`, `UserInterface_Debug`, `GridProcessor`, `GUIDProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `GameStateProcessor`, `WorldGenRuntimeData`, `Player`, `MonoBehaviour`, `ProjectCamera`, `CellSpacePartitioning`, `FoliageProcessor`, `DebugProcessor`, `Utils`, `PlayerSaveData`, `EnemySpawner`, `SaveProcessor`, `Enemy`, `AIPath`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **Why does `Player` connect `Player` to `TwitchChatProcessor`, `TwitchUser`, `Pet`, `BuildingPlacer`, `PlayerProcessor`, `HealthHandler`, `RoleHandler`, `GameEventProcessor`, `SimpleCancelBuildingPlacer`, `StatModifiers`, `UserInterface_Debug`, `CommandDictionary`, `MiscCommands`, `CharacterModelHandler`, `StreamTownSessionBridge`, `PoolableObject`, `UserInterface_DisplayUsernames`, `BuildingRuntimeData`, `UnitTextDisplay`, `TargetSensor`, `UserInterface_Roles`, `Utils`, `SaveProcessor`, `LabelDisplayProcessor`, `VoteEvent`, `Resource`, `.SetTargetType`, `Station`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _232 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.0782312925170068 - nodes in this community are weakly interconnected._
- **Should `RenderAssets` be split into smaller, more focused modules?**
  _Cohesion score 0.09250243427458617 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/presentation.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10367063492063493 - nodes in this community are weakly interconnected._