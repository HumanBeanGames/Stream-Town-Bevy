# Graph Report - Stream-Town-Bevy  (2026-08-27)

## Corpus Check
- 670 files · ~1,773,754 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8904 nodes · 25970 edges · 322 communities (298 shown, 24 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1040 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3137d8d5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UserInterface
- BuildingProcessor
- ScriptablesProcessorInfrastructure
- PlayerSaveData
- Res
- setup_rendering
- TwitchChatProcessor
- finish_world_reveal
- BottomBarInterface
- stream_town_game/src/lib.rs
- SettingsProcessor
- Result
- Result
- simulation.rs
- TechTreeIOUtility
- HealthHandler
- CellSpacePartitioning
- save.rs
- Option
- command.rs
- Result
- Access_Dropdown
- STSM_Idle_Player
- RenderAssets
- SensorBase
- PlayerSettings
- BinaryReader
- StableId
- SaveFileData
- world.rs
- ResMut
- WorldGenProcessor
- .Log
- stream_town_domain/src/content.rs
- GenerationSettings
- .SetTargetType
- BinarySaveCodec
- SettingsData
- IProcessor
- TechnologyGraphViewState
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/main.rs
- AnimationControllerDef
- runtime_console.rs
- legacy.rs
- SelectedPlayer
- ResourceDataSaveData
- ObjectPoolingProcessor
- AudioHandler
- StreamTownSessionBridge
- Pet
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- String
- PlayerRoleData
- World.Generation.Settings
- TechTreeEditorWindow
- BuildingBase
- TransformSaveData
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- ToolState
- SaveProcessor
- .new
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- .RestoreWorldState
- UserInterface_ObjectSelection
- PoolableObject
- UserInterface_TownVote
- TechTreeGraphView
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Node_SO
- Handle
- Access_Text
- SelectedBuilding
- sync_stream_only_capture
- Goal
- TechTreeRuntimeData
- FoliageProcessor
- tools_ui
- .AddEvent
- config.rs
- MonoBehaviour
- convert_fbx_to_glb.py
- STSM_Helper_Build
- Resource
- .LoadGameAsync
- STSM_StateAction
- stream_town_domain/src/presentation.rs
- .Draw
- StateMachine
- ResourceRuntimeData
- TownGoalProcessor
- MainMenuManager
- ResourceProcessor
- LoadingManager
- Station
- WindController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- VfxSeagullSpawner
- Character
- Result
- GridNode
- twitch.rs
- GameEventProcessor
- .new
- VoteEvent
- unity_color_filter
- GUIDComponent
- SnapToGridMouseMovement
- AIPath
- ObjectiveDef
- GateController
- DirectBroadcastRuntime
- stream_town_domain/src/lib.rs
- AnimationHandler
- encode_broadcast_session
- StringUtils
- Enemy
- UnitTextDisplay
- TechTree.Elements
- Coordinator
- GameEvent
- stream_town_migrate/src/presentation.rs
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- World.Generation
- RuntimeData Template
- Key Rules
- TechTreeNode
- .LoadSceneAsync
- xtask/src/lib.rs
- Targetable
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- Sensors
- UserInterface_RulerVote
- BTreeMap
- ObjectiveSaveData
- Stream Town Reloaded - Architecture Documentation
- STSM_Helper_Attack
- MiscCommands
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- TwitchTransport
- CredentialVault
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- .RenderResourceType
- Q: There are still no animations.
- xtask/src/main.rs
- List
- CreditsProcessor
- RandomEnabler
- BuildingModelHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- .tick
- GridProcessor
- CommonEnums.cs
- Access_Toggle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ResourceHolder
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- TechNodeData
- WorldGenerationReferenceExporter
- ScriptableObject
- ProjectCamera
- BevyMigrationExporter
- CommandDictionary
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- PlayerProcessor
- Target
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
- DebugProcessor
- Requirement
- SimpleMusicController
- SensorProcessor
- STSM_HelperBase
- Key Rules
- FoliageGenerationSettings
- RuntimeData Template
- Character Animation Regression Checklist
- SelectedObject
- ScriptKeywordProcessor
- FPSDisplay
- SelectedResource
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
- Access_GOList
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- technology_tab
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- EquipmentHandlerEditor
- extraction-spec.md
- UI_TechOption
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- RoleHandler
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- Editor
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- SimpleDisableAfterTime
- Q: If there is more to do, keep going.
- SelectedEnemy
- Autosave
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- .GetResourceAssets
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- UIElementWrapper
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxAnimationController
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- stream_town_tools/src/main.rs
- UserInterface_TownGoal
- StreamTown.Migration
- KeepKingVote
- TargetProcessor
- stream_operator_live_button
- Processors
- SelectedEnemyCamp
- record_gpu_readiness
- CreateProjectScopeProcessors.cs
- Utils
- capture_direct_broadcast_frame
- direct_broadcast.rs
- String
- DontDestroyOnLoad
- RotationHandler
- BuildingDataSettings
- VFXArrowPointer
- TL_API
- NewKingVote
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- EnemyModelHandler
- runtime_tab
- SimpleScreenShot
- format_game_master_ids
- BuildPlacerData
- UnityGraphics
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- TownEvent
- preview_lerp_color
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- .on_frame_arrived
- WorldGenSaveData
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 360 edges
2. `WorldSimulation` - 166 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ContentCatalog` - 151 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 134 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `handle_twitch_event()` --calls--> `unity_command_usage()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_assignment_penalty_spreads_farmers_across_farms()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (322 total, 24 thin omitted)

### Community 0 - "UserInterface"
Cohesion: 0.05
Nodes (22): InputButton, SharedTypes, int, ChangeTimeStamp, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+14 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 3 - "PlayerSaveData"
Cohesion: 0.06
Nodes (24): List, Component, PetType, Dictionary, List, Mesh, Transform, Vector3 (+16 more)

### Community 4 - "Res"
Cohesion: 0.03
Nodes (241): Aabb, AccessibilityNode, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink (+233 more)

### Community 5 - "setup_rendering"
Cohesion: 0.05
Nodes (74): AmbientLight, ActiveMaterialHandles, apply_material_overrides(), authored_post_process_stack(), building_material(), building_snow_strength(), BuildingMaterialInstance, character_material() (+66 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (22): Container, ContainerBuilder, TimeProcessor, bool, float, Func, int, PlayerExistsByIDDelegate (+14 more)

### Community 7 - "finish_world_reveal"
Cohesion: 0.04
Nodes (71): AssetId, advance_loading_phase(), advance_loading_runtime(), append_terrain_quad(), append_terrain_skirt(), asset_root_collection_ready(), authored_rotating_node_names(), autosave_game() (+63 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.05
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (338): AccessibleNode, AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, AnimationTransitionPlayback, accessibility_settings_selection(), AccessibilityActionDispatch (+330 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 12 - "Result"
Cohesion: 0.09
Nodes (61): AnimationFloatKeyframe, AnimationTangent, append_vec3_keys(), convert_chimney_smoke(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id() (+53 more)

### Community 13 - "simulation.rs"
Cohesion: 0.08
Nodes (25): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+17 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (12): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, STSM_Action_Heal, Action, bool, float (+4 more)

### Community 16 - "CellSpacePartitioning"
Cohesion: 0.12
Nodes (13): Bounds, CellPartitioningEditor, bool, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "Option"
Cohesion: 0.04
Nodes (194): GameConfig, ContentCatalog, RoleDef, BTreeSet, StationDef, GridPos, ActorState, RoleProgress (+186 more)

### Community 19 - "command.rs"
Cohesion: 0.19
Nodes (29): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+21 more)

### Community 20 - "Result"
Cohesion: 0.18
Nodes (11): DeviceAuthorization, OAuthClient, Client, Debug, Formatter, Result, Vec, StoredOAuthToken (+3 more)

### Community 21 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 22 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (21): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+13 more)

### Community 23 - "RenderAssets"
Cohesion: 0.05
Nodes (153): ArchetypeDef, ArchetypeKind, ArchetypeScene, PetDef, PetModelDef, PresentationCatalog, actor_material(), actor_scene_budget() (+145 more)

### Community 24 - "SensorBase"
Cohesion: 0.19
Nodes (4): float, List, SensorRuntimeData, SensorBase

### Community 25 - "PlayerSettings"
Cohesion: 0.06
Nodes (62): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+54 more)

### Community 26 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 27 - "StableId"
Cohesion: 0.12
Nodes (21): Display, FromStr, StableId, complete_gameplay_scenario_round_trips(), EnemyCampState, RaidState, BTreeSet, Result (+13 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "world.rs"
Cohesion: 0.05
Nodes (79): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+71 more)

### Community 30 - "ResMut"
Cohesion: 0.04
Nodes (139): AccessibilityFocusVisualQuery, AnyResult, AppExit, DirectBroadcastControl, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus() (+131 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - ".Log"
Cohesion: 0.05
Nodes (19): Action, ProcessorStartupContext, HideInCallstack, Object, DebugLogCategory, Action, CancellationToken, LoadSceneMode (+11 more)

### Community 33 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (43): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, EnemyDef, EnemyModelSetDef, EnemyRunAnimation (+35 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.15
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "BinarySaveCodec"
Cohesion: 0.15
Nodes (7): Action, int, UTF8Encoding, BinarySaveCodec, int, PlayerRoleSaveData, BinaryWriter

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "IProcessor"
Cohesion: 0.04
Nodes (39): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+31 more)

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.06
Nodes (59): ContentError, Result, TechTree, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize (+51 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.07
Nodes (20): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+12 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.11
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.14
Nodes (11): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (29): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds(), consumes_trigger_when_any_state_transition_fires() (+21 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "legacy.rs"
Cohesion: 0.15
Nodes (42): ActorCustomization, StreamUserType, binary_fixture(), BinaryParser, clamped_cell(), decode_json(), json_active_goal(), json_buildings() (+34 more)

### Community 47 - "SelectedPlayer"
Cohesion: 0.10
Nodes (4): List, SelectedPlayer, List, SelectedPlayerGroup

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.24
Nodes (10): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+2 more)

### Community 49 - "ObjectPoolingProcessor"
Cohesion: 0.09
Nodes (12): Container, ContainerBuilder, LabelDisplayProcessor, bool, Container, ContainerBuilder, float, int (+4 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "Pet"
Cohesion: 0.14
Nodes (8): bool, Dictionary, float, Transform, Pet, Animator, int, PetModel

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "String"
Cohesion: 0.17
Nodes (23): ActorKind, actor_prefix(), content_id(), convert(), duration_days(), entity_id(), ImportReport, legacy_objective_matches() (+15 more)

### Community 57 - "PlayerRoleData"
Cohesion: 0.10
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 58 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "BuildingBase"
Cohesion: 0.10
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 61 - "TransformSaveData"
Cohesion: 0.13
Nodes (12): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+4 more)

### Community 62 - "CameraController"
Cohesion: 0.06
Nodes (17): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+9 more)

### Community 63 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.08
Nodes (14): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+6 more)

### Community 67 - "ToolState"
Cohesion: 0.15
Nodes (23): Arc, Default, Duration, Mutex, Receiver, Sender, Vec, start_twitch_authorization() (+15 more)

### Community 68 - "SaveProcessor"
Cohesion: 0.05
Nodes (13): Container, ContainerBuilder, GUIDProcessor, Container, ContainerBuilder, float, SaveProcessor, int (+5 more)

### Community 69 - ".new"
Cohesion: 0.15
Nodes (18): absolute_path(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), conversion_preserves_mesh_and_relocates_invalid_positions(), conversion_rejects_malformed_retained_mesh(), decode_binary(), decode_legacy() (+10 more)

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (130): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+122 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - ".RestoreWorldState"
Cohesion: 0.20
Nodes (4): float, int, TimeRuntimeData, IEnumerable

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "PoolableObject"
Cohesion: 0.07
Nodes (24): BoxCollider, List, Quaternion, Vector3, bool, Dictionary, GUIDRuntimeData, Dictionary (+16 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 78 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "Node_SO"
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 83 - "Handle"
Cohesion: 0.04
Nodes (92): BackgroundColor, AccessibilityMotionDefaults, authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension (+84 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 86 - "sync_stream_only_capture"
Cohesion: 0.18
Nodes (16): camera_targets_primary_window(), Assets, Commands, Entity, Handle, HashMap, Image, PrimaryWindow (+8 more)

### Community 87 - "Goal"
Cohesion: 0.16
Nodes (4): EventType, Action, Dictionary, Goal

### Community 88 - "TechTreeRuntimeData"
Cohesion: 0.16
Nodes (5): bool, Dictionary, float, int, TechTreeRuntimeData

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "tools_ui"
Cohesion: 0.19
Nodes (21): content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), poll_tool_job_events(), preview_grid_point(), role_i32(), role_u16() (+13 more)

### Community 92 - "config.rs"
Cohesion: 0.10
Nodes (30): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig (+22 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (111): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+103 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "STSM_Helper_Build"
Cohesion: 0.13
Nodes (6): AttackUnit, int, STSM_Helper_Build, STSM_Action_Build, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (37): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+29 more)

### Community 97 - ".LoadGameAsync"
Cohesion: 0.12
Nodes (14): Action, CancellationToken, List, Task, int, string, uint, EnemySaveData (+6 more)

### Community 98 - "STSM_StateAction"
Cohesion: 0.19
Nodes (5): bool, float, int, STSM_StateAction, AnimationName

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (73): AnimationConditionMode, AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe (+65 more)

### Community 100 - ".Draw"
Cohesion: 0.11
Nodes (19): NodeUnlockSaveData, Port, Button, EnumField, UnlockVisualElement, Action, Button, EnumField (+11 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.10
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.07
Nodes (27): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+19 more)

### Community 106 - "LoadingManager"
Cohesion: 0.10
Nodes (14): Dictionary, float, GameObject, Image, string, TextMeshProUGUI, Transform, LoadingManager (+6 more)

### Community 107 - "Station"
Cohesion: 0.07
Nodes (15): Station, Dictionary, float, int, List, Queue, Transform, List (+7 more)

### Community 108 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

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
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "Character"
Cohesion: 0.06
Nodes (21): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+13 more)

### Community 115 - "Result"
Cohesion: 0.36
Nodes (16): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role(), delete_selected_technology_group() (+8 more)

### Community 116 - "GridNode"
Cohesion: 0.11
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 117 - "twitch.rs"
Cohesion: 0.15
Nodes (18): channel_point_reward_tag_survives_privmsg_conversion(), envelope_from_privmsg(), IngestsResponse, message_confirms_channel_join(), OAuthErrorResponse, Option, StreamKeyData, StreamKeyResponse (+10 more)

### Community 118 - "GameEventProcessor"
Cohesion: 0.09
Nodes (11): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType, EventTester (+3 more)

### Community 119 - ".new"
Cohesion: 0.03
Nodes (146): AccessibilityActionRequest, generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), action_ranges_and_tower_acquisition_are_euclidean(), agent_facing_matches_unity_rotation_and_action_targets(), animated_character_receiver_scope_follows_only_the_player_rig_hierarchy() (+138 more)

### Community 120 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 126 - "ObjectiveDef"
Cohesion: 0.17
Nodes (12): ObjectiveDef, ObjectiveKind, objective_increment(), ObjectiveEvent, ObjectiveProgress, BTreeMap, Vec, RulerVoteKind (+4 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "DirectBroadcastRuntime"
Cohesion: 0.13
Nodes (23): apply_direct_broadcast_control(), AuthorizationEvent, BroadcastPrerequisites, BroadcastTarget, CadenceTick, configure_direct_broadcast(), DirectBroadcastPhase, DirectBroadcastRuntime (+15 more)

### Community 129 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 130 - "AnimationHandler"
Cohesion: 0.08
Nodes (14): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+6 more)

### Community 131 - "encode_broadcast_session"
Cohesion: 0.21
Nodes (27): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, capture_process_audio(), discard_pending_audio(), encode_broadcast_session() (+19 more)

### Community 133 - "Enemy"
Cohesion: 0.06
Nodes (20): Action, float, Enemy, float, int, List, Transform, EnemySpawner (+12 more)

### Community 134 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.07
Nodes (19): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+11 more)

