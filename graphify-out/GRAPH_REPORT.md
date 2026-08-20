# Graph Report - Stream-Town-Bevy  (2026-08-21)

## Corpus Check
- 640 files · ~1,671,658 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7925 nodes · 22176 edges · 271 communities (253 shown, 18 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1021 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e617938d`
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
- Commands
- SettingsProcessor
- MeshSaveData
- Character
- Option
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- Result
- Station
- DayAndNightProcessor
- PoolableObject
- BuildingPlacer
- StableId
- UnitHealthBar
- MiscCommands
- Res
- TechTreeGraphView
- SaveFileData
- Player
- simulation.rs
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- RoleHandler
- update_environment_presentation
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
- AudioHandler
- StreamTownSessionBridge
- PlayerSettings
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Utils
- .SetTargetType
- STSM_GoToLocation
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
- NavGrid
- UserInterface_ObjectSelection
- Transform
- Goal
- AnimationHandler
- TwitchBotSetupWindow
- .AddEvent
- WorldUtils
- Option
- Vec
- Access_Text
- PlayerRoleData
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- CommonEnums.cs
- RaidEvent
- RoleData
- STSM_Action_PlayerBase
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
- ToolState
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- UserInterface_RulerVote
- STSM_Idle_Player
- convert
- Target
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- .SetGeneratedResources
- unity_color_filter
- TechTreeNode
- SnapToGridMouseMovement
- AIPath
- BuildingDamageMaterialHandler
- SimpleRotateOnAxis
- KeepKingVote
- UpdateGraphBounds
- CommandDictionary
- stream_town_tools/src/main.rs
- ResourceHolder
- UI_TechOption
- Result
- ConfirmCheck
- SimpleDisableAfterTime
- runtime_console.rs
- GateController
- SelectedPlayerGroup
- ObjectPoolingProcessor
- BuildingDataSettings
- What You Must Do When Invoked
- RuntimeData Template
- .Log
- RuntimeData Template
- Key Rules
- SelectedBuilding
- Pet
- add_file
- Vec
- DontDestroyOnLoad
- CampGenerationSettings
- Stream Town Reloaded - Architecture Documentation
- tools_ui
- .HandleSceneLoaded
- DebugSettings
- UnitTextDisplay
- Stream Town Reloaded - Architecture Documentation
- NewKingVote
- .InitializeAndActivateProcessorsAsync
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- ObjectiveSaveData
- IRuntimeDataScriptable
- Access_Toggle
- StatusBar
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- SelectedPlayer
- TransformSaveData
- ResourceRuntimeData
- TL_API
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- ObjectiveSaveData
- SelectedEnemy
- Age
- STSM_Idle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- WorldGenSaveData
- MonoBehaviour
- .default
- STSM_Helper_Build
- UserInterface_GameMenu
- UserInterface_TownGoal
- TargetProcessor
- Targetable
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- Coordinator.cs
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- TechNodeData
- SimpleScreenShot
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- ResourceTarget
- FoliageGenerationSettings
- Requirement
- ObjectPoolingRuntimeData
- RotationHandler
- import_save
- Key Rules
- ScriptablesProcessorInfrastructure
- RuntimeData Template
- TechTree_SO
- ResourceGenerationSettings
- ScriptKeywordProcessor
- FPSDisplay
- ObjectiveDef
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
- .InjectRuntimeData
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
1. `StableId` - 335 edges
2. `WorldSimulation` - 164 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 131 edges
8. `WorldGenProcessor` - 114 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

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

## Communities (271 total, 18 thin omitted)

### Community 0 - "World.Generation.Settings"
Cohesion: 0.04
Nodes (36): Transform, PlayerSpawnPoint, bool, float, VisualEffect, VfxAnimationController, CampGenerationSettings, List (+28 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.03
Nodes (42): BuildingBase, bool, float, int, List, UnityEvent, BuildPlacerData, GameObject (+34 more)

### Community 2 - "world.rs"
Cohesion: 0.10
Nodes (42): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash(), fnv_mix() (+34 more)

### Community 3 - "BTreeMap"
Cohesion: 0.11
Nodes (36): MaterialDef, PrefabPresentationBinding, RendererMaterialBinding, BTreeMap, animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses() (+28 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.02
Nodes (83): ContainerBuilder, ResourceDataSettingsInstaller, int, AudioSettings, List, CampGenSettings, float, Material (+75 more)

### Community 5 - ".default"
Cohesion: 0.03
Nodes (136): GameConfig, generate_world(), generate_world_with_content(), GeneratedWorld, advance_falling_fish(), agent_facing_matches_unity_rotation_and_action_targets(), append_terrain_skirt(), apply_recruit_group_order() (+128 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (22): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData, bool, float (+14 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Commands"
Cohesion: 0.03
Nodes (139): BackgroundColor, ChimneySmokeDef, ActionPresentation, actor_combat_visual(), actor_material(), animate_agents(), animate_chimney_smoke_particles(), animate_healing_effects() (+131 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "MeshSaveData"
Cohesion: 0.14
Nodes (9): Mesh, Vector3, bool, int, MeshSaveData, float, Vector2SaveData, float (+1 more)

### Community 12 - "Character"
Cohesion: 0.07
Nodes (20): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+12 more)

### Community 13 - "Option"
Cohesion: 0.07
Nodes (92): ArchetypeScene, PresentationCatalog, actor_detail_budget(), actor_scene_budget(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), animation_property_value(), archetype_by_source(), archetype_scene_for_age() (+84 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.09
Nodes (11): Func, Action, float, Enemy, Action, bool, float, int (+3 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (41): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+33 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "Result"
Cohesion: 0.09
Nodes (48): AvatarMaskDef, avatar_mask_id(), clip_id(), controller_id(), convert(), convert_avatar_masks(), convert_chimney_smoke(), convert_clips() (+40 more)

### Community 19 - "Station"
Cohesion: 0.07
Nodes (15): Station, Dictionary, float, int, List, Queue, Transform, List (+7 more)

### Community 20 - "DayAndNightProcessor"
Cohesion: 0.13
Nodes (7): Container, ContainerBuilder, DayAndNightProcessor, IPostInitializeProcessor, bool, float, DayAndNightRuntimeData

### Community 21 - "PoolableObject"
Cohesion: 0.06
Nodes (19): Container, ContainerBuilder, GUIDProcessor, BoxCollider, List, Quaternion, Vector3, bool (+11 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "StableId"
Cohesion: 0.07
Nodes (39): FromStr, StableId, BuildingState, complete_gameplay_scenario_round_trips(), EnemyCampState, FishGodState, RaidState, BTreeMap (+31 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "MiscCommands"
Cohesion: 0.09
Nodes (6): Vector3, BuildingCommands, Dictionary, MiscCommands, Dictionary, MessageSender

### Community 26 - "Res"
Cohesion: 0.04
Nodes (204): Aabb, AccumulatedMouseMotion, AccumulatedMouseScroll, AnimationTransitions, AppExit, AudioSink, ActorNameOverlay, advance_loading_phase() (+196 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, GameMasterCommands, TwitchClientProcessor, PlayerCommands, RoleCommands (+2 more)

### Community 30 - "simulation.rs"
Cohesion: 0.08
Nodes (32): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+24 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (14): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+6 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.13
Nodes (8): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType, EventTester

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "RoleHandler"
Cohesion: 0.08
Nodes (11): RoleSlotModifier, int, RoleHandler, bool, Dictionary, UnityEvent, Container, ContainerBuilder (+3 more)

### Community 36 - "update_environment_presentation"
Cohesion: 0.13
Nodes (21): AmbientLight, animate_weather_particles(), building_snow_strength(), environment_palette(), environment_palette_covers_every_season_and_weather(), EnvironmentPalette, EnvironmentPresentation, grass_season_controls() (+13 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.04
Nodes (38): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+30 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.08
Nodes (77): ContentCatalog, GridPos, ActorState, String, action_animation_speed(), action_cooldown(), actor_accepts_resource(), actor_archetype() (+69 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.07
Nodes (20): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+12 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (124): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+116 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (28): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+20 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (288): AnyResult, active_event_text(), ActorHealthFill, ActorHealthOverlay, adjust_settings_menu(), advance_loading_runtime(), agent_action_animation(), agent_is_moving() (+280 more)

### Community 46 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (67): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), glb_animation_names(), infer_missing_parameters(), inline_file_id(), normalized_path(), parse_animation_events() (+59 more)

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (37): ActorCustomization, StreamUserType, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies() (+29 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".Draw"
Cohesion: 0.09
Nodes (22): NodeUnlockSaveData, Port, Button, EnumField, UnlockVisualElement, Action, Button, EnumField (+14 more)

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
Cohesion: 0.08
Nodes (15): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, TwitchClientProcessor (+7 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Utils"
Cohesion: 0.05
Nodes (12): DisableOnAwake, SelectionBase, STStateMachine.States, Utils, Behaviours, Animation, STStateMachine, Pathfinding (+4 more)

### Community 57 - ".SetTargetType"
Cohesion: 0.15
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "Result"
Cohesion: 0.31
Nodes (4): BinaryParser<'a>, decode_binary(), Result, LegacyWorldState

### Community 61 - "UserInterface"
Cohesion: 0.05
Nodes (16): InputButton, SharedTypes, int, ChangeTimeStamp, Image, TextMeshProUGUI, UIRoleDisplay, DataStructures (+8 more)

### Community 62 - "CameraController"
Cohesion: 0.05
Nodes (17): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+9 more)

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
Cohesion: 0.06
Nodes (56): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet, Default (+48 more)

### Community 68 - "Objective"
Cohesion: 0.15
Nodes (3): Action, int, Objective

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
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.06
Nodes (20): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+12 more)

### Community 74 - "NavGrid"
Cohesion: 0.12
Nodes (22): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+14 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "Transform"
Cohesion: 0.07
Nodes (40): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), animation_target_for_track(), building_construction_stage(), building_node_visibility() (+32 more)

### Community 77 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.05
Nodes (25): AnimationHandler, Animator, bool, Dictionary, float, int, bool, int (+17 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "Option"
Cohesion: 0.14
Nodes (28): animation_take_name(), color_value(), convert_embedded_model_clips(), convert_post_process(), embedded_clip_id(), extracts_indexed_material_properties(), field_bool(), field_f32() (+20 more)

### Community 83 - "Vec"
Cohesion: 0.04
Nodes (93): Added, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, Assets, ActivePetVisual, ActorAnimationDriver (+85 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRoleData"
Cohesion: 0.09
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+6 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.13
Nodes (12): Bounds, bool, Vector2, BSPCell, Dictionary, float, int, List (+4 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.08
Nodes (9): ProjectileShooter, float, int, string, SensorBase, bool, float, UnityEvent (+1 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (21): Vector3, List, List, Dictionary, List, TargetRuntimeData, TargetableData, Dictionary (+13 more)

### Community 91 - "RaidEvent"
Cohesion: 0.08
Nodes (15): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+7 more)

### Community 92 - "RoleData"
Cohesion: 0.08
Nodes (20): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+12 more)

### Community 93 - "STSM_Action_PlayerBase"
Cohesion: 0.07
Nodes (12): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, int, STSM_Helper_Attack (+4 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "RoleDataContainer"
Cohesion: 0.10
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 96 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (19): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+11 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (17): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (73): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+65 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.06
Nodes (15): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnChatCommandReceivedArgs (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.10
Nodes (12): PlayerDeathHandler, bool, float, Vector3, bool, List, string, uint (+4 more)

### Community 102 - "IProcessor.cs"
Cohesion: 0.22
Nodes (8): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport, ProcessorStartupStage

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - "ToolState"
Cohesion: 0.15
Nodes (22): apply_foliage_draft(), redo_catalog_edit(), refresh_catalog_drafts(), refresh_foliage_draft(), Arc, Default, Mutex, Receiver (+14 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.21
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

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

### Community 114 - "STSM_Idle_Player"
Cohesion: 0.13
Nodes (6): STSM_Action_GatherResource, bool, float, uint, Vector3, STSM_Idle_Player

### Community 115 - "convert"
Cohesion: 0.17
Nodes (17): ActorKind, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert() (+9 more)

### Community 116 - "Target"
Cohesion: 0.10
Nodes (10): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Sensors, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects (+2 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.18
Nodes (10): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+2 more)

### Community 119 - "Resource"
Cohesion: 0.03
Nodes (41): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+33 more)

### Community 120 - ".SetGeneratedResources"
Cohesion: 0.15
Nodes (16): List, Material, materials, Mesh, meshes, Dictionary, int, List (+8 more)

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

### Community 125 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 126 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "CommandDictionary"
Cohesion: 0.10
Nodes (10): Action, Dictionary, IReadOnlyList, List, CommandDictionary, OnMessageReceivedArgs, OnChatCommandReceivedArgs, OnMessageReceivedArgs (+2 more)

### Community 130 - "stream_town_tools/src/main.rs"
Cohesion: 0.16
Nodes (23): bounded_ui_index(), format_runtime_frame_times(), inject_runtime_command(), launch_runtime_game(), poll_runtime_console(), preview_lerp_color(), refresh_technology_draft(), role_draft() (+15 more)

### Community 131 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 132 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 133 - "Result"
Cohesion: 0.23
Nodes (22): apply_role_draft(), apply_technology_draft(), checked_in_authoring_assets_pass_headless_validation(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node() (+14 more)

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

### Community 138 - "SelectedPlayerGroup"
Cohesion: 0.21
Nodes (3): List, List, SelectedPlayerGroup

### Community 139 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (24): Container, IMainThreadInitializableProcessor, IProcessor, Container, ContainerBuilder, DebugProcessor, Action, Container (+16 more)

### Community 140 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - ".Log"
Cohesion: 0.10
Nodes (8): HideInCallstack, Object, Action, CancellationToken, Container, LoadSceneMode, Scene, Task

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

### Community 149 - "Vec"
Cohesion: 0.16
Nodes (19): binary_fixture(), BinaryParser, legacy_objective_matches(), LegacyDecodedSave, LegacyGoal, LegacyObjective, objective_target_matches(), put_f32() (+11 more)

### Community 151 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "tools_ui"
Cohesion: 0.15
Nodes (22): content_tab(), delete_selected_role(), draw_world_preview(), duplicate_selected_role(), inspector_tab(), migration_tab(), poll_tool_job_events(), poll_twitch_tool_events() (+14 more)

### Community 155 - "DebugSettings"
Cohesion: 0.31
Nodes (4): Dictionary, DebugSettings, DebugLogCategory, SerializedScriptableObject

### Community 156 - "UnitTextDisplay"
Cohesion: 0.12
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 159 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.19
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

### Community 163 - "ObjectiveSaveData"
Cohesion: 0.17
Nodes (8): Button, EnumField, ObjectiveVisualElement, ObjectiveSaveData, ObjectiveType, EnemyType, TownGoal.Enumerations, VisualElement

### Community 164 - "IRuntimeDataScriptable"
Cohesion: 0.06
Nodes (25): Container, ContainerBuilder, LabelDisplayProcessor, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, Dictionary (+17 more)

### Community 165 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 166 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 167 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.13
Nodes (12): Container, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool, int (+4 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 171 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 172 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

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
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 179 - "STSM_Idle"
Cohesion: 0.17
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

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

### Community 184 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 185 - "MonoBehaviour"
Cohesion: 0.01
Nodes (118): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+110 more)

### Community 186 - ".default"
Cohesion: 0.21
Nodes (14): authority_tab(), default_catalog_path(), default_config_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_game_config(), PathBuf, Self (+6 more)

### Community 188 - "STSM_Helper_Build"
Cohesion: 0.18
Nodes (5): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase

### Community 190 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 191 - "TargetProcessor"
Cohesion: 0.20
Nodes (6): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor

### Community 192 - "Targetable"
Cohesion: 0.07
Nodes (19): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+11 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "Coordinator.cs"
Cohesion: 0.09
Nodes (5): InputButton, UserInterface.MainMenu, Settings, PlayerControls, World.Generation

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "TechNodeData"
Cohesion: 0.25
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

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
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 209 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 212 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 213 - "import_save"
Cohesion: 0.43
Nodes (7): absolute_path(), backup_candidate(), import_preserves_source_and_recovers_named_backup(), import_save(), ImportReport, Path, PathBuf

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "TechTree_SO"
Cohesion: 0.33
Nodes (3): NodeGroup_SO, List, TechTree_SO

### Community 218 - "ResourceGenerationSettings"
Cohesion: 0.33
Nodes (5): AnimationCurve, bool, int, List, ResourceGenerationSettings

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "ObjectiveDef"
Cohesion: 0.60
Nodes (4): ObjectiveDef, ObjectiveKind, objective_increment(), ObjectiveEvent

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "PlayerSaveData"
Cohesion: 0.08
Nodes (18): Component, Dictionary, Transform, bool, int, List, string, InventoryEntrySaveData (+10 more)

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
Nodes (23): Audio provenance, Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows, Binaries, Commands (+15 more)

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
Cohesion: 0.04
Nodes (23): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, RoleScriptablesEditor (+15 more)

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

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
Cohesion: 0.12
Nodes (15): HashSet, Func, HashSet, List, Material, Mesh, Resource, Vector2 (+7 more)

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
Cohesion: 0.05
Nodes (10): BuildCostModifier, ObjectSelectionProcessor, Processors, World, Level, Processors.Editor, MetaData, Buildings (+2 more)

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
- **297 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+292 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `SettingsProcessor`, `MeshSaveData`, `Character`, `ObjectPoolingProcessor`, `PoolableObject`, `SaveFileData`, `WorldGenProcessor`, `GameEventProcessor`, `RoleHandler`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `ResourceDataSaveData`, `StreamTownSessionBridge`, `MonoBehaviour`, `UserInterface_GameMenu`, `WorldSaveData`, `.AddEvent`, `FoliageProcessor`, `PlayerSaveData`, `PlayerProcessor`, `TownGoalProcessor`, `MainMenuManager`, `Resource`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `.GenerateFromSettings`, `ObjectPoolingProcessor`, `.Log`, `PoolableObject`, `Player`, `UserInterface_Debug`, `GridProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `MonoBehaviour`, `CellSpacePartitioning`, `FoliageProcessor`, `RaidEvent`, `PlayerSaveData`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `EnemySpawner`, `Target`, `AIPath`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `world.rs`, `BTreeMap`, `stream_town_tools/src/main.rs`, `.default`, `Result`, `runtime_console.rs`, `Commands`, `Option`, `stream_town_domain/src/content.rs`, `save.rs`, `Result`, `Vec`, `tools_ui`, `Res`, `simulation.rs`, `ContentCatalog`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `stream_town_game/src/lib.rs`, `stream_town_migrate/src/presentation.rs`, `twitch.rs`, `command.rs`, `Transform`, `Option`, `Vec`, `ObjectiveDef`, `stream_town_domain/src/presentation.rs`, `ToolState`, `convert`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _297 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World.Generation.Settings` be split into smaller, more focused modules?**
  _Cohesion score 0.03695324283559578 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.031692406692406694 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10453283996299723 - nodes in this community are weakly interconnected._