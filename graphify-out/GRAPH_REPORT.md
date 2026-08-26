# Graph Report - Stream-Town-Bevy  (2026-08-26)

## Corpus Check
- 670 files · ~1,761,369 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8777 nodes · 25416 edges · 323 communities (298 shown, 25 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1034 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2d38357c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- TownGoal.Data
- BuildingProcessor
- world.rs
- WorldGenSaveData
- ScriptablesProcessorInfrastructure
- GlobalAudioController
- TwitchChatProcessor
- Res
- BottomBarInterface
- .new
- SettingsProcessor
- Option
- Mesh
- UnityAsset
- TechTreeIOUtility
- HealthHandler
- Sensors
- save.rs
- ContentCatalog
- StableId
- twitch.rs
- UIElementWrapper
- STSM_Idle_Player
- Vec3
- AnimationHandler
- command.rs
- BinarySaveCodec
- WorldSimulation
- SaveFileData
- .Update
- Query
- WorldGenProcessor
- UserInterface_Debug
- BTreeMap
- GenerationSettings
- BuildingBase
- stream_town_domain/src/content.rs
- SettingsData
- SeasonProcessor
- TechnologyGraphViewState
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- PlayerProcessor
- AnimationControllerRuntime
- runtime_console.rs
- UserInterface_RulerVote
- RoleHandler
- ResourceDataSaveData
- encode_broadcast_session
- IRuntimeDataScriptable
- StreamTownSessionBridge
- Pet
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- legacy.rs
- simulation.rs
- World.Generation.Settings
- TechTreeEditorWindow
- Result
- SelectableObject
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Targetable
- GameEventProcessor
- .GetMissingDataScriptableDependencies
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- advance_world_loading_cover
- UserInterface_ObjectSelection
- .Log
- UserInterface_TownVote
- TechTreeNode
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Node_SO
- RenderAssets
- Access_Text
- component_field_value
- stream_town_game/src/lib.rs
- Goal
- TargetProcessor
- FoliageProcessor
- tools_ui
- RaidEvent
- config.rs
- MonoBehaviour
- convert_fbx_to_glb.py
- STSM_Action_PlayerBase
- Resource
- SaveProcessor
- EnemyModelHandler
- PresentationCatalog
- .Draw
- StateMachine
- SensorProcessor
- TownGoalProcessor
- MainMenuManager
- ResourceProcessor
- LoadingManager
- Station
- TargetableHealth
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- VfxSeagullSpawner
- Processors
- stream_town_tools/src/main.rs
- GridSystem.Partitioning
- stream_town_migrate/src/main.rs
- GameEvent
- LabelDisplayProcessor
- VoteEvent
- unity_color_filter
- PoolableObject
- SnapToGridMouseMovement
- AIPath
- stream_town_migrate/src/presentation.rs
- GateController
- direct_broadcast.rs
- .EnsureValidCredentials
- STSM_StateAction
- DirectBroadcastRuntime
- StringUtils
- EnemySpawner
- UnitTextDisplay
- ErrorData
- Coordinator
- TerrainGenSettings
- spawn_main_menu
- UnitHealthBar
- Easings
- What You Must Do When Invoked
- RuntimeData Template
- String
- RuntimeData Template
- Key Rules
- TimeProcessor
- CommonEnums.cs
- xtask/src/lib.rs
- CellSpacePartitioning
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- UserInterface
- convert_fish_schools
- Result
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- save_secrets_fields
- Result
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- WorldInstanceDeterminism
- DebugProcessor
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- SimpleMusicController
- PlayerSaveData
- Q: There are still no animations.
- xtask/src/main.rs
- List
- MiscCommands
- RandomEnabler
- String
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- BuildingSettings
- SelectedBuilding
- GridProcessor
- Access_Toggle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ResourceRuntimeData
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SimpleScreenShot
- UserInterface_TownGoal
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- BevyMigrationExporter
- CommandDictionary
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- World.Generation
- UserInterface_GameMenu
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- WorldGenRuntimeData
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- EditorUtils
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- .InitializeAndActivateProcessorsAsync
- DebugSettings
- Requirement
- TL_Secrets
- IProcessor.cs
- TwitchUser
- Key Rules
- ResourceHolder
- RuntimeData Template
- Character Animation Regression Checklist
- SelectedObject
- ScriptKeywordProcessor
- FPSDisplay
- NodeSaveData
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
- BuildPlacerData
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- GameStateProcessor
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- BuildingDamageMaterialHandler
- extraction-spec.md
- TechNodeData
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- PlayerRole
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- Editor
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- DontDestroyOnLoad
- Q: If there is more to do, keep going.
- SelectedEnemy
- BroadcastConfig
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- VFXArrowPointer
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- SelectedResource
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxParticlePosition
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- .default
- TL_API
- StreamTown.Migration
- TransformSaveData
- .new
- ObjectiveSaveData
- PlayerControls
- CreditsProcessor
- record_gpu_readiness
- CreateProjectScopeProcessors.cs
- SimpleDisableAfterTime
- PlayerSettings
- DirectTwitchBroadcastPlugin
- IntWrapper
- .InjectRuntimeData
- EquipmentHandlerEditor
- ObjectPoolingRuntimeData
- TechTreeSearchWindow
- HealthModifier
- ObjectSelectionProcessor.Editor.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- RotationHandler
- SelectedEnemyCamp
- UnityGraphics
- .StartupSequence
- Utils
- .InjectRuntimeData
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- setup_camera
- BroadcastTarget
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- PlayerRoleSaveData
- FoliageGenerationSettings
- vcpkg.json
- .GetCompatiblePorts
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 354 edges
2. `WorldSimulation` - 163 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `ContentCatalog` - 148 edges
7. `Player` - 142 edges
8. `RenderAssets` - 130 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `handle_twitch_event()` --calls--> `unity_command_usage()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_assignment_penalty_spreads_farmers_across_farms()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `battering_ram_targets_and_damages_buildings_from_authored_mask()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (323 total, 25 thin omitted)

### Community 0 - "TownGoal.Data"
Cohesion: 0.06
Nodes (18): InputButton, SharedTypes, int, ChangeTimeStamp, VisualElement, StyleUtility, DataStructures, TownGoal.Enumerations (+10 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "world.rs"
Cohesion: 0.05
Nodes (74): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+66 more)

### Community 3 - "WorldGenSaveData"
Cohesion: 0.09
Nodes (15): Mesh, Vector3, bool, int, MeshSaveData, List, SaveGameData, float (+7 more)

### Community 4 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (4): Reflex.Core, Data.Containers, MetaData, ScriptablesProcessorInfrastructure

### Community 5 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "Res"
Cohesion: 0.03
Nodes (205): AccessibilityFocusVisualQuery, AccumulatedMouseScroll, AnimationTransitions, AppExit, DirectBroadcastControl, accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus() (+197 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - ".new"
Cohesion: 0.03
Nodes (123): AccessibilityActionRequest, generate_world(), generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), action_ranges_and_tower_acquisition_are_euclidean(), agent_facing_matches_unity_rotation_and_action_targets() (+115 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.06
Nodes (12): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+4 more)

### Community 11 - "Option"
Cohesion: 0.03
Nodes (126): AmbientLight, Assets, ArchetypeDef, ArchetypeKind, ArchetypeScene, HealthDef, RotatingNodeDef, Option (+118 more)

### Community 12 - "Mesh"
Cohesion: 0.07
Nodes (28): AssetId, authored_rotating_node_names(), building_model_node_names(), enemy_model_node_names(), GpuReadinessExpected, GpuReadinessProbe, GpuReadinessShared, GpuReadinessSnapshot (+20 more)

### Community 13 - "UnityAsset"
Cohesion: 0.19
Nodes (43): aged_buildings(), archetype_kind(), building_cost_reductions(), building_level_caps(), convert_export(), field_value(), foliage_layers(), generated_record_ids() (+35 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (13): PlayerDeathHandler, bool, float, Vector3, Action, float, Enemy, Action (+5 more)

### Community 16 - "Sensors"
Cohesion: 0.09
Nodes (7): STStateMachine.States, Behaviours, Animation, Sensors, STStateMachine, Pathfinding, STStateMachine.Helpers

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "ContentCatalog"
Cohesion: 0.03
Nodes (224): GameConfig, BuildingDef, BuildingModelDef, ContentCatalog, StationDef, StorageModelDef, TargetingScoreDef, GridPos (+216 more)

### Community 19 - "StableId"
Cohesion: 0.07
Nodes (31): ObjectiveDef, round_trips_through_serde(), Err, Formatter, FromStr, Into, Result, Self (+23 more)

### Community 20 - "twitch.rs"
Cohesion: 0.07
Nodes (51): SecretsAuthorizationEvent, bot_and_broadcaster_tokens_use_distinct_vault_entries(), broadcaster_oauth_uses_only_the_stream_key_scope(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg() (+43 more)

### Community 21 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 22 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (22): STSM_HelperDeposit, float, STSM_Action_DepositResource, STSM_Action_GatherResource, bool, float, GameObject, int (+14 more)

### Community 23 - "Vec3"
Cohesion: 0.11
Nodes (28): actor_combat_visual(), animate_chimney_smoke_particles(), BuildingEffectKind, BuildingEffectParticle, chimney_alpha_step(), chimney_emission_and_world_transform_are_deterministic(), chimney_emitter_world_position(), chimney_particle_scale() (+20 more)

### Community 24 - "AnimationHandler"
Cohesion: 0.11
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 25 - "command.rs"
Cohesion: 0.19
Nodes (29): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+21 more)

### Community 26 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 27 - "WorldSimulation"
Cohesion: 0.12
Nodes (14): capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), BTreeMap, Option, Result, VecDeque, SimulationError, technology_vote_starts_persistent_goal_and_unlocks_after_all_objectives() (+6 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - ".Update"
Cohesion: 0.17
Nodes (16): List, Material, materials, Mesh, meshes, Dictionary, int, List (+8 more)

### Community 30 - "Query"
Cohesion: 0.04
Nodes (154): Aabb, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AudioSink, ActivePetVisual, ActorAnimationDriver (+146 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (23): HashSet, Action, bool, BoxCollider, Container, Func, GameObject, HashSet (+15 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 33 - "BTreeMap"
Cohesion: 0.10
Nodes (46): assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), convert(), convert_avatar_masks(), convert_controllers(), convert_materials(), convert_model_materials() (+38 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - "BuildingBase"
Cohesion: 0.07
Nodes (14): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+6 more)

### Community 36 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (33): ArchetypeBounds, AuthoredRecord, AuthoredValue, ContentError, EnemyDef, EnemyModelSetDef, EnemyRunAnimation, EnemySpawnerDef (+25 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.06
Nodes (26): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+18 more)

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.07
Nodes (56): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+48 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.14
Nodes (11): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "PlayerProcessor"
Cohesion: 0.09
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.11
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+18 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 47 - "RoleHandler"
Cohesion: 0.04
Nodes (23): PlayerInventory, Dictionary, PlayerRoleData, AudioClip, bool, float, int, ResourceInventory (+15 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "encode_broadcast_session"
Cohesion: 0.22
Nodes (12): AtomicBool, BroadcastController, BroadcastMetrics, BroadcastMetricsSnapshot, encode_broadcast_session(), AtomicU64, Drop, Receiver (+4 more)

### Community 50 - "IRuntimeDataScriptable"
Cohesion: 0.06
Nodes (26): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+18 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.15
Nodes (42): ActorCustomization, StreamUserType, binary_fixture(), BinaryParser, clamped_cell(), decode_json(), json_active_goal(), json_buildings() (+34 more)

### Community 57 - "simulation.rs"
Cohesion: 0.07
Nodes (28): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+20 more)

### Community 58 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 61 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "TargetSensor"
Cohesion: 0.08
Nodes (12): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+4 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (15): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+7 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.08
Nodes (15): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+7 more)

### Community 67 - "Targetable"
Cohesion: 0.07
Nodes (18): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+10 more)

### Community 68 - "GameEventProcessor"
Cohesion: 0.06
Nodes (14): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType, EventTester (+6 more)

### Community 69 - ".GetMissingDataScriptableDependencies"
Cohesion: 0.22
Nodes (3): Container, IEnumerable, Type

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.09
Nodes (46): PassiveResourceContribution, asset(), authored_value(), building_model_definitions(), building_node_age(), component(), component_at(), component_reference_name() (+38 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "advance_world_loading_cover"
Cohesion: 0.14
Nodes (28): AccessibilityNode, advance_world_loading_cover(), Hud, hud_play_time(), hud_season_meter_percent(), HudMetric, loaded_asset_counts(), loading_cover_ready() (+20 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - ".Log"
Cohesion: 0.06
Nodes (24): HideInCallstack, Object, DebugLogCategory, Action, bool, BoxCollider, CancellationToken, Container (+16 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 78 - "TechTreeNode"
Cohesion: 0.06
Nodes (21): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+13 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "RenderAssets"
Cohesion: 0.03
Nodes (127): BackgroundColor, accessibility_button_enabled(), AccessibilityMotionDefaults, AccessibleButtonNodeQuery, actor_material(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image() (+119 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "component_field_value"
Cohesion: 0.21
Nodes (25): ArchetypesById, archetype_bounds(), building_placements(), BuildingPlacement, component_field_value(), component_type(), convert_archetypes(), disable_after_milliseconds() (+17 more)

### Community 86 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (334): AccessibleNode, AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, accessibility_settings_selection(), AccessibilityActionDispatch, AccessibilityCandidate (+326 more)

### Community 87 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 88 - "TargetProcessor"
Cohesion: 0.15
Nodes (9): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor, Dictionary, List (+1 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "tools_ui"
Cohesion: 0.17
Nodes (23): authority_tab(), content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), preview_grid_point(), preview_lerp_color(), role_i32() (+15 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - "config.rs"
Cohesion: 0.10
Nodes (28): ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet (+20 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (111): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+103 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "STSM_Action_PlayerBase"
Cohesion: 0.13
Nodes (5): AttackUnit, STSM_Action_Build, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 96 - "Resource"
Cohesion: 0.04
Nodes (36): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer (+28 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (19): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+11 more)

### Community 98 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (9): bool, int, List, EnemyModelHandler, bool, float, Vector3, STSM_Action_EnemyAttack (+1 more)

### Community 99 - "PresentationCatalog"
Cohesion: 0.05
Nodes (91): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef (+83 more)

### Community 100 - ".Draw"
Cohesion: 0.13
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "SensorProcessor"
Cohesion: 0.31
Nodes (3): Container, ContainerBuilder, SensorProcessor

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.13
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.10
Nodes (18): Container, Dictionary, float, int, materialIndex, Matrix4x4, meshIndex, Resource (+10 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "Station"
Cohesion: 0.05
Nodes (24): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+16 more)

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
Cohesion: 0.09
Nodes (13): Container, ContainerBuilder, DayAndNightProcessor, float, Material, Volume, DayAndNightSettings, ContainerBuilder (+5 more)

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "Processors"
Cohesion: 0.08
Nodes (15): UserInterface.MainMenu, Processors, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Character.Enumerations, Core, World (+7 more)

### Community 115 - "stream_town_tools/src/main.rs"
Cohesion: 0.09
Nodes (74): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot, commit_catalog_candidate(), create_technology_group(), create_technology_node() (+66 more)

### Community 116 - "GridSystem.Partitioning"
Cohesion: 0.09
Nodes (14): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+6 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "GameEvent"
Cohesion: 0.09
Nodes (8): EventType, Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 119 - "LabelDisplayProcessor"
Cohesion: 0.32
Nodes (3): Container, ContainerBuilder, LabelDisplayProcessor

### Community 120 - "VoteEvent"
Cohesion: 0.12
Nodes (12): int, List, NewKingVote, PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent (+4 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "PoolableObject"
Cohesion: 0.07
Nodes (12): Container, ContainerBuilder, GUIDProcessor, Component, Transform, bool, Dictionary, GUIDRuntimeData (+4 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 126 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.09
Nodes (55): animation_take_name(), animator_component(), animator_reference_path(), array_index(), clip_id(), color_value(), convert_clips(), convert_embedded_model_clips() (+47 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.17
Nodes (15): AudioFrame, black_rgba_frame(), capture_direct_broadcast_frame(), ingest(), ingest_selection_prefers_default_or_named_region(), MediaInput, Commands, Image (+7 more)

### Community 129 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 130 - "STSM_StateAction"
Cohesion: 0.13
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 131 - "DirectBroadcastRuntime"
Cohesion: 0.18
Nodes (16): begin_direct_broadcast(), configure_direct_broadcast(), DirectBroadcastRuntime, poll_direct_broadcast_authorization(), poll_direct_broadcast_worker(), restart_direct_broadcast(), Arc, Default (+8 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 135 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 136 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 137 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 138 - "spawn_main_menu"
Cohesion: 0.16
Nodes (18): animate_loading_icon(), apply_authored_main_menu_camera(), apply_loading_icon_rotation(), authored_scene_rotation(), embedded_main_menu_scene(), LoadingIconSpinner, main_menu_baked_decoration_indices(), main_menu_cloud_prism_transform() (+10 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "String"
Cohesion: 0.17
Nodes (23): ActorKind, actor_prefix(), content_id(), convert(), duration_days(), entity_id(), ImportReport, legacy_objective_matches() (+15 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TimeProcessor"
Cohesion: 0.21
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 147 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (24): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+16 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.28
Nodes (14): add_file(), add_tree(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result (+6 more)

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

### Community 153 - "UserInterface"
Cohesion: 0.08
Nodes (10): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Pets, UserInterface, Combat, SavingAndLoading.SavableObjects (+2 more)

### Community 154 - "convert_fish_schools"
Cohesion: 0.15
Nodes (18): convert_chimney_smoke(), convert_fish_schools(), hierarchy_age(), material_id(), parse_avatar_mask(), parse_yaml_documents(), parses_authored_chimney_particle_sections(), parses_authored_fireworks_graph_parameters() (+10 more)

### Community 155 - "Result"
Cohesion: 0.09
Nodes (67): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), controller_id(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id() (+59 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.13
Nodes (12): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, VisualElement, Button, EnumField, UnlockVisualElement (+4 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "save_secrets_fields"
Cohesion: 0.19
Nodes (16): AnyResult, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), open_twitch_verification_uri(), open_twitch_verification_uri_with(), player_settings_path(), FnOnce (+8 more)

### Community 159 - "Result"
Cohesion: 0.20
Nodes (13): bandwidth_test_url_is_constructed_without_logging_the_key(), BroadcastEncoder, BroadcastPrerequisites, build_ingest_url(), inspect_broadcast_prerequisites(), linked_ffmpeg_encodes_h264_aac_flv_without_a_subprocess(), resolve_broadcast_target(), Formatter (+5 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 163 - "DebugProcessor"
Cohesion: 0.05
Nodes (20): Container, IMainThreadInitializableProcessor, IProcessor, Container, ContainerBuilder, DebugProcessor, Action, Container (+12 more)

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "PlayerSaveData"
Cohesion: 0.10
Nodes (16): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int (+8 more)

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 171 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 172 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 173 - "String"
Cohesion: 0.20
Nodes (16): animation_parameter_name(), archetype_scenes(), authored_mask(), child_technology_guids(), collect_model_dependencies(), decomposes_combined_unity_flag_values(), glb_asset_path(), mask_ids() (+8 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 178 - "GridProcessor"
Cohesion: 0.13
Nodes (9): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, Container, ContainerBuilder (+1 more)

### Community 179 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 185 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.03
Nodes (67): ContainerBuilder, AllBuildingDataSettingsInstaller, int, AudioSettings, List, CampGenSettings, List, FoliageGenSettings (+59 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.25
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

### Community 203 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

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

### Community 209 - "DebugSettings"
Cohesion: 0.36
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 212 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 213 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "NodeSaveData"
Cohesion: 0.18
Nodes (8): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO

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
Cohesion: 0.22
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

### Community 234 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

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
Cohesion: 0.20
Nodes (10): 1. Secure the old credentials, 2. Register the Twitch application, 3. Configure and authorize `HumanBeanBot`, 4. Prepare the channel, 5. Bind the Fish God Channel Points reward, 6. Authorize direct broadcasting, 7. Choose broadcast quality and test bandwidth, 8. Go live without OBS (+2 more)

### Community 240 - "Accessibility"
Cohesion: 0.29
Nodes (6): Accessibility, Automated verification, Keyboard operation, Persisted preferences, Screen-reader contract, Windows Narrator acceptance

### Community 243 - "GameStateProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GameStateProcessor

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

### Community 251 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 253 - "TechNodeData"
Cohesion: 0.12
Nodes (11): List, Node_SO, TechNodeData, IEnumerable, Button, GameObject, Image, Slider (+3 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "PlayerRole"
Cohesion: 0.04
Nodes (26): RoleSlotModifier, int, RoleSlot, bool, int, Dictionary, int, RoleDataContainer (+18 more)

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "Editor"
Cohesion: 0.04
Nodes (17): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, WindControllerEditor (+9 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 268 - "BroadcastConfig"
Cohesion: 0.26
Nodes (10): BroadcastConfig, BroadcastEncoderPreference, capture_process_audio(), encoder_candidates(), open_audio_encoder(), open_video_encoder(), Self, broadcast_encoder_label() (+2 more)

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

### Community 273 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

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

### Community 280 - "VfxParticlePosition"
Cohesion: 0.22
Nodes (4): Transform, VisualEffect, VfxParticlePosition, VFX

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - ".default"
Cohesion: 0.17
Nodes (20): checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_game_config(), load_technology_layout() (+12 more)

### Community 288 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 289 - ".new"
Cohesion: 0.15
Nodes (18): absolute_path(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), conversion_preserves_mesh_and_relocates_invalid_positions(), conversion_rejects_malformed_retained_mesh(), decode_binary(), decode_legacy() (+10 more)

### Community 290 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 291 - "PlayerControls"
Cohesion: 0.25
Nodes (3): InputButton, Settings, PlayerControls

### Community 292 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 293 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "SimpleDisableAfterTime"
Cohesion: 0.05
Nodes (17): Transform, PlayerSpawnPoint, Image, TextMeshProUGUI, UIRoleDisplay, float, GameObject, SimpleDisableAfterTime (+9 more)

### Community 296 - "PlayerSettings"
Cohesion: 0.06
Nodes (66): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+58 more)

### Community 297 - "DirectTwitchBroadcastPlugin"
Cohesion: 0.50
Nodes (3): DirectTwitchBroadcastPlugin, App, Plugin

### Community 298 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 300 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 301 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 302 - "TechTreeSearchWindow"
Cohesion: 0.32
Nodes (6): List, Texture2D, TechTreeSearchWindow, ISearchWindowProvider, SearchTreeEntry, SearchWindowContext

### Community 303 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (14): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+6 more)

### Community 307 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 309 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 310 - ".StartupSequence"
Cohesion: 0.20
Nodes (3): IEnumerator, LoadSceneMode, Scene

### Community 311 - "Utils"
Cohesion: 0.05
Nodes (9): BuildCostModifier, RoleScriptablesEditor, Utils, Level, ScriptablesEditor, Buildings, SavingAndLoading, SavingAndLoading.Structs (+1 more)

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 315 - "BroadcastTarget"
Cohesion: 0.38
Nodes (6): AuthorizationEvent, BroadcastTarget, DirectBroadcastPhase, DirectBroadcastSnapshot, Debug, String

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 318 - "PlayerRoleSaveData"
Cohesion: 0.40
Nodes (3): List, int, PlayerRoleSaveData

### Community 319 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 321 - ".GetCompatiblePorts"
Cohesion: 0.50
Nodes (3): List, Port, NodeAdapter

## Knowledge Gaps
- **362 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+357 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `TownGoal.Data`, `ScriptablesProcessorInfrastructure`, `EnemySpawner`, `Editor`, `UnitTextDisplay`, `StringUtils`, `Easings`, `Sensors`, `CommonEnums.cs`, `UserInterface`, `GenerationSettings`, `UpdateGraphBounds`, `SimpleDisableAfterTime`, `RandomEnabler`, `SimpleScreenShot`, `World.Generation`, `BuildingPlacer`, `FPSDisplay`, `MonoBehaviour`, `Processors`, `GridSystem.Partitioning`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `PlayerRole`, `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `HealthHandler`, `TimeProcessor`, `UserInterface`, `WorldGenProcessor`, `UserInterface_Debug`, `DebugProcessor`, `TechTreeProcessor`, `RoleHandler`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `GameEventProcessor`, `.Log`, `UserInterface_TownVote`, `BuildingPlacer`, `RaidEvent`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEvent`, `VoteEvent`, `PoolableObject`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `world.rs`, `Res`, `.new`, `Option`, `UnityAsset`, `String`, `save.rs`, `ContentCatalog`, `twitch.rs`, `Vec3`, `command.rs`, `convert_fish_schools`, `WorldSimulation`, `Result`, `Query`, `BTreeMap`, `stream_town_domain/src/content.rs`, `TechnologyGraphViewState`, `AnimationControllerRuntime`, `runtime_console.rs`, `String`, `simulation.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `RenderAssets`, `component_field_value`, `stream_town_game/src/lib.rs`, `tools_ui`, `config.rs`, `PresentationCatalog`, `stream_town_tools/src/main.rs`, `stream_town_migrate/src/presentation.rs`?**
  _High betweenness centrality (0.030) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _362 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TownGoal.Data` be split into smaller, more focused modules?**
  _Cohesion score 0.05959183673469388 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07184325108853411 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05375139977603583 - nodes in this community are weakly interconnected._