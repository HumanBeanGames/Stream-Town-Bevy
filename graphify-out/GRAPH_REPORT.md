# Graph Report - Stream-Town-Bevy  (2026-09-02)

## Corpus Check
- 670 files · ~1,820,102 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9424 nodes · 28406 edges · 308 communities (279 shown, 29 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1059 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4abae1a2`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- WorldGenRuntimeData
- BuildingProcessor
- stream_town_migrate/src/presentation.rs
- SeasonProcessor
- MenuRuntime
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- RoleDataContainer
- BottomBarInterface
- .new
- SettingsProcessor
- UserInterface_Debug
- config.rs
- ScriptablesProcessorInfrastructure
- TechTreeIOUtility
- HealthHandler
- String
- save.rs
- drive_converted_animations
- command.rs
- twitch.rs
- NavGrid
- update_environment_presentation
- stream_town_tools/src/main.rs
- Result
- RenderAssets
- Node_SO
- .GenerateFromSettings
- SaveFileData
- Ui
- Res
- WorldGenProcessor
- CellSpacePartitioning
- TechTreeNode
- MeshData
- .SetTargetType
- stream_operator_chat_controls
- SettingsData
- update_vote_panels
- BinarySaveCodec
- IRuntimeDataScriptable
- TechTreeProcessor
- CharacterModelHandler
- PlayerSaveData
- AnimationControllerDef
- MainMenuManager
- legacy.rs
- GridPos
- ResourceDataSaveData
- stream_town_migrate/src/main.rs
- AudioHandler
- StreamTownSessionBridge
- GlobalAudioController
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- Value
- BevyMigrationExporter
- MonoBehaviour
- TechTreeEditorWindow
- GridNode
- StableId
- CameraController
- STSM_Action_PlayerBase
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- .CreateEnumField
- GamestateJukebox
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- WorldSimulation
- Target
- RoleHandler
- .Log
- Objective
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- STSM_GoToLocation
- twitch_tab
- Access_Text
- Character
- BuildingBase
- .new
- PlayerSettings
- FoliageProcessor
- SnapToGridMouseMovement
- GameEventProcessor
- generate_world_from_layers
- String
- convert_fbx_to_glb.py
- BuildingSettings
- Resource
- SaveProcessor
- UserInterface_GameMenu
- stream_town_domain/src/presentation.rs
- WorldInstanceDeterminism
- StateMachine
- EnemyModelHandler
- TownGoalProcessor
- NativeGameAudioRouting
- ResourceProcessor
- LoadingManager
- .EnsureValidCredentials
- SelectedPlayerGroup
- CustomLogHandler
- LevelHandler
- WeatherProcessor
- UpdateGraphBounds
- DontDestroyOnLoad
- ScriptablesEditor
- RoleProcessor
- RotationHandler
- PlayerInputProcessor
- DirectBroadcastControl
- .Draw
- VoteEvent
- KeepKingVote
- SaveDataMapper
- Coordinator
- AIPath
- Targetable
- GateController
- direct_broadcast.rs
- WindController
- .StartupSequence
- Option
- IProcessor
- DayAndNightProcessor
- Goal
- TechTree.Elements
- Editor
- AstarPath
- DirectBroadcastRuntime
- UnitHealthBar
- BuildingDamageMaterialHandler
- What You Must Do When Invoked
- RuntimeData Template
- SelectedPlayer
- RuntimeData Template
- Key Rules
- ConfirmCheck
- STSM_Idle
- xtask/src/lib.rs
- BuildPlacerData
- Utils
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- UserInterface_TownVote
- Easings
- Self
- TL_Secrets
- Stream Town Reloaded - Architecture Documentation
- .UserIsSubscribed
- Instant
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_ObjectSelection
- EditorUtils
- Access_Toggle
- UserInterface_DisplayUsernames
- UserInterface_TownGoal
- SimpleMusicController
- Q: There are still no animations.
- xtask/src/main.rs
- parse_model_clip_events
- UserInterface_BuildingHealthBar
- SeasonAudioData
- AnimationHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- UIElementWrapper
- Season
- PlayerInputRuntimeData
- Result
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- VfxSeagullSpawner
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- TechNodeData
- STSM_Action_DepositResource
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- SelectedEnemy
- FoliageGenerationSettings
- stream_town_migrate/src/menu_scene.rs
- SelectedResource
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- world.rs
- LabelDisplayProcessor
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- PlayerSpawnPoint
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- ErrorData
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- TechnologyTreeGroup
- TwitchCameraRequest
- update_stream_operator_chat
- Station
- StringUtils
- CellPartitioningEditor
- Key Rules
- .InitializeAndActivateProcessorsAsync
- RuntimeData Template
- Character Animation Regression Checklist
- OpenNode
- ScriptKeywordProcessor
- FPSDisplay
- TechTreeSearchWindow
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
- WorldGenConfig
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
- TwitchUser
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- .MoveCamera
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Q: If there is more to do, keep going.
- NodeUnlockData
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- STSM_HelperBase
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- SimpleDisableAfterTime
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ModelPreviewRuntime
- Stream Town Twitch commands
- Processors
- Requirement
- DebugSettings
- TL_API
- CreateProjectScopeProcessors.cs
- AllSeasonSettings
- BuildingModelHandler
- UnityGraphics
- ToolState
- SimpleScreenShot
- WorldGenSaveData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- .RefreshSceneBindingsAndTryGenerate
- UnitTravelToPosition
- UserInterface
- ObjectSelectionProcessor.Editor.cs
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- main
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- vcpkg.json
- Autosave
- TwitchClientRuntimeData
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 422 edges
2. `ContentCatalog` - 182 edges
3. `WorldSimulation` - 180 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `ToolState` - 138 edges
9. `RenderAssets` - 137 edges
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
- `builders_reserve_distinct_unoccupied_construction_approaches()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (308 total, 29 thin omitted)

### Community 0 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (89): AnimationClipDef, MaterialDef, PrefabPresentationBinding, animation_state_id(), animation_state_machine_id(), animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses() (+81 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.13
Nodes (4): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 4 - "MenuRuntime"
Cohesion: 0.05
Nodes (100): AccessibilityFocusVisualQuery, AnyResult, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibleButtonScope, apply_settings_draft() (+92 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (236): accessibility_settings_selection(), AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityCandidate, AccessibilityHighContrastText, ActionPresentation, actor_combat_visual(), actor_detail_budget() (+228 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (28): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData, float, Func (+20 more)

### Community 7 - "RoleDataContainer"
Cohesion: 0.12
Nodes (11): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, AllRoleDataSettings, bool (+3 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - ".new"
Cohesion: 0.02
Nodes (236): AccessibilityActionRequest, accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean(), ActiveMaterialHandles, animated_character_receiver_scope_follows_only_the_player_rig_hierarchy() (+228 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 12 - "config.rs"
Cohesion: 0.08
Nodes (39): broadcast_render_mode_default(), BroadcastConfig, BroadcastEncoderPreference, BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic() (+31 more)

### Community 13 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (5): int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.10
Nodes (11): Func, PlayerDeathHandler, bool, float, Vector3, Action, bool, float (+3 more)

### Community 16 - "String"
Cohesion: 0.10
Nodes (49): AnimationParameterDef, animator_component(), animator_reference_path(), convert_post_process(), field_f32(), field_str(), field_u64(), field_value() (+41 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "drive_converted_animations"
Cohesion: 0.06
Nodes (63): AnimationClip, AnimationTargetId, ActivePetVisual, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve() (+55 more)

### Community 19 - "command.rs"
Cohesion: 0.06
Nodes (62): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+54 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (67): BTreeSet, TwitchConfig, secrets_restart_requirements(), SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), BroadcasterSession, channel_point_reward_tag_survives_privmsg_conversion() (+59 more)

### Community 21 - "NavGrid"
Cohesion: 0.17
Nodes (15): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building(), reconstruct_path() (+7 more)

### Community 22 - "update_environment_presentation"
Cohesion: 0.09
Nodes (38): AmbientLight, authored_post_process_stack(), blend_environment_palette(), building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced (+30 more)

### Community 23 - "stream_town_tools/src/main.rs"
Cohesion: 0.05
Nodes (82): apply_building_draft(), archetype_kind_choice(), AssetEditorSection, authoring_apply_preserves_the_complete_local_twitch_setup(), authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft (+74 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - "RenderAssets"
Cohesion: 0.05
Nodes (154): PresentationCatalog, actor_material(), actor_scene_budget(), AgentAnimation, animate_agents(), animate_building_effects(), animate_chimney_smoke_particles(), animate_falling_fish() (+146 more)

### Community 26 - "Node_SO"
Cohesion: 0.14
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 27 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Vector2, Vector3, Action, IEnumerator (+9 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Ui"
Cohesion: 0.09
Nodes (64): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+56 more)

### Community 30 - "Res"
Cohesion: 0.02
Nodes (282): AccessibilityNode, Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AssetId, AudioSink, AccessibilityRuntime (+274 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (16): Action, Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable (+8 more)

### Community 32 - "CellSpacePartitioning"
Cohesion: 0.09
Nodes (19): Bounds, Container, ContainerBuilder, GridProcessor, bool, List, Vector2, BSPCell (+11 more)

### Community 33 - "TechTreeNode"
Cohesion: 0.06
Nodes (23): Color, Foldout, List, Sprite, Vector2, VisualElement, TechTreeNode, Port (+15 more)

### Community 34 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.10
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "stream_operator_chat_controls"
Cohesion: 0.08
Nodes (28): AccessibleNode, moderate_selected_operator_user(), operator_chat_scroll_rows(), operator_window_close_requests_exit(), Changed, Interaction, MessageReader, MouseWheel (+20 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "update_vote_panels"
Cohesion: 0.04
Nodes (98): AccessibilityMotionDefaults, authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform (+90 more)

### Community 39 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 40 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (28): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+20 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (6): Action, Container, ContainerBuilder, EventType, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "PlayerSaveData"
Cohesion: 0.08
Nodes (21): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+13 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 46 - "legacy.rs"
Cohesion: 0.10
Nodes (54): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, checked_in_schema_one_fixture_imports_retained_terrain() (+46 more)

### Community 47 - "GridPos"
Cohesion: 0.03
Nodes (210): GameConfig, StationDef, DirtyRegion, GridPos, ActorState, RoleProgress, Default, String (+202 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.24
Nodes (10): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+2 more)

### Community 49 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (14): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+6 more)

### Community 52 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, IPostInitializeProcessor, TextMeshProUGUI, UI_RulerOption (+10 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.11
Nodes (13): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+5 more)

### Community 56 - "Value"
Cohesion: 0.20
Nodes (27): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+19 more)

### Community 57 - "BevyMigrationExporter"
Cohesion: 0.12
Nodes (22): bool, GameObject, HashSet, int, List, long, MenuItem, string (+14 more)

### Community 58 - "MonoBehaviour"
Cohesion: 0.01
Nodes (144): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+136 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "GridNode"
Cohesion: 0.12
Nodes (10): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+2 more)

### Community 61 - "StableId"
Cohesion: 0.03
Nodes (211): GameplayConfig, BTreeMap, ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue (+203 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (9): bool, Camera, float, int, PlayerInput, Transform, Vector2, CameraController (+1 more)

### Community 63 - "STSM_Action_PlayerBase"
Cohesion: 0.10
Nodes (9): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Heal, STSM_Action_PlayerAttack (+1 more)

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

### Community 68 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
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

### Community 73 - "WorldSimulation"
Cohesion: 0.05
Nodes (53): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown() (+45 more)

### Community 74 - "Target"
Cohesion: 0.08
Nodes (13): STStateMachine.States, Units, Behaviours, Target, Utils.Pooling, Sensors, GridSystem.Partitioning, STStateMachine (+5 more)

### Community 75 - "RoleHandler"
Cohesion: 0.06
Nodes (16): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleHandler (+8 more)

### Community 76 - ".Log"
Cohesion: 0.02
Nodes (65): Container, ContainerBuilder, GUIDProcessor, bool, List, ObjectPoolingSettings, Action, CancellationToken (+57 more)

### Community 77 - "Objective"
Cohesion: 0.14
Nodes (5): Action, int, Objective, ObjectiveType, ObjectiveData

### Community 78 - "technology_graph.rs"
Cohesion: 0.05
Nodes (71): ContentError, Result, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id() (+63 more)

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
Cohesion: 0.14
Nodes (8): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation

### Community 83 - "twitch_tab"
Cohesion: 0.20
Nodes (16): broadcast_encoder_label(), Duration, Sender, start_twitch_authorization(), start_twitch_broadcast_authorization(), start_twitch_broadcast_clear(), start_twitch_broadcast_diagnostic(), start_twitch_clear() (+8 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "Character"
Cohesion: 0.07
Nodes (15): Pets.Enumerations, StreamTown.EditorTools, TownGoal, Core, Pets, GameEventSystem, GameEventSystem.Events, SavingAndLoading (+7 more)

### Community 86 - "BuildingBase"
Cohesion: 0.11
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 87 - ".new"
Cohesion: 0.12
Nodes (17): closing_the_operator_window_requests_a_graceful_game_exit(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), enemy_operator_countdown_matches_the_unity_day_boundary(), graceful_stop_replaces_capture_with_the_offline_frame_before_aborting(), operator_panel_uses_compact_telemetry_and_bottom_left_live_control(), operator_stop_cancels_an_in_flight_session_without_restarting_it() (+9 more)

### Community 88 - "PlayerSettings"
Cohesion: 0.06
Nodes (64): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+56 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "GameEventProcessor"
Cohesion: 0.03
Nodes (36): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+28 more)

### Community 92 - "generate_world_from_layers"
Cohesion: 0.31
Nodes (9): generate_world_from_layers(), generate_world_with_content_observed(), observed_generation_reports_every_real_stage_without_changing_output(), round_to_even(), FnMut, smooth_noise_step(), unity_terrain_height_curve(), WorldGenerationStage (+1 more)

### Community 93 - "String"
Cohesion: 0.15
Nodes (38): ability_choices(), action_animation_choices(), add_archetype_scene(), building_model_node_choices(), buildings_tab(), cached_gltf_metadata(), content_tab_contents(), create_model_archetype() (+30 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (46): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+38 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (27): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+19 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (67): AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef, AnimationTransformTrack, AnimationVec3Keyframe (+59 more)

### Community 100 - "WorldInstanceDeterminism"
Cohesion: 0.20
Nodes (7): Material, Resource, int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "EnemyModelHandler"
Cohesion: 0.07
Nodes (13): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, int, STSM_Helper_Attack, int (+5 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.12
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "NativeGameAudioRouting"
Cohesion: 0.14
Nodes (11): NativeGameAudioClip, NativeGameAudioMix, NativeGameAudioRouting, NativeGameAudioRoutingInner, NativeGameAudioState, NativeGameAudioVoice, pcm16_wav_clip(), pcm16_wav_data() (+3 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (49): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+41 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.17
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 112 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 114 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 115 - "RoleProcessor"
Cohesion: 0.06
Nodes (14): Container, ContainerBuilder, int, List, RoleProcessor, Dictionary, MiscCommands, bool (+6 more)

### Community 116 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 117 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 118 - "DirectBroadcastControl"
Cohesion: 0.17
Nodes (7): BroadcastStopDisposition, DirectBroadcastControl, exit_after_broadcast_stops(), AppExit, MessageWriter, stream_operator_restart_button(), StreamOperatorRestartButtonQuery

### Community 119 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 120 - "VoteEvent"
Cohesion: 0.09
Nodes (13): int, List, NewKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float (+5 more)

### Community 122 - "SaveDataMapper"
Cohesion: 0.08
Nodes (20): ResourceInventory, bool, int, Dictionary, Mesh, Vector3, SaveDataMapper, bool (+12 more)

### Community 123 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 124 - "AIPath"
Cohesion: 0.18
Nodes (14): bool, float, int, Vector3, AIPath, GraphCollision, GraphNode, GraphUpdateObject (+6 more)

### Community 126 - "Targetable"
Cohesion: 0.04
Nodes (34): CollectResource, ProjectileShooter, float, int, string, Action, float, Enemy (+26 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.04
Nodes (60): append_direct_broadcast_diagnostic_to(), AuthorizationEvent, average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), BroadcastPrerequisites, build_ingest_url(), direct_broadcast_diagnostics_are_persisted_without_a_live_session(), direct_broadcast_log_path() (+52 more)

### Community 129 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 130 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 131 - "Option"
Cohesion: 0.09
Nodes (46): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastEncoder, BroadcastMetrics, BroadcastTarget, capture_process_audio() (+38 more)

### Community 132 - "IProcessor"
Cohesion: 0.06
Nodes (16): Container, IProcessor, Container, ContainerBuilder, CreditsProcessor, Action, Container, ContainerBuilder (+8 more)

### Community 133 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 134 - "Goal"
Cohesion: 0.17
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 135 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 136 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 137 - "AstarPath"
Cohesion: 0.31
Nodes (5): string, Type, AstarData, AstarPath, NavGraph

### Community 138 - "DirectBroadcastRuntime"
Cohesion: 0.07
Nodes (54): append_direct_broadcast_diagnostic(), apply_direct_broadcast_control(), arm_stream_only_readback(), BroadcastMetricsSnapshot, camera_targets_primary_window(), capture_direct_broadcast_frame(), cleanup_completed_stream_only_readbacks(), configure_direct_broadcast() (+46 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 147 - "STSM_Idle"
Cohesion: 0.17
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 150 - "Utils"
Cohesion: 0.04
Nodes (14): RoleScriptablesEditor, DisableOnAwake, SelectionBase, List, SimpleEventOnStart, SimpleHideRendererOnAwake, GameObject, SimpleRandomModelEnabled (+6 more)

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.12
Nodes (18): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+10 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 155 - "Self"
Cohesion: 0.17
Nodes (7): loading_progress_is_recursively_derived_from_real_work(), LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, Self, WorldLoadingWork

### Community 156 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 159 - "Instant"
Cohesion: 0.19
Nodes (10): bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), CadenceTick, duration_as_micros(), Duration, Instant, stream_readback_due(), twitch_live_request_timeout() (+2 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 163 - "EditorUtils"
Cohesion: 0.15
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 164 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 167 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "parse_model_clip_events"
Cohesion: 0.31
Nodes (9): AnimationEventDef, AnimationObjectReference, inline_mapping_value(), parse_animation_events(), parse_model_clip_events(), parse_object_reference(), parses_normalized_animation_events_from_model_importer_clips(), parses_property_curves_and_animation_events_without_unity_types() (+1 more)

### Community 171 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 172 - "SeasonAudioData"
Cohesion: 0.57
Nodes (3): SeasonAudioData, AudioClip, List

### Community 173 - "AnimationHandler"
Cohesion: 0.08
Nodes (15): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+7 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 177 - "Season"
Cohesion: 0.38
Nodes (5): bool, float, int, SeasonRuntimeData, Season

### Community 178 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 179 - "Result"
Cohesion: 0.11
Nodes (49): AnimationFloatKeyframe, AnimationPropertyCurve, AnimationTangent, append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id() (+41 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "VfxSeagullSpawner"
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "TechNodeData"
Cohesion: 0.21
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

### Community 185 - "STSM_Action_DepositResource"
Cohesion: 0.33
Nodes (3): float, STSM_Action_DepositResource, STStateBase

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (85): ContainerBuilder, AllBuildingDataSettingsInstaller, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+77 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 190 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "world.rs"
Cohesion: 0.11
Nodes (35): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_world_to_grid(), avalanche_instance_hash(), changing_seed_changes_world_hash(), different_town_seeds_produce_different_resource_and_foliage_layouts(), fnv_mix(), foliage_visual_variant() (+27 more)

### Community 195 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

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

### Community 203 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

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

### Community 208 - "TechnologyTreeGroup"
Cohesion: 0.20
Nodes (6): Color, float, string, TechnologyTreeGroup, GroupSaveData, Group

### Community 209 - "TwitchCameraRequest"
Cohesion: 0.40
Nodes (4): bool, int, Vector3, TwitchCameraRequest

### Community 210 - "update_stream_operator_chat"
Cohesion: 0.17
Nodes (12): bounded_history_f32(), operator_restart_button_requests_a_stream_restart(), BackgroundColor, Node, Without, StreamOperatorChatScrollThumb, update_stream_operator_chat(), BorderColor (+4 more)

### Community 211 - "Station"
Cohesion: 0.03
Nodes (59): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+51 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.17
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "TechTreeSearchWindow"
Cohesion: 0.32
Nodes (6): List, Texture2D, TechTreeSearchWindow, ISearchWindowProvider, SearchTreeEntry, SearchWindowContext

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

### Community 234 - "WorldGenConfig"
Cohesion: 0.21
Nodes (17): WorldGenConfig, authored_grid_centre(), cell_hash(), foliage_horizontal_hash(), generate_shoreline_resources(), hash_world(), horizontal_hash(), legacy_resource_navigation() (+9 more)

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

### Community 253 - "SelectedBuilding"
Cohesion: 0.08
Nodes (5): SelectedBuilding, SelectedEnemyCamp, object, UnityAction, SelectedObject

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

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

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

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

### Community 273 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

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

### Community 285 - "Stream Town Twitch commands"
Cohesion: 0.40
Nodes (4): Moderator and game-master commands, Player commands, Ruler and operator commands, Stream Town Twitch commands

### Community 288 - "Processors"
Cohesion: 0.05
Nodes (15): BuildCostModifier, InputButton, UserInterface.MainMenu, PlayerControls.ObjectSelection, Processors, World, Level, MetaData (+7 more)

### Community 291 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 292 - "DebugSettings"
Cohesion: 0.48
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "AllSeasonSettings"
Cohesion: 0.18
Nodes (10): float, int, Material, AllSeasonSettings, Color, float, int, VisualEffect (+2 more)

### Community 296 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 298 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (69): apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot (+61 more)

### Community 300 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 304 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.03
Nodes (34): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, Action (+26 more)

### Community 309 - "UserInterface"
Cohesion: 0.05
Nodes (18): InputButton, SharedTypes, int, ChangeTimeStamp, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+10 more)

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
- **383 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+378 more)
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
- `drive_tidal_music()` (2× useful, score=1.966606185) _(code changed — re-verify)_
- `WorldSimulation` (2× useful, score=1.71789778) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `DayAndNightProcessor`, `TechTree.Elements`, `ScriptablesProcessorInfrastructure`, `SimpleDisableAfterTime`, `Easings`, `Processors`, `MeshData`, `SimpleScreenShot`, `UserInterface`, `ScriptableObject`, `LabelDisplayProcessor`, `.CreateEnumField`, `Target`, `BuildingPlacer`, `Station`, `StringUtils`, `Character`, `SnapToGridMouseMovement`, `FPSDisplay`, `UpdateGraphBounds`, `RandomEnabler`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `stream_town_migrate/src/presentation.rs`, `stream_town_game/src/lib.rs`, `config.rs`, `String`, `save.rs`, `drive_converted_animations`, `command.rs`, `twitch.rs`, `update_environment_presentation`, `stream_town_tools/src/main.rs`, `RenderAssets`, `Ui`, `Res`, `ToolState`, `AnimationControllerDef`, `legacy.rs`, `GridPos`, `Result`, `stream_town_migrate/src/menu_scene.rs`, `world.rs`, `stream_town_migrate/src/content.rs`, `WorldSimulation`, `technology_graph.rs`, `String`, `stream_town_domain/src/presentation.rs`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `BuildingProcessor`, `IProcessor`, `DayAndNightProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `UserInterface_Debug`, `SelectedPlayer`, `UserInterface_TownVote`, `WorldGenProcessor`, `TechTreeProcessor`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `Target`, `RoleHandler`, `.Log`, `BuildingPlacer`, `GameEventProcessor`, `Resource`, `SaveProcessor`, `TownGoalProcessor`, `SelectedPlayerGroup`, `RoleProcessor`, `VoteEvent`, `Targetable`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _383 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07315233785822021 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/presentation.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06077606358111267 - nodes in this community are weakly interconnected._
- **Should `SeasonProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.1282051282051282 - nodes in this community are weakly interconnected._