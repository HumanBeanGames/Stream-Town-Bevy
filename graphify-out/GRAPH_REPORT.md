# Graph Report - Stream-Town-Bevy  (2026-09-04)

## Corpus Check
- 672 files · ~1,840,100 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9700 nodes · 29722 edges · 312 communities (290 shown, 22 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1063 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7cd17d4f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ContentCatalog
- BuildingProcessor
- parse_controller
- SeasonProcessor
- Res
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- .new
- BottomBarInterface
- AnimationHandler
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- TownGoalProcessor
- xtask/src/main.rs
- World.Generation
- command.rs
- twitch.rs
- NavGrid
- IRuntimeDataScriptable
- ScriptablesProcessorInfrastructure
- TechnologyGraphLayout
- Node_SO
- PlayerSaveData
- ObjectiveSaveData
- SaveFileData
- String
- Query
- WorldGenProcessor
- PathBuf
- save.rs
- GenerationSettings
- UserInterface_TownVote
- PlayerSettings
- SettingsData
- RenderAssets
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- StableId
- String
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- stream_town_migrate/src/content.rs
- .GetResourceAssets
- BroadcastConfig
- AudioHandler
- StreamTownSessionBridge
- VfxSeagullSpawner
- TwitchClientProcessor
- PlayerRoleData
- .SerializeComponent
- legacy.rs
- BevyMigrationExporter
- ScriptableObject
- TechTreeEditorWindow
- DayAndNightProcessor
- stream_town_domain/src/content.rs
- CameraController
- Result
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- .new
- GamestateJukebox
- Option
- models.rs
- Tiler
- GridPos
- Utils
- Result
- RoleHandler
- CommonEnums.cs
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- PlayerProcessor
- Access_Text
- encode_broadcast_session
- world.rs
- BuildingBase
- Goal
- FoliageProcessor
- SnapToGridMouseMovement
- RaidEvent
- .Log
- MonoBehaviour
- convert_fbx_to_glb.py
- BroadcastController
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- TechTreeNode
- StateMachine
- GameEventProcessor
- TownGoal.Data
- TwitchUser
- ResourceProcessor
- LoadingManager
- LabelDisplayProcessor
- UnitHealthBar
- CustomLogHandler
- LevelHandler
- ProjectCamera
- UpdateGraphBounds
- GlobalAudioController
- ScriptablesEditor
- RoleDataSettings
- Editor
- tidal_music.rs
- STSM_StateAction
- .CreateEnumField
- VoteEvent
- TechTreeGraphView
- drive_tidal_music
- STSM_Idle_Player
- AIPath
- PlayerInputProcessor
- GateController
- direct_broadcast.rs
- .EnsureValidCredentials
- Coordinator
- UserInterface_Roles
- SelectedObject
- Enemy
- stream_town_domain/src/lib.rs
- TimeProcessor
- NativeGameAudioRouting
- TargetSensor
- Option
- next_agent_goal
- WeatherProcessor
- What You Must Do When Invoked
- RuntimeData Template
- GridProcessor
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldSaveData
- xtask/src/lib.rs
- EnemyModelHandler
- VfxAnimationController
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- .write
- ResourceRuntimeData
- DirectBroadcastRuntime
- SelectableObject
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- UserInterface_ObjectSelection
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_TownGoal
- MiscCommands
- WorldGenSaveData
- UserInterface_DisplayUsernames
- PlayerRole
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- SimpleMusicController
- EditorHelpers
- update_enemy_music_intensity
- Vec4
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- STSM_Idle
- Access_Dropdown
- List
- .Draw
- stream_town_migrate/src/presentation.rs
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- EditorUtils
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Easings
- CommandDictionary
- WorldGenerationReferenceExporter
- TL_Secrets
- .RenderResourceType
- EnemyNavigationField
- UserInterface_BuildingHealthBar
- stream_town_migrate/src/menu_scene.rs
- Access_GOList
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- BuildingDataSettings
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- .ValidateTokenAsync
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- ResourceHolder
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- SelectedEnemy
- Common Patterns
- SelectedBuilding
- MeshSaveData
- SelectedResource
- WorldGenRuntimeData
- StringUtils
- stream_town_tools/src/main.rs
- Key Rules
- STSM_HelperBase
- RuntimeData Template
- Character Animation Regression Checklist
- HealthModifier
- ScriptKeywordProcessor
- FPSDisplay
- record_gpu_readiness
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
- UnitTextDisplay
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- PlayerDeathHandler
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- Access_Toggle
- extraction-spec.md
- SelectedPlayerGroup
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- Vec
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- parse_model_clip_events
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- legacy_roles_tab
- Q: If there is more to do, keep going.
- Access_TextInput
- draw_world_preview
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- SelectedEnemyCamp
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Value
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- main
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch command reference
- SimpleScreenShot
- ObjectSelectionProcessor.Editor.cs
- DontDestroyOnLoad
- RotationHandler
- Requirement
- BuildCostModifier.cs
- StatusBar
- CreateProjectScopeProcessors.cs
- RandomEnabler
- .ExportModification
- ToolState
- UnityGraphics
- FoliageGenerationSettings
- setup_rendering
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- tidal_plugin
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .SetTargetType
- vcpkg.json
- Targetable
- FFmpeg runtime and relinking
- StreamTown.Migration

## God Nodes (most connected - your core abstractions)
1. `StableId` - 474 edges
2. `ContentCatalog` - 211 edges
3. `WorldSimulation` - 210 edges
4. `GridPos` - 175 edges
5. `Utils` - 159 edges
6. `Processors` - 156 edges
7. `ScriptablesProcessorInfrastructure` - 150 edges
8. `Player` - 142 edges
9. `RenderAssets` - 142 edges
10. `ToolState` - 138 edges

## Surprising Connections (you probably didn't know these)
- `generate_world()` --calls--> `default_resource_generation_layers()`  [INFERRED]
  bevy-port/crates/stream_town_domain/src/world.rs → bevy-port/crates/stream_town_domain/src/content.rs
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

## Communities (312 total, 22 thin omitted)

### Community 0 - "ContentCatalog"
Cohesion: 0.06
Nodes (99): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, ActorState, RoleProgress, Default, String (+91 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (16): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+8 more)

### Community 2 - "parse_controller"
Cohesion: 0.15
Nodes (21): animation_state_id(), animation_state_machine_id(), avatar_mask_id(), clip_id(), inline_file_id(), parse_blend_tree(), parse_child_references(), parse_controller() (+13 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.07
Nodes (18): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+10 more)

### Community 4 - "Res"
Cohesion: 0.03
Nodes (205): AccessibilityFocusVisualQuery, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), AccessibilityRuntime, AccessibleButtonNodeQuery, AccessibleButtonScope, advance_loading_phase() (+197 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (292): accessibility_should_clear_focus(), AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityCandidate, AccessibilityHighContrastText, actor_detail_budget(), actor_name_color(), ActorHealthFill (+284 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.07
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - ".new"
Cohesion: 0.02
Nodes (229): AccessibilityActionRequest, generate_world(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean(), advance_falling_fish() (+221 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "AnimationHandler"
Cohesion: 0.10
Nodes (11): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+3 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.06
Nodes (13): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+5 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.06
Nodes (11): Dictionary, DebugSettings, bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField (+3 more)

### Community 12 - "config.rs"
Cohesion: 0.12
Nodes (25): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Default (+17 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.10
Nodes (11): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+3 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (40): archive_purge_backup_history(), AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes() (+32 more)

### Community 18 - "World.Generation"
Cohesion: 0.04
Nodes (34): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller (+26 more)

### Community 19 - "command.rs"
Cohesion: 0.08
Nodes (54): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+46 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (65): BTreeSet, TwitchConfig, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization (+57 more)

### Community 21 - "NavGrid"
Cohesion: 0.12
Nodes (26): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavGrid, NavigationError, neighbour_candidates() (+18 more)

### Community 22 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (14): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+6 more)

### Community 23 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.06
Nodes (12): ContainerBuilder, MetaDataInstaller, ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, StreamTown.EditorTools, Reflex.Core (+4 more)

### Community 24 - "TechnologyGraphLayout"
Cohesion: 0.12
Nodes (29): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), Default, Result (+21 more)

### Community 25 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 26 - "PlayerSaveData"
Cohesion: 0.11
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 27 - "ObjectiveSaveData"
Cohesion: 0.06
Nodes (25): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+17 more)

### Community 28 - "SaveFileData"
Cohesion: 0.08
Nodes (23): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+15 more)

### Community 29 - "String"
Cohesion: 0.08
Nodes (91): ArchetypeKind, ability_choices(), action_animation_choices(), animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice() (+83 more)

### Community 30 - "Query"
Cohesion: 0.03
Nodes (219): AccessibilityNode, Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver (+211 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (29): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+21 more)

### Community 32 - "PathBuf"
Cohesion: 0.05
Nodes (43): AnyResult, automatic_resume_save_path(), EnemyPathOpenNode, is_jump_start_path(), jump_start_saves_are_identifiable_and_never_reuse_an_existing_path(), jump_start_snapshot_path(), jump_start_working_path(), legacy_unity_settings_path() (+35 more)

### Community 33 - "save.rs"
Cohesion: 0.12
Nodes (40): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+32 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.07
Nodes (30): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+22 more)

### Community 35 - "UserInterface_TownVote"
Cohesion: 0.08
Nodes (18): Dictionary, TechVote, Button, GameObject, Image, Slider, TextMeshProUGUI, Transform (+10 more)

### Community 36 - "PlayerSettings"
Cohesion: 0.11
Nodes (30): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+22 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "RenderAssets"
Cohesion: 0.04
Nodes (192): MainMenuSceneReference, Option, PresentationCatalog, actor_material(), actor_scene_budget(), agent_facing_matches_unity_rotation_and_action_targets(), agent_path_world_target(), AgentCommandQueue (+184 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (13): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, string (+5 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "StableId"
Cohesion: 0.04
Nodes (83): ObjectiveDef, ObjectiveKind, Display, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase() (+75 more)

### Community 43 - "String"
Cohesion: 0.02
Nodes (172): AccessibleNode, AnimationClip, AnimationTargetId, AssetId, accessibility_settings_selection(), active_event_text(), add_animation_composition(), add_animation_layer_branch() (+164 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (33): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+25 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.13
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 47 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (130): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+122 more)

### Community 48 - ".GetResourceAssets"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 49 - "BroadcastConfig"
Cohesion: 0.09
Nodes (30): BroadcastConfig, BroadcastEncoderPreference, amf_quality_profile_keeps_static_grid_detail_between_keyframes(), AuthorizationEvent, BroadcastPrerequisites, BroadcastTarget, configure_amf_quality(), encoder_candidates() (+22 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "PlayerRoleData"
Cohesion: 0.07
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+6 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.11
Nodes (47): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id() (+39 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 58 - "ScriptableObject"
Cohesion: 0.02
Nodes (97): ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, GameSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller (+89 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.07
Nodes (53): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, default_resource_generation_layers(), EnemyCampGenerationDef (+45 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Result"
Cohesion: 0.11
Nodes (52): AnimationFloatKeyframe, AnimationTangent, append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id(), json_f32() (+44 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.14
Nodes (11): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, ContainerBuilder, ColorAdjustments (+3 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.09
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - ".new"
Cohesion: 0.16
Nodes (17): closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting(), operator_live_button_applies_stop_in_the_pressed_frame() (+9 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "Option"
Cohesion: 0.05
Nodes (61): ArchetypeDef, ArchetypeScene, animated_pets_resolve_their_own_unity_controllers_and_rigs(), animation_root_name(), apply_authored_ui_fonts(), archetype_by_source(), archetype_scene_for_age(), building_scene_for_state() (+53 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "GridPos"
Cohesion: 0.07
Nodes (96): DirtyRegion, GridPos, EnemyCampState, GeneratedWorld, active_resource_at(), agent_action_facing_grid(), agent_path(), AgentCommand (+88 more)

### Community 74 - "Utils"
Cohesion: 0.03
Nodes (41): InputButton, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Utils, Processors, Pets.Enumerations (+33 more)

### Community 75 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 76 - "RoleHandler"
Cohesion: 0.10
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 77 - "CommonEnums.cs"
Cohesion: 0.05
Nodes (23): PlacementProbe, float, PlacementProbeHandler, TargetSettings, ContainerBuilder, TargetSettingsInstaller, TargetableData, Action (+15 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.12
Nodes (38): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+30 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.14
Nodes (13): bool, CancellationTokenSource, int, long, MenuItem, string, DeviceCodeResponse, ErrorResponse (+5 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.27
Nodes (5): GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 83 - "PlayerProcessor"
Cohesion: 0.08
Nodes (10): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnChatCommandReceivedArgs (+2 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "encode_broadcast_session"
Cohesion: 0.10
Nodes (35): AtomicBool, BroadcastEncoder, BroadcastMetrics, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), discard_pending_audio(), duration_as_micros(), encode_broadcast_session() (+27 more)

### Community 86 - "world.rs"
Cohesion: 0.08
Nodes (65): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), cell_hash(), changing_seed_changes_world_hash() (+57 more)

### Community 87 - "BuildingBase"
Cohesion: 0.10
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 88 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - ".Log"
Cohesion: 0.04
Nodes (45): Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor, Action, bool (+37 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (120): Api, CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, PersistentScoped, ContainerBuilder (+112 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "BroadcastController"
Cohesion: 0.11
Nodes (17): AudioFrame, AudioInput, BroadcastController, BroadcastMetricsSnapshot, capture_direct_broadcast_frame(), capture_process_audio(), LiveVerification, LiveVerificationEvent (+9 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (45): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+37 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (30): Container, ContainerBuilder, GUIDProcessor, Action, CancellationToken, Component, Container, ContainerBuilder (+22 more)

### Community 98 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (68): AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+60 more)

### Community 100 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "GameEventProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+9 more)

### Community 103 - "TownGoal.Data"
Cohesion: 0.09
Nodes (10): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 104 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.08
Nodes (24): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+16 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "LabelDisplayProcessor"
Cohesion: 0.12
Nodes (9): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, float, ParticleSystem (+1 more)

### Community 108 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "RoleDataSettings"
Cohesion: 0.10
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 116 - "Editor"
Cohesion: 0.06
Nodes (14): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, string (+6 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.16
Nodes (20): composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine(), format_number(), intensity_changes_every_authored_low_pass_filter(), intensity_cycles_per_second(), intensity_makes_the_struck_voice_brighter_sharper_and_not_excessively_louder(), intensity_smoothing_has_a_fifteen_second_time_constant() (+12 more)

### Community 118 - "STSM_StateAction"
Cohesion: 0.12
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 119 - ".CreateEnumField"
Cohesion: 0.14
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 120 - "VoteEvent"
Cohesion: 0.06
Nodes (21): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, float (+13 more)

### Community 121 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 122 - "drive_tidal_music"
Cohesion: 0.32
Nodes (13): drive_tidal_music(), intensity_program_needs_update(), IntensitySongInput, report_once(), NativeAudioRouting, Option, silence_music(), stop_tidal_music() (+5 more)

### Community 123 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (16): RoleData, AudioClip, bool, float, int, Sprite, string, STSM_Action_GatherResource (+8 more)

### Community 124 - "AIPath"
Cohesion: 0.12
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.05
Nodes (46): append_direct_broadcast_diagnostic_to(), bandwidth_test_url_is_constructed_without_logging_the_key(), bounded_history_f32(), build_ingest_url(), configure_direct_broadcast(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), DirectTwitchBroadcastPlugin, draw_centered_label() (+38 more)

### Community 129 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "UserInterface_Roles"
Cohesion: 0.20
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 132 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 133 - "Enemy"
Cohesion: 0.06
Nodes (17): Transform, Action, float, Enemy, float, int, List, Transform (+9 more)

### Community 134 - "stream_town_domain/src/lib.rs"
Cohesion: 0.13
Nodes (13): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError (+5 more)

### Community 135 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 136 - "NativeGameAudioRouting"
Cohesion: 0.14
Nodes (12): AutomaticBroadcastStart, NativeGameAudioClip, NativeGameAudioMix, NativeGameAudioRouting, NativeGameAudioState, NativeGameAudioVoice, pcm16_wav_clip(), pcm16_wav_data() (+4 more)

### Community 137 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 138 - "Option"
Cohesion: 0.07
Nodes (57): arm_stream_only_readback(), camera_targets_primary_window(), cleanup_completed_stream_only_readbacks(), disarm_stream_only_readbacks(), gpu_readbacks_are_published_in_render_order_even_when_they_finish_out_of_order(), moderate_selected_operator_user(), publish_stream_only_frame(), Assets (+49 more)

### Community 139 - "next_agent_goal"
Cohesion: 0.19
Nodes (19): ActionPresentation, actor_combat_visual(), authored_assignment_penalty_spreads_farmers_across_farms(), battering_ram_targets_and_damages_buildings_from_authored_mask(), builder_completes_and_upgrades_authored_construction(), building_base_max_health(), combat_goal_damages_kills_and_respawns(), CombatVisualKind (+11 more)

### Community 140 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GridProcessor"
Cohesion: 0.05
Nodes (24): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, CellPartitioningEditor, GridProcessorEditor (+16 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 147 - "WorldSaveData"
Cohesion: 0.11
Nodes (18): int, string, ObjectiveSaveData, bool, float, List, string, TechTreeSaveData (+10 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "EnemyModelHandler"
Cohesion: 0.14
Nodes (8): bool, int, List, EnemyModelHandler, bool, float, Vector3, STSM_Action_EnemyAttack

### Community 150 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - ".write"
Cohesion: 0.33
Nodes (6): PlayerSettingsStore, Into, Path, PathBuf, Result, store_recovers_last_valid_backup()

### Community 154 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 155 - "DirectBroadcastRuntime"
Cohesion: 0.06
Nodes (42): append_direct_broadcast_diagnostic(), apply_direct_broadcast_control(), average_milliseconds(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastStopDisposition, CadenceTick, direct_broadcast_log_path() (+34 more)

### Community 156 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.09
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 159 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 163 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 164 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "PlayerRole"
Cohesion: 0.04
Nodes (29): SimpleToggleCarry, RoleSlotModifier, int, CharacterModelHandler, bool, int, List, Transform (+21 more)

### Community 167 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 170 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 171 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 172 - "update_enemy_music_intensity"
Cohesion: 0.20
Nodes (12): point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, Res, ResMut, Time (+4 more)

### Community 173 - "Vec4"
Cohesion: 0.06
Nodes (35): AccessibilityMotionDefaults, BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension (+27 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 176 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 177 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 178 - ".Draw"
Cohesion: 0.13
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 179 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (110): AnimationClipDef, MaterialDef, PrefabPresentationBinding, TextureDef, animation_take_name(), animator_component(), animator_reference_path(), array_index() (+102 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "EditorUtils"
Cohesion: 0.18
Nodes (5): Color, List, Texture2D, EditorUtils, DirectoryInfo

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 185 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 188 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 189 - "EnemyNavigationField"
Cohesion: 0.38
Nodes (4): EnemyClusterNode, EnemyNavigationField, grid_octile_distance(), try_agent_path_for_goal()

### Community 190 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (42): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, String, Vec (+34 more)

### Community 192 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - ".ValidateTokenAsync"
Cohesion: 0.33
Nodes (6): CancellationToken, Dictionary, Task, UnityWebRequest, TokenValidationResponse, WebResponse

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.25
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

### Community 203 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "MeshSaveData"
Cohesion: 0.11
Nodes (12): Mesh, Vector3, int, uint, EnemyCampSaveData, bool, int, MeshSaveData (+4 more)

### Community 211 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 213 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (71): animation_property_curves_editor(), apply_building_draft(), authoring_apply_preserves_the_complete_local_twitch_setup(), broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, character_model_choices_include_converted_hierarchy_nodes() (+63 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "HealthModifier"
Cohesion: 0.22
Nodes (6): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), ErasedRenderAssets, GpuImage, GpuRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

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
Cohesion: 0.15
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

### Community 234 - "UnitTextDisplay"
Cohesion: 0.22
Nodes (6): bool, Color, float, string, UnitTextDisplay, TextMeshPro

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

### Community 243 - "PlayerDeathHandler"
Cohesion: 0.25
Nodes (4): PlayerDeathHandler, bool, float, Vector3

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

### Community 251 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "Vec"
Cohesion: 0.43
Nodes (8): binary_fixture(), BinaryParser, put_f32(), put_i32(), put_string(), put_u32(), Vec, Cursor

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "parse_model_clip_events"
Cohesion: 0.32
Nodes (8): inline_mapping_value(), parse_animation_events(), parse_model_clip_events(), parse_object_reference(), parse_property_curves(), parses_normalized_animation_events_from_model_importer_clips(), parses_property_curves_and_animation_events_without_unity_types(), unity_scalar()

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "legacy_roles_tab"
Cohesion: 0.39
Nodes (8): apply_role_draft(), delete_selected_role(), duplicate_selected_role(), legacy_roles_tab(), refresh_role_draft(), role_draft(), role_editor_applies_every_reference_family_without_partial_mutation(), RoleDraft

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 268 - "draw_world_preview"
Cohesion: 0.29
Nodes (8): draw_world_preview(), preview_grid_point(), preview_lerp_color(), Color32, Pos2, Rect, terrain_preview_color(), WorldPreviewLayer

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

### Community 276 - "Value"
Cohesion: 0.20
Nodes (26): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+18 more)

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.07
Nodes (56): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+48 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.13
Nodes (15): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+7 more)

### Community 287 - "SimpleScreenShot"
Cohesion: 0.18
Nodes (5): UnitTravelToPosition, Vector3, int, string, SimpleScreenShot

### Community 290 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 291 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 293 - "StatusBar"
Cohesion: 0.33
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 299 - "ToolState"
Cohesion: 0.06
Nodes (91): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft(), AssetEditorSection, authoring_snapshot() (+83 more)

### Community 301 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 303 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 304 - "setup_rendering"
Cohesion: 0.04
Nodes (85): AmbientLight, ActiveMaterialHandles, authored_color_grading(), authored_post_process_stack(), authored_rgb_filter(), blend_environment_palette(), building_damage_intensity(), building_damage_value() (+77 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+4 more)

### Community 307 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - ".SetTargetType"
Cohesion: 0.15
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 325 - "Targetable"
Cohesion: 0.03
Nodes (48): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+40 more)

## Knowledge Gaps
- **394 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+389 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=4.53424798) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=3.237941093) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.962292656) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.477432826) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.366566747) _(code changed — re-verify)_
- `SkinnedMesh` (2× useful, score=1.997632118)
- `drive_tidal_music()` (2× useful, score=1.966606185) _(code changed — re-verify)_
- `WorldSimulation` (2× useful, score=1.71789778) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `Enemy`, `World.Generation`, `ScriptablesProcessorInfrastructure`, `ObjectiveSaveData`, `SimpleScreenShot`, `GenerationSettings`, `RandomEnabler`, `Easings`, `ScriptableObject`, `CommonEnums.cs`, `BuildingPlacer`, `StringUtils`, `SnapToGridMouseMovement`, `FPSDisplay`, `MonoBehaviour`, `Resource`, `TownGoal.Data`, `UpdateGraphBounds`, `RoleDataSettings`, `Editor`, `.CreateEnumField`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `ContentCatalog`, `parse_controller`, `Res`, `stream_town_game/src/lib.rs`, `stream_town_domain/src/lib.rs`, `.new`, `legacy_roles_tab`, `next_agent_goal`, `config.rs`, `draw_world_preview`, `command.rs`, `twitch.rs`, `TechnologyGraphLayout`, `ModelPreviewRuntime`, `String`, `Query`, `save.rs`, `RenderAssets`, `String`, `AnimationControllerDef`, `ToolState`, `stream_town_migrate/src/content.rs`, `setup_rendering`, `stream_town_migrate/src/presentation.rs`, `legacy.rs`, `stream_town_domain/src/content.rs`, `EnemyNavigationField`, `stream_town_migrate/src/menu_scene.rs`, `Result`, `Option`, `GridPos`, `technology_graph.rs`, `stream_town_tools/src/main.rs`, `world.rs`, `stream_town_domain/src/presentation.rs`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `UserInterface_Roles`, `Enemy`, `TwitchChatProcessor`, `TimeProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `IProcessor`, `WorldGenProcessor`, `UserInterface_TownVote`, `PlayerRole`, `TechTreeProcessor`, `UIProcessor`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `PlayerRoleData`, `Utils`, `RoleHandler`, `BuildingPlacer`, `RaidEvent`, `.Log`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `VoteEvent`, `SelectedPlayerGroup`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _394 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.06019377447948877 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06599326599326599 - nodes in this community are weakly interconnected._
- **Should `parse_controller` be split into smaller, more focused modules?**
  _Cohesion score 0.14761904761904762 - nodes in this community are weakly interconnected._