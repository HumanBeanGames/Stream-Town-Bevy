# Graph Report - Stream-Town-Bevy  (2026-09-03)

## Corpus Check
- 670 files · ~1,827,887 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9525 nodes · 28944 edges · 313 communities (285 shown, 28 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1063 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a55d9f47`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- update_environment_presentation
- BuildingProcessor
- String
- SeasonProcessor
- MenuRuntime
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- MetaData
- BottomBarInterface
- AnimationHandler
- SettingsProcessor
- UserInterface_Debug
- config.rs
- CellSpacePartitioning
- TechTreeIOUtility
- HealthHandler
- ScriptablesProcessorInfrastructure
- save.rs
- String
- command.rs
- twitch.rs
- NavGrid
- Station
- twitch_tab
- Result
- Option
- Node_SO
- TechTree.Elements
- SaveFileData
- Ui
- Res
- WorldGenProcessor
- generate_world
- TechTreeGraphView
- GenerationSettings
- Targetable
- GridPos
- SettingsData
- RenderAssets
- BinarySaveCodec
- ObjectSelectionProcessor
- TechTreeProcessor
- PoolableObject
- PlayerSaveData
- AnimationControllerDef
- MainMenuManager
- GlobalAudioController
- ContentCatalog
- ResourceDataSaveData
- stream_town_migrate/src/main.rs
- AudioHandler
- StreamTownSessionBridge
- Projectile
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- Value
- BevyMigrationExporter
- ScriptableObject
- TechTreeEditorWindow
- TechnologyGraphLayout
- stream_town_domain/src/content.rs
- CameraController
- stream_town_migrate/src/presentation.rs
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- TechTreeNode
- GamestateJukebox
- Result
- models.rs
- Tiler
- StableId
- Utils
- RoleHandler
- ResourceHolder
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_Idle_Player
- String
- Access_Text
- stream_town_migrate/src/content.rs
- String
- .new
- settings.rs
- FoliageProcessor
- SnapToGridMouseMovement
- GameEvent
- STSM_StateAction
- MonoBehaviour
- convert_fbx_to_glb.py
- NativeGameAudioRouting
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- .Update
- StateMachine
- GameEventProcessor
- TownGoalProcessor
- TwitchUser
- ResourceProcessor
- LoadingManager
- drive_tidal_music
- String
- CustomLogHandler
- LevelHandler
- PlayerProcessor
- UpdateGraphBounds
- UnitTravelToPosition
- ScriptablesEditor
- GameStateProcessor
- DayAndNightProcessor
- SelectedPlayer
- Option
- .CreateEnumField
- Instant
- UserInterface_RulerVote
- WindController
- .EnsureValidCredentials
- AIPath
- PlayerInputProcessor
- GateController
- direct_broadcast.rs
- SelectedPlayerGroup
- Coordinator
- encode_broadcast_session
- ResourceRuntimeData
- EnemySpawner
- Goal
- VoteEvent
- Editor
- TargetSensor
- DirectBroadcastRuntime
- WorldInstanceDeterminism
- .UserIsSubscribed
- What You Must Do When Invoked
- RuntimeData Template
- GridProcessor
- RuntimeData Template
- Key Rules
- ConfirmCheck
- WorldGenSaveData
- xtask/src/lib.rs
- legacy.rs
- SelectedResource
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- .SetTargetType
- WeatherProcessor
- .count
- SelectableObject
- Stream Town Reloaded - Architecture Documentation
- IProcessor
- .new
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_ObjectSelection
- UserInterface_TownGoal
- Access_Toggle
- UserInterface_DisplayUsernames
- IRuntimeDataScriptable
- SimpleMusicController
- Q: There are still no animations.
- xtask/src/main.rs
- SimpleScreenShot
- DataStructures
- TL_Secrets
- TechnologyTreeGroup
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- UIElementWrapper
- List
- .Log
- Option
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- EditorUtils
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- Easings
- SelectedObject
- WorldGenerationReferenceExporter
- World.Generation
- Access_Dropdown
- SelectedEnemy
- Autosave
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- world.rs
- RuntimeConsoleStatus
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UnityAsset
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- DayAndNightSettings
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- preview_grid_point
- FoliageGenerationSettings.cs
- TimeProcessor
- CommonEnums.cs
- StringUtils
- stream_town_tools/src/main.rs
- Key Rules
- .DrawDataFieldAndLabel
- RuntimeData Template
- Character Animation Regression Checklist
- SelectedBuilding
- ScriptKeywordProcessor
- FPSDisplay
- EventProcessor
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
- UI_TechOption
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- record_gpu_readiness
- extraction-spec.md
- Access_GOList
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- HealthModifier
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- PlayerInputRuntimeData
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- VfxSeagullSpawner
- Q: If there is more to do, keep going.
- BuildingModelHandler
- PlacementProbeHandler
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- .Draw
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- DontDestroyOnLoad
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- main
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch command reference
- KeepKingVote
- ObjectSelectionProcessor.Editor.cs
- GameConfig
- SelectedEnemyCamp
- Requirement
- NewKingVote
- Access_TextInput
- CreateProjectScopeProcessors.cs
- ScriptableObjectAssetData
- .ExportModification
- InventoryEntrySaveData
- Vec3
- ToolState
- StreamUserType
- UnityGraphics
- animate_loading_icon
- .InjectRuntimeData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- TwitchClientRuntimeData
- UserInterface_TownVote
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 451 edges
2. `ContentCatalog` - 199 edges
3. `WorldSimulation` - 195 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 141 edges
9. `ToolState` - 138 edges
10. `GridPos` - 134 edges

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

## Communities (313 total, 28 thin omitted)

### Community 0 - "update_environment_presentation"
Cohesion: 0.11
Nodes (26): AmbientLight, authored_rgb_filter(), blend_environment_palette(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstances, environment_palette(), environment_palette_covers_every_season_and_weather() (+18 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (36): BuildingBase, bool, float, int, List, UnityEvent, TilerBuilding, bool (+28 more)

### Community 2 - "String"
Cohesion: 0.12
Nodes (39): AnimationEventDef, AnimationObjectReference, AnimationParameterDef, glb_animation_names(), infer_missing_parameters(), inline_file_id(), inline_mapping_value(), parse_animation_events() (+31 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.07
Nodes (18): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+10 more)

### Community 4 - "MenuRuntime"
Cohesion: 0.03
Nodes (161): AccessibilityFocusVisualQuery, AnyResult, PlayerSettings, Default, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus() (+153 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (378): AccessibilityActionRequest, accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_settings_selection(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), AccessibilityActionDispatch, AccessibilityCandidate (+370 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (26): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+18 more)

### Community 7 - "MetaData"
Cohesion: 0.29
Nodes (3): ContainerBuilder, MetaDataInstaller, MetaData

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "AnimationHandler"
Cohesion: 0.05
Nodes (20): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+12 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 12 - "config.rs"
Cohesion: 0.12
Nodes (25): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), Default (+17 more)

### Community 13 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.05
Nodes (21): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy (+13 more)

### Community 16 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (8): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, Settings, ScriptablesProcessorInfrastructure

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "String"
Cohesion: 0.02
Nodes (156): AccessibleNode, AnimationClip, AnimationTargetId, active_event_text(), add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+148 more)

### Community 19 - "command.rs"
Cohesion: 0.17
Nodes (34): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+26 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (65): BTreeSet, TwitchConfig, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization (+57 more)

### Community 21 - "NavGrid"
Cohesion: 0.12
Nodes (22): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+14 more)

### Community 22 - "Station"
Cohesion: 0.06
Nodes (24): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+16 more)

### Community 23 - "twitch_tab"
Cohesion: 0.20
Nodes (16): broadcast_encoder_label(), Duration, Sender, start_twitch_authorization(), start_twitch_broadcast_authorization(), start_twitch_broadcast_clear(), start_twitch_broadcast_diagnostic(), start_twitch_clear() (+8 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - "Option"
Cohesion: 0.04
Nodes (180): ArchetypeDef, ArchetypeScene, PresentationCatalog, ActiveMaterialHandles, animate_healing_effects(), animation_property_value(), apply_authored_main_menu_camera(), archetype_by_source() (+172 more)

### Community 26 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 27 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Ui"
Cohesion: 0.09
Nodes (68): ArchetypeKind, animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice() (+60 more)

### Community 30 - "Res"
Cohesion: 0.03
Nodes (273): Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver, ActorNameOverlay (+265 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "generate_world"
Cohesion: 0.11
Nodes (31): RoleProgress, Default, changing_seed_changes_world_hash(), generate_world(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), ActionPresentation, archetype_id_by_source() (+23 more)

### Community 33 - "TechTreeGraphView"
Cohesion: 0.08
Nodes (17): Vector2, GroupSaveData, int, List, Port, Vector2, TechTreeGraphView, List (+9 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "Targetable"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 36 - "GridPos"
Cohesion: 0.08
Nodes (78): GridPos, GeneratedWorld, active_resource_at(), agent_action_facing_grid(), agent_path(), AgentGoal, authored_assignment_penalty_spreads_farmers_across_farms(), battering_ram_targets_and_damages_buildings_from_authored_mask() (+70 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "RenderAssets"
Cohesion: 0.04
Nodes (114): AccessibilityMotionDefaults, actor_material(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform (+106 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.12
Nodes (9): Camera, Container, InputButton, List, UnityAction, Vector2, Vector3, ObjectSelectionProcessor (+1 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "PoolableObject"
Cohesion: 0.07
Nodes (18): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, Dictionary, float (+10 more)

### Community 43 - "PlayerSaveData"
Cohesion: 0.04
Nodes (39): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, int (+31 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 47 - "ContentCatalog"
Cohesion: 0.06
Nodes (97): BuildingDef, ContentCatalog, TargetingScoreDef, ActorState, String, action_animation_speed(), action_cooldown(), actor_accepts_resource() (+89 more)

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
Cohesion: 0.11
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "Value"
Cohesion: 0.22
Nodes (28): ActorCustomization, BinaryParser, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+20 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.20
Nodes (14): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+6 more)

### Community 58 - "ScriptableObject"
Cohesion: 0.02
Nodes (94): ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, GameSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller (+86 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "TechnologyGraphLayout"
Cohesion: 0.10
Nodes (35): ContentError, Result, TechTree, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize (+27 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (42): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingModelDef, default_resource_generation_layers(), EnemyDef, EnemyModelSetDef, EnemyRunAnimation (+34 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (81): animation_state_id(), animation_state_machine_id(), animation_take_name(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id(), convert_chimney_smoke() (+73 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.09
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "Result"
Cohesion: 0.18
Nodes (40): aged_buildings(), building_cost_reductions(), building_level_caps(), ContentConversionReport, convert(), convert_export(), enemy_camp_generation_layers(), field_value() (+32 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.12
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "StableId"
Cohesion: 0.05
Nodes (65): ObjectiveDef, Display, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), building_upgrade_reenters_the_full_construction_phase(), BuildingState (+57 more)

### Community 74 - "Utils"
Cohesion: 0.03
Nodes (47): BuildCostModifier, InputButton, Color, GameUserType, UserColours, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection (+39 more)

### Community 75 - "RoleHandler"
Cohesion: 0.03
Nodes (33): SimpleToggleCarry, RoleSlotModifier, int, CharacterModelHandler, bool, int, List, Transform (+25 more)

### Community 76 - "ResourceHolder"
Cohesion: 0.09
Nodes (17): CollectResource, AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent (+9 more)

### Community 77 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 78 - "technology_graph.rs"
Cohesion: 0.12
Nodes (38): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+30 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.04
Nodes (36): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+28 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (21): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+13 more)

### Community 83 - "String"
Cohesion: 0.17
Nodes (12): AuthorizationEvent, format_minutes_seconds(), LiveVerification, LiveVerificationEvent, LiveVerificationTarget, PreparedBroadcast, Display, Drop (+4 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_migrate/src/content.rs"
Cohesion: 0.12
Nodes (34): asset(), component(), component_at(), converted_rotating_axis(), converts_active_catalog_references_and_round_trips_ron(), converts_building_role_slot_modifiers(), converts_disable_after_time_lifetime(), converts_enemy_combat_and_camp_spawn_data() (+26 more)

### Community 86 - "String"
Cohesion: 0.14
Nodes (33): ability_choices(), action_animation_choices(), apply_role_draft(), delete_selected_role(), discover_model_assets(), discover_texture_assets(), discovered_model_assets_are_project_relative_glbs(), duplicate_selected_role() (+25 more)

### Community 87 - ".new"
Cohesion: 0.11
Nodes (28): BroadcastConfig, BroadcastEncoderPreference, amf_quality_profile_keeps_static_grid_detail_between_keyframes(), BroadcastTarget, closing_the_operator_window_requests_a_graceful_game_exit(), configure_amf_quality(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it() (+20 more)

### Community 88 - "settings.rs"
Cohesion: 0.05
Nodes (57): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), Error, Into, Option, Path, PathBuf (+49 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "GameEvent"
Cohesion: 0.04
Nodes (28): SortBuildingByLowerLevel, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+20 more)

### Community 92 - "STSM_StateAction"
Cohesion: 0.05
Nodes (22): AttackUnit, RotationHandler, float, Quaternion, Vector3, int, STSM_Helper_Attack, int (+14 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (121): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, PersistentScoped, ContainerBuilder, Volume (+113 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "NativeGameAudioRouting"
Cohesion: 0.10
Nodes (14): NativeGameAudioClip, NativeGameAudioMix, NativeGameAudioRouting, NativeGameAudioState, NativeGameAudioVoice, pcm16_wav_clip(), pcm16_wav_data(), Default (+6 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (44): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+36 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.07
Nodes (29): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+21 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (73): AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef (+65 more)

### Community 100 - ".Update"
Cohesion: 0.17
Nodes (16): List, Material, materials, Mesh, meshes, Dictionary, int, List (+8 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "TwitchUser"
Cohesion: 0.22
Nodes (7): ActivityStatus, bool, float, string, UserType, TwitchUser, Character.Enumerations

### Community 105 - "ResourceProcessor"
Cohesion: 0.09
Nodes (19): Container, ContainerBuilder, Dictionary, float, int, materialIndex, Matrix4x4, meshIndex (+11 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "drive_tidal_music"
Cohesion: 0.17
Nodes (26): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature, player_music_gain() (+18 more)

### Community 108 - "String"
Cohesion: 0.15
Nodes (20): animation_parameter_name(), archetype_scenes(), authored_mask(), building_placements(), BuildingPlacement, child_technology_guids(), collect_model_dependencies(), decomposes_combined_unity_flag_values() (+12 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.13
Nodes (11): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Dictionary, DebugSettings (+3 more)

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "PlayerProcessor"
Cohesion: 0.06
Nodes (15): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+7 more)

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 113 - "UnitTravelToPosition"
Cohesion: 0.12
Nodes (8): Api, UnitTravelToPosition, Vector3, TL_API, Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "GameStateProcessor"
Cohesion: 0.15
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 116 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 118 - "Option"
Cohesion: 0.05
Nodes (60): bounded_history_f32(), camera_targets_primary_window(), cleanup_completed_stream_only_readbacks(), disarm_stream_only_readbacks(), gpu_readbacks_are_published_in_render_order_even_when_they_finish_out_of_order(), moderate_selected_operator_user(), operator_chat_scroll_rows(), operator_restart_button_requests_a_stream_restart() (+52 more)

### Community 119 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 120 - "Instant"
Cohesion: 0.24
Nodes (8): BroadcastStopDisposition, CadenceTick, duration_as_micros(), Duration, Instant, stream_readback_due(), twitch_live_request_timeout(), VideoCadence

### Community 121 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 122 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 123 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 124 - "AIPath"
Cohesion: 0.11
Nodes (20): Action, bool, float, int, string, Type, Vector3, AIPath (+12 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.06
Nodes (41): append_direct_broadcast_diagnostic_to(), average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), configure_direct_broadcast(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), DirectBroadcastSnapshot, DirectTwitchBroadcastPlugin (+33 more)

### Community 130 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 131 - "encode_broadcast_session"
Cohesion: 0.09
Nodes (43): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastEncoder, BroadcastMetrics, BroadcastPrerequisites, capture_process_audio() (+35 more)

### Community 132 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 135 - "VoteEvent"
Cohesion: 0.21
Nodes (8): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, string, VoteOption

### Community 136 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 137 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 138 - "DirectBroadcastRuntime"
Cohesion: 0.06
Nodes (46): append_direct_broadcast_diagnostic(), apply_direct_broadcast_control(), arm_stream_only_readback(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), BroadcastMetricsSnapshot, capture_direct_broadcast_frame(), direct_broadcast_log_path() (+38 more)

### Community 139 - "WorldInstanceDeterminism"
Cohesion: 0.30
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 140 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "GridProcessor"
Cohesion: 0.06
Nodes (22): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, GridProcessorEditor, int (+14 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 147 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "legacy.rs"
Cohesion: 0.10
Nodes (49): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell() (+41 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.12
Nodes (18): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+10 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - ".SetTargetType"
Cohesion: 0.19
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 154 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 155 - ".count"
Cohesion: 0.04
Nodes (81): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), authored_rotating_node_names(), begin_world_loading() (+73 more)

### Community 156 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "IProcessor"
Cohesion: 0.08
Nodes (16): CancellationToken, Task, Action, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor (+8 more)

### Community 159 - ".new"
Cohesion: 0.19
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 163 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 164 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "IRuntimeDataScriptable"
Cohesion: 0.06
Nodes (25): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Action (+17 more)

### Community 167 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 171 - "DataStructures"
Cohesion: 0.15
Nodes (7): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, Data, DateTime

### Community 172 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 173 - "TechnologyTreeGroup"
Cohesion: 0.25
Nodes (5): Color, float, string, TechnologyTreeGroup, Group

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UIElementWrapper"
Cohesion: 0.25
Nodes (5): GameObject, List, PresetButtons, ContainerBuilder, UIElementWrapper

### Community 177 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 178 - ".Log"
Cohesion: 0.05
Nodes (27): Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, Action, bool, BoxCollider (+19 more)

### Community 179 - "Option"
Cohesion: 0.08
Nodes (69): AnimationClipDef, MaterialDef, animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value() (+61 more)

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

### Community 185 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "World.Generation"
Cohesion: 0.04
Nodes (39): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller (+31 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (25): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+17 more)

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "world.rs"
Cohesion: 0.08
Nodes (61): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), cell_hash(), different_town_seeds_produce_different_resource_and_foliage_layouts() (+53 more)

### Community 195 - "RuntimeConsoleStatus"
Cohesion: 0.24
Nodes (10): BTreeMap, RuntimeConsoleStatus, format_runtime_frame_times(), inject_runtime_command(), launch_runtime_game(), poll_runtime_console(), runtime_actions_sequence_after_latest_acknowledgement(), runtime_console_attached() (+2 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UnityAsset"
Cohesion: 0.16
Nodes (36): ArchetypesById, StorageModelDef, storage_node_visible(), archetype_bounds(), archetype_kind(), building_model_definitions(), building_node_age(), component_field_value() (+28 more)

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

### Community 203 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

### Community 209 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 210 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 211 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (24): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+16 more)

### Community 213 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (73): animation_property_curves_editor(), apply_building_draft(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft (+65 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "EventProcessor"
Cohesion: 0.22
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

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
Cohesion: 0.17
Nodes (7): Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary, MiscCommands

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

### Community 243 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

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

### Community 253 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

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

### Community 264 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

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

### Community 273 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

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

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.33
Nodes (6): Authoritative files, Future role behavior scripting (not implemented), Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ModelPreviewRuntime"
Cohesion: 0.07
Nodes (55): apply_preview_material_overrides(), apply_preview_node_visibility(), canonical_preview_node_name(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene (+47 more)

### Community 285 - "Stream Town Twitch command reference"
Cohesion: 0.13
Nodes (15): Building catalog and costs, Building IDs (BIDs), Camera and locating citizens, Character appearance, Getting started, Information, Moderator and game-master commands, Placing ordinary buildings (+7 more)

### Community 289 - "GameConfig"
Cohesion: 0.10
Nodes (42): GameConfig, GameplayConfig, BTreeMap, EnemyCampGenerationDef, RoleDef, BTreeSet, StationDef, active_station_ids() (+34 more)

### Community 291 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 292 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 293 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 297 - "InventoryEntrySaveData"
Cohesion: 0.40
Nodes (4): bool, int, string, InventoryEntrySaveData

### Community 298 - "Vec3"
Cohesion: 0.07
Nodes (51): actor_combat_visual(), advance_falling_fish(), agent_facing_matches_unity_rotation_and_action_targets(), agent_path_world_target(), AgentLocomotion, animate_chimney_smoke_particles(), BuildingEffectKind, BuildingEffectParticle (+43 more)

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (68): add_archetype_scene(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot (+60 more)

### Community 300 - "StreamUserType"
Cohesion: 0.40
Nodes (4): StreamUserType, pending_stream_user_type(), should_show_actor_name(), legacy_user_type()

### Community 301 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 303 - "animate_loading_icon"
Cohesion: 0.67
Nodes (4): animate_loading_icon(), apply_loading_icon_rotation(), LoadingIconSpinner, UiTransform

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (14): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+6 more)

### Community 309 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **394 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+389 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **28 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `EnemySpawner`, `ScriptablesProcessorInfrastructure`, `TechTree.Elements`, `GenerationSettings`, `SimpleScreenShot`, `DataStructures`, `AudioHandler`, `Easings`, `ScriptableObject`, `World.Generation`, `BuildingPlacer`, `CommonEnums.cs`, `StringUtils`, `.DrawDataFieldAndLabel`, `SnapToGridMouseMovement`, `FPSDisplay`, `MonoBehaviour`, `EventProcessor`, `Resource`, `UpdateGraphBounds`, `.CreateEnumField`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `update_environment_presentation`, `String`, `stream_town_game/src/lib.rs`, `config.rs`, `save.rs`, `String`, `command.rs`, `twitch.rs`, `legacy.rs`, `Option`, `.count`, `Ui`, `Res`, `.new`, `generate_world`, `GameConfig`, `GridPos`, `RenderAssets`, `Vec3`, `ToolState`, `AnimationControllerDef`, `ContentCatalog`, `Option`, `TechnologyGraphLayout`, `stream_town_domain/src/content.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/presentation.rs`, `world.rs`, `UnityAsset`, `Result`, `technology_graph.rs`, `stream_town_tools/src/main.rs`, `String`, `settings.rs`, `stream_town_domain/src/presentation.rs`, `String`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `SelectedPlayerGroup`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `IProcessor`, `WorldGenProcessor`, `NewKingVote`, `IRuntimeDataScriptable`, `TechTreeProcessor`, `PoolableObject`, `Player`, `.Log`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `UserInterface_TownVote`, `Utils`, `RoleHandler`, `BuildingPlacer`, `TimeProcessor`, `GameEvent`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `SelectedPlayer`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _394 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `update_environment_presentation` be split into smaller, more focused modules?**
  _Cohesion score 0.1076923076923077 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.036036036036036036 - nodes in this community are weakly interconnected._
- **Should `String` be split into smaller, more focused modules?**
  _Cohesion score 0.11875843454790823 - nodes in this community are weakly interconnected._