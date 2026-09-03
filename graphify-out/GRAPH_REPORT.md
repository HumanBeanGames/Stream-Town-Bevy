# Graph Report - Stream-Town-Bevy  (2026-09-04)

## Corpus Check
- 672 files · ~1,840,287 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9701 nodes · 29726 edges · 320 communities (295 shown, 25 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1063 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c37f4554`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ContentCatalog
- BuildingProcessor
- stream_town_migrate/src/content.rs
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
- World.Generation.Settings
- CharacterModelHandler
- command.rs
- twitch.rs
- NavGrid
- IRuntimeDataScriptable
- SettingsProcessor.cs
- TechnologyGraphLayout
- Node_SO
- PlayerSaveData
- TechTree.Elements
- SaveFileData
- Ui
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
- Option
- AnimationControllerDef
- MainMenuManager
- UIProcessor
- UnityAsset
- ResourceDataSaveData
- Targetable
- AudioHandler
- StreamTownSessionBridge
- VfxSeagullSpawner
- TwitchClientProcessor
- PlayerProcessor
- .SerializeComponent
- legacy.rs
- BevyMigrationExporter
- ScriptablesProcessorInfrastructure
- TechTreeEditorWindow
- DayAndNightProcessor
- stream_town_domain/src/content.rs
- CameraController
- stream_town_migrate/src/presentation.rs
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- .new
- GamestateJukebox
- begin_world_loading
- models.rs
- Tiler
- runtime_console.rs
- Utils
- component_field_value
- CommonEnums.cs
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- PlayerRoleData
- Access_Text
- encode_broadcast_session
- generate_world_from_layers
- SensorProcessor
- Goal
- FoliageProcessor
- SnapToGridMouseMovement
- GameEvent
- ObjectPoolingProcessor
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
- TechTree_SO
- TwitchUser
- ResourceProcessor
- LoadingManager
- LabelDisplayProcessor
- UnitHealthBar
- CustomLogHandler
- SelectedBuilding
- ProjectCamera
- UpdateGraphBounds
- GlobalAudioController
- ScriptablesEditor
- world.rs
- Editor
- tidal_music.rs
- STSM_StateAction
- .CreateEnumField
- VoteEvent
- TechTreeGraphView
- drive_tidal_music
- DebugProcessor
- AIPath
- PlayerInputProcessor
- GateController
- direct_broadcast.rs
- .EnsureValidCredentials
- Coordinator
- String
- SelectedObject
- EnemySpawner
- stream_town_domain/src/lib.rs
- GUIDComponent
- NativeGameAudioRouting
- TargetSensor
- Option
- TechnologyTreeGroup
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
- .default
- ResourceRuntimeData
- DirectBroadcastRuntime
- SelectableObject
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- UserInterface_ObjectSelection
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_RulerVote
- MiscCommands
- TransformSaveData
- UserInterface_DisplayUsernames
- RoleHandler
- WindController
- Q: There are still no animations.
- stream_town_migrate/src/main.rs
- SimpleMusicController
- EditorHelpers
- stream_town_migrate/src/technology_layout.rs
- Handle
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- STSM_Idle
- Access_Dropdown
- List
- .Draw
- String
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- EditorUtils
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Easings
- WorldInstanceDeterminism
- WorldGenerationReferenceExporter
- TL_Secrets
- .RenderResourceType
- IProcessor.cs
- UserInterface_BuildingHealthBar
- stream_town_migrate/src/menu_scene.rs
- Access_GOList
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- .UserIsSubscribed
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- .new
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- ResourceHolder
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- SelectedEnemy
- Common Patterns
- GameStateProcessor
- hash_world
- SelectedResource
- LoadingWorkNode
- StringUtils
- matching_role_animation_state
- Key Rules
- STSM_HelperBase
- RuntimeData Template
- Character Animation Regression Checklist
- Target
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
- update_stream_operator_chat
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
- .DrawDataFieldAndLabel
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- BuildingModelHandler
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- Option
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- EventProcessor
- Q: If there is more to do, keep going.
- Access_TextInput
- draw_world_preview
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- SelectedEnemyCamp
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Enemy
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
- OpenNode
- StatusBar
- CreateDefaultSettingsAssets.cs
- SimpleDisableAfterTime
- .ExportModification
- PlayerInputRuntimeData
- TL_API
- stream_town_tools/src/main.rs
- PlacementProbeHandler
- ScriptableObjectAssetData
- PooledObjectData
- FoliageGenerationSettings
- embedded_presentation
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- tidal_plugin
- ChangeTimeStamp
- Autosave
- TwitchClientRuntimeData
- .SetGroupSelectionArea
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .SetTargetType
- vcpkg.json
- Station
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
- `falling_fish_uses_authored_gravity_terrain_bounce_and_lifetime_loss()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (320 total, 25 thin omitted)

### Community 0 - "ContentCatalog"
Cohesion: 0.09
Nodes (65): ContentCatalog, RoleDef, ActorState, String, action_animation_speed(), action_cooldown(), actor_accepts_resource(), actor_archetype() (+57 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (37): BuildingBase, bool, float, int, List, UnityEvent, TilerBuilding, bool (+29 more)

### Community 2 - "stream_town_migrate/src/content.rs"
Cohesion: 0.11
Nodes (37): PassiveResourceContribution, asset(), authored_value(), building_placements(), BuildingPlacement, component(), component_at(), converts_active_catalog_references_and_round_trips_ron() (+29 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 4 - "Res"
Cohesion: 0.02
Nodes (214): AccessibilityFocusVisualQuery, AccessibilityNode, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibilityAnnouncement, AccessibilityRuntime (+206 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (350): accessibility_settings_selection(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, ActionPresentation, active_event_text(), actor_detail_budget(), actor_name_color() (+342 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.04
Nodes (31): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData, bool, float (+23 more)

### Community 7 - ".new"
Cohesion: 0.02
Nodes (313): AccessibilityActionRequest, StationDef, DirtyRegion, GridPos, ActorKind, generate_world(), generate_world_with_content(), GeneratedWorld (+305 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.11
Nodes (27): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig (+19 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.10
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.11
Nodes (11): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+3 more)

### Community 16 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 17 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 18 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 19 - "command.rs"
Cohesion: 0.17
Nodes (34): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+26 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (65): BTreeSet, TwitchConfig, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization (+57 more)

### Community 21 - "NavGrid"
Cohesion: 0.15
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), calculate_topology_signature(), can_plan_for_three_hundred_agents(), diagonal_steps_do_not_cut_blocked_corners(), grid(), NavGrid, NavigationError, neighbour_candidates() (+13 more)

### Community 22 - "IRuntimeDataScriptable"
Cohesion: 0.10
Nodes (18): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+10 more)

### Community 23 - "SettingsProcessor.cs"
Cohesion: 0.15
Nodes (8): CameraProcessor, Vector3, UnityGraphics, URP, MetaData, Settings, FieldInfo, ShadowResolution

### Community 24 - "TechnologyGraphLayout"
Cohesion: 0.12
Nodes (24): ContentError, EnemyDef, PetDef, ResourceReward, BTreeMap, BTreeSet, Result, TechNode (+16 more)

### Community 25 - "Node_SO"
Cohesion: 0.14
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 26 - "PlayerSaveData"
Cohesion: 0.10
Nodes (16): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int (+8 more)

### Community 27 - "TechTree.Elements"
Cohesion: 0.09
Nodes (17): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+9 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Ui"
Cohesion: 0.08
Nodes (85): ArchetypeKind, ability_choices(), action_animation_choices(), animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice() (+77 more)

### Community 30 - "Query"
Cohesion: 0.03
Nodes (220): Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver, ActorNameOverlay (+212 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "PathBuf"
Cohesion: 0.08
Nodes (33): AnyResult, automatic_resume_save_path(), is_jump_start_path(), jump_start_saves_are_identifiable_and_never_reuse_an_existing_path(), jump_start_snapshot_path(), jump_start_working_path(), legacy_unity_settings_path(), load_player_settings() (+25 more)

### Community 33 - "save.rs"
Cohesion: 0.06
Nodes (78): actor_state(), corruption_recovery_checks_older_backup_generations(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind (+70 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 36 - "PlayerSettings"
Cohesion: 0.11
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "RenderAssets"
Cohesion: 0.04
Nodes (185): GameConfig, ArchetypeDef, ArchetypeScene, PresentationCatalog, actor_material(), actor_scene_budget(), advance_falling_fish(), agent_facing_matches_unity_rotation_and_action_targets() (+177 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "StableId"
Cohesion: 0.04
Nodes (82): ObjectiveDef, Display, FromStr, StableId, ActorCustomization, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase() (+74 more)

### Community 43 - "Option"
Cohesion: 0.03
Nodes (120): AnimationClip, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve(), advance_animation_crossfade() (+112 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (33): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+25 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 47 - "UnityAsset"
Cohesion: 0.20
Nodes (42): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), enemy_camp_generation_layers(), field_value(), foliage_layers(), generated_record_ids() (+34 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "Targetable"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 50 - "AudioHandler"
Cohesion: 0.06
Nodes (17): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, ContainerBuilder (+9 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "PlayerProcessor"
Cohesion: 0.06
Nodes (17): StatModifiers, Dictionary, Action, Container, ContainerBuilder, List, Transform, Vector3 (+9 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.07
Nodes (86): SavedTerrainMesh, StreamUserType, absolute_path(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, BinaryParser<'a> (+78 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 58 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (87): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+79 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.11
Nodes (32): AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, default_resource_generation_layers(), EnemyModelSetDef, EnemyRunAnimation, EnemyWeaponModelDef (+24 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (97): AnimationFloatKeyframe, AnimationTangent, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id() (+89 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - ".new"
Cohesion: 0.16
Nodes (18): closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), explicit_automatic_start_uses_the_normal_broadcast_configuration_path_once(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting(), operator_live_button_applies_stop_in_the_pressed_frame() (+10 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "begin_world_loading"
Cohesion: 0.08
Nodes (29): AccessibleNode, AssetId, authored_rotating_node_names(), begin_world_loading(), building_model_node_names(), CommandAcknowledgementRuntime, enemy_model_node_names(), equipment_node_names() (+21 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.12
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 74 - "Utils"
Cohesion: 0.04
Nodes (27): BuildCostModifier, InputButton, DataStructures, TownGoal.Data, Utils, Processors, Pets.Enumerations, StreamTown.EditorTools (+19 more)

### Community 75 - "component_field_value"
Cohesion: 0.16
Nodes (29): ArchetypesById, ArchetypeBounds, EnemySpawnerDef, TargetingScoreDef, WeightedEnemySpawn, archetype_bounds(), archetype_kind(), building_model_definitions() (+21 more)

### Community 76 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (16): List, Vector3, Dictionary, List, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus (+8 more)

### Community 77 - "Objective"
Cohesion: 0.09
Nodes (10): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.12
Nodes (38): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+30 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 83 - "PlayerRoleData"
Cohesion: 0.12
Nodes (8): PlayerRoleData, AudioClip, bool, float, int, List, int, PlayerRoleSaveData

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "encode_broadcast_session"
Cohesion: 0.10
Nodes (35): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, BroadcastMetrics, BroadcastTarget, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), discard_pending_audio() (+27 more)

### Community 86 - "generate_world_from_layers"
Cohesion: 0.18
Nodes (24): WorldGenConfig, ResourceGenerationLayerDef, authored_grid_centre(), authored_world_to_grid(), foliage_horizontal_hash(), generate_authored_resources(), generate_candidate_mask(), generate_foliage() (+16 more)

### Community 87 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 88 - "Goal"
Cohesion: 0.13
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "GameEvent"
Cohesion: 0.04
Nodes (27): SortBuildingByLowerLevel, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+19 more)

### Community 92 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (30): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+22 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (117): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, Volume (+109 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "BroadcastController"
Cohesion: 0.13
Nodes (17): AtomicBool, BroadcastController, LiveVerification, LiveVerificationEvent, publish_latest_video(), Drop, Error, Mutex (+9 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (39): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+31 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.05
Nodes (33): Container, ContainerBuilder, GUIDProcessor, Action, CancellationToken, Component, Container, ContainerBuilder (+25 more)

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
Cohesion: 0.08
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 103 - "TechTree_SO"
Cohesion: 0.14
Nodes (9): int, TechTreeSettings, InputButton, SharedTypes, NodeGroup_SO, List, TechTree_SO, TechTree.ScriptableObjects (+1 more)

### Community 104 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.09
Nodes (24): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+16 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 108 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "SelectedBuilding"
Cohesion: 0.06
Nodes (8): bool, float, PassiveResourceIncrementer, int, UnityEvent, LevelHandler, RoleLevelHandler, SelectedBuilding

### Community 111 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "world.rs"
Cohesion: 0.14
Nodes (20): avalanche_instance_hash(), changing_seed_changes_world_hash(), different_town_seeds_produce_different_resource_and_foliage_layouts(), fnv_mix(), foliage_visual_variant(), foliage_visual_yaw_milliradians(), generated_resources_preserve_unity_target_types_and_reachable_fish(), GeneratedFoliage (+12 more)

### Community 116 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 117 - "tidal_music.rs"
Cohesion: 0.16
Nodes (20): composition_is_one_transitionable_native_expression(), euclidean_steps(), every_supported_intensity_parses_in_the_native_engine(), format_number(), intensity_changes_every_authored_low_pass_filter(), intensity_cycles_per_second(), intensity_makes_the_struck_voice_brighter_sharper_and_not_excessively_louder(), intensity_smoothing_has_a_fifteen_second_time_constant() (+12 more)

### Community 118 - "STSM_StateAction"
Cohesion: 0.10
Nodes (11): int, STSM_Action_Attack, bool, float, Vector3, STSM_Action_EnemyAttack, bool, float (+3 more)

### Community 119 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 120 - "VoteEvent"
Cohesion: 0.10
Nodes (13): List, KeepKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float, IReadOnlyDictionary (+5 more)

### Community 121 - "TechTreeGraphView"
Cohesion: 0.08
Nodes (17): Vector2, GroupSaveData, int, List, Port, Vector2, TechTreeGraphView, List (+9 more)

### Community 122 - "drive_tidal_music"
Cohesion: 0.14
Nodes (25): drive_tidal_music(), intensity_program_needs_update(), IntensitySongInput, point_inside_viewport(), position_is_onscreen(), report_once(), Camera, GlobalTransform (+17 more)

### Community 123 - "DebugProcessor"
Cohesion: 0.05
Nodes (22): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+14 more)

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
Nodes (51): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), append_direct_broadcast_diagnostic_to(), AuthorizationEvent, average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), BroadcastPrerequisites, build_ingest_url(), configure_amf_quality() (+43 more)

### Community 129 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "String"
Cohesion: 0.14
Nodes (22): animation_parameter_name(), archetype_scenes(), authored_mask(), child_technology_guids(), collect_model_dependencies(), ContentConversionReport, convert(), decomposes_combined_unity_flag_values() (+14 more)

### Community 132 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "stream_town_domain/src/lib.rs"
Cohesion: 0.13
Nodes (13): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError (+5 more)

### Community 135 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 136 - "NativeGameAudioRouting"
Cohesion: 0.10
Nodes (21): AudioFrame, AudioInput, capture_process_audio(), NativeGameAudioClip, NativeGameAudioMix, NativeGameAudioRouting, NativeGameAudioRoutingInner, NativeGameAudioState (+13 more)

### Community 137 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 138 - "Option"
Cohesion: 0.07
Nodes (50): camera_targets_primary_window(), cleanup_completed_stream_only_readbacks(), disarm_stream_only_readbacks(), gpu_readbacks_are_published_in_render_order_even_when_they_finish_out_of_order(), moderate_selected_operator_user(), operator_chat_scroll_rows(), Assets, BTreeMap (+42 more)

### Community 139 - "TechnologyTreeGroup"
Cohesion: 0.12
Nodes (12): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, Color, float (+4 more)

### Community 140 - "WeatherProcessor"
Cohesion: 0.10
Nodes (17): float, int, Material, AllSeasonSettings, Container, ContainerBuilder, WeatherProcessor, Color (+9 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GridProcessor"
Cohesion: 0.08
Nodes (16): GridProcessorEditor, int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor (+8 more)

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

### Community 149 - "EnemyModelHandler"
Cohesion: 0.20
Nodes (5): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler

### Community 150 - "VfxAnimationController"
Cohesion: 0.13
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - ".default"
Cohesion: 0.19
Nodes (12): default_ui_scale_percent(), PlayerSettingsStore, rejects_ui_scales_outside_the_supported_readable_range(), Into, Path, PathBuf, Result, Self (+4 more)

### Community 154 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 155 - "DirectBroadcastRuntime"
Cohesion: 0.05
Nodes (54): append_direct_broadcast_diagnostic(), apply_direct_broadcast_control(), arm_stream_only_readback(), AutomaticBroadcastStart, bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastMetricsSnapshot, BroadcastStopDisposition (+46 more)

### Community 156 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 159 - "UserInterface_ObjectSelection"
Cohesion: 0.16
Nodes (12): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+4 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_RulerVote"
Cohesion: 0.10
Nodes (13): int, List, NewKingVote, Slider, TextMeshProUGUI, UIRuntimeData, TextMeshProUGUI, UI_RulerOption (+5 more)

### Community 163 - "MiscCommands"
Cohesion: 0.27
Nodes (3): Dictionary, MiscCommands, EnemyType

### Community 164 - "TransformSaveData"
Cohesion: 0.05
Nodes (31): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+23 more)

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "RoleHandler"
Cohesion: 0.03
Nodes (46): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+38 more)

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

### Community 172 - "stream_town_migrate/src/technology_layout.rs"
Cohesion: 0.23
Nodes (17): GraphPoint, TechnologyNodeLayout, AuthoredGroup, AuthoredNode, build_layout(), checked_in_layout_exactly_matches_the_unity_graph_conversion(), convert(), parse_point() (+9 more)

### Community 173 - "Handle"
Cohesion: 0.03
Nodes (113): AccessibilityMotionDefaults, apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension (+105 more)

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
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 179 - "String"
Cohesion: 0.10
Nodes (52): MaterialDef, PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks() (+44 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 185 - "WorldInstanceDeterminism"
Cohesion: 0.29
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 188 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 189 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 190 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - ".new"
Cohesion: 0.23
Nodes (9): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), generated_instance_counts_match_the_sanitized_unity_save_oracle(), positive_noise_offset(), Self, SystemRandom, town_layer_noise_offset(), town_layer_seed_hash() (+1 more)

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

### Community 208 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 209 - "hash_world"
Cohesion: 0.26
Nodes (12): cell_hash(), hash_world(), horizontal_hash(), legacy_resource_navigation(), legacy_v1_world_hash(), legacy_v2_world_hash(), legacy_v3_world_hash(), legacy_variable_resource_amounts() (+4 more)

### Community 211 - "LoadingWorkNode"
Cohesion: 0.36
Nodes (5): LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, WorldLoadingWork

### Community 213 - "matching_role_animation_state"
Cohesion: 0.27
Nodes (10): debug_fingerprint(), default_role_preview_animation(), matching_role_animation_state(), player_animation_controller(), role_preview_animation_choices(), role_preview_animation_request(), role_preview_uses_shipping_rig_animation_and_composition_rules(), role_preview_visible_nodes() (+2 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "STSM_HelperBase"
Cohesion: 0.15
Nodes (5): int, STSM_Helper_Attack, StateMachine, string, STSM_HelperBase

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "Target"
Cohesion: 0.05
Nodes (29): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, UnitTravelToPosition, Vector3 (+21 more)

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

### Community 234 - "update_stream_operator_chat"
Cohesion: 0.20
Nodes (10): bounded_history_f32(), Node, Without, StreamOperatorChatScrollThumb, update_stream_operator_chat(), BorderColor, StreamOperatorChatBadgeQuery, StreamOperatorChatInputTextQuery (+2 more)

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

### Community 253 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "Option"
Cohesion: 0.09
Nodes (42): AnimationClipDef, animation_take_name(), animator_component(), animator_reference_path(), color_value(), convert_clips(), convert_embedded_model_clips(), convert_post_process() (+34 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 268 - "draw_world_preview"
Cohesion: 0.25
Nodes (9): EnemyCampGenerationDef, draw_world_preview(), preview_grid_point(), preview_lerp_color(), Color32, Pos2, Rect, terrain_preview_color() (+1 more)

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

### Community 276 - "Enemy"
Cohesion: 0.38
Nodes (3): Action, float, Enemy

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
Nodes (57): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+49 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.13
Nodes (15): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+7 more)

### Community 287 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 290 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 291 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 292 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Ord, Ordering, PartialOrd, Self

### Community 293 - "StatusBar"
Cohesion: 0.33
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 294 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "SimpleDisableAfterTime"
Cohesion: 0.03
Nodes (27): PersistentScoped, Transform, PlayerSpawnPoint, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI, UI_VoteObjectiveRow (+19 more)

### Community 297 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 299 - "stream_town_tools/src/main.rs"
Cohesion: 0.05
Nodes (165): ObjectiveKind, add_archetype_scene(), apply_building_draft(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft() (+157 more)

### Community 301 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 302 - "PooledObjectData"
Cohesion: 0.40
Nodes (4): GameObject, int, string, PooledObjectData

### Community 303 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 304 - "embedded_presentation"
Cohesion: 0.03
Nodes (106): AmbientLight, ActiveMaterialHandles, apply_material_overrides(), archetype_by_source(), authored_post_process_stack(), authored_rgb_filter(), begin_animation_crossfade(), blend_environment_palette() (+98 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.05
Nodes (20): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+12 more)

### Community 307 - "tidal_plugin"
Cohesion: 0.67
Nodes (3): Path, tidal_plugin(), TidalPlugin

### Community 308 - "ChangeTimeStamp"
Cohesion: 0.50
Nodes (3): int, ChangeTimeStamp, DateTime

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - ".SetTargetType"
Cohesion: 0.15
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 325 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

## Knowledge Gaps
- **394 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+389 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `EnemySpawner`, `TechTree.Elements`, `SimpleScreenShot`, `GenerationSettings`, `SimpleDisableAfterTime`, `AudioHandler`, `Easings`, `ScriptablesProcessorInfrastructure`, `CommonEnums.cs`, `BuildingPlacer`, `StringUtils`, `SnapToGridMouseMovement`, `Target`, `FPSDisplay`, `MonoBehaviour`, `TechTree_SO`, `LabelDisplayProcessor`, `UpdateGraphBounds`, `.CreateEnumField`, `.DrawDataFieldAndLabel`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `ContentCatalog`, `stream_town_migrate/src/content.rs`, `String`, `Res`, `stream_town_game/src/lib.rs`, `stream_town_domain/src/lib.rs`, `.new`, `Option`, `config.rs`, `draw_world_preview`, `command.rs`, `twitch.rs`, `TechnologyGraphLayout`, `ModelPreviewRuntime`, `Ui`, `Query`, `save.rs`, `RenderAssets`, `Option`, `AnimationControllerDef`, `stream_town_tools/src/main.rs`, `UnityAsset`, `embedded_presentation`, `String`, `legacy.rs`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/presentation.rs`, `.new`, `runtime_console.rs`, `component_field_value`, `technology_graph.rs`, `matching_role_animation_state`, `generate_world_from_layers`, `stream_town_domain/src/presentation.rs`, `world.rs`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `Enemy`, `IProcessor`, `WorldGenProcessor`, `UserInterface_RulerVote`, `UserInterface_TownVote`, `RoleHandler`, `TechTreeProcessor`, `UIProcessor`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `BuildingPlacer`, `PlayerRoleData`, `Target`, `GameEvent`, `ObjectPoolingProcessor`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _394 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.08701923076923077 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.03571428571428571 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10609756097560975 - nodes in this community are weakly interconnected._