# Graph Report - Stream-Town-Bevy  (2026-08-25)

## Corpus Check
- 667 files · ~1,754,237 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8607 nodes · 24984 edges · 312 communities (283 shown, 29 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1033 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4f6c135b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UserInterface
- BuildingProcessor
- world.rs
- BinarySaveCodec
- MonoBehaviour
- setup_rendering
- TwitchChatProcessor
- finish_world_reveal
- BottomBarInterface
- BTreeMap
- SettingsProcessor
- BuildingPlacer
- World.Generation.Settings
- Units
- TechTreeIOUtility
- HealthHandler
- config.rs
- save.rs
- StableId
- EnemyModelHandler
- twitch.rs
- IRuntimeDataScriptable
- Target
- RoleHandler
- UnitHealthBar
- command.rs
- SensorProcessor
- TechTreeSearchWindow
- SaveFileData
- stream_town_migrate/src/presentation.rs
- Query
- WorldGenProcessor
- UserInterface_Debug
- GameEvent
- GenerationSettings
- PlayerProcessor
- .CreateEnumField
- SettingsData
- SeasonProcessor
- TechnologyGraphLayout
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- simulation.rs
- AnimationControllerDef
- runtime_console.rs
- UserInterface_RulerVote
- .RecalculateStats
- FoliageGenerationSettings
- .Draw
- AudioHandler
- StreamTownSessionBridge
- WorldGenSaveData
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- BuildingBase
- SelectedObject
- STSM_StateAction
- TechTreeEditorWindow
- Result
- TechnologyGraphViewState
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- .StartupSequence
- GameEventProcessor
- PoolableObject
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- UserInterface_TownGoal
- UserInterface_ObjectSelection
- Targetable
- UserInterface_TownVote
- TechTreeNode
- TwitchBotSetupWindow
- BuildingResourceModelHandler
- WorldUtils
- Node_SO
- RenderAssets
- Access_Text
- legacy.rs
- stream_town_game/src/lib.rs
- stream_town_migrate/src/technology_layout.rs
- Option
- FoliageProcessor
- Processors
- RaidEvent
- stream_town_domain/src/lib.rs
- SelectedBuilding
- convert_fbx_to_glb.py
- RoleDataSettings
- Resource
- SaveProcessor
- .GetMissingDataScriptableDependencies
- stream_town_domain/src/presentation.rs
- World.Generation
- StateMachine
- AnimationHandler
- TownGoalProcessor
- MainMenuManager
- ResourceProcessor
- LoadingManager
- CampGenerationSettings
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- VfxSeagullSpawner
- SeasonDataSettings
- ErrorData
- GridNode
- stream_town_migrate/src/main.rs
- Goal
- .StartMusic
- VoteEvent
- unity_color_filter
- Access_Dropdown
- SnapToGridMouseMovement
- AIPath
- .CalculateDayCount
- GateController
- UpdateGraphBounds
- RotationHandler
- TransformSaveData
- IProcessor
- StringUtils
- BuildingDamageMaterialHandler
- stream_town_tools/src/main.rs
- TechTree.Elements
- SelectedResource
- STSM_Idle_Player
- BuildingDataSettings
- MiscCommands
- setup_world_audio_sources
- What You Must Do When Invoked
- RuntimeData Template
- Value
- RuntimeData Template
- Key Rules
- DayAndNightSettings
- Pet
- add_file
- CellSpacePartitioning
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- Character
- STSM_HelperBase
- Result
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WindController
- Access_GOList
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- EquipmentHandlerEditor
- PlayerInputProcessor
- IProcessor.cs
- TargetableHealth
- SimpleMusicController
- GameEventSettings
- Q: There are still no animations.
- xtask/src/main.rs
- List
- tools_ui
- RandomEnabler
- Access_Toggle
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- BuildingModelHandler
- WeatherProcessor
- SelectedEnemyCamp
- SimpleDisableAfterTime
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- PassiveResourceIncrementer
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- PlayerInputRuntimeData
- twitch_tab
- WorldGenerationReferenceExporter
- ScriptableObject
- ProjectCamera
- BevyMigrationExporter
- Utils
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- VfxAnimationController
- UserInterface_GameMenu
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- Season
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- .InitializeAndActivateProcessorsAsync
- WorldGenRuntimeData
- Requirement
- CommandDictionary
- SelectedEnemy
- .OnGUI
- Key Rules
- Access_TextInput
- RuntimeData Template
- Character Animation Regression Checklist
- Easings
- ScriptKeywordProcessor
- FPSDisplay
- NodeUnlockData
- Processor Template
- Common Patterns
- Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system.
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- bevy-port/README.md
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- TL_API
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- RuntimeCommandQueues
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- String
- extraction-spec.md
- .Log
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- SelectedPlayer
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- .MoveCamera
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Coordinator
- Q: If there is more to do, keep going.
- UnitTextDisplay
- .visit
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- UnitTravelToPosition
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- .InjectRuntimeData
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Editor
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- setup_camera
- DontDestroyOnLoad
- StreamTown.Migration
- PlayerSaveData
- STSM_Action_PlayerBase
- PlayerSpawnPoint
- CreateProjectScopeProcessors.cs
- PlayerSettings
- ScriptablesProcessorInfrastructure
- SimpleScreenShot
- VfxParticlePosition
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ObjectSelectionProcessor.Editor.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- ForwardRendererInstaller
- IntWrapper
- UnityGraphics
- VideoSettingsPresetsInstaller
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Globals
- .RestoreObjectiveProgress
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- Autosave

## God Nodes (most connected - your core abstractions)
1. `StableId` - 369 edges
2. `WorldSimulation` - 175 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ContentCatalog` - 154 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 126 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `handle_twitch_event()` --calls--> `unity_command_usage()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (312 total, 29 thin omitted)

### Community 0 - "UserInterface"
Cohesion: 0.07
Nodes (10): InputButton, SharedTypes, TownGoal.Data, StreamTown.EditorTools, TownGoal, UserInterface, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (16): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+8 more)

### Community 2 - "world.rs"
Cohesion: 0.06
Nodes (67): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+59 more)

### Community 3 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 4 - "MonoBehaviour"
Cohesion: 0.02
Nodes (87): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+79 more)

### Community 5 - "setup_rendering"
Cohesion: 0.05
Nodes (80): AmbientLight, ActiveMaterialHandles, authored_post_process_stack(), building_material(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstances, character_material() (+72 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "finish_world_reveal"
Cohesion: 0.04
Nodes (70): AccessibilityNode, AssetId, AtomicU64, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), AnimatedCharacterShadowReceiver, asset_root_collection_ready() (+62 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "BTreeMap"
Cohesion: 0.12
Nodes (42): AnimationClipDef, MaterialDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks(), convert_clips() (+34 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "BuildingPlacer"
Cohesion: 0.06
Nodes (25): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+17 more)

### Community 12 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 13 - "Units"
Cohesion: 0.07
Nodes (9): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Animation, Sensors, STStateMachine, Pathfinding (+1 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.06
Nodes (15): Func, PlayerDeathHandler, bool, float, Vector3, Action, float, Enemy (+7 more)

### Community 16 - "config.rs"
Cohesion: 0.14
Nodes (21): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Default, Option, Result, Self (+13 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "StableId"
Cohesion: 0.03
Nodes (275): AnimationTransitionOutcome, GameConfig, GameplayConfig, BTreeMap, ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef (+267 more)

### Community 19 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 20 - "twitch.rs"
Cohesion: 0.08
Nodes (43): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join() (+35 more)

### Community 21 - "IRuntimeDataScriptable"
Cohesion: 0.13
Nodes (16): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, Dictionary, GameObject (+8 more)

### Community 22 - "Target"
Cohesion: 0.12
Nodes (7): Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies, GUIDSystem

### Community 23 - "RoleHandler"
Cohesion: 0.03
Nodes (41): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleData (+33 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "command.rs"
Cohesion: 0.19
Nodes (29): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+21 more)

### Community 26 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 27 - "TechTreeSearchWindow"
Cohesion: 0.16
Nodes (9): GroupSaveData, Vector2, List, Texture2D, TechTreeSearchWindow, IManipulator, ISearchWindowProvider, SearchTreeEntry (+1 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (87): animation_state_id(), animation_state_machine_id(), animation_take_name(), animator_component(), animator_reference_path(), avatar_mask_id(), clip_id(), color_value() (+79 more)

### Community 30 - "Query"
Cohesion: 0.03
Nodes (282): Aabb, AccessibilityFocusVisualQuery, AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer (+274 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (29): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+21 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.08
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEvent"
Cohesion: 0.08
Nodes (9): EventType, Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime (+1 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.07
Nodes (30): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+22 more)

### Community 35 - "PlayerProcessor"
Cohesion: 0.07
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, List (+3 more)

### Community 36 - ".CreateEnumField"
Cohesion: 0.11
Nodes (17): ChildrenSaveData, List, Vector2, NodeSaveData, NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement (+9 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.14
Nodes (3): SeasonProcessorEditor, Container, SeasonProcessor

### Community 39 - "TechnologyGraphLayout"
Cohesion: 0.20
Nodes (15): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap, Default, Result (+7 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.07
Nodes (20): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+12 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.14
Nodes (11): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "simulation.rs"
Cohesion: 0.06
Nodes (36): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+28 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (29): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires() (+21 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 47 - ".RecalculateStats"
Cohesion: 0.17
Nodes (8): StatModifiers, Dictionary, Dictionary, List, Queue, Transform, PlayerRuntimeData, StatType

### Community 48 - "FoliageGenerationSettings"
Cohesion: 0.08
Nodes (24): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+16 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (14): Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities, Capacity (+6 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 52 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "BuildingBase"
Cohesion: 0.09
Nodes (9): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, TargetableBuilding (+1 more)

### Community 57 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 58 - "STSM_StateAction"
Cohesion: 0.13
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "TechnologyGraphViewState"
Cohesion: 0.18
Nodes (25): center_world(), content_bounds(), cubic_bezier(), draw_connection(), draw_grid(), draw_minimap(), fit_bounds(), fit_handles_large_unity_coordinate_ranges() (+17 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (9): bool, Camera, float, int, PlayerInput, Transform, Vector2, CameraController (+1 more)

### Community 63 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.08
Nodes (14): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+6 more)

### Community 67 - ".StartupSequence"
Cohesion: 0.20
Nodes (3): IEnumerator, LoadSceneMode, Scene

### Community 68 - "GameEventProcessor"
Cohesion: 0.07
Nodes (12): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType, EventTester (+4 more)

### Community 69 - "PoolableObject"
Cohesion: 0.04
Nodes (36): CollectResource, Container, ContainerBuilder, GUIDProcessor, AnimationCurve, bool, int, object (+28 more)

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (130): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+122 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "Targetable"
Cohesion: 0.03
Nodes (47): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+39 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 78 - "TechTreeNode"
Cohesion: 0.06
Nodes (23): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Port (+15 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingResourceModelHandler"
Cohesion: 0.17
Nodes (5): BuildingResourceModelHandler, GameObject, BuildingResourceModelHandlerEditor, UnityEvent, StorageStatus

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "Node_SO"
Cohesion: 0.14
Nodes (12): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+4 more)

### Community 83 - "RenderAssets"
Cohesion: 0.03
Nodes (142): BackgroundColor, accessibility_button_enabled(), AccessibilityMotionDefaults, AccessibleButtonNodeQuery, AccessibleButtonScope, actor_material(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu() (+134 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "legacy.rs"
Cohesion: 0.10
Nodes (54): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, checked_in_schema_one_fixture_imports_retained_terrain() (+46 more)

### Community 86 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (357): AccessibilityActionRequest, AnyResult, generate_world(), generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_settings_selection(), AccessibilityActionDispatch (+349 more)

### Community 87 - "stream_town_migrate/src/technology_layout.rs"
Cohesion: 0.28
Nodes (15): GraphPoint, AuthoredGroup, AuthoredNode, build_layout(), checked_in_layout_exactly_matches_the_unity_graph_conversion(), convert(), parse_point(), parse_unity_graph() (+7 more)

### Community 88 - "Option"
Cohesion: 0.04
Nodes (157): ArchetypeDef, ArchetypeKind, ArchetypeScene, HealthDef, RotatingNodeDef, Option, PresentationCatalog, actor_detail_budget() (+149 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "Processors"
Cohesion: 0.07
Nodes (11): InputButton, UserInterface.MainMenu, Processors, World, MetaData, GameEventSystem, Audio, Settings (+3 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "RoleDataSettings"
Cohesion: 0.12
Nodes (14): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+6 more)

### Community 96 - "Resource"
Cohesion: 0.04
Nodes (34): DepositResources, ResourceStorageModifier, float, int, PlayerInventory, Dictionary, ResourceInventory, bool (+26 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.07
Nodes (20): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+12 more)

### Community 98 - ".GetMissingDataScriptableDependencies"
Cohesion: 0.22
Nodes (3): Container, IEnumerable, Type

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (73): AnimationConditionMode, AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe (+65 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "AnimationHandler"
Cohesion: 0.10
Nodes (11): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.13
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

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
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.17
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 115 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 116 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "Goal"
Cohesion: 0.11
Nodes (11): bool, Dictionary, float, int, TechTreeRuntimeData, List, TownGoalRuntimeData, EventType (+3 more)

### Community 119 - ".StartMusic"
Cohesion: 0.49
Nodes (3): SeasonAudioData, AudioClip, List

### Community 120 - "VoteEvent"
Cohesion: 0.11
Nodes (14): int, List, NewKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float (+6 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 126 - ".CalculateDayCount"
Cohesion: 0.40
Nodes (3): float, int, TimeRuntimeData

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 130 - "TransformSaveData"
Cohesion: 0.13
Nodes (12): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+4 more)

### Community 131 - "IProcessor"
Cohesion: 0.05
Nodes (21): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, Container, ContainerBuilder (+13 more)

### Community 133 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 134 - "stream_town_tools/src/main.rs"
Cohesion: 0.09
Nodes (72): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot, checked_in_authoring_assets_pass_headless_validation(), commit_catalog_candidate(), create_technology_group() (+64 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.08
Nodes (17): int, ChangeTimeStamp, Vector2, GroupSaveData, List, TechTreeSaveData_SO, VisualElement, StyleUtility (+9 more)

### Community 137 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (22): STSM_HelperDeposit, float, STSM_Action_DepositResource, STSM_Action_GatherResource, bool, float, GameObject, int (+14 more)

### Community 138 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 139 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 140 - "setup_world_audio_sources"
Cohesion: 0.24
Nodes (10): procedural_ambience_wav(), procedural_ambience_wav_is_valid_deterministic_and_seamless(), procedural_seagull_call_wav(), procedural_seagull_calls_are_valid_distinct_wav_sources(), RoleActionAudioCache, AudioSource, FnMut, setup_world_audio_sources() (+2 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "Value"
Cohesion: 0.20
Nodes (27): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+19 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Character"
Cohesion: 0.06
Nodes (21): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+13 more)

### Community 154 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 155 - "Result"
Cohesion: 0.09
Nodes (61): AnimationFloatKeyframe, AnimationTangent, append_vec3_keys(), convert_chimney_smoke(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id() (+53 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, LabelDisplayProcessor, float, ParticleSystem, VFXArrowPointer

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 163 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 164 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 171 - "tools_ui"
Cohesion: 0.15
Nodes (25): authority_tab(), content_tab(), draw_world_preview(), inspector_tab(), load_game_config(), migration_tab(), preview_grid_point(), preview_lerp_color() (+17 more)

### Community 172 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 173 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 177 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 179 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 185 - "twitch_tab"
Cohesion: 0.17
Nodes (17): format_game_master_ids(), parse_game_master_ids(), poll_twitch_tool_events(), BTreeSet, Duration, Sender, start_twitch_authorization(), start_twitch_clear() (+9 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.03
Nodes (73): ContainerBuilder, WorldGenDebugSettingsInstaller, int, AudioSettings, List, FoliageGenSettings, bool, List (+65 more)

### Community 188 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "Utils"
Cohesion: 0.09
Nodes (4): BuildCostModifier, Utils, Level, Buildings

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "VfxAnimationController"
Cohesion: 0.25
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Current attempts, Do not retry unchanged, Next narrow diagnostic pass, Tree and Foliage Flicker Regression Checklist, What did not fix the flicker, What did work

### Community 200 - "Bevy Migration Status"
Cohesion: 0.22
Nodes (7): Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity, Validation, Original project notes, Stream Town: Bevy Migration

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "Season"
Cohesion: 0.38
Nodes (5): bool, float, int, SeasonRuntimeData, Season

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.17
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.21
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 209 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 213 - ".OnGUI"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

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

### Community 224 - "Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system., Source Nodes

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

### Community 229 - "bevy-port/README.md"
Cohesion: 0.29
Nodes (4): Audio provenance, Binaries, Commands, Stream Town Bevy

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

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "Q: Why are we vendoring Bevy Tidal and not just using the library that exists??"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why are we vendoring Bevy Tidal and not just using the library that exists??, Source Nodes

### Community 238 - "Twitch setup"
Cohesion: 0.25
Nodes (8): 1. Secure the old credentials, 2. Register the Twitch application, 3. Authorize `HumanBeanBot`, 4. Prepare the channel, 5. Bind the Fish God Channel Points reward, 6. Configure OBS, Connection controls and diagnostics, Twitch setup

### Community 240 - "Accessibility"
Cohesion: 0.29
Nodes (6): Accessibility, Automated verification, Keyboard operation, Persisted preferences, Screen-reader contract, Windows Narrator acceptance

### Community 243 - "RuntimeCommandQueues"
Cohesion: 0.38
Nodes (7): AgentCommandQueue, BuildingCommandQueue, BuildingRuntimeCommand, CameraCommandQueue, CameraRequest, VecDeque, RuntimeCommandQueues

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Q: How are shipping visual and audio parity implemented in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How are shipping visual and audio parity implemented in the Bevy migration?, Source Nodes

### Community 248 - "Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?"
Cohesion: 0.50
Nodes (3): Answer, Outcome, Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "String"
Cohesion: 0.03
Nodes (93): AccessibleNode, AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, active_event_text(), add_animation_composition(), add_animation_layer_branch() (+85 more)

### Community 253 - ".Log"
Cohesion: 0.05
Nodes (27): Dictionary, DebugSettings, IMainThreadInitializableProcessor, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory (+19 more)

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

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "UnitTextDisplay"
Cohesion: 0.12
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 268 - ".visit"
Cohesion: 0.60
Nodes (3): ContentError, Result, valid_asset_path()

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 270 - "Q: The Bevy Tidal repo is now public, so fix the integration."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The Bevy Tidal repo is now public, so fix the integration., Source Nodes

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "Editor"
Cohesion: 0.20
Nodes (4): BuildingPlacerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 288 - "PlayerSaveData"
Cohesion: 0.06
Nodes (26): Dictionary, Mesh, Vector3, SaveDataMapper, bool, int, List, string (+18 more)

### Community 289 - "STSM_Action_PlayerBase"
Cohesion: 0.10
Nodes (9): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Heal, STSM_Action_PlayerAttack (+1 more)

### Community 290 - "PlayerSpawnPoint"
Cohesion: 0.07
Nodes (13): Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, Image, TextMeshProUGUI, UIRoleDisplay (+5 more)

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 296 - "PlayerSettings"
Cohesion: 0.06
Nodes (67): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+59 more)

### Community 297 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 299 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 300 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 301 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 302 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 303 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (13): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+5 more)

### Community 307 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 308 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 309 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 312 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "Globals"
Cohesion: 0.33
Nodes (3): RoleScriptablesEditor, Globals, ScriptablesEditor

### Community 315 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

## Knowledge Gaps
- **353 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+348 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **29 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=4.53424798) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=3.237941093) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.962292656) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.477432826) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.366566747)
- `SkinnedMesh` (2× useful, score=1.997632118)
- `drive_tidal_music()` (2× useful, score=1.966606185)
- `WorldSimulation` (2× useful, score=1.71789778) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `UserInterface`, `UpdateGraphBounds`, `StringUtils`, `MonoBehaviour`, `TechTree.Elements`, `BuildingPlacer`, `UnitTextDisplay`, `Units`, `Target`, `Character`, `PlayerSpawnPoint`, `GenerationSettings`, `ScriptablesProcessorInfrastructure`, `SimpleScreenShot`, `RandomEnabler`, `SimpleDisableAfterTime`, `Targetable`, `Easings`, `Processors`, `FPSDisplay`, `World.Generation`, `DayAndNightProcessor`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `SelectedPlayer`, `BuildingProcessor`, `IProcessor`, `MonoBehaviour`, `TwitchChatProcessor`, `BottomBarInterface`, `BuildingPlacer`, `HealthHandler`, `RoleHandler`, `Character`, `WorldGenProcessor`, `UserInterface_Debug`, `GameEvent`, `TechTreeProcessor`, `.RecalculateStats`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `GameEventProcessor`, `UserInterface_TownVote`, `RaidEvent`, `Resource`, `SaveProcessor`, `TownGoalProcessor`, `DayAndNightProcessor`, `VoteEvent`, `.Log`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `IProcessor`, `MonoBehaviour`, `TwitchChatProcessor`, `CellSpacePartitioning`, `Target`, `UserInterface_Debug`, `PlayerProcessor`, `Player`, `TwitchClientProcessor`, `ScriptableObject`, `ProjectCamera`, `.StartupSequence`, `PoolableObject`, `WorldGenRuntimeData`, `FoliageProcessor`, `RaidEvent`, `SaveProcessor`, `ResourceProcessor`, `CampGenerationSettings`, `DayAndNightProcessor`, `AIPath`, `.Log`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _353 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `UserInterface` be split into smaller, more focused modules?**
  _Cohesion score 0.07422402159244265 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05704365079365079 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0577324973876698 - nodes in this community are weakly interconnected._