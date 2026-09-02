# Graph Report - Stream-Town-Bevy  (2026-09-03)

## Corpus Check
- 672 files · ~1,832,549 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9595 nodes · 29212 edges · 332 communities (309 shown, 23 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1062 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b1133820`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PlayerRoleData
- BuildingProcessor
- BTreeMap
- SeasonProcessor
- ResMut
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- simulation.rs
- BottomBarInterface
- AnimationHandler
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- TownGoalProcessor
- save.rs
- .new
- command.rs
- twitch.rs
- NavGrid
- Targetable
- .new
- WorldGenConfig
- String
- Node_SO
- TechTree.Elements
- SaveFileData
- String
- Res
- WorldGenProcessor
- World.Generation.Settings
- TechTreeGraphView
- GenerationSettings
- VoteEvent
- settings.rs
- SettingsData
- RenderAssets
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- Self
- Vec
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- Result
- ResourceDataSaveData
- .Log
- AudioHandler
- StreamTownSessionBridge
- VfxSeagullSpawner
- TwitchClientProcessor
- Processors
- .SerializeComponent
- legacy.rs
- BevyMigrationExporter
- ScriptableObject
- TechTreeEditorWindow
- DayAndNightProcessor
- stream_town_domain/src/content.rs
- CameraController
- stream_town_migrate/src/presentation.rs
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- GamestateJukebox
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- StableId
- Utils
- SimpleDisableAfterTime
- ResourceHolder
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- PlayerProcessor
- Access_Text
- Result
- world.rs
- .new
- UserInterface_RulerVote
- FoliageProcessor
- SnapToGridMouseMovement
- RaidEvent
- ObjectPoolingProcessor
- MonoBehaviour
- convert_fbx_to_glb.py
- NativeGameAudioRouting
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- xtask/src/main.rs
- StateMachine
- GameEventProcessor
- Result
- TwitchUser
- ResourceProcessor
- LoadingManager
- UI_TechOption
- UnitHealthBar
- CustomLogHandler
- LevelHandler
- generate_world_from_layers
- UpdateGraphBounds
- GlobalAudioController
- ScriptablesEditor
- KeepKingVote
- .EnsureValidCredentials
- tidal_music.rs
- DirectBroadcastRuntime
- .CreateEnumField
- String
- EventProcessor
- DontDestroyOnLoad
- TargetMask
- AIPath
- PlayerInputProcessor
- GateController
- direct_broadcast.rs
- CharacterModelHandler
- Coordinator
- Option
- UnityAsset
- EnemySpawner
- NewKingVote
- TimeProcessor
- RoleProcessor
- TargetSensor
- DirectBroadcastControl
- OpenNode
- WeatherProcessor
- What You Must Do When Invoked
- RuntimeData Template
- Editor
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldSaveData
- xtask/src/lib.rs
- SimpleScreenShot
- TL_Secrets
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- .SetTargetType
- TL_API
- stream_operator_chat_controls
- SelectableObject
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- SelectedPlayer
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_ObjectSelection
- RoleDataSettings
- PlayerInventory
- UserInterface_DisplayUsernames
- RoleHandler
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- SimpleMusicController
- SelectedResource
- Target
- ShaderRef
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- Goal
- Access_Dropdown
- List
- TechTreeNode
- Option
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- EditorUtils
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Easings
- .Draw
- WorldGenerationReferenceExporter
- TransformSaveData
- SaveState
- Autosave
- UserInterface_BuildingHealthBar
- stream_town_migrate/src/menu_scene.rs
- CommonEnums.cs
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- IRuntimeDataScriptable
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- SensorProcessor
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- GUIDComponent
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- String
- Common Patterns
- SelectedBuilding
- PlayerSaveData
- UserInterface
- String
- StringUtils
- stream_town_tools/src/main.rs
- Key Rules
- drive_tidal_music
- RuntimeData Template
- Character Animation Regression Checklist
- .RecalculateStats
- ScriptKeywordProcessor
- FPSDisplay
- BuildingDataSettings
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
- CommandDictionary
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- .write
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- UIElementWrapper
- extraction-spec.md
- SelectedEnemy
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- STSM_Idle_Player
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- PlayerInputRuntimeData
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- ScriptablesProcessorInfrastructure
- Q: If there is more to do, keep going.
- roles_tab
- BuildPlacerData
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- TargetProcessor
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- convert
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- ErrorData
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- .default
- Stream Town Twitch command reference
- import_save
- ObjectSelectionProcessor.Editor.cs
- TechNodeData
- PlayerSettings
- Requirement
- update_enemy_music_intensity
- GridProcessor
- CreateProjectScopeProcessors.cs
- Access_GOList
- .ExportModification
- record_gpu_readiness
- cached_gltf_metadata
- ToolState
- draw_world_preview
- UnityGraphics
- PassiveResourceIncrementer
- WorldGenSaveData
- player_window_mode
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- RotationHandler
- LabelDisplayProcessor
- UserInterface_TownVote
- VfxAnimationController
- .StartGoalFromNode
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- VfxParticlePosition
- IntWrapper
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- vcpkg.json
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- SimpleRotateOnAxis
- Station
- PersistentScoped
- RandomEnabler
- ObjectiveSaveData
- UIRoleDisplay
- FFmpeg runtime and relinking
- StreamTown.Migration

## God Nodes (most connected - your core abstractions)
1. `StableId` - 453 edges
2. `ContentCatalog` - 203 edges
3. `WorldSimulation` - 200 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 141 edges
9. `GridPos` - 140 edges
10. `ToolState` - 138 edges

## Surprising Connections (you probably didn't know these)
- `generate_world()` --calls--> `default_resource_generation_layers()`  [INFERRED]
  bevy-port/crates/stream_town_domain/src/world.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `shipping_world_seeds_authored_enemy_camps_deterministically()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (332 total, 23 thin omitted)

### Community 0 - "PlayerRoleData"
Cohesion: 0.14
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, List, int, PlayerRoleSaveData

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (27): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, bool (+19 more)

### Community 2 - "BTreeMap"
Cohesion: 0.10
Nodes (48): MaterialDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks(), convert_clips(), convert_controllers() (+40 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.09
Nodes (12): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+4 more)

### Community 4 - "ResMut"
Cohesion: 0.02
Nodes (209): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AnyResult, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibilityAnnouncement (+201 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (302): accessibility_motion_preferences_preserve_authored_parameters(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, AccessibilityMotionDefaults, action_ranges_and_tower_acquisition_are_euclidean() (+294 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (26): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+18 more)

### Community 7 - "simulation.rs"
Cohesion: 0.05
Nodes (47): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown() (+39 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "AnimationHandler"
Cohesion: 0.05
Nodes (26): AnimationHandler, Animator, bool, Dictionary, float, int, EnemyModelHandlerEditor, bool (+18 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.10
Nodes (32): broadcast_render_mode_default(), BroadcastConfig, BroadcastEncoderPreference, BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic() (+24 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.14
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (17): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy (+9 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (38): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+30 more)

### Community 18 - ".new"
Cohesion: 0.03
Nodes (231): generate_world(), generate_world_with_content(), accessibility_navigation_preserves_editable_text_focus(), ActiveMaterialHandles, agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_id_by_source() (+223 more)

### Community 19 - "command.rs"
Cohesion: 0.06
Nodes (65): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+57 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (64): SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_oauth_identity() (+56 more)

### Community 21 - "NavGrid"
Cohesion: 0.17
Nodes (15): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building(), reconstruct_path() (+7 more)

### Community 22 - "Targetable"
Cohesion: 0.12
Nodes (8): bool, BoxCollider, float, int, Transform, Vector3, Targetable, IPooledObjectReset

### Community 23 - ".new"
Cohesion: 0.21
Nodes (16): algorithmic_generation_matches_unity_validation_fingerprints(), authored_world_to_grid(), generate_authored_resources(), generate_candidate_mask(), generate_foliage(), positive_noise_offset(), Option, Self (+8 more)

### Community 24 - "WorldGenConfig"
Cohesion: 0.21
Nodes (17): WorldGenConfig, authored_grid_centre(), cell_hash(), foliage_horizontal_hash(), generate_shoreline_resources(), hash_world(), horizontal_hash(), legacy_resource_navigation() (+9 more)

### Community 25 - "String"
Cohesion: 0.03
Nodes (138): AccessibilityNode, AssetId, accessibility_settings_selection(), active_event_text(), advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready() (+130 more)

### Community 26 - "Node_SO"
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 27 - "TechTree.Elements"
Cohesion: 0.08
Nodes (17): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+9 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "String"
Cohesion: 0.08
Nodes (91): ArchetypeKind, ability_choices(), action_animation_choices(), animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice() (+83 more)

### Community 30 - "Res"
Cohesion: 0.03
Nodes (236): Added, AmbientLight, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver (+228 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (28): HashSet, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject, HashSet (+20 more)

### Community 32 - "World.Generation.Settings"
Cohesion: 0.04
Nodes (35): Slider, TextMeshProUGUI, UI_Objective, GameObject, SimpleRandomModelEnabled, CampGenerationSettings, List, CampGenerationSettingsContainer (+27 more)

### Community 33 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 36 - "settings.rs"
Cohesion: 0.16
Nodes (19): BuildingHealthDisplayMode, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), imports_unity_json_indices_and_clamps_values(), imports_unity_subscriber_name_display_index(), InterfaceSettings (+11 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "RenderAssets"
Cohesion: 0.04
Nodes (184): ArchetypeDef, ArchetypeScene, PetDef, PetModelDef, RotatingNodeDef, Option, PresentationCatalog, actor_material() (+176 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.11
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "Self"
Cohesion: 0.16
Nodes (7): LoadingWork, LoadingWorkNode, main_menu_loading_progress(), patch_resume_path_overrides_fixed_smoke_path_without_hiding_the_town_catalog(), IntoIterator, Self, WorldLoadingWork

### Community 43 - "Vec"
Cohesion: 0.04
Nodes (94): AnimationClip, AnimationTargetId, AnimationBlendSelection, single_motion(), WeightedAnimationMotion, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve() (+86 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+18 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 47 - "Result"
Cohesion: 0.15
Nodes (47): aged_buildings(), authored_mask(), building_cost_reductions(), building_level_caps(), collect_model_dependencies(), ContentConversionReport, convert(), convert_export() (+39 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".Log"
Cohesion: 0.08
Nodes (11): Action, Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, Container, ContainerBuilder (+3 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 52 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "Processors"
Cohesion: 0.05
Nodes (13): InputButton, UserInterface.MainMenu, PlayerControls.ObjectSelection, Processors, World, MetaData, GameEventSystem, Audio (+5 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.16
Nodes (40): ActorCustomization, StreamUserType, binary_fixture(), BinaryParser, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal(), json_buildings() (+32 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 58 - "ScriptableObject"
Cohesion: 0.02
Nodes (84): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+76 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.11
Nodes (29): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingModelDef, ContentError, default_resource_generation_layers(), EnemyDef, EnemyModelSetDef (+21 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (89): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id(), convert_chimney_smoke(), convert_fireworks() (+81 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.08
Nodes (52): RoleEquipmentDef, animation_parameter_name(), archetype_scenes(), asset(), authored_value(), building_placements(), BuildingPlacement, child_technology_guids() (+44 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "StableId"
Cohesion: 0.02
Nodes (317): AnimationTransitionOutcome, GameConfig, GameplayConfig, BTreeMap, BuildingDef, ContentCatalog, EnemySpawnerDef, PassiveResourceContribution (+309 more)

### Community 74 - "Utils"
Cohesion: 0.05
Nodes (18): BuildCostModifier, Color, GameUserType, UserColours, Utils, Pets.Enumerations, Animation, Core (+10 more)

### Community 75 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 76 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 77 - "Objective"
Cohesion: 0.07
Nodes (14): Slider, TextMeshProUGUI, UIRuntimeData, Action, int, Objective, Dictionary, GameObject (+6 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.06
Nodes (70): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+62 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 83 - "PlayerProcessor"
Cohesion: 0.06
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Result"
Cohesion: 0.10
Nodes (28): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), BroadcastEncoder, BroadcastMetrics, configure_amf_quality(), configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), duration_as_micros(), encoder_input_format() (+20 more)

### Community 86 - "world.rs"
Cohesion: 0.15
Nodes (19): authored_foliage_is_deterministic_and_respects_habitat_and_resources(), avalanche_instance_hash(), changing_seed_changes_world_hash(), different_town_seeds_produce_different_resource_and_foliage_layouts(), fnv_mix(), foliage_visual_variant(), foliage_visual_yaw_milliradians(), generated_instance_counts_match_the_sanitized_unity_save_oracle() (+11 more)

### Community 87 - ".new"
Cohesion: 0.09
Nodes (26): bandwidth_test_never_claims_to_be_publicly_live(), BroadcastMetricsSnapshot, BroadcastTarget, closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary() (+18 more)

### Community 88 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 89 - "FoliageProcessor"
Cohesion: 0.06
Nodes (36): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+28 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (37): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+29 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.02
Nodes (101): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ChannelDataInstaller (+93 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "NativeGameAudioRouting"
Cohesion: 0.21
Nodes (6): AutomaticBroadcastStart, NativeGameAudioRouting, pcm16_wav_clip(), pcm16_wav_data(), Default, stream_only_game_audio_is_muted_locally_and_mixed_before_the_monitor()

### Community 96 - "Resource"
Cohesion: 0.05
Nodes (24): DepositResources, ResourceStorageModifier, float, int, Dictionary, float, TradeSettings, int (+16 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (24): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+16 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (76): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+68 more)

### Community 100 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (40): archive_purge_backup_history(), AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes() (+32 more)

### Community 101 - "StateMachine"
Cohesion: 0.07
Nodes (15): PlayerDeathHandler, bool, float, Vector3, StateMachine, string, STSM_HelperBase, bool (+7 more)

### Community 102 - "GameEventProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+9 more)

### Community 103 - "Result"
Cohesion: 0.25
Nodes (7): BinaryParser<'a>, decode_binary(), FnMut, Result, Self, T, LegacyWorldState

### Community 104 - "TwitchUser"
Cohesion: 0.22
Nodes (7): ActivityStatus, bool, float, string, UserType, TwitchUser, Character.Enumerations

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 108 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 109 - "CustomLogHandler"
Cohesion: 0.13
Nodes (11): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Dictionary, DebugSettings (+3 more)

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "generate_world_from_layers"
Cohesion: 0.31
Nodes (9): generate_world_from_layers(), generate_world_with_content_observed(), observed_generation_reports_every_real_stage_without_changing_output(), round_to_even(), FnMut, smooth_noise_step(), unity_terrain_height_curve(), WorldGenerationStage (+1 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 116 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 117 - "tidal_music.rs"
Cohesion: 0.14
Nodes (23): composition_uses_native_equivalents_for_supplied_strudel_features(), cycle_boundary_accumulator_carries_frame_overshoot(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine(), format_number(), intensity_cycle_seconds(), intensity_cycles_per_second(), intensity_smoothing_has_a_fifteen_second_time_constant() (+15 more)

### Community 118 - "DirectBroadcastRuntime"
Cohesion: 0.07
Nodes (57): apply_direct_broadcast_control(), arm_stream_only_readback(), begin_twitch_live_verification(), CadenceTick, camera_targets_primary_window(), capture_direct_broadcast_frame(), cleanup_completed_stream_only_readbacks(), configure_direct_broadcast() (+49 more)

### Community 119 - ".CreateEnumField"
Cohesion: 0.14
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 120 - "String"
Cohesion: 0.22
Nodes (9): AuthorizationEvent, BroadcastPrerequisites, format_minutes_seconds(), Display, Formatter, String, seconds_until_enemy_night(), stream_operator_enemy_status() (+1 more)

### Community 121 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 122 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 123 - "TargetMask"
Cohesion: 0.17
Nodes (8): List, Vector3, TargetableData, Dictionary, List, StationUpdate, TargetFlagHelper, TargetMask

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (58): append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), bounded_history_f32(), build_ingest_url(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), direct_broadcast_log_path() (+50 more)

### Community 129 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "Option"
Cohesion: 0.10
Nodes (39): AtomicBool, AudioFrame, AudioInput, BroadcastController, capture_process_audio(), DirectBroadcastSnapshot, discard_pending_audio(), encode_broadcast_session() (+31 more)

### Community 132 - "UnityAsset"
Cohesion: 0.19
Nodes (32): ArchetypesById, archetype_bounds(), archetype_kind(), building_model_definitions(), building_node_age(), component_field_value(), component_reference_name(), component_reference_names() (+24 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 135 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 136 - "RoleProcessor"
Cohesion: 0.09
Nodes (7): Container, ContainerBuilder, int, List, RoleProcessor, Dictionary, MiscCommands

### Community 137 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 138 - "DirectBroadcastControl"
Cohesion: 0.14
Nodes (12): BroadcastStopDisposition, DirectBroadcastControl, exit_after_broadcast_stops(), operator_window_close_requests_exit(), request_automatic_broadcast_start(), AppExit, MessageReader, MessageWriter (+4 more)

### Community 139 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 140 - "WeatherProcessor"
Cohesion: 0.11
Nodes (13): Container, ContainerBuilder, WeatherProcessor, Color, float, int, VisualEffect, SeasonDataSettings (+5 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "Editor"
Cohesion: 0.04
Nodes (26): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, RoleScriptablesEditor, WindControllerEditor (+18 more)

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
Cohesion: 0.17
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 150 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - ".SetTargetType"
Cohesion: 0.17
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 155 - "stream_operator_chat_controls"
Cohesion: 0.09
Nodes (24): AccessibleNode, moderate_selected_operator_user(), operator_chat_scroll_rows(), Changed, Interaction, MouseWheel, Window, send_operator_chat_message() (+16 more)

### Community 156 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.08
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 159 - "SelectedPlayer"
Cohesion: 0.07
Nodes (7): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedPlayer, List, SelectedPlayerGroup

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 163 - "RoleDataSettings"
Cohesion: 0.08
Nodes (18): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip (+10 more)

### Community 164 - "PlayerInventory"
Cohesion: 0.09
Nodes (13): PlayerInventory, Dictionary, ResourceInventory, bool, int, float, int, Queue (+5 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "RoleHandler"
Cohesion: 0.05
Nodes (20): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+12 more)

### Community 167 - "WindController"
Cohesion: 0.13
Nodes (7): GameObject, MenuItem, EditorHelpers, float, Material, Vector2, WindController

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 170 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 172 - "Target"
Cohesion: 0.07
Nodes (14): STStateMachine.States, Units, Behaviours, Target, Utils.Pooling, Sensors, GridSystem.Partitioning, STStateMachine (+6 more)

### Community 173 - "ShaderRef"
Cohesion: 0.09
Nodes (4): CritterMaterialExtension, GrassMaterialExtension, TreeMaterialExtension, ShaderRef

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "Goal"
Cohesion: 0.12
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 176 - "Access_Dropdown"
Cohesion: 0.06
Nodes (18): Camera, Quaternion, Vector3, ProjectCamera, Access_AADropdown, Access_AODropdown, Access_AutosaveTimerDropdown, Access_CameraAADropdown (+10 more)

### Community 177 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 178 - "TechTreeNode"
Cohesion: 0.13
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 179 - "Option"
Cohesion: 0.14
Nodes (29): animation_take_name(), animator_component(), animator_reference_path(), color_value(), convert_embedded_model_clips(), convert_post_process(), field_array(), field_bool() (+21 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 185 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 188 - "SaveState"
Cohesion: 0.20
Nodes (6): ChannelData, string, Container, PSAccess, bool, SaveState

### Community 190 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+36 more)

### Community 192 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (14): BuildingResourceModelHandler, GameObject, UnityEvent, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType (+6 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "IRuntimeDataScriptable"
Cohesion: 0.10
Nodes (18): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+10 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

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

### Community 203 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "String"
Cohesion: 0.21
Nodes (21): inline_file_id(), parse_blend_tree(), parse_child_references(), parse_conditions(), parse_controller(), parse_layers(), parse_model_material_remaps(), parse_parameters() (+13 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "PlayerSaveData"
Cohesion: 0.12
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 210 - "UserInterface"
Cohesion: 0.06
Nodes (17): InputButton, SharedTypes, int, ChangeTimeStamp, NodeGroup_SO, List, TechTree_SO, DataStructures (+9 more)

### Community 211 - "String"
Cohesion: 0.21
Nodes (17): ImportReport, json_pet_name(), legacy_objective_matches(), legacy_pet_name(), LegacyDecodedSave, LegacyGoal, LegacyObjective, objective_target_matches() (+9 more)

### Community 213 - "stream_town_tools/src/main.rs"
Cohesion: 0.07
Nodes (63): AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authority_tab(), broadcast_encoder_label(), canonical_preview_node_name(), checked_in_authoring_assets_pass_headless_validation(), debug_fingerprint(), default_catalog_path() (+55 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "drive_tidal_music"
Cohesion: 0.28
Nodes (15): drive_tidal_music(), intensity_program_needs_update(), IntensitySongInput, report_once(), NativeAudioRouting, Option, Res, ResMut (+7 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - ".RecalculateStats"
Cohesion: 0.26
Nodes (3): StatModifiers, Dictionary, StatType

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "BuildingDataSettings"
Cohesion: 0.15
Nodes (10): Dictionary, BuildingDataContainer, int, ResourceCostData, AllBuildingDataSettings, bool, float, Sprite (+2 more)

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

### Community 234 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

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

### Community 243 - ".write"
Cohesion: 0.29
Nodes (8): PlayerSettingsStore, PlayerSettingsStoreError, Into, Path, PathBuf, Result, SpannedError, store_recovers_last_valid_backup()

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

### Community 251 - "UIElementWrapper"
Cohesion: 0.07
Nodes (15): GameObject, List, PresetButtons, Access_AOToggle, Access_ChannelNameInput, ContainerBuilder, Access_EdgeScrollingToggle, Access_MouseControlsToggle (+7 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "STSM_Idle_Player"
Cohesion: 0.06
Nodes (16): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_GatherResource (+8 more)

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (5): int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "roles_tab"
Cohesion: 0.26
Nodes (14): apply_role_draft(), delete_selected_role(), duplicate_selected_role(), legacy_roles_tab(), refresh_role_draft(), role_draft(), role_editor_applies_every_reference_family_without_partial_mutation(), role_i32() (+6 more)

### Community 268 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

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

### Community 273 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "convert"
Cohesion: 0.23
Nodes (13): ActorKind, actor_prefix(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), duration_days(), entity_id() (+5 more)

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - ".default"
Cohesion: 0.05
Nodes (70): apply_building_draft(), apply_preview_material_overrides(), apply_preview_node_visibility(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, drive_model_preview_animation(), foliage_editor_rejects_invalid_generation_values_without_mutation() (+62 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.13
Nodes (15): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+7 more)

### Community 287 - "import_save"
Cohesion: 0.29
Nodes (11): absolute_path(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), decode_legacy(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save() (+3 more)

### Community 289 - "TechNodeData"
Cohesion: 0.27
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

### Community 290 - "PlayerSettings"
Cohesion: 0.24
Nodes (10): AudioMixSettings, CameraSettings, PlayerSettings, Default, authored_color_grading(), color_grading_for_state(), player_msaa(), player_music_gain() (+2 more)

### Community 291 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 292 - "update_enemy_music_intensity"
Cohesion: 0.24
Nodes (10): point_inside_viewport(), position_is_onscreen(), Camera, GlobalTransform, Query, Time, Vec2, Vec3 (+2 more)

### Community 293 - "GridProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 297 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), ErasedRenderAssets, GpuImage, GpuRenderAssets, PipelineCache, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 298 - "cached_gltf_metadata"
Cohesion: 0.40
Nodes (6): cached_gltf_metadata(), discover_texture_assets(), GltfMetadata, import_texture_asset(), inspect_gltf_asset(), texture_discovery_and_gltf_metadata_are_typed_project_assets()

### Community 299 - "ToolState"
Cohesion: 0.08
Nodes (75): BTreeMap, RuntimeConsoleStatus, add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft() (+67 more)

### Community 300 - "draw_world_preview"
Cohesion: 0.25
Nodes (9): EnemyCampGenerationDef, draw_world_preview(), preview_grid_point(), preview_lerp_color(), Color32, Pos2, Rect, terrain_preview_color() (+1 more)

### Community 301 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 302 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 303 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 304 - "player_window_mode"
Cohesion: 0.29
Nodes (8): DisplayMode, PostProcessAntiAliasing, Option, UnitySettingsData, VideoSettings, player_window_mode(), startup_window_mode(), WindowMode

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+4 more)

### Community 307 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 308 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 309 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 310 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 315 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 318 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 319 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 321 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 322 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 323 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 324 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

### Community 325 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 327 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 328 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 329 - "UIRoleDisplay"
Cohesion: 0.50
Nodes (3): Image, TextMeshProUGUI, UIRoleDisplay

## Knowledge Gaps
- **394 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+389 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `EnemySpawner`, `ScriptablesProcessorInfrastructure`, `Editor`, `SimpleScreenShot`, `TechTree.Elements`, `World.Generation.Settings`, `GenerationSettings`, `Target`, `LabelDisplayProcessor`, `Processors`, `Easings`, `ScriptableObject`, `CommonEnums.cs`, `SimpleRotateOnAxis`, `RandomEnabler`, `SimpleDisableAfterTime`, `BuildingPlacer`, `UserInterface`, `StringUtils`, `SnapToGridMouseMovement`, `FPSDisplay`, `MonoBehaviour`, `BuildingDataSettings`, `UpdateGraphBounds`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `BTreeMap`, `stream_town_game/src/lib.rs`, `simulation.rs`, `roles_tab`, `config.rs`, `save.rs`, `.new`, `command.rs`, `twitch.rs`, `convert`, `.new`, `String`, `.default`, `String`, `Res`, `RenderAssets`, `Vec`, `AnimationControllerDef`, `draw_world_preview`, `ToolState`, `Result`, `Option`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/presentation.rs`, `stream_town_migrate/src/content.rs`, `technology_graph.rs`, `String`, `String`, `stream_town_tools/src/main.rs`, `world.rs`, `stream_town_domain/src/presentation.rs`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `PlayerRoleData`, `BuildingProcessor`, `EnemySpawner`, `NewKingVote`, `TwitchChatProcessor`, `RoleProcessor`, `TimeProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `IProcessor`, `WorldGenProcessor`, `SelectedPlayer`, `RoleHandler`, `TechTreeProcessor`, `Target`, `UIProcessor`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UserInterface_TownVote`, `BuildingPlacer`, `.RecalculateStats`, `RaidEvent`, `ObjectPoolingProcessor`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _394 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PlayerRoleData` be split into smaller, more focused modules?**
  _Cohesion score 0.1368421052631579 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.04013157894736842 - nodes in this community are weakly interconnected._
- **Should `BTreeMap` be split into smaller, more focused modules?**
  _Cohesion score 0.0975177304964539 - nodes in this community are weakly interconnected._