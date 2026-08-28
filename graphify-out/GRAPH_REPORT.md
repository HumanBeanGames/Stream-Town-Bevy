# Graph Report - Stream-Town-Bevy  (2026-08-28)

## Corpus Check
- 671 files · ~1,804,449 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9236 nodes · 27486 edges · 317 communities (293 shown, 24 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1051 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2ac53c00`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Vec
- BuildingProcessor
- String
- SeasonProcessor
- Res
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- GameConfig
- BottomBarInterface
- RoleHandler
- SettingsProcessor
- UserInterface_Debug
- config.rs
- WorldGenSaveData
- TechTreeIOUtility
- HealthHandler
- UnityAsset
- save.rs
- RenderAssets
- command.rs
- twitch.rs
- NavGrid
- STSM_GoToLocation
- generate_world
- Result
- Option
- Node_SO
- StableId
- SaveFileData
- UserInterface
- ResMut
- WorldGenProcessor
- CellSpacePartitioning
- DebugProcessor
- GenerationSettings
- .SetTargetType
- BinarySaveCodec
- SettingsData
- Handle
- TechnologyGraphLayout
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- .new
- AnimationControllerDef
- World.Generation.Settings
- String
- ContentCatalog
- ResourceDataSaveData
- stream_town_migrate/src/main.rs
- IRuntimeDataScriptable
- StreamTownSessionBridge
- Station
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- legacy.rs
- PoolableObject
- MonoBehaviour
- TechTreeEditorWindow
- buildings_tab
- stream_town_domain/src/content.rs
- CameraController
- PlayerProcessor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Pet
- Objective
- GamestateJukebox
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- MainMenuManager
- UserInterface_ObjectSelection
- .Log
- UserInterface_TownVote
- technology_graph.rs
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Targetable
- Option
- Access_Text
- RoleData
- update_environment_presentation
- .new
- PlayerSettings
- FoliageProcessor
- SnapToGridMouseMovement
- RaidEvent
- GeneratedWorld
- Ui
- convert_fbx_to_glb.py
- convert
- Resource
- SaveProcessor
- LabelDisplayProcessor
- stream_town_domain/src/presentation.rs
- .Draw
- StateMachine
- Buildings
- TownGoalProcessor
- TimeProcessor
- ResourceProcessor
- LoadingManager
- CommonEnums.cs
- MiscCommands
- CustomLogHandler
- LevelHandler
- BinaryReader
- TargetSensor
- Goal
- advance_world_loading_cover
- EnemyModelHandler
- stream_town_tools/src/main.rs
- SavingAndLoading.Structs
- GameEventProcessor
- STSM_Idle
- VoteEvent
- unity_color_filter
- STSM_HelperBase
- VfxSeagullSpawner
- AIPath
- GUIDComponent
- GateController
- .EnsureValidCredentials
- WindController
- Coordinator
- BroadcastController
- GameEventSystem
- EnemySpawner
- TechNodeData
- TechTree.Elements
- IProcessor
- BuildingBase
- sync_stream_only_capture
- UnitHealthBar
- encode_broadcast_session
- What You Must Do When Invoked
- RuntimeData Template
- WorldInstanceDeterminism
- RuntimeData Template
- Key Rules
- TechTreeNode
- DontDestroyOnLoad
- xtask/src/lib.rs
- TechnologyTreeGroup
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- CommandDictionary
- Access_Toggle
- Option
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- InventorySaveData
- UserInterface_RulerVote
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- stream_town_migrate/src/technology_layout.rs
- SelectableObject
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- TL_Secrets
- Q: There are still no animations.
- xtask/src/main.rs
- List
- .UserIsSubscribed
- ResourceHolder
- TargetProcessor
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- UIElementWrapper
- convert
- record_gpu_readiness
- stream_town_migrate/src/presentation.rs
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- .StartupSequence
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- AnimationHandler
- Utils
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- BevyMigrationExporter
- roles_tab
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SimpleMusicController
- Target
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- DayAndNightProcessor
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- WorldGenRuntimeData
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- BuildingDataSettings
- Easings
- tools_ui
- DirectBroadcastRuntime
- SensorBase
- BuildPlacerData
- Key Rules
- .InitializeAndActivateProcessorsAsync
- RuntimeData Template
- Character Animation Regression Checklist
- SimpleScreenShot
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
- seed_generated_enemy_camps
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
- HealthModifier
- extraction-spec.md
- SelectedBuilding
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- PlayerRoleData
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- GridNode
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- FoliageGenerationSettings
- Q: If there is more to do, keep going.
- TL_API
- SimpleDisableAfterTime
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- UnityGraphics
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Processors
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxAnimationController
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- apply_preview_material_overrides
- Stream Town Twitch commands
- StreamTown.Migration
- main
- RotationHandler
- DebugSettings
- Requirement
- import_save
- IntWrapper
- CreateProjectScopeProcessors.cs
- PendingChatCommand
- ObjectiveSaveData
- direct_broadcast.rs
- UnitTravelToPosition
- ToolState
- ScriptablesProcessorInfrastructure
- .RefreshSceneBindingsAndTryGenerate
- run_broadcast_worker
- EnemyCampSaveData
- PlayerRoleSaveData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- FoliageSaveData
- PlayerInputProcessor
- TwitchClientRuntimeData
- NodeGroup_SO
- Autosave
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 408 edges
2. `ContentCatalog` - 174 edges
3. `WorldSimulation` - 171 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `ToolState` - 137 edges
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

## Communities (317 total, 24 thin omitted)

### Community 0 - "Vec"
Cohesion: 0.04
Nodes (90): AnimationClip, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve(), advance_animation_crossfade() (+82 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "String"
Cohesion: 0.10
Nodes (52): MaterialDef, PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks() (+44 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.04
Nodes (39): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+31 more)

### Community 4 - "Res"
Cohesion: 0.04
Nodes (188): Added, AnimatedBy, AnimationGraphHandle, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver, ActorNameOverlay (+180 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (252): AccessibleNode, accessibility_settings_selection(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, active_event_text(), actor_combat_visual(), actor_detail_budget() (+244 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "GameConfig"
Cohesion: 0.11
Nodes (43): GameConfig, GameplayConfig, BTreeMap, RoleDef, BTreeSet, StationDef, active_station_ids(), actor_idle_anchor() (+35 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.05
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "RoleHandler"
Cohesion: 0.04
Nodes (22): RoleSlotModifier, int, RoleHandler, bool, Dictionary, UnityEvent, RoleSlot, bool (+14 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.11
Nodes (28): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet (+20 more)

### Community 13 - "WorldGenSaveData"
Cohesion: 0.09
Nodes (17): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+9 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (15): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy, STSM_Action_Heal (+7 more)

### Community 16 - "UnityAsset"
Cohesion: 0.19
Nodes (45): aged_buildings(), building_cost_reductions(), building_level_caps(), building_node_age(), convert_export(), enemy_camp_generation_layers(), field_value(), foliage_layers() (+37 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "RenderAssets"
Cohesion: 0.05
Nodes (141): actor_material(), actor_scene_budget(), advance_falling_fish(), animate_chimney_smoke_particles(), animate_healing_effects(), apply_agent_commands(), apply_authored_main_menu_camera(), authored_scene_rotation() (+133 more)

### Community 19 - "command.rs"
Cohesion: 0.06
Nodes (57): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+49 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (55): SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_oauth_identity(), envelope_from_privmsg() (+47 more)

### Community 21 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 22 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 23 - "generate_world"
Cohesion: 0.13
Nodes (36): changing_seed_changes_world_hash(), generate_world(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), actor_accepts_resource(), AgentGoal, authored_assignment_penalty_spreads_farmers_across_farms(), battering_ram_targets_and_damages_buildings_from_authored_mask() (+28 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - "Option"
Cohesion: 0.03
Nodes (150): AssetId, PresentationCatalog, ActiveMaterialHandles, advance_loading_phase(), advance_loading_runtime(), animation_property_value(), animation_root_name(), append_terrain_quad() (+142 more)

### Community 26 - "Node_SO"
Cohesion: 0.13
Nodes (12): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+4 more)

### Community 27 - "StableId"
Cohesion: 0.05
Nodes (70): ObjectiveDef, Display, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow() (+62 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "UserInterface"
Cohesion: 0.05
Nodes (16): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Enumerations, TownGoal.Data, StreamTown.EditorTools (+8 more)

### Community 30 - "ResMut"
Cohesion: 0.04
Nodes (149): AccessibilityFocusVisualQuery, AnyResult, AppExit, DirectBroadcastControl, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus() (+141 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 33 - "DebugProcessor"
Cohesion: 0.05
Nodes (16): AttackUnit, Container, ContainerBuilder, DebugProcessor, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_GatherResource (+8 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.12
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "BinarySaveCodec"
Cohesion: 0.15
Nodes (6): Action, int, List, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "Handle"
Cohesion: 0.04
Nodes (94): AccessibilityMotionDefaults, authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform (+86 more)

### Community 39 - "TechnologyGraphLayout"
Cohesion: 0.15
Nodes (20): ContentError, Result, TechTree, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize (+12 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - ".new"
Cohesion: 0.02
Nodes (192): AccessibilityActionRequest, ArchetypeDef, ArchetypeScene, RotatingNodeDef, generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface() (+184 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (33): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+25 more)

### Community 45 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 46 - "String"
Cohesion: 0.17
Nodes (20): binary_schemas_one_through_three_decode_and_validate_trailer(), decode_binary(), decode_legacy(), ImportReport, json_pet_name(), legacy_objective_matches(), legacy_pet_name(), LegacyDecodedSave (+12 more)

### Community 47 - "ContentCatalog"
Cohesion: 0.06
Nodes (86): ContentCatalog, GridPos, ActorState, RoleProgress, Default, String, action_animation_speed(), action_cooldown() (+78 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.24
Nodes (10): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+2 more)

### Community 49 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 50 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (28): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+20 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "Station"
Cohesion: 0.07
Nodes (15): Station, Dictionary, float, int, List, Queue, Transform, List (+7 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.15
Nodes (42): ActorCustomization, StreamUserType, pending_stream_user_type(), binary_fixture(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json() (+34 more)

### Community 57 - "PoolableObject"
Cohesion: 0.06
Nodes (20): Container, ContainerBuilder, GUIDProcessor, BoxCollider, List, Quaternion, Vector3, bool (+12 more)

### Community 58 - "MonoBehaviour"
Cohesion: 0.01
Nodes (116): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+108 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "buildings_tab"
Cohesion: 0.14
Nodes (25): ability_choices(), action_animation_choices(), building_model_node_choices(), buildings_tab(), cached_gltf_metadata(), delete_selected_building(), discover_texture_assets(), duplicate_selected_building() (+17 more)

### Community 61 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (40): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, default_resource_generation_layers(), EnemyDef, EnemyModelSetDef (+32 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "PlayerProcessor"
Cohesion: 0.09
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 68 - "Objective"
Cohesion: 0.08
Nodes (11): Action, int, Objective, ObjectiveType, Dictionary, GameObject, Image, RectTransform (+3 more)

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (80): ArchetypesById, animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask(), authored_value() (+72 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.05
Nodes (21): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+13 more)

### Community 74 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.05
Nodes (19): SelectedEnemy, SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource, BoxCollider, Button (+11 more)

### Community 76 - ".Log"
Cohesion: 0.05
Nodes (29): Action, Action, CancellationToken, Task, IAsyncInitializableProcessor, ProcessorStartupContext, HideInCallstack, Object (+21 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (12): Slider, TextMeshProUGUI, UIRuntimeData, bool, Button, GameObject, List, Slider (+4 more)

### Community 78 - "technology_graph.rs"
Cohesion: 0.12
Nodes (38): center_world(), connection_hit_test(), connection_hit_testing_selects_curve_without_selecting_distant_space(), connection_remove_button(), content_bounds(), cubic_bezier(), draw_connection(), draw_connection_with_color() (+30 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Targetable"
Cohesion: 0.08
Nodes (18): Vector3, List, Dictionary, List, TargetRuntimeData, bool, BoxCollider, float (+10 more)

### Community 83 - "Option"
Cohesion: 0.08
Nodes (31): BroadcastMetricsSnapshot, moderate_selected_operator_user(), BackgroundColor, Changed, Interaction, MessageReader, Option, Query (+23 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 86 - "update_environment_presentation"
Cohesion: 0.08
Nodes (37): AmbientLight, authored_color_grading(), authored_rgb_filter(), building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced (+29 more)

### Community 87 - ".new"
Cohesion: 0.12
Nodes (21): AudioFrame, AudioInput, capture_process_audio(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), discard_pending_audio(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), operator_panel_uses_compact_telemetry_and_bottom_left_live_control() (+13 more)

### Community 88 - "PlayerSettings"
Cohesion: 0.06
Nodes (60): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+52 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (18): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+10 more)

### Community 92 - "GeneratedWorld"
Cohesion: 0.09
Nodes (60): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), avalanche_instance_hash(), cell_hash(), fnv_mix() (+52 more)

### Community 93 - "Ui"
Cohesion: 0.11
Nodes (60): animation_assets_editor(), animation_condition_mode_choice(), animation_controller_editor(), animation_events_editor(), animation_layer_blend_choice(), animation_layers_editor(), animation_parameter_kind_choice(), animation_parameters_editor() (+52 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "convert"
Cohesion: 0.40
Nodes (6): ContentConversionReport, convert(), normalized_path(), Path, T, write_ron_atomic()

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (42): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+34 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.04
Nodes (50): Action, CancellationToken, Component, Container, ContainerBuilder, Dictionary, float, List (+42 more)

### Community 98 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (68): AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+60 more)

### Community 100 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "Buildings"
Cohesion: 0.04
Nodes (14): BuildCostModifier, BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor (+6 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "TimeProcessor"
Cohesion: 0.21
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "CommonEnums.cs"
Cohesion: 0.07
Nodes (25): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+17 more)

### Community 108 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 109 - "CustomLogHandler"
Cohesion: 0.17
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "BinaryReader"
Cohesion: 0.22
Nodes (3): CancellationToken, Func, BinaryReader

### Community 112 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 113 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 114 - "advance_world_loading_cover"
Cohesion: 0.15
Nodes (26): AccessibilityNode, advance_world_loading_cover(), Hud, hud_play_time(), hud_season_meter_percent(), HudTechnologyProgressFill, loading_cover_ready(), loading_display_percent() (+18 more)

### Community 115 - "EnemyModelHandler"
Cohesion: 0.07
Nodes (13): bool, int, List, EnemyModelHandler, int, STSM_Helper_Attack, int, STSM_Action_Attack (+5 more)

### Community 116 - "stream_town_tools/src/main.rs"
Cohesion: 0.06
Nodes (72): AssetEditorSection, authority_tab(), broadcast_encoder_label(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, character_model_choices_include_converted_hierarchy_nodes(), checked_in_authoring_assets_pass_headless_validation() (+64 more)

### Community 117 - "SavingAndLoading.Structs"
Cohesion: 0.10
Nodes (3): SavingAndLoading, SavingAndLoading.Structs, World.Generation

### Community 118 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 119 - "STSM_Idle"
Cohesion: 0.22
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 120 - "VoteEvent"
Cohesion: 0.11
Nodes (14): int, List, NewKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float (+6 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 123 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.12
Nodes (19): bool, float, int, string, Type, Vector3, AIPath, AstarData (+11 more)

### Community 126 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 129 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 130 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 131 - "BroadcastController"
Cohesion: 0.11
Nodes (24): AtomicBool, BroadcastController, BroadcastMetrics, CadenceTick, duration_as_micros(), LiveVerification, publish_latest_video(), Arc (+16 more)

### Community 132 - "GameEventSystem"
Cohesion: 0.07
Nodes (18): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+10 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "TechNodeData"
Cohesion: 0.12
Nodes (11): List, Node_SO, TechNodeData, IEnumerable, Button, GameObject, Image, Slider (+3 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.07
Nodes (19): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+11 more)

### Community 136 - "IProcessor"
Cohesion: 0.05
Nodes (21): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, Container, ContainerBuilder (+13 more)

### Community 137 - "BuildingBase"
Cohesion: 0.08
Nodes (16): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+8 more)

### Community 138 - "sync_stream_only_capture"
Cohesion: 0.17
Nodes (17): camera_targets_primary_window(), Assets, Commands, Entity, Handle, HashMap, Image, PrimaryWindow (+9 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "encode_broadcast_session"
Cohesion: 0.12
Nodes (26): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), encode_broadcast_session(), encoder_candidates(), encoder_input_format() (+18 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WorldInstanceDeterminism"
Cohesion: 0.30
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TechTreeNode"
Cohesion: 0.07
Nodes (19): Color, Foldout, List, Sprite, Vector2, VisualElement, TechTreeNode, Group (+11 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "TechnologyTreeGroup"
Cohesion: 0.22
Nodes (6): Color, float, string, TechnologyTreeGroup, GroupSaveData, Group

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 154 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 155 - "Option"
Cohesion: 0.09
Nodes (42): AnimationClipDef, animation_take_name(), animator_component(), animator_reference_path(), color_value(), convert_clips(), convert_embedded_model_clips(), convert_post_process() (+34 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.14
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "InventorySaveData"
Cohesion: 0.29
Nodes (6): bool, int, List, string, InventoryEntrySaveData, InventorySaveData

### Community 159 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "stream_town_migrate/src/technology_layout.rs"
Cohesion: 0.29
Nodes (14): AuthoredGroup, AuthoredNode, build_layout(), checked_in_layout_exactly_matches_the_unity_graph_conversion(), convert(), parse_point(), parse_unity_graph(), parses_group_and_node_positions_from_unity_yaml() (+6 more)

### Community 163 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 167 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 171 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 172 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 173 - "TargetProcessor"
Cohesion: 0.20
Nodes (6): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 177 - "convert"
Cohesion: 0.26
Nodes (12): ActorKind, actor_prefix(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), duration_days(), entity_id() (+4 more)

### Community 178 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 179 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (97): AnimationFloatKeyframe, AnimationTangent, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id() (+89 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - ".StartupSequence"
Cohesion: 0.15
Nodes (4): Container, IEnumerable, IEnumerator, Type

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "AnimationHandler"
Cohesion: 0.07
Nodes (18): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+10 more)

### Community 185 - "Utils"
Cohesion: 0.07
Nodes (7): RoleScriptablesEditor, Utils, Animation, ScriptablesEditor, STStateMachine, Pathfinding, Character

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.03
Nodes (70): int, AudioSettings, List, CampGenSettings, List, FoliageGenSettings, bool, ParticleSystem (+62 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "roles_tab"
Cohesion: 0.32
Nodes (12): apply_role_draft(), delete_selected_role(), duplicate_selected_role(), legacy_roles_tab(), refresh_role_draft(), role_draft(), role_editor_applies_every_reference_family_without_partial_mutation(), role_i32() (+4 more)

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 195 - "Target"
Cohesion: 0.08
Nodes (13): STStateMachine.States, Units, Behaviours, Target, Utils.Pooling, Sensors, Pets, GridSystem.Partitioning (+5 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "DayAndNightProcessor"
Cohesion: 0.09
Nodes (13): Container, ContainerBuilder, DayAndNightProcessor, float, Material, Volume, DayAndNightSettings, ContainerBuilder (+5 more)

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

### Community 203 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

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

### Community 208 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 210 - "tools_ui"
Cohesion: 0.18
Nodes (11): debug_fingerprint(), migration_tab(), poll_tool_job_events(), Debug, searchable_stable_id_vec_editor(), start_xtask_job(), tools_ui(), update_preview_request() (+3 more)

### Community 211 - "DirectBroadcastRuntime"
Cohesion: 0.09
Nodes (32): apply_direct_broadcast_control(), average_milliseconds(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), capture_direct_broadcast_frame(), configure_direct_broadcast(), DirectBroadcastPhase, DirectBroadcastRuntime (+24 more)

### Community 212 - "SensorBase"
Cohesion: 0.19
Nodes (4): float, List, SensorRuntimeData, SensorBase

### Community 213 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

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

### Community 218 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "TechTreeSearchWindow"
Cohesion: 0.28
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

### Community 234 - "seed_generated_enemy_camps"
Cohesion: 0.29
Nodes (8): EnemyCampGenerationDef, enemy_camp_candidate(), enemy_camp_has_town_route(), enemy_camp_region_is_clear(), enemy_camp_town_reachable_cells(), rounded_milli_cells(), HashSet, seed_generated_enemy_camps()

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

### Community 251 - "HealthModifier"
Cohesion: 0.29
Nodes (5): HealthModifier, bool, float, GameObject, HealUnit

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "PlayerRoleData"
Cohesion: 0.09
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 268 - "SimpleDisableAfterTime"
Cohesion: 0.10
Nodes (8): Transform, PlayerSpawnPoint, float, GameObject, SimpleDisableAfterTime, float, Vector3, SimpleRotateOnAxis

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

### Community 276 - "Processors"
Cohesion: 0.05
Nodes (12): WindControllerEditor, ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, Processors.Editor, MetaData, Audio (+4 more)

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "VfxAnimationController"
Cohesion: 0.13
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "apply_preview_material_overrides"
Cohesion: 0.08
Nodes (46): apply_preview_material_overrides(), drive_model_preview_animation(), frame_model_preview(), ModelPreviewCamera, ModelPreviewRuntime, ModelPreviewScene, preview_material_overrides(), preview_material_resolution_matches_runtime_binding_precedence() (+38 more)

### Community 285 - "Stream Town Twitch commands"
Cohesion: 0.40
Nodes (4): Moderator and game-master commands, Player commands, Ruler and operator commands, Stream Town Twitch commands

### Community 289 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 290 - "DebugSettings"
Cohesion: 0.48
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 291 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 292 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 293 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "PendingChatCommand"
Cohesion: 0.53
Nodes (6): CommandOrigin, PendingChatCommand, require_game_master(), require_ruler_or_staff(), require_staff(), Result

### Community 296 - "ObjectiveSaveData"
Cohesion: 0.40
Nodes (3): int, string, ObjectiveSaveData

### Community 297 - "direct_broadcast.rs"
Cohesion: 0.10
Nodes (22): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), configure_amf_quality(), draw_centered_sensitive_label(), gpu_readback_padding_is_removed_without_corrupting_rows(), ingest(), ingest_selection_prefers_default_or_named_region() (+14 more)

### Community 299 - "ToolState"
Cohesion: 0.09
Nodes (75): ArchetypeKind, add_archetype_scene(), apply_building_draft(), apply_enemy_camp_generation_draft(), apply_foliage_draft(), apply_objective_draft(), apply_resource_generation_draft(), apply_technology_draft() (+67 more)

### Community 300 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 302 - "run_broadcast_worker"
Cohesion: 0.14
Nodes (17): AuthorizationEvent, BroadcastPrerequisites, BroadcastTarget, DirectTwitchBroadcastPlugin, LiveVerificationEvent, LiveVerificationTarget, PreparedBroadcast, App (+9 more)

### Community 303 - "EnemyCampSaveData"
Cohesion: 0.40
Nodes (3): int, uint, EnemyCampSaveData

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (14): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+6 more)

### Community 308 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

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
- **381 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+376 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `GameEventSystem`, `EnemySpawner`, `TechTree.Elements`, `SimpleDisableAfterTime`, `Processors`, `UserInterface`, `GenerationSettings`, `UpdateGraphBounds`, `ScriptablesProcessorInfrastructure`, `MonoBehaviour`, `Target`, `ScriptablesEditor`, `BuildingPlacer`, `Easings`, `SnapToGridMouseMovement`, `SimpleScreenShot`, `FPSDisplay`, `LabelDisplayProcessor`, `Buildings`, `CommonEnums.cs`, `RandomEnabler`, `SavingAndLoading.Structs`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `Vec`, `String`, `Res`, `stream_town_game/src/lib.rs`, `GameConfig`, `config.rs`, `UnityAsset`, `save.rs`, `RenderAssets`, `command.rs`, `twitch.rs`, `generate_world`, `Option`, `Option`, `apply_preview_material_overrides`, `TechnologyGraphLayout`, `PendingChatCommand`, `.new`, `AnimationControllerDef`, `ToolState`, `String`, `ContentCatalog`, `convert`, `stream_town_migrate/src/presentation.rs`, `buildings_tab`, `stream_town_domain/src/content.rs`, `roles_tab`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `technology_graph.rs`, `tools_ui`, `update_environment_presentation`, `GeneratedWorld`, `Ui`, `stream_town_domain/src/presentation.rs`, `seed_generated_enemy_camps`, `stream_town_tools/src/main.rs`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `PlayerRoleData`, `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `IProcessor`, `RoleHandler`, `BottomBarInterface`, `UserInterface_Debug`, `HealthHandler`, `WorldGenProcessor`, `TechTreeProcessor`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `PoolableObject`, `MonoBehaviour`, `Target`, `.Log`, `UserInterface_TownVote`, `BuildingPlacer`, `RoleData`, `RaidEvent`, `Resource`, `SaveProcessor`, `TimeProcessor`, `GameEventProcessor`, `VoteEvent`?**
  _High betweenness centrality (0.023) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _381 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Vec` be split into smaller, more focused modules?**
  _Cohesion score 0.044444444444444446 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.059887005649717516 - nodes in this community are weakly interconnected._
- **Should `String` be split into smaller, more focused modules?**
  _Cohesion score 0.09954751131221719 - nodes in this community are weakly interconnected._