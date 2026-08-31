# Graph Report - Stream-Town-Bevy  (2026-09-01)

## Corpus Check
- 671 files · ~1,811,859 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9359 nodes · 27976 edges · 319 communities (292 shown, 27 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1056 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `cf572fe8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- retargeted_animation_clip
- BuildingProcessor
- BTreeMap
- SeasonProcessor
- Query
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- TimeProcessor
- BottomBarInterface
- GUIDProcessor
- SettingsProcessor
- UserInterface_Debug
- config.rs
- FoliageGenerationSettings
- TechTreeIOUtility
- HealthHandler
- .count
- save.rs
- .new
- command.rs
- twitch.rs
- NavGrid
- STSM_Idle_Player
- Option
- Result
- Res
- Node_SO
- simulation.rs
- SaveFileData
- PlayerProcessor
- ResMut
- WorldGenProcessor
- CellSpacePartitioning
- update_vote_panels
- GenerationSettings
- TargetableHealth
- World.Generation.Settings
- SettingsData
- Vec4
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- VideoCadence
- AnimationControllerDef
- MainMenuManager
- legacy.rs
- StableId
- ResourceDataSaveData
- stream_town_migrate/src/main.rs
- AudioHandler
- StreamTownSessionBridge
- GlobalAudioController
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Value
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
- DayAndNightProcessor
- GameStateProcessor
- SelectedObject
- .Log
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- UIElementWrapper
- Access_Text
- Twitch/Utils/Utils.cs
- TechnologyGraphLayout
- .new
- PlayerSettings
- FoliageProcessor
- SnapToGridMouseMovement
- GameEvent
- generate_world_from_layers
- Ui
- convert_fbx_to_glb.py
- .EnsureValidCredentials
- Resource
- SaveProcessor
- GridNode
- stream_town_domain/src/presentation.rs
- stream_town_tools/src/main.rs
- StateMachine
- Editor
- TownGoalProcessor
- PlayerInputProcessor
- ResourceProcessor
- LoadingManager
- UnityAsset
- LabelDisplayProcessor
- CustomLogHandler
- LevelHandler
- ResourceHolder
- TargetSensor
- Result
- ScriptablesEditor
- PlayerRoleData
- .default
- String
- ResourceStorageModifier
- TechTreeNode
- VoteEvent
- unity_color_filter
- RoleProcessor
- IProcessor
- AIPath
- Targetable
- GateController
- direct_broadcast.rs
- WindController
- Coordinator
- Option
- TL_Secrets
- EnemySpawner
- UserInterface_RulerVote
- TechTree.Elements
- Self
- SensorProcessor
- sync_stream_only_capture
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- UserInterface_TownVote
- RuntimeData Template
- Key Rules
- MiscCommands
- DontDestroyOnLoad
- xtask/src/lib.rs
- SelectedPlayer
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- World.Generation
- .UserIsSubscribed
- String
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- world.rs
- WeatherProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_ObjectSelection
- TransformSaveData
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- SimpleMusicController
- Q: There are still no animations.
- xtask/src/main.rs
- RoleDataSettings
- Target
- SelectableObject
- CommandDictionary
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- Access_Toggle
- PlayerControls
- generate_shoreline_resources
- stream_town_migrate/src/presentation.rs
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- VfxAnimationController
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- CommonEnums.cs
- .Draw
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- SelectedResource
- IRuntimeDataScriptable
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SimpleDisableAfterTime
- Processors
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- string
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- BuildingDamageMaterialHandler
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- Easings
- .new
- capture_direct_broadcast_frame
- DirectBroadcastRuntime
- WorldInstanceDeterminism
- TerrainGenSettings
- Key Rules
- ParallelProgressReporter
- RuntimeData Template
- Character Animation Regression Checklist
- ObjectSelectionProcessor.Editor.cs
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
- GridProcessor
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
- Vec
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- STSM_HelperBase
- Q: If there is more to do, keep going.
- UI_TechOption
- ResourceDataSettings
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- UnityGraphics
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- EventProcessor
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
- Sensors
- SimpleScreenShot
- CampGenerationSettings
- Requirement
- OpenNode
- TL_API
- CreateProjectScopeProcessors.cs
- resolve_broadcast_target
- SelectedEnemy
- append_direct_broadcast_diagnostic
- IntWrapper
- ToolState
- ScriptablesProcessorInfrastructure
- PostProcessingInstaller
- TwitchClientRuntimeData
- RenderPipelineInstaller
- PlayerSaveData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- PlacementProbeHandler
- Node_SO.cs
- StyleUtility
- VFX
- Autosave
- .snapshot
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- DataStructures
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
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
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generated_terrain_chunks_cover_the_grid_with_watertight_seams()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (319 total, 27 thin omitted)

### Community 0 - "retargeted_animation_clip"
Cohesion: 0.14
Nodes (27): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), animation_target_for_track(), authored_player_run_clip_loop_pose_correction_closes_the_seam(), close_rotation_loop() (+19 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.03
Nodes (52): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, TilerBuilding (+44 more)

### Community 2 - "BTreeMap"
Cohesion: 0.09
Nodes (51): MaterialDef, PrefabPresentationBinding, RendererMaterialBinding, BTreeMap, TextureDef, animator_component(), animator_reference_path(), array_index() (+43 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.08
Nodes (19): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+11 more)

### Community 4 - "Query"
Cohesion: 0.04
Nodes (168): Added, AnimatedBy, AnimationGraphHandle, AudioSink, ActivePetVisual, ActorAnimationDriver, ActorHealthOverlay, ActorNameOverlay (+160 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (266): accessibility_motion_preferences_preserve_authored_parameters(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, action_ranges_and_tower_acquisition_are_euclidean(), actor_detail_budget(), ActorHealthFill (+258 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "GUIDProcessor"
Cohesion: 0.14
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.11
Nodes (28): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet (+20 more)

### Community 13 - "FoliageGenerationSettings"
Cohesion: 0.06
Nodes (26): Mesh, Vector3, int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData (+18 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (12): Action, float, Enemy, int, STSM_Helper_Attack, Action, bool, float (+4 more)

### Community 16 - ".count"
Cohesion: 0.04
Nodes (89): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), apply_authored_main_menu_camera(), asset_root_collection_ready(), authored_main_menu_mesh() (+81 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - ".new"
Cohesion: 0.03
Nodes (226): AccessibilityActionRequest, AccessibleNode, DirtyRegion, GridPos, generate_world(), generate_world_with_content(), GeneratedFoliage, GeneratedResource (+218 more)

### Community 19 - "command.rs"
Cohesion: 0.06
Nodes (57): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+49 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (64): SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_oauth_identity() (+56 more)

### Community 21 - "NavGrid"
Cohesion: 0.17
Nodes (15): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building(), reconstruct_path() (+7 more)

### Community 22 - "STSM_Idle_Player"
Cohesion: 0.08
Nodes (9): STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint, Vector3 (+1 more)

### Community 23 - "Option"
Cohesion: 0.03
Nodes (139): AnimationTransitionOutcome, AnimationTransitionPlayback, ArchetypeDef, ArchetypeKind, ArchetypeScene, PresentationCatalog, RulerVoteKind, ActiveMaterialHandles (+131 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - "Res"
Cohesion: 0.05
Nodes (180): AmbientLight, AnimationTransitions, actor_material(), actor_scene_budget(), agent_is_moving(), AgentAnimation, animate_agents(), animate_building_effects() (+172 more)

### Community 26 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 27 - "simulation.rs"
Cohesion: 0.06
Nodes (42): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value() (+34 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "PlayerProcessor"
Cohesion: 0.06
Nodes (17): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+9 more)

### Community 30 - "ResMut"
Cohesion: 0.04
Nodes (147): AccessibilityFocusVisualQuery, AnyResult, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibilityAnnouncement, AccessibilityRuntime (+139 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 33 - "update_vote_panels"
Cohesion: 0.07
Nodes (69): accessibility_settings_selection(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), current_event_texture(), GoLiveConfirmationBody, GoLiveConfirmationRoot (+61 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 36 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "Vec4"
Cohesion: 0.06
Nodes (35): AccessibilityMotionDefaults, BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension (+27 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.05
Nodes (19): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+11 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "VideoCadence"
Cohesion: 0.21
Nodes (7): duration_as_micros(), Duration, Error, twitch_live_request_timeout(), VideoCadence, CapturedWindowFrame, InternalCaptureControl

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+18 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "legacy.rs"
Cohesion: 0.11
Nodes (47): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id() (+39 more)

### Community 47 - "StableId"
Cohesion: 0.03
Nodes (195): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, Display, FromStr, StableId, ActorState (+187 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (15): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+7 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "GlobalAudioController"
Cohesion: 0.19
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.10
Nodes (21): Bounds, Color, Component, GameObject, HashSet, List, MenuItem, Object (+13 more)

### Community 56 - "Value"
Cohesion: 0.20
Nodes (26): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+18 more)

### Community 57 - "Station"
Cohesion: 0.07
Nodes (18): Station, Dictionary, float, int, List, Queue, Transform, Container (+10 more)

### Community 58 - "MonoBehaviour"
Cohesion: 0.02
Nodes (103): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+95 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "String"
Cohesion: 0.14
Nodes (35): ability_choices(), action_animation_choices(), add_archetype_scene(), archetype_kind_choice(), building_model_node_choices(), cached_gltf_metadata(), content_tab_contents(), create_model_archetype() (+27 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (48): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, default_resource_generation_layers(), EnemyCampGenerationDef (+40 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "AnimationHandler"
Cohesion: 0.04
Nodes (29): AnimationHandler, Animator, bool, Dictionary, float, int, RotationHandler, float (+21 more)

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
Cohesion: 0.14
Nodes (8): bool, Dictionary, float, Transform, Pet, Animator, int, PetModel

### Community 68 - "GameEventProcessor"
Cohesion: 0.11
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.12
Nodes (36): asset(), authored_mask(), building_model_definitions(), building_node_age(), component(), component_at(), component_reference_name(), component_reference_names() (+28 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.12
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 74 - "GameStateProcessor"
Cohesion: 0.15
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 75 - "SelectedObject"
Cohesion: 0.10
Nodes (6): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup

### Community 76 - ".Log"
Cohesion: 0.04
Nodes (45): bool, List, ObjectPoolingSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor (+37 more)

### Community 77 - "Objective"
Cohesion: 0.08
Nodes (13): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+5 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.12
Nodes (38): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+30 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.10
Nodes (20): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+12 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 83 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Twitch/Utils/Utils.cs"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 86 - "TechnologyGraphLayout"
Cohesion: 0.12
Nodes (30): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap, Default (+22 more)

### Community 87 - ".new"
Cohesion: 0.10
Nodes (21): closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting(), NativeGameAudioRouting, operator_panel_uses_compact_telemetry_and_bottom_left_live_control() (+13 more)

### Community 88 - "PlayerSettings"
Cohesion: 0.06
Nodes (67): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+59 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "GameEvent"
Cohesion: 0.05
Nodes (24): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+16 more)

### Community 92 - "generate_world_from_layers"
Cohesion: 0.18
Nodes (23): WorldGenConfig, authored_grid_centre(), authored_world_to_grid(), foliage_horizontal_hash(), generate_authored_resources(), generate_candidate_mask(), generate_foliage(), generate_world_from_layers() (+15 more)

### Community 93 - "Ui"
Cohesion: 0.10
Nodes (61): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+53 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 96 - "Resource"
Cohesion: 0.04
Nodes (37): DepositResources, PlayerInventory, Dictionary, ResourceInventory, bool, int, Dictionary, float (+29 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (27): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+19 more)

### Community 98 - "GridNode"
Cohesion: 0.09
Nodes (14): GridProcessorEditor, GridSystemEditor, int, List, Vector2, GridNode, Color, CollisionColours (+6 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (74): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+66 more)

### Community 100 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (63): animation_property_curves_editor(), apply_building_draft(), AssetEditorSection, broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, canonical_preview_node_name() (+55 more)

### Community 101 - "StateMachine"
Cohesion: 0.10
Nodes (12): PlayerDeathHandler, bool, float, Vector3, bool, List, string, uint (+4 more)

### Community 102 - "Editor"
Cohesion: 0.06
Nodes (12): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, RoleScriptablesEditor (+4 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "UnityAsset"
Cohesion: 0.15
Nodes (36): ArchetypesById, RotatingNodeDef, archetype_bounds(), archetype_kind(), component_field_value(), component_type(), convert_archetypes(), converted_rotating_axis() (+28 more)

### Community 108 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.13
Nodes (11): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Dictionary, DebugSettings (+3 more)

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "ResourceHolder"
Cohesion: 0.09
Nodes (17): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent (+9 more)

### Community 112 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 113 - "Result"
Cohesion: 0.25
Nodes (31): aged_buildings(), authored_value(), building_cost_reductions(), building_level_caps(), convert_export(), enemy_camp_generation_layers(), generated_record_ids(), insert_source_record() (+23 more)

### Community 114 - "ScriptablesEditor"
Cohesion: 0.06
Nodes (20): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+12 more)

### Community 115 - "PlayerRoleData"
Cohesion: 0.11
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, StatType

### Community 116 - ".default"
Cohesion: 0.11
Nodes (29): authority_tab(), character_model_choices_include_converted_hierarchy_nodes(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_presentation_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation() (+21 more)

### Community 117 - "String"
Cohesion: 0.11
Nodes (25): animation_parameter_name(), archetype_scenes(), building_placements(), BuildingPlacement, child_technology_guids(), collect_model_dependencies(), ContentConversionReport, convert() (+17 more)

### Community 118 - "ResourceStorageModifier"
Cohesion: 0.13
Nodes (7): BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, UnityEvent, StorageStatus

### Community 119 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 120 - "VoteEvent"
Cohesion: 0.11
Nodes (11): int, List, NewKingVote, PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent (+3 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "RoleProcessor"
Cohesion: 0.08
Nodes (13): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+5 more)

### Community 123 - "IProcessor"
Cohesion: 0.12
Nodes (12): Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor (+4 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 126 - "Targetable"
Cohesion: 0.08
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (68): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), bandwidth_test_url_is_constructed_without_logging_the_key(), bounded_history_f32(), build_ingest_url(), CadenceTick, configure_amf_quality(), draw_centered_label(), ingest() (+60 more)

### Community 129 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 130 - "Coordinator"
Cohesion: 0.07
Nodes (21): Coordinator, StartupState, Action, bool, CancellationToken, CancellationTokenSource, Container, Dictionary (+13 more)

### Community 131 - "Option"
Cohesion: 0.11
Nodes (39): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, capture_process_audio(), DirectBroadcastSnapshot, discard_pending_audio() (+31 more)

### Community 132 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "UserInterface_RulerVote"
Cohesion: 0.13
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.08
Nodes (19): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+11 more)

### Community 136 - "Self"
Cohesion: 0.17
Nodes (7): loading_progress_is_recursively_derived_from_real_work(), LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, Self, WorldLoadingWork

### Community 137 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 138 - "sync_stream_only_capture"
Cohesion: 0.15
Nodes (19): camera_targets_primary_window(), operator_restart_button_requests_a_stream_restart(), Assets, BackgroundColor, Commands, Entity, Handle, HashMap (+11 more)

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

### Community 143 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "MiscCommands"
Cohesion: 0.16
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 147 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.12
Nodes (18): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+10 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 154 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 155 - "String"
Cohesion: 0.08
Nodes (64): AnimationParameterDef, animation_take_name(), clip_id(), convert_clips(), convert_embedded_model_clips(), convert_post_process(), embedded_clip_id(), field_bool() (+56 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.13
Nodes (12): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+4 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "world.rs"
Cohesion: 0.20
Nodes (15): avalanche_instance_hash(), changing_seed_changes_world_hash(), fnv_mix(), foliage_visual_variant(), foliage_visual_yaw_milliradians(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), lerp() (+7 more)

### Community 159 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 163 - "TransformSaveData"
Cohesion: 0.13
Nodes (12): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+4 more)

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

### Community 170 - "RoleDataSettings"
Cohesion: 0.16
Nodes (11): Dictionary, int, RoleDataContainer, AllRoleDataSettings, AudioClip, bool, float, int (+3 more)

### Community 171 - "Target"
Cohesion: 0.10
Nodes (10): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, Environment, SavingAndLoading.SavableObjects (+2 more)

### Community 172 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 173 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 177 - "PlayerControls"
Cohesion: 0.15
Nodes (4): InputButton, UserInterface.MainMenu, MetaData, PlayerControls

### Community 178 - "generate_shoreline_resources"
Cohesion: 0.24
Nodes (13): cell_hash(), generate_shoreline_resources(), hash_world(), horizontal_hash(), legacy_resource_navigation(), legacy_v1_world_hash(), legacy_v2_world_hash(), legacy_v3_world_hash() (+5 more)

### Community 179 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (76): AvatarMaskDef, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert_avatar_masks(), convert_chimney_smoke() (+68 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "VfxAnimationController"
Cohesion: 0.04
Nodes (22): Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, Image, TextMeshProUGUI, UIRoleDisplay (+14 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "CommonEnums.cs"
Cohesion: 0.10
Nodes (18): Vector3, TargetSettings, TargetableData, Dictionary, List, Foliage, FoliageSaveType, FoliageType (+10 more)

### Community 185 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.03
Nodes (69): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+61 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 190 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (15): CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, Dictionary, InputButton, PlayerInput (+7 more)

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+36 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 195 - "Processors"
Cohesion: 0.06
Nodes (19): TownGoal.Data, Processors, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Character.Enumerations, Core, World (+11 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "string"
Cohesion: 0.20
Nodes (12): bool, int, long, string, NeutralComponent, NeutralExport, NeutralField, NeutralGameObject (+4 more)

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
Cohesion: 0.13
Nodes (17): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+9 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - ".new"
Cohesion: 0.31
Nodes (6): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), generated_instance_counts_match_the_sanitized_unity_save_oracle(), positive_noise_offset(), Self, SystemRandom

### Community 210 - "capture_direct_broadcast_frame"
Cohesion: 0.22
Nodes (8): capture_direct_broadcast_frame(), gpu_readback_padding_is_removed_without_corrupting_rows(), publish_stream_only_frame(), remove_gpu_row_padding(), On, Time, SensitiveScreenActive, ReadbackComplete

### Community 211 - "DirectBroadcastRuntime"
Cohesion: 0.08
Nodes (40): apply_direct_broadcast_control(), AuthorizationEvent, bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastPrerequisites, BroadcastStopDisposition, BroadcastTarget, DirectBroadcastControl (+32 more)

### Community 212 - "WorldInstanceDeterminism"
Cohesion: 0.30
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 213 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

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

### Community 221 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

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

### Community 234 - "GridProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, GridProcessor

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
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "RoleHandler"
Cohesion: 0.07
Nodes (18): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+10 more)

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "Vec"
Cohesion: 0.43
Nodes (8): binary_fixture(), BinaryParser, put_f32(), put_i32(), put_string(), put_u32(), Vec, Cursor

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

### Community 268 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

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

### Community 276 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

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
Cohesion: 0.08
Nodes (53): apply_preview_material_overrides(), apply_preview_node_visibility(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene, player_preview_material_overrides() (+45 more)

### Community 285 - "Stream Town Twitch commands"
Cohesion: 0.40
Nodes (4): Moderator and game-master commands, Player commands, Ruler and operator commands, Stream Town Twitch commands

### Community 287 - "Utils"
Cohesion: 0.06
Nodes (7): BuildCostModifier, Utils, Level, Buildings, SavingAndLoading, SavingAndLoading.Structs, GameResources

### Community 288 - "Sensors"
Cohesion: 0.07
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 289 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 290 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 291 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 292 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "resolve_broadcast_target"
Cohesion: 0.29
Nodes (6): configure_direct_broadcast(), DirectTwitchBroadcastPlugin, resolve_broadcast_target(), App, Plugin, select_ingest()

### Community 297 - "append_direct_broadcast_diagnostic"
Cohesion: 0.22
Nodes (10): append_direct_broadcast_diagnostic(), append_direct_broadcast_diagnostic_to(), average_milliseconds(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), direct_broadcast_log_path(), micros_to_milliseconds(), rate_per_second(), report_stream_health() (+2 more)

### Community 298 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 299 - "ToolState"
Cohesion: 0.08
Nodes (79): apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot (+71 more)

### Community 300 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (6): int, AudioSettings, Reflex.Core, Data.Containers, Settings, ScriptablesProcessorInfrastructure

### Community 301 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 303 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 304 - "PlayerSaveData"
Cohesion: 0.07
Nodes (22): List, PetType, Dictionary, List, SaveDataMapper, bool, int, List (+14 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (13): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+5 more)

### Community 308 - "Node_SO.cs"
Cohesion: 0.40
Nodes (3): InputButton, SharedTypes, Data

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 315 - "DataStructures"
Cohesion: 0.25
Nodes (4): int, ChangeTimeStamp, DataStructures, DateTime

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **383 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+378 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **27 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `EnemySpawner`, `TechTree.Elements`, `World.Generation`, `.CreateEnumField`, `Sensors`, `SimpleScreenShot`, `GenerationSettings`, `UpdateGraphBounds`, `Target`, `ScriptablesProcessorInfrastructure`, `PlayerControls`, `AudioHandler`, `Node_SO.cs`, `VfxAnimationController`, `CommonEnums.cs`, `MonoBehaviour`, `SimpleDisableAfterTime`, `Processors`, `BuildingPlacer`, `Easings`, `Twitch/Utils/Utils.cs`, `SnapToGridMouseMovement`, `FPSDisplay`, `Editor`, `LabelDisplayProcessor`, `ScriptablesEditor`, `RandomEnabler`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `retargeted_animation_clip`, `BTreeMap`, `Query`, `stream_town_game/src/lib.rs`, `config.rs`, `.count`, `save.rs`, `.new`, `command.rs`, `twitch.rs`, `Option`, `Res`, `simulation.rs`, `String`, `world.rs`, `ToolState`, `AnimationControllerDef`, `legacy.rs`, `stream_town_migrate/src/presentation.rs`, `String`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `technology_graph.rs`, `TechnologyGraphLayout`, `Ui`, `stream_town_domain/src/presentation.rs`, `stream_town_tools/src/main.rs`, `UnityAsset`, `Result`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `RoleHandler`, `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `TimeProcessor`, `BottomBarInterface`, `GUIDProcessor`, `UserInterface_Debug`, `HealthHandler`, `UserInterface_TownVote`, `SelectedPlayer`, `WorldGenProcessor`, `TechTreeProcessor`, `Target`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `GameEventProcessor`, `SelectedObject`, `.Log`, `BuildingPlacer`, `GameEvent`, `Resource`, `SaveProcessor`, `PlayerRoleData`, `VoteEvent`, `RoleProcessor`, `IProcessor`?**
  _High betweenness centrality (0.023) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _383 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `retargeted_animation_clip` be split into smaller, more focused modules?**
  _Cohesion score 0.1396011396011396 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.027950675278919555 - nodes in this community are weakly interconnected._
- **Should `BTreeMap` be split into smaller, more focused modules?**
  _Cohesion score 0.08862745098039215 - nodes in this community are weakly interconnected._