# Graph Report - Stream-Town-Bevy  (2026-08-28)

## Corpus Check
- 671 files · ~1,789,031 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 9070 nodes · 26686 edges · 314 communities (290 shown, 24 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1045 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `08d95e59`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- build_converted_animation
- BuildingProcessor
- BTreeMap
- SeasonProcessor
- Query
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- CommonEnums.cs
- BottomBarInterface
- RoleDataContainer
- SettingsProcessor
- UserInterface_Debug
- config.rs
- WorldGenSaveData
- TechTreeIOUtility
- HealthHandler
- Targetable
- save.rs
- Option
- command.rs
- twitch.rs
- NavGrid
- STSM_GoToLocation
- world.rs
- Result
- .count
- Node_SO
- simulation.rs
- SaveFileData
- DataStructures
- Res
- WorldGenProcessor
- CellSpacePartitioning
- STSM_Idle_Player
- GenerationSettings
- TargetableHealth
- BinarySaveCodec
- SettingsData
- RenderAssets
- TechnologyGraphViewState
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- .new
- AnimationControllerDef
- runtime_console.rs
- legacy.rs
- StableId
- ResourceDataSaveData
- stream_town_migrate/src/main.rs
- AudioHandler
- StreamTownSessionBridge
- Station
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- Value
- PoolableObject
- MonoBehaviour
- TechTreeEditorWindow
- String
- Vec3
- CameraController
- TargetSensor
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
- ScriptablesProcessorInfrastructure
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Utils
- stream_operator_live_button
- Access_Text
- SelectedObject
- sync_stream_only_capture
- .new
- PlayerSettings
- FoliageProcessor
- SnapToGridMouseMovement
- Enemy
- generate_world_from_layers
- TargetProcessor
- convert_fbx_to_glb.py
- BuildingDataSettings
- Resource
- SaveProcessor
- UnitTextDisplay
- stream_town_domain/src/presentation.rs
- .Draw
- StateMachine
- Editor
- TownGoalProcessor
- .RestoreWorldState
- ResourceProcessor
- LoadingManager
- Self
- GUIDComponent
- CustomLogHandler
- LevelHandler
- TownGoalProcessor.cs
- DebugProcessor
- Goal
- TechTreeNode
- EnemyModelHandler
- stream_town_tools/src/main.rs
- IRuntimeDataScriptable
- GameEventProcessor
- Season
- VoteEvent
- unity_color_filter
- SelectableObject
- .EnsureValidCredentials
- AIPath
- UserInterface_TownGoal
- GateController
- RotationHandler
- BuildingSettings
- SelectedEnemyCamp
- Option
- TwitchUser
- DayAndNightProcessor
- BinaryWriter
- TechTree.Elements
- Coordinator
- BuildingBase
- EditorUtils
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- WorldInstanceDeterminism
- RuntimeData Template
- Key Rules
- TechTreeGraphView
- DontDestroyOnLoad
- xtask/src/lib.rs
- TL_Secrets
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- FrameCapture
- Access_Toggle
- String
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- PlayerSaveData
- UserInterface_RulerVote
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- EnemyWeaponModel
- FoliageData
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- SelectedEnemy
- Q: There are still no animations.
- xtask/src/main.rs
- List
- SelectedResource
- ResourceHolder
- StringUtils
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- UIElementWrapper
- SeasonDataSettings
- record_gpu_readiness
- stream_town_migrate/src/presentation.rs
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- .StartupSequence
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- AnimationHandler
- WeatherProcessor
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- BevyMigrationExporter
- ErrorData
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SimpleMusicController
- Sensors
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- IProcessor.cs
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- WorldGenRuntimeData
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- BuildingDamageMaterialHandler
- Easings
- GlobalAudioController
- DirectBroadcastRuntime
- SensorProcessor
- EquipmentHandlerEditor
- Key Rules
- .InitializeAndActivateProcessorsAsync
- RuntimeData Template
- Character Animation Regression Checklist
- .RenderFoliageType
- ScriptKeywordProcessor
- FPSDisplay
- Vec
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
- String
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
- STSM_StateAction
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
- GridProcessor
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- FoliageGenerationSettings.cs
- Q: If there is more to do, keep going.
- SimpleToggleCarry
- StatusBar
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- UnityGraphics
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- FoliageRuntimeData
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxSeagullSpawner
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- ToolState
- Stream Town Twitch commands
- StreamTown.Migration
- PlayerInputRuntimeData
- PlacementProbeHandler
- FoliageGenSettings
- Requirement
- WaterFoliageGenSettings
- Ui
- CreateDefaultSettingsAssets.cs
- BuildingRuntimeData
- .InjectRuntimeData
- direct_broadcast.rs
- SeasonAudioData
- Result
- Processors
- .RefreshSceneBindingsAndTryGenerate
- verify_twitch_public_stream
- TransformSaveData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- PlayerInputProcessor
- Autosave
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- VideoCadence
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 388 edges
2. `WorldSimulation` - 171 edges
3. `ContentCatalog` - 170 edges
4. `Utils` - 159 edges
5. `Processors` - 156 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 134 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `combat_goal_damages_kills_and_respawns()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `enemies_advance_on_town_hall_while_defenders_acquire_them()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `food_roles_only_select_their_authored_target_types()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (314 total, 24 thin omitted)

### Community 0 - "build_converted_animation"
Cohesion: 0.06
Nodes (55): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, AnimationTransitionPlayback, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve() (+47 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "BTreeMap"
Cohesion: 0.09
Nodes (51): MaterialDef, PrefabPresentationBinding, RendererMaterialBinding, BTreeMap, TextureDef, animator_component(), animator_reference_path(), array_index() (+43 more)

### Community 3 - "SeasonProcessor"
Cohesion: 0.14
Nodes (4): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 4 - "Query"
Cohesion: 0.04
Nodes (182): Aabb, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, ActivePetVisual (+174 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (296): AccessibleNode, AnyResult, accessibility_settings_selection(), accessibility_should_clear_focus(), AccessibilityActionDispatch, AccessibilityAnnouncement, AccessibilityCandidate, AccessibilityHighContrastText (+288 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (28): Container, ContainerBuilder, TimeProcessor, bool, float, Func, int, PlayerExistsByIDDelegate (+20 more)

### Community 7 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (16): BuildingResourceModelHandler, GameObject, UnityEvent, Dictionary, MiscCommands, EnemyType, Foliage, FoliageSaveType (+8 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "RoleDataContainer"
Cohesion: 0.06
Nodes (15): RoleSlot, bool, int, ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer (+7 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "config.rs"
Cohesion: 0.10
Nodes (30): broadcast_render_mode_default(), BroadcastEncoderPreference, BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration() (+22 more)

### Community 13 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.10
Nodes (12): Func, PlayerDeathHandler, bool, float, Vector3, Action, bool, float (+4 more)

### Community 16 - "Targetable"
Cohesion: 0.11
Nodes (7): bool, BoxCollider, float, int, Transform, Vector3, Targetable

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "Option"
Cohesion: 0.03
Nodes (188): AnimationTransitionOutcome, ArchetypeDef, ArchetypeKind, ArchetypeScene, HealthDef, RotatingNodeDef, Option, MainMenuModelInstance (+180 more)

### Community 19 - "command.rs"
Cohesion: 0.11
Nodes (37): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+29 more)

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (55): SecretsAuthorizationEvent, bot_and_broadcaster_oauth_keep_chat_and_moderation_authority_separate(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_oauth_identity(), envelope_from_privmsg() (+47 more)

### Community 21 - "NavGrid"
Cohesion: 0.13
Nodes (20): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, OpenNode, path_routes_around_dynamic_building() (+12 more)

### Community 22 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 23 - "world.rs"
Cohesion: 0.11
Nodes (28): authored_foliage_is_deterministic_and_respects_habitat_and_resources(), AuthoredResourceLayer, avalanche_instance_hash(), cell_hash(), changing_seed_changes_world_hash(), fnv_mix(), foliage_visual_variant(), foliage_visual_yaw_milliradians() (+20 more)

### Community 24 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 25 - ".count"
Cohesion: 0.05
Nodes (76): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), append_terrain_quad(), append_terrain_skirt(), asset_root_collection_ready() (+68 more)

### Community 26 - "Node_SO"
Cohesion: 0.14
Nodes (13): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+5 more)

### Community 27 - "simulation.rs"
Cohesion: 0.05
Nodes (43): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value() (+35 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "DataStructures"
Cohesion: 0.29
Nodes (4): int, ChangeTimeStamp, DataStructures, DateTime

### Community 30 - "Res"
Cohesion: 0.04
Nodes (179): AccessibilityFocusVisualQuery, AppExit, DirectBroadcastControl, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), AccessibilityRuntime, AccessibleButtonNodeQuery (+171 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (23): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+15 more)

### Community 32 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 33 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (15): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, Vector3 (+7 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 36 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "RenderAssets"
Cohesion: 0.04
Nodes (109): AccessibilityMotionDefaults, actor_material(), apply_authored_main_menu_camera(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension (+101 more)

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.06
Nodes (59): ContentError, Result, TechTree, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize (+51 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.18
Nodes (10): CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool, GameObject (+2 more)

### Community 43 - ".new"
Cohesion: 0.03
Nodes (156): AccessibilityActionRequest, generate_world(), generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_tagging_tolerates_ui_removed_before_deferred_annotation(), action_ranges_and_tower_acquisition_are_euclidean() (+148 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires(), controller() (+18 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "legacy.rs"
Cohesion: 0.11
Nodes (46): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id() (+38 more)

### Community 47 - "StableId"
Cohesion: 0.03
Nodes (234): GameConfig, GameplayConfig, BTreeMap, BuildingDef, ContentCatalog, EnemyDef, PassiveResourceContribution, ResourceReward (+226 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.17
Nodes (3): MenuItem, StreamTownSessionBridge, FrameCapture

### Community 52 - "Station"
Cohesion: 0.08
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "Value"
Cohesion: 0.19
Nodes (28): ActorCustomization, StreamUserType, should_show_actor_name(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies() (+20 more)

### Community 57 - "PoolableObject"
Cohesion: 0.05
Nodes (31): ProjectileShooter, float, int, string, Container, ContainerBuilder, GUIDProcessor, BoxCollider (+23 more)

### Community 58 - "MonoBehaviour"
Cohesion: 0.01
Nodes (107): Api, CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller (+99 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "String"
Cohesion: 0.17
Nodes (27): ability_choices(), action_animation_choices(), apply_building_draft(), building_model_node_choices(), buildings_tab(), delete_selected_building(), duplicate_selected_building(), equipment_node_choices() (+19 more)

### Community 61 - "Vec3"
Cohesion: 0.05
Nodes (63): AmbientLight, animate_healing_effects(), apply_authored_local_rotation(), BuildingEffectKind, BuildingEffectParticle, BuildingMaterialInstance, chimney_particle_scale(), ChimneySmokeEmitterRuntime (+55 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "TargetSensor"
Cohesion: 0.16
Nodes (4): bool, float, UnityEvent, TargetSensor

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

### Community 68 - "Objective"
Cohesion: 0.11
Nodes (6): int, string, ObjectiveSaveData, Action, int, Objective

### Community 69 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.05
Nodes (156): ArchetypesById, ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingModelDef, EnemyCampGenerationDef, EnemyModelSetDef, EnemyRunAnimation (+148 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - ".Log"
Cohesion: 0.07
Nodes (16): Container, ContainerBuilder, GameStateProcessor, Action, bool, CancellationToken, Container, ContainerBuilder (+8 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 78 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (63): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, AudioSettingsInstaller, ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, CampGenSettingsInstaller (+55 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Utils"
Cohesion: 0.04
Nodes (17): BuildCostModifier, InputButton, PlayerControls.ObjectSelection, Units, Utils, Target, Utils.Pooling, World (+9 more)

### Community 83 - "stream_operator_live_button"
Cohesion: 0.10
Nodes (23): BroadcastMetricsSnapshot, BroadcastPrerequisites, DirectBroadcastPhase, DirectBroadcastSnapshot, operator_live_button_label(), prepared_broadcast_can_start(), BackgroundColor, Query (+15 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 86 - "sync_stream_only_capture"
Cohesion: 0.17
Nodes (17): camera_targets_primary_window(), Assets, Commands, Entity, Handle, HashMap, Image, PrimaryWindow (+9 more)

### Community 87 - ".new"
Cohesion: 0.15
Nodes (15): BroadcastConfig, controller_counts_replaced_video_without_rejecting_the_newest_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), ending_stream_returns_the_operator_to_main_menu_after_shutdown(), operator_panel_uses_compact_telemetry_and_bottom_left_live_control(), operator_stop_cancels_an_in_flight_session_without_restarting_it(), Self, VecDeque (+7 more)

### Community 88 - "PlayerSettings"
Cohesion: 0.06
Nodes (61): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+53 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.20
Nodes (10): Bounds, Container, Dictionary, HashSet, Material, Matrix4x4, Mesh, FoliageProcessor (+2 more)

### Community 90 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 91 - "Enemy"
Cohesion: 0.06
Nodes (20): Action, float, Enemy, Animator, GameObject, IEnumerator, int, FishGodEvent (+12 more)

### Community 92 - "generate_world_from_layers"
Cohesion: 0.18
Nodes (23): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_grid_centre(), authored_world_to_grid(), generate_authored_resources(), generate_candidate_mask(), generate_foliage(), generate_shoreline_fish() (+15 more)

### Community 93 - "TargetProcessor"
Cohesion: 0.13
Nodes (10): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, List, TargetProcessor, Dictionary (+2 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (42): DepositResources, ResourceStorageModifier, float, int, PlayerInventory, Dictionary, ResourceInventory, bool (+34 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (22): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+14 more)

### Community 98 - "UnitTextDisplay"
Cohesion: 0.15
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (74): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+66 more)

### Community 100 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "Editor"
Cohesion: 0.04
Nodes (23): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, string (+15 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - ".RestoreWorldState"
Cohesion: 0.24
Nodes (4): float, int, TimeRuntimeData, IEnumerable

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "Self"
Cohesion: 0.17
Nodes (7): loading_progress_is_recursively_derived_from_real_work(), LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, Self, WorldLoadingWork

### Community 108 - "GUIDComponent"
Cohesion: 0.16
Nodes (10): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveableResource (+2 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "TownGoalProcessor.cs"
Cohesion: 0.33
Nodes (3): InputButton, SharedTypes, Data

### Community 112 - "DebugProcessor"
Cohesion: 0.09
Nodes (12): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, HideInCallstack, Object (+4 more)

### Community 113 - "Goal"
Cohesion: 0.10
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 114 - "TechTreeNode"
Cohesion: 0.12
Nodes (12): Color, Foldout, List, Sprite, Vector2, TechTreeNode, Port, Capacity (+4 more)

### Community 115 - "EnemyModelHandler"
Cohesion: 0.14
Nodes (8): bool, int, List, EnemyModelHandler, bool, float, Vector3, STSM_Action_EnemyAttack

### Community 116 - "stream_town_tools/src/main.rs"
Cohesion: 0.10
Nodes (40): authority_tab(), building_draft(), building_editor_preserves_the_complete_template_record(), BuildingDraft, character_model_choices_include_converted_hierarchy_nodes(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path() (+32 more)

### Community 117 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (29): Container, ContainerBuilder, LabelDisplayProcessor, bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller (+21 more)

### Community 118 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 119 - "Season"
Cohesion: 0.16
Nodes (11): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, bool, float (+3 more)

### Community 120 - "VoteEvent"
Cohesion: 0.09
Nodes (15): int, List, NewKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float (+7 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 123 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 126 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 129 - "BuildingSettings"
Cohesion: 0.15
Nodes (6): bool, Dictionary, int, BuildingSettings, ContainerBuilder, BuildingSettingsInstaller

### Community 131 - "Option"
Cohesion: 0.17
Nodes (31): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, BroadcastTarget, capture_process_audio(), discard_pending_audio() (+23 more)

### Community 132 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 133 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 134 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 135 - "TechTree.Elements"
Cohesion: 0.08
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 136 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 137 - "BuildingBase"
Cohesion: 0.08
Nodes (10): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, TargetableBuilding (+2 more)

### Community 138 - "EditorUtils"
Cohesion: 0.18
Nodes (5): Color, List, Texture2D, EditorUtils, DirectoryInfo

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "Result"
Cohesion: 0.13
Nodes (21): BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), copy_packed_video_frame(), encoder_input_format(), encoder_is_hardware(), inspect_broadcast_prerequisites(), linked_ffmpeg_encodes_h264_aac_flv_without_a_subprocess(), open_audio_encoder() (+13 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "WorldInstanceDeterminism"
Cohesion: 0.29
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Group, int, List, Port (+13 more)

### Community 147 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "FrameCapture"
Cohesion: 0.22
Nodes (10): bool, double, float, int, IReadOnlyList, List, long, string (+2 more)

### Community 154 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 155 - "String"
Cohesion: 0.08
Nodes (64): AnimationParameterDef, animation_take_name(), clip_id(), convert_clips(), convert_embedded_model_clips(), convert_post_process(), embedded_clip_id(), field_bool() (+56 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.12
Nodes (14): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, VisualElement, Button, EnumField, UnlockVisualElement (+6 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "PlayerSaveData"
Cohesion: 0.07
Nodes (24): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, bool (+16 more)

### Community 159 - "UserInterface_RulerVote"
Cohesion: 0.13
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

### Community 163 - "FoliageData"
Cohesion: 0.24
Nodes (6): List, Material, Mesh, Quaternion, Vector3, FoliageData

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 172 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 177 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 178 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 179 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (76): AvatarMaskDef, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert_avatar_masks(), convert_chimney_smoke() (+68 more)

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
Cohesion: 0.15
Nodes (7): AnimationHandler, Animator, bool, Dictionary, float, int, AnimationName

### Community 185 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (75): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, bool, ParticleSystem (+67 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (42): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuResourceVisual, Vec, adjacent_farm_tiles_share_one_flat_plateau(), adjacent_foundations_sample_the_unmodified_generated_surface() (+34 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 195 - "Sensors"
Cohesion: 0.07
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

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
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 210 - "GlobalAudioController"
Cohesion: 0.23
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 211 - "DirectBroadcastRuntime"
Cohesion: 0.11
Nodes (27): apply_direct_broadcast_control(), average_milliseconds(), bandwidth_test_never_claims_to_be_publicly_live(), begin_twitch_live_verification(), capture_direct_broadcast_frame(), DirectBroadcastRuntime, micros_to_milliseconds(), poll_direct_broadcast_authorization() (+19 more)

### Community 212 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 213 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

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

### Community 218 - ".RenderFoliageType"
Cohesion: 0.32
Nodes (6): Dictionary, int, Material, Matrix4x4, Mesh, FoliageRenderer

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "Vec"
Cohesion: 0.43
Nodes (8): binary_fixture(), BinaryParser, put_f32(), put_i32(), put_string(), put_u32(), Vec, Cursor

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

### Community 234 - "String"
Cohesion: 0.43
Nodes (7): foliage_horizontal_hash(), horizontal_hash(), resource_horizontal_hash(), Item, Iterator, String, shoreline_approaches()

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

### Community 251 - "STSM_StateAction"
Cohesion: 0.12
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "RoleHandler"
Cohesion: 0.03
Nodes (49): RoleSlotModifier, int, PlayerRoleData, AudioClip, bool, float, int, RoleData (+41 more)

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "GridProcessor"
Cohesion: 0.09
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 268 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

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
Cohesion: 0.40
Nodes (4): Vector3, UnityGraphics, FieldInfo, ShadowResolution

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "FoliageRuntimeData"
Cohesion: 0.33
Nodes (6): Dictionary, List, Material, Matrix4x4, Mesh, FoliageRuntimeData

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
Cohesion: 0.08
Nodes (17): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, AudioClip (+9 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "ToolState"
Cohesion: 0.10
Nodes (33): broadcast_encoder_label(), poll_tool_job_events(), poll_twitch_tool_events(), RoleDraft, Arc, Default, Duration, Mutex (+25 more)

### Community 285 - "Stream Town Twitch commands"
Cohesion: 0.40
Nodes (4): Moderator and game-master commands, Player commands, Ruler and operator commands, Stream Town Twitch commands

### Community 288 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 290 - "FoliageGenSettings"
Cohesion: 0.40
Nodes (4): ContainerBuilder, FoliageGenSettingsInstaller, List, FoliageGenSettings

### Community 291 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 292 - "WaterFoliageGenSettings"
Cohesion: 0.40
Nodes (4): ContainerBuilder, WaterFoliageGenSettingsInstaller, List, WaterFoliageGenSettings

### Community 293 - "Ui"
Cohesion: 0.16
Nodes (28): apply_role_draft(), content_tab(), delete_selected_role(), draw_world_preview(), duplicate_selected_role(), legacy_content_tab(), legacy_roles_tab(), migration_tab() (+20 more)

### Community 294 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 297 - "direct_broadcast.rs"
Cohesion: 0.07
Nodes (39): amf_quality_profile_keeps_static_grid_detail_between_keyframes(), AuthorizationEvent, bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), configure_amf_quality(), configure_direct_broadcast(), draw_centered_sensitive_label(), gpu_readback_padding_is_removed_without_corrupting_rows() (+31 more)

### Community 298 - "SeasonAudioData"
Cohesion: 0.57
Nodes (3): SeasonAudioData, AudioClip, List

### Community 299 - "Result"
Cohesion: 0.22
Nodes (25): apply_foliage_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot, commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group() (+17 more)

### Community 300 - "Processors"
Cohesion: 0.05
Nodes (26): ObjectSelectionProcessor, UserInterface.MainMenu, TownGoal.Data, Processors, Pets.Enumerations, StreamTown.EditorTools, TownGoal, Character.Enumerations (+18 more)

### Community 302 - "verify_twitch_public_stream"
Cohesion: 0.33
Nodes (5): DirectTwitchBroadcastPlugin, LiveVerificationTarget, App, Plugin, verify_twitch_public_stream()

### Community 303 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.04
Nodes (30): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, Action (+22 more)

### Community 308 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 318 - "VideoCadence"
Cohesion: 0.16
Nodes (10): CadenceTick, duration_as_micros(), Duration, Error, Instant, twitch_live_request_timeout(), video_cadence_skips_stale_slots_instead_of_bursting_after_a_stall(), VideoCadence (+2 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **382 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+377 more)
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

- **Why does `Utils` connect `Utils` to `DayAndNightProcessor`, `TechTree.Elements`, `CommonEnums.cs`, `DataStructures`, `GenerationSettings`, `UpdateGraphBounds`, `Processors`, `StringUtils`, `MonoBehaviour`, `ScriptableObject`, `Sensors`, `ScriptablesProcessorInfrastructure`, `BuildingPlacer`, `Easings`, `SnapToGridMouseMovement`, `FPSDisplay`, `UnitTextDisplay`, `Editor`, `TownGoalProcessor.cs`, `RandomEnabler`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `build_converted_animation`, `BTreeMap`, `Query`, `stream_town_game/src/lib.rs`, `config.rs`, `save.rs`, `Option`, `command.rs`, `twitch.rs`, `world.rs`, `.count`, `simulation.rs`, `String`, `ToolState`, `Res`, `Ui`, `RenderAssets`, `TechnologyGraphViewState`, `.new`, `AnimationControllerDef`, `runtime_console.rs`, `legacy.rs`, `Result`, `stream_town_migrate/src/presentation.rs`, `String`, `Vec3`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `stream_town_domain/src/presentation.rs`, `stream_town_tools/src/main.rs`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `RoleHandler`, `BuildingProcessor`, `DayAndNightProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `RoleDataContainer`, `UserInterface_Debug`, `WorldGenProcessor`, `TechTreeProcessor`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `.Log`, `UserInterface_TownVote`, `BuildingPlacer`, `Utils`, `Enemy`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `VoteEvent`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _382 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `build_converted_animation` be split into smaller, more focused modules?**
  _Cohesion score 0.0632996632996633 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07256894049346879 - nodes in this community are weakly interconnected._
- **Should `BTreeMap` be split into smaller, more focused modules?**
  _Cohesion score 0.08862745098039215 - nodes in this community are weakly interconnected._