### Community 136 - "Coordinator"
Cohesion: 0.09
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 137 - "GameEvent"
Cohesion: 0.04
Nodes (27): Transform, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+19 more)

### Community 138 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (87): animation_state_id(), animation_state_machine_id(), animation_take_name(), animator_component(), animator_reference_path(), avatar_mask_id(), clip_id(), color_value() (+79 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "Result"
Cohesion: 0.09
Nodes (31): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), copy_packed_video_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), encoder_candidates() (+23 more)

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

### Community 146 - "TechTreeNode"
Cohesion: 0.11
Nodes (13): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Group (+5 more)

### Community 147 - ".LoadSceneAsync"
Cohesion: 0.21
Nodes (5): Task, bool, float, string, LoadingProgressReporter

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "Targetable"
Cohesion: 0.14
Nodes (7): bool, BoxCollider, float, int, Transform, Vector3, Targetable

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Sensors"
Cohesion: 0.07
Nodes (12): HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours, Animation (+4 more)

### Community 154 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 155 - "BTreeMap"
Cohesion: 0.12
Nodes (42): AnimationClipDef, MaterialDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks(), convert_clips() (+34 more)

### Community 156 - "ObjectiveSaveData"
Cohesion: 0.15
Nodes (10): List, Vector2, NodeSaveData, Button, EnumField, ObjectiveVisualElement, ObjectiveSaveData, ObjectiveType (+2 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "STSM_Helper_Attack"
Cohesion: 0.18
Nodes (4): int, STSM_Helper_Attack, int, STSM_Action_Attack

### Community 159 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "TwitchTransport"
Cohesion: 0.15
Nodes (13): ensure_bot_identity(), Arc, Drop, Mutex, Receiver, Sender, run_transport(), TwitchControl (+5 more)

### Community 163 - "CredentialVault"
Cohesion: 0.39
Nodes (3): bot_and_broadcaster_tokens_use_distinct_vault_entries(), broadcaster_oauth_uses_only_the_stream_key_scope(), CredentialVault

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 167 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 171 - "CreditsProcessor"
Cohesion: 0.22
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 172 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 173 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - ".tick"
Cohesion: 0.29
Nodes (3): healing_and_food_revives_preserve_health_invariants(), ruler_vote_rejects_duplicates_and_invalid_candidates(), scheduled_ruler_elections_pause_resolve_and_restore_roles()

### Community 177 - "GridProcessor"
Cohesion: 0.13
Nodes (9): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, Container, ContainerBuilder (+1 more)

### Community 178 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (21): Vector3, List, List, Dictionary, List, TargetRuntimeData, TargetableData, Dictionary (+13 more)

### Community 179 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ResourceHolder"
Cohesion: 0.24
Nodes (5): AnimationCurve, bool, int, object, ResourceHolder

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "TechNodeData"
Cohesion: 0.29
Nodes (3): List, Node_SO, TechNodeData

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.03
Nodes (80): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+72 more)

### Community 188 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.10
Nodes (47): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+39 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "PlayerProcessor"
Cohesion: 0.07
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnChatCommandReceivedArgs (+6 more)

### Community 195 - "Target"
Cohesion: 0.09
Nodes (10): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Pets, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects (+2 more)

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
Cohesion: 0.18
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

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

### Community 208 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.11
Nodes (9): CancellationToken, Task, CancellationToken, Exception, Task, ProcessorStartupReport, ProcessorStartupStage, Dictionary (+1 more)

### Community 209 - "DebugProcessor"
Cohesion: 0.05
Nodes (27): CollectResource, Dictionary, DebugSettings, Container, ContainerBuilder, DebugProcessor, Container, ContainerBuilder (+19 more)

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 212 - "SensorProcessor"
Cohesion: 0.31
Nodes (3): Container, ContainerBuilder, SensorProcessor

### Community 213 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "FoliageGenerationSettings"
Cohesion: 0.14
Nodes (12): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings, int (+4 more)

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

### Community 234 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

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

### Community 243 - "technology_tab"
Cohesion: 0.36
Nodes (9): authoring_snapshot(), AuthoringSnapshot, push_authoring_undo(), redo_authoring_edit(), refresh_catalog_drafts(), refresh_foliage_draft(), reload_content_catalog(), technology_tab() (+1 more)

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

### Community 251 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 253 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "RoleHandler"
Cohesion: 0.04
Nodes (41): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+33 more)

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
Cohesion: 0.10
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

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

### Community 273 - ".GetResourceAssets"
Cohesion: 0.36
Nodes (4): Material, materials, Mesh, meshes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "UIElementWrapper"
Cohesion: 0.12
Nodes (9): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_TextInput, TMP_InputField, ContainerBuilder (+1 more)

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
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "stream_town_tools/src/main.rs"
Cohesion: 0.12
Nodes (36): authority_tab(), broadcast_encoder_label(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips() (+28 more)

### Community 285 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 289 - "TargetProcessor"
Cohesion: 0.32
Nodes (3): Container, ContainerBuilder, TargetProcessor

### Community 290 - "stream_operator_live_button"
Cohesion: 0.17
Nodes (9): BroadcastMetricsSnapshot, Query, Text, With, stream_operator_live_button(), StreamOperatorInfoText, StreamOperatorLiveButtonText, update_stream_operator_info() (+1 more)

### Community 291 - "Processors"
Cohesion: 0.05
Nodes (12): ObjectSelectionProcessor, InputButton, Transform, PlayerSpawnPoint, UserInterface.MainMenu, Processors, World, Processors.Editor (+4 more)

### Community 293 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "Utils"
Cohesion: 0.04
Nodes (17): BuildCostModifier, RoleScriptablesEditor, DisableOnAwake, SelectionBase, List, SimpleEventOnStart, SimpleHideRendererOnAwake, GameObject (+9 more)

### Community 296 - "capture_direct_broadcast_frame"
Cohesion: 0.29
Nodes (6): capture_direct_broadcast_frame(), publish_stream_only_frame(), On, Time, SensitiveScreenActive, ReadbackComplete

### Community 297 - "direct_broadcast.rs"
Cohesion: 0.10
Nodes (24): average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), DirectTwitchBroadcastPlugin, draw_centered_sensitive_label(), gpu_readback_padding_is_removed_without_corrupting_rows(), ingest(), ingest_selection_prefers_default_or_named_region() (+16 more)

### Community 298 - "String"
Cohesion: 0.46
Nodes (4): Into, Self, String, I

### Community 300 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 301 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 302 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 304 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.05
Nodes (14): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, OnChatCommandReceivedArgs (+6 more)

### Community 307 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 308 - "runtime_tab"
Cohesion: 0.33
Nodes (7): format_runtime_frame_times(), inject_runtime_command(), launch_runtime_game(), poll_runtime_console(), runtime_actions_sequence_after_latest_acknowledgement(), runtime_tab(), send_runtime_action()

### Community 309 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 310 - "format_game_master_ids"
Cohesion: 0.40
Nodes (6): format_game_master_ids(), parse_game_master_ids(), poll_twitch_tool_events(), BTreeSet, sync_twitch_tool_fields(), twitch_tool_normalizes_game_master_ids_and_reward_field()

### Community 311 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 312 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 315 - "preview_lerp_color"
Cohesion: 1.00
Nodes (3): preview_lerp_color(), Color32, terrain_preview_color()

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 318 - ".on_frame_arrived"
Cohesion: 0.28
Nodes (5): duration_as_micros(), Duration, Error, CapturedWindowFrame, InternalCaptureControl

### Community 319 - "WorldGenSaveData"
Cohesion: 0.09
Nodes (17): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+9 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **365 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+360 more)
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

- **Why does `Utils` connect `Utils` to `UserInterface`, `ScriptablesProcessorInfrastructure`, `StringUtils`, `Enemy`, `UnitTextDisplay`, `TechTree.Elements`, `SimpleDisableAfterTime`, `World.Generation`, `Sensors`, `ObjectiveSaveData`, `GenerationSettings`, `Processors`, `UpdateGraphBounds`, `RandomEnabler`, `CommonEnums.cs`, `SimpleScreenShot`, `CameraController`, `Target`, `WorldSaveData`, `BuildingPlacer`, `FPSDisplay`, `Character`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `RoleHandler`, `BuildingProcessor`, `Enemy`, `TwitchChatProcessor`, `BottomBarInterface`, `GameEvent`, `HealthHandler`, `WorldGenProcessor`, `.Log`, `IProcessor`, `TechTreeProcessor`, `SelectedPlayer`, `NewKingVote`, `ObjectPoolingProcessor`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `PlayerRoleData`, `Target`, `SaveProcessor`, `UserInterface_TownVote`, `BuildingPlacer`, `.AddEvent`, `MonoBehaviour`, `Resource`, `GameEventProcessor`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `stream_town_domain/src/lib.rs`, `Res`, `setup_rendering`, `stream_town_game/src/lib.rs`, `stream_town_migrate/src/presentation.rs`, `Result`, `simulation.rs`, `save.rs`, `Option`, `command.rs`, `RenderAssets`, `BTreeMap`, `stream_town_tools/src/main.rs`, `world.rs`, `stream_town_domain/src/content.rs`, `TechnologyGraphViewState`, `AnimationControllerDef`, `runtime_console.rs`, `.tick`, `String`, `TownEvent`, `stream_town_migrate/src/menu_scene.rs`, `ToolState`, `stream_town_migrate/src/content.rs`, `Handle`, `tools_ui`, `config.rs`, `stream_town_domain/src/presentation.rs`, `Result`, `twitch.rs`, `.new`, `ObjectiveDef`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _365 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `UserInterface` be split into smaller, more focused modules?**
  _Cohesion score 0.046365914786967416 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05803571428571429 - nodes in this community are weakly interconnected._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.08098891730605286 - nodes in this community are weakly interconnected._