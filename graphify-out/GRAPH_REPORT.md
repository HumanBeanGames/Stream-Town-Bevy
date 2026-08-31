# Graph Report - Stream-Town-Bevy  (2026-09-01)

## Corpus Check
- 670 files · ~1,812,244 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9337 nodes · 27978 edges · 322 communities (291 shown, 31 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1056 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `422a068a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- BuildingBase
- BuildingProcessor
- String
- SeasonProcessor
- Res
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- IRuntimeDataScriptable
- BottomBarInterface
- generate_world
- SettingsProcessor
- UserInterface_Debug
- config.rs
- WorldGenSaveData
- TechTreeIOUtility
- HealthHandler
- advance_world_loading_cover
- save.rs
- Vec
- command.rs
- twitch.rs
- NavGrid
- .Log
- Option
- Result
- PresentationCatalog
- Node_SO
- StableId
- SaveFileData
- BinaryReader
- RuntimeConfig
- WorldGenProcessor
- CellSpacePartitioning
- String
- MeshData
- .SetTargetType
- World.Generation.Settings
- SettingsData
- RenderAssets
- BinarySaveCodec
- DebugProcessor
- TechTreeProcessor
- PlayerRole
- .on_frame_arrived
- AnimationControllerDef
- MainMenuManager
- String
- ContentCatalog
- ResourceDataSaveData
- stream_town_migrate/src/main.rs
- AudioHandler
- StreamTownSessionBridge
- GlobalAudioController
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- legacy.rs
- Station
- MonoBehaviour
- TechTreeEditorWindow
- String
- stream_town_domain/src/content.rs
- CameraController
- AnimationHandler
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- GameEventProcessor
- GamestateJukebox
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- EnemyModelHandler
- GameStateProcessor
- SelectedObject
- ObjectPoolingProcessor
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- UIElementWrapper
- Access_Text
- TwitchUser
- drive_tidal_music
- .new
- settings.rs
- FoliageProcessor
- SnapToGridMouseMovement
- RaidEvent
- world.rs
- Ui
- convert_fbx_to_glb.py
- .EnsureValidCredentials
- Resource
- SaveProcessor
- GridNode
- stream_town_domain/src/presentation.rs
- twitch_tab
- StateMachine
- BuildingModelHandler
- TownGoalProcessor
- .Update
- ResourceProcessor
- LoadingManager
- Goal
- LabelDisplayProcessor
- CustomLogHandler
- LevelHandler
- ResourceHolder
- TargetSensor
- List
- ScriptablesEditor
- PlayerRoleData
- stream_town_tools/src/main.rs
- tools_ui
- Globals
- TechTreeNode
- VoteEvent
- ResourceRuntimeData
- RoleProcessor
- IProcessor.cs
- AIPath
- Targetable
- GateController
- direct_broadcast.rs
- WindController
- Coordinator
- Option
- TL_Secrets
- DayAndNightProcessor
- STSM_Idle
- TechTree.Elements
- Editor
- StationSensor
- sync_stream_only_capture
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- UserInterface_TownGoal
- RuntimeData Template
- Key Rules
- roles_tab
- DontDestroyOnLoad
- xtask/src/lib.rs
- Season
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- World.Generation
- .UserIsSubscribed
- Option
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- StringUtils
- .ValidateTokenAsync
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_ObjectSelection
- EnemyCampSaveData
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- SimpleMusicController
- Q: There are still no animations.
- xtask/src/main.rs
- EnemyWeaponModel
- Target
- NodeUnlockData
- CommandDictionary
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- Access_Toggle
- Processors
- ResourceTarget
- stream_town_migrate/src/presentation.rs
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- VfxAnimationController
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- UnitTextDisplay
- .Draw
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- SelectedResource
- FoliageGenerationSettings
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SimpleDisableAfterTime
- UserInterface
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- BevyMigrationExporter
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- BuildingDamageMaterialHandler
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- RoleSlot
- .GetAvailableContainers
- publish_stream_only_frame
- DirectBroadcastControl
- WorldInstanceDeterminism
- HealthModifier
- Key Rules
- .InitializeAndActivateProcessorsAsync
- RuntimeData Template
- Character Animation Regression Checklist
- RotationHandler
- ScriptKeywordProcessor
- FPSDisplay
- TechTreeGraphView
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
- IProcessor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- RandomEnabler
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- record_gpu_readiness
- extraction-spec.md
- SelectedBuilding
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- RoleHandler
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- .RefreshSceneBindingsAndTryGenerate
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- STSM_HelperBase
- Q: If there is more to do, keep going.
- UI_TechOption
- DebugSettings
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- UnityGraphics
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- import_save
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxSeagullSpawner
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch commands
- Utils
- Character
- SimpleScreenShot
- CampGenerationSettings
- Requirement
- ResourceGenerationSettings
- TL_API
- CreateProjectScopeProcessors.cs
- String
- SelectedEnemy
- DirectBroadcastRuntime
- BuildPlacerData
- ToolState
- ScriptablesProcessorInfrastructure
- SimpleRotateOnAxis
- TwitchClientRuntimeData
- UnitTravelToPosition
- InventorySaveData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- ObjectiveSaveData
- .AddGoalFollowed
- SimpleEventOnStart
- StreamTown.Migration
- Autosave
- PlayerRoleSaveData
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- FoliageSaveData
- main
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- preview_grid_point
- GameplayConfig
- vcpkg.json
- .ExportModification
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 415 edges
2. `ContentCatalog` - 178 edges
3. `WorldSimulation` - 174 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `ToolState` - 138 edges
9. `RenderAssets` - 134 edges
10. `WorldGenProcessor` - 114 edges

## Surprising Connections (you probably didn't know these)
- `generate_world()` --calls--> `default_resource_generation_layers()`  [INFERRED]
  bevy-port/crates/stream_town_domain/src/world.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_enemies_drive_damage_range_cadence_and_weighted_spawning()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (322 total, 31 thin omitted)

### Community 0 - "BuildingBase"
Cohesion: 0.07
Nodes (22): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Action (+14 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.05
Nodes (24): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+16 more)

### Community 2 - "String"
Cohesion: 0.10
Nodes (52): MaterialDef, PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks() (+44 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 4 - "Res"
Cohesion: 0.03
Nodes (243): Added, AnimationTransitions, AudioSink, ActorAnimationDriver, ActorHealthOverlay, ActorNameOverlay, Agent, agent_is_moving() (+235 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (320): generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText (+312 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (27): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+19 more)

### Community 7 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (29): Container, ContainerBuilder, TargetProcessor, Container, ContainerBuilder, TimeProcessor, Container, ContainerBuilder (+21 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "generate_world"
Cohesion: 0.12
Nodes (38): generate_world(), action_cooldown(), ActionPresentation, actor_movement_speed(), AgentGoal, authored_assignment_penalty_spreads_farmers_across_farms(), battering_ram_targets_and_damages_buildings_from_authored_mask(), builder_completes_and_upgrades_authored_construction() (+30 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.11
Nodes (27): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Default (+19 more)

### Community 13 - "WorldGenSaveData"
Cohesion: 0.10
Nodes (17): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+9 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.09
Nodes (12): Func, Action, float, Enemy, STSM_Action_Heal, Action, bool, float (+4 more)

### Community 16 - "advance_world_loading_cover"
Cohesion: 0.04
Nodes (74): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), authored_rotating_node_names(), begin_world_loading_cover() (+66 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "Vec"
Cohesion: 0.03
Nodes (111): AccessibleNode, AnimationClip, AnimationTargetId, ActivePetVisual, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+103 more)

### Community 19 - "command.rs"
Cohesion: 0.06
Nodes (55): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+47 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (67): BTreeSet, TwitchConfig, secrets_restart_requirements(), SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion() (+59 more)

### Community 21 - "NavGrid"
Cohesion: 0.11
Nodes (24): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+16 more)

### Community 22 - ".Log"
Cohesion: 0.06
Nodes (15): AttackUnit, HideInCallstack, Object, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack (+7 more)

### Community 23 - "Option"
Cohesion: 0.03
Nodes (129): AmbientLight, AnimatedBy, AnimationGraphHandle, ActiveMaterialHandles, actor_detail_budget(), animate_main_menu_clouds(), animation_detail_budget(), animation_root_name() (+121 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - "PresentationCatalog"
Cohesion: 0.04
Nodes (143): ArchetypeScene, PresentationCatalog, actor_combat_visual(), actor_material(), actor_scene_budget(), advance_falling_fish(), AgentCommand, AgentCommandQueue (+135 more)

### Community 26 - "Node_SO"
Cohesion: 0.15
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 27 - "StableId"
Cohesion: 0.05
Nodes (77): ObjectiveDef, ObjectiveKind, Display, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState (+69 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 30 - "RuntimeConfig"
Cohesion: 0.03
Nodes (165): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AnyResult, PlayerSettings, Default, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active() (+157 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (31): Action, HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func (+23 more)

### Community 32 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 33 - "String"
Cohesion: 0.04
Nodes (77): accessibility_settings_selection(), AccessibilityAnnouncement, AccessibilityRuntime, active_event_text(), announce_accessibility_state(), announce_technology_vote(), building_definition_id(), CommandFeedback (+69 more)

### Community 34 - "MeshData"
Cohesion: 0.11
Nodes (19): Action, IEnumerator, Vector2, Noise, List, Mesh, Vector2, Vector3 (+11 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "RenderAssets"
Cohesion: 0.04
Nodes (100): AccessibilityMotionDefaults, apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform, building_effect_material() (+92 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 40 - "DebugProcessor"
Cohesion: 0.05
Nodes (26): IMainThreadInitializableProcessor, Container, ContainerBuilder, DebugProcessor, Camera, Container, ContainerBuilder, InputButton (+18 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "PlayerRole"
Cohesion: 0.08
Nodes (17): SimpleToggleCarry, RoleSlotModifier, int, CharacterModelHandler, bool, int, List, Transform (+9 more)

### Community 43 - ".on_frame_arrived"
Cohesion: 0.24
Nodes (6): duration_as_micros(), Duration, Error, twitch_live_request_timeout(), CapturedWindowFrame, InternalCaptureControl

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (33): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+25 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "String"
Cohesion: 0.12
Nodes (31): actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), decode_binary(), decode_legacy() (+23 more)

### Community 47 - "ContentCatalog"
Cohesion: 0.07
Nodes (102): GameConfig, ContentCatalog, GridPos, ActorState, String, GeneratedWorld, action_animation_speed(), active_station_ids() (+94 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.24
Nodes (10): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+2 more)

### Community 49 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (14): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+6 more)

### Community 52 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.06
Nodes (19): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+11 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.14
Nodes (42): ActorCustomization, StreamUserType, pending_stream_user_type(), binary_fixture(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json() (+34 more)

### Community 57 - "Station"
Cohesion: 0.04
Nodes (43): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+35 more)

### Community 58 - "MonoBehaviour"
Cohesion: 0.01
Nodes (112): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+104 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "String"
Cohesion: 0.12
Nodes (34): ability_choices(), action_animation_choices(), animation_quat_keys_editor(), animation_transform_tracks_editor(), animation_vec3_keys_editor(), building_model_node_choices(), cached_gltf_metadata(), discover_model_assets() (+26 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (55): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+47 more)

### Community 62 - "CameraController"
Cohesion: 0.06
Nodes (17): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+9 more)

### Community 63 - "AnimationHandler"
Cohesion: 0.09
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+7 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.08
Nodes (15): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+7 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (131): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+123 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "EnemyModelHandler"
Cohesion: 0.07
Nodes (13): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, int, STSM_Helper_Attack, int (+5 more)

### Community 74 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 75 - "SelectedObject"
Cohesion: 0.09
Nodes (6): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup

### Community 76 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (38): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+30 more)

### Community 77 - "Objective"
Cohesion: 0.15
Nodes (3): Action, int, Objective

### Community 78 - "technology_graph.rs"
Cohesion: 0.06
Nodes (66): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), Default, Result (+58 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.14
Nodes (13): bool, CancellationTokenSource, int, long, MenuItem, string, DeviceCodeResponse, ErrorResponse (+5 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (20): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+12 more)

### Community 81 - "WorldUtils"
Cohesion: 0.15
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.11
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 83 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "TwitchUser"
Cohesion: 0.22
Nodes (7): ActivityStatus, bool, float, string, UserType, TwitchUser, Character.Enumerations

### Community 86 - "drive_tidal_music"
Cohesion: 0.17
Nodes (26): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature, player_music_gain() (+18 more)

### Community 87 - ".new"
Cohesion: 0.10
Nodes (21): closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting(), NativeGameAudioRouting, operator_panel_uses_compact_telemetry_and_bottom_left_live_control() (+13 more)

### Community 88 - "settings.rs"
Cohesion: 0.09
Nodes (34): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+26 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (20): Animator, GameObject, int, FishGodEvent, bool, IEnumerator, int, List (+12 more)

### Community 92 - "world.rs"
Cohesion: 0.08
Nodes (60): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), cell_hash(), changing_seed_changes_world_hash() (+52 more)

### Community 93 - "Ui"
Cohesion: 0.12
Nodes (60): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+52 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (41): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+33 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.04
Nodes (49): Action, CancellationToken, Component, Container, ContainerBuilder, Dictionary, float, List (+41 more)

### Community 98 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (68): AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+60 more)

### Community 100 - "twitch_tab"
Cohesion: 0.20
Nodes (16): broadcast_encoder_label(), Duration, Sender, start_twitch_authorization(), start_twitch_broadcast_authorization(), start_twitch_broadcast_clear(), start_twitch_broadcast_diagnostic(), start_twitch_clear() (+8 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - ".Update"
Cohesion: 0.17
Nodes (16): List, Material, materials, Mesh, meshes, Dictionary, int, List (+8 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.11
Nodes (13): Container, ContainerBuilder, Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor, bool (+5 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "Goal"
Cohesion: 0.13
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 108 - "LabelDisplayProcessor"
Cohesion: 0.14
Nodes (9): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, float, ParticleSystem (+1 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.15
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 112 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 113 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "PlayerRoleData"
Cohesion: 0.08
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+6 more)

### Community 116 - "stream_town_tools/src/main.rs"
Cohesion: 0.07
Nodes (57): apply_building_draft(), AssetEditorSection, authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, character_model_choices_include_converted_hierarchy_nodes(), checked_in_authoring_assets_pass_headless_validation() (+49 more)

### Community 117 - "tools_ui"
Cohesion: 0.14
Nodes (17): debug_fingerprint(), default_role_preview_animation(), matching_role_animation_state(), migration_tab(), player_animation_controller(), poll_tool_job_events(), role_preview_animation_request(), role_preview_uses_shipping_rig_animation_and_composition_rules() (+9 more)

### Community 118 - "Globals"
Cohesion: 0.15
Nodes (8): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, Globals, DirectoryInfo

### Community 119 - "TechTreeNode"
Cohesion: 0.09
Nodes (15): ChildrenSaveData, Color, float, string, TechnologyTreeGroup, Color, Foldout, List (+7 more)

### Community 120 - "VoteEvent"
Cohesion: 0.06
Nodes (23): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, TechVote (+15 more)

### Community 121 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 122 - "RoleProcessor"
Cohesion: 0.05
Nodes (19): Dictionary, int, RoleDataContainer, Container, ContainerBuilder, int, List, RoleProcessor (+11 more)

### Community 123 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.12
Nodes (19): bool, float, int, string, Type, Vector3, AIPath, AstarData (+11 more)

### Community 126 - "Targetable"
Cohesion: 0.07
Nodes (19): uint, GUIDComponent, List, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject (+11 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.05
Nodes (51): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic_to(), bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), CadenceTick, configure_amf_quality(), configure_direct_broadcast(), direct_broadcast_diagnostics_are_persisted_without_a_live_session() (+43 more)

### Community 129 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 130 - "Coordinator"
Cohesion: 0.10
Nodes (15): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, IEnumerable (+7 more)

### Community 131 - "Option"
Cohesion: 0.12
Nodes (36): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, BroadcastTarget, capture_process_audio(), discard_pending_audio() (+28 more)

### Community 132 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 133 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 134 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 135 - "TechTree.Elements"
Cohesion: 0.07
Nodes (20): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, Vector2, GroupSaveData (+12 more)

### Community 136 - "Editor"
Cohesion: 0.14
Nodes (5): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 137 - "StationSensor"
Cohesion: 0.13
Nodes (6): float, List, SensorRuntimeData, SensorBase, UnityEvent, StationSensor

### Community 138 - "sync_stream_only_capture"
Cohesion: 0.07
Nodes (44): arm_stream_only_readback(), bounded_history_f32(), BroadcastMetricsSnapshot, camera_targets_primary_window(), cleanup_completed_stream_only_readbacks(), disarm_stream_only_readbacks(), operator_restart_button_requests_a_stream_restart(), Assets (+36 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "Result"
Cohesion: 0.12
Nodes (24): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), encoder_candidates(), encoder_input_format(), encoder_is_hardware() (+16 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "roles_tab"
Cohesion: 0.32
Nodes (12): apply_role_draft(), delete_selected_role(), duplicate_selected_role(), legacy_roles_tab(), refresh_role_draft(), role_draft(), role_editor_applies_every_reference_family_without_partial_mutation(), role_i32() (+4 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "Season"
Cohesion: 0.22
Nodes (7): Color, float, int, VisualEffect, SeasonDataSettings, Season, Gradient

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 154 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 155 - "Option"
Cohesion: 0.09
Nodes (42): AnimationClipDef, animation_take_name(), animator_component(), animator_reference_path(), color_value(), convert_clips(), convert_embedded_model_clips(), convert_post_process() (+34 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.13
Nodes (12): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+4 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - ".ValidateTokenAsync"
Cohesion: 0.33
Nodes (6): CancellationToken, Dictionary, Task, UnityWebRequest, TokenValidationResponse, WebResponse

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 163 - "EnemyCampSaveData"
Cohesion: 0.50
Nodes (3): int, uint, EnemyCampSaveData

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 167 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

### Community 171 - "Target"
Cohesion: 0.13
Nodes (11): UserInterface.MainMenu, TownGoal, Target, Utils.Pooling, GridSystem.Partitioning, Combat, Environment, SavingAndLoading.SavableObjects (+3 more)

### Community 173 - "CommandDictionary"
Cohesion: 0.13
Nodes (8): Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary, MiscCommands, EnemyType

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 177 - "Processors"
Cohesion: 0.05
Nodes (11): ObjectSelectionProcessor, InputButton, Transform, PlayerSpawnPoint, Processors, World, Processors.Editor, MetaData (+3 more)

### Community 178 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 179 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (97): AnimationFloatKeyframe, AnimationTangent, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id() (+89 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "UnitTextDisplay"
Cohesion: 0.12
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 185 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (88): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+80 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 190 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 195 - "UserInterface"
Cohesion: 0.04
Nodes (29): InputButton, SharedTypes, int, ChangeTimeStamp, Color, GameUserType, UserColours, Slider (+21 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Current attempts, Do not retry unchanged, Next narrow diagnostic pass, Tree and Foliage Flicker Regression Checklist, What did not fix the flicker, What did work

### Community 200 - "Bevy Migration Status"
Cohesion: 0.40
Nodes (5): Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity, Validation

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

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

### Community 208 - "RoleSlot"
Cohesion: 0.25
Nodes (3): RoleSlot, bool, int

### Community 210 - "publish_stream_only_frame"
Cohesion: 0.17
Nodes (11): capture_direct_broadcast_frame(), gpu_readback_padding_is_removed_without_corrupting_rows(), gpu_readbacks_are_published_in_render_order_even_when_they_finish_out_of_order(), publish_stream_only_frame(), remove_gpu_row_padding(), BTreeMap, On, Time (+3 more)

### Community 211 - "DirectBroadcastControl"
Cohesion: 0.12
Nodes (17): apply_direct_broadcast_control(), DirectBroadcastControl, exit_after_broadcast_stops(), operator_window_close_requests_exit(), poll_direct_broadcast_authorization(), poll_twitch_live_verification(), return_to_main_menu_after_broadcast_stops(), AppExit (+9 more)

### Community 212 - "WorldInstanceDeterminism"
Cohesion: 0.29
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 213 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.21
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "TechTreeGraphView"
Cohesion: 0.09
Nodes (16): Vector2, int, List, Port, Vector2, TechTreeGraphView, List, Texture2D (+8 more)

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
Cohesion: 0.20
Nodes (6): Audio provenance, Binaries, Commands, Stream Town Bevy, Original project notes, Stream Town: Bevy Migration

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

### Community 234 - "IProcessor"
Cohesion: 0.05
Nodes (18): Container, ContainerBuilder, GridProcessor, Container, IProcessor, Container, ContainerBuilder, CreditsProcessor (+10 more)

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

### Community 243 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

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

### Community 251 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), ErasedRenderAssets, GpuImage, GpuRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "RoleHandler"
Cohesion: 0.10
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

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

### Community 264 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 268 - "DebugSettings"
Cohesion: 0.48
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

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

### Community 273 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.07
Nodes (55): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+47 more)

### Community 285 - "Stream Town Twitch commands"
Cohesion: 0.40
Nodes (4): Moderator and game-master commands, Player commands, Ruler and operator commands, Stream Town Twitch commands

### Community 287 - "Utils"
Cohesion: 0.04
Nodes (16): BuildCostModifier, RoleScriptablesEditor, DisableOnAwake, SelectionBase, SimpleHideRendererOnAwake, GameObject, SimpleRandomModelEnabled, Utils (+8 more)

### Community 288 - "Character"
Cohesion: 0.06
Nodes (10): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Animation, Sensors, STStateMachine, Pathfinding (+2 more)

### Community 289 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 290 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 291 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 292 - "ResourceGenerationSettings"
Cohesion: 0.33
Nodes (5): AnimationCurve, bool, int, List, ResourceGenerationSettings

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "String"
Cohesion: 0.14
Nodes (14): AuthorizationEvent, BroadcastPrerequisites, DirectTwitchBroadcastPlugin, format_minutes_seconds(), LiveVerification, LiveVerificationEvent, LiveVerificationTarget, PreparedBroadcast (+6 more)

### Community 297 - "DirectBroadcastRuntime"
Cohesion: 0.11
Nodes (25): append_direct_broadcast_diagnostic(), average_milliseconds(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastStopDisposition, direct_broadcast_log_path(), DirectBroadcastPhase, DirectBroadcastRuntime (+17 more)

### Community 298 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (67): BTreeMap, RuntimeConsoleStatus, add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft() (+59 more)

### Community 300 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 301 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

### Community 304 - "InventorySaveData"
Cohesion: 0.29
Nodes (6): bool, int, List, string, InventoryEntrySaveData, InventorySaveData

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.04
Nodes (21): Player, Dictionary, GameObject, Vector3, Vector3, Action, Container, ContainerBuilder (+13 more)

### Community 307 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **383 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+378 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **31 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=4.53424798) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=3.237941093) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.962292656) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.477432826) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.366566747)
- `SkinnedMesh` (2× useful, score=1.997632118)
- `drive_tidal_music()` (2× useful, score=1.966606185) _(code changed — re-verify)_
- `WorldSimulation` (2× useful, score=1.71789778) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `DayAndNightProcessor`, `TechTree.Elements`, `World.Generation`, `.CreateEnumField`, `StringUtils`, `Character`, `SimpleScreenShot`, `MeshData`, `UpdateGraphBounds`, `Target`, `ScriptablesProcessorInfrastructure`, `SimpleRotateOnAxis`, `Processors`, `SimpleEventOnStart`, `UnitTextDisplay`, `Station`, `CameraController`, `SimpleDisableAfterTime`, `UserInterface`, `BuildingPlacer`, `SnapToGridMouseMovement`, `FPSDisplay`, `RandomEnabler`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `String`, `Res`, `stream_town_game/src/lib.rs`, `generate_world`, `config.rs`, `advance_world_loading_cover`, `save.rs`, `Vec`, `command.rs`, `twitch.rs`, `roles_tab`, `Option`, `PresentationCatalog`, `Option`, `String`, `RenderAssets`, `ToolState`, `AnimationControllerDef`, `String`, `ContentCatalog`, `stream_town_migrate/src/presentation.rs`, `String`, `stream_town_domain/src/content.rs`, `GameplayConfig`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `technology_graph.rs`, `world.rs`, `Ui`, `stream_town_domain/src/presentation.rs`, `stream_town_tools/src/main.rs`, `tools_ui`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `RoleHandler`, `BuildingProcessor`, `DayAndNightProcessor`, `TwitchChatProcessor`, `IRuntimeDataScriptable`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `WorldGenProcessor`, `TechTreeProcessor`, `PlayerRole`, `Target`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `GameEventProcessor`, `SelectedObject`, `ObjectPoolingProcessor`, `BuildingPlacer`, `RaidEvent`, `Resource`, `SaveProcessor`, `IProcessor`, `PlayerRoleData`, `VoteEvent`, `RoleProcessor`?**
  _High betweenness centrality (0.023) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _383 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingBase` be split into smaller, more focused modules?**
  _Cohesion score 0.06585365853658537 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05144230769230769 - nodes in this community are weakly interconnected._
- **Should `String` be split into smaller, more focused modules?**
  _Cohesion score 0.09954751131221719 - nodes in this community are weakly interconnected._