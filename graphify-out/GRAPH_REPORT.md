# Graph Report - Stream-Town-Bevy  (2026-08-27)

## Corpus Check
- 670 files · ~1,768,549 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8811 nodes · 25625 edges · 321 communities (297 shown, 24 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1035 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `26b09be4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- TownGoal.Data
- BuildingProcessor
- generate_world_from_layers
- PlayerSaveData
- Query
- BuildingScriptablesEditor.cs
- TwitchChatProcessor
- LoadingWorkNode
- BottomBarInterface
- .new
- SettingsProcessor
- setup_rendering
- Result
- NavGrid
- TechTreeIOUtility
- HealthHandler
- CellSpacePartitioning
- save.rs
- StableId
- GlobalAudioController
- twitch.rs
- Access_Dropdown
- STSM_GoToLocation
- Option
- StationProcessor
- command.rs
- BinarySaveCodec
- simulation.rs
- SaveFileData
- world.rs
- Res
- WorldGenProcessor
- UserInterface_Debug
- IRuntimeDataScriptable
- GenerationSettings
- .SetTargetType
- BuildingResourceModelHandler
- SettingsData
- SeasonProcessor
- TechnologyGraphViewState
- ObjectSelectionProcessor
- TechTreeProcessor
- PlayerRole
- PlayerProcessor
- AnimationControllerDef
- runtime_console.rs
- UserInterface_RulerVote
- RoleHandler
- ResourceDataSaveData
- SensorProcessor
- AudioHandler
- StreamTownSessionBridge
- Pet
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- legacy.rs
- PlayerRoleData
- World.Generation.Settings
- TechTreeEditorWindow
- Result
- TransformSaveData
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- RoleDataContainer
- Objective
- UserInterface_Roles
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- TimeProcessor
- UserInterface_ObjectSelection
- ObjectPoolingProcessor
- UserInterface_TownVote
- TechTreeGraphView
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Node_SO
- Handle
- Access_Text
- PlayerInventory
- stream_town_game/src/lib.rs
- Goal
- MiscCommands
- FoliageProcessor
- tools_ui
- GameEvent
- config.rs
- MonoBehaviour
- convert_fbx_to_glb.py
- STSM_Idle_Player
- Resource
- SaveProcessor
- CommonEnums.cs
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
- TwitchUser
- stream_town_tools/src/main.rs
- GridProcessor
- stream_town_migrate/src/main.rs
- GameEventProcessor
- Vec
- VoteEvent
- unity_color_filter
- GUIDComponent
- SnapToGridMouseMovement
- AIPath
- GameStateProcessor
- GateController
- DirectBroadcastRuntime
- WeatherProcessor
- AnimationHandler
- encode_broadcast_session
- StringUtils
- EnemySpawner
- LabelDisplayProcessor
- TechTree.Elements
- Coordinator
- SelectableObject
- String
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- String
- RuntimeData Template
- Key Rules
- TechTreeNode
- RoleData
- xtask/src/lib.rs
- Targetable
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- HealthModifier
- PlayerInputProcessor
- stream_town_migrate/src/presentation.rs
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- ResourceTarget
- BroadcastConfig
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- .GenerateFromSettings
- EventProcessor
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- ResourceData
- .RenderResourceType
- Q: There are still no animations.
- xtask/src/main.rs
- List
- RandomEnabler
- BuildingModelHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- .Update
- PassiveResourceIncrementer
- KeepKingVote
- Access_Toggle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- NewKingVote
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SimpleScreenShot
- ErrorData
- WorldGenerationReferenceExporter
- ScriptableObject
- ProjectCamera
- BevyMigrationExporter
- CommandDictionary
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- .new
- UserInterface_GameMenu
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
- IProcessor
- DebugProcessor
- Requirement
- SimpleMusicController
- IProcessor.cs
- Easings
- Key Rules
- ResourceHolder
- RuntimeData Template
- Character Animation Regression Checklist
- SelectedBuilding
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
- parse_model_clip_events
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- ObjectiveSaveData
- extraction-spec.md
- UI_TechOption
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- RoleProcessor
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
- VideoCadence
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- TradeProcessor
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- LabelDisplayProcessor.cs
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- .default
- UIRoleDisplay
- StreamTown.Migration
- Access_TextInput
- import_save
- OpenNode
- Coordinator.cs
- record_gpu_readiness
- CreateDefaultSettingsAssets.cs
- PlayerSpawnPoint
- PlayerSettings
- direct_broadcast.rs
- PlayerInputRuntimeData
- AudioMixerInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- PlacementProbeHandler
- ScriptableObjectAssetData
- ObjectSelectionProcessor.Editor.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- RotationHandler
- CellPartitioningEditor
- UI_Objective
- SimpleLookAtCamera
- Utils
- FoliageGenerationSettings.cs
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- .InjectRuntimeData
- .RefreshSceneData
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- WorldGenSaveData
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 358 edges
2. `WorldSimulation` - 166 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ContentCatalog` - 151 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 132 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `handle_twitch_event()` --calls--> `unity_command_usage()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (321 total, 24 thin omitted)

### Community 0 - "TownGoal.Data"
Cohesion: 0.09
Nodes (10): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (36): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, TilerBuilding (+28 more)

### Community 2 - "generate_world_from_layers"
Cohesion: 0.19
Nodes (22): WorldGenConfig, authored_grid_centre(), authored_world_to_grid(), foliage_horizontal_hash(), generate_authored_resources(), generate_candidate_mask(), generate_foliage(), generate_world_from_layers() (+14 more)

### Community 3 - "PlayerSaveData"
Cohesion: 0.11
Nodes (16): Dictionary, bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int (+8 more)

### Community 4 - "Query"
Cohesion: 0.04
Nodes (154): Aabb, Added, AnimatedBy, AnimationGraph, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, Assets (+146 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (24): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+16 more)

### Community 7 - "LoadingWorkNode"
Cohesion: 0.36
Nodes (5): LoadingWork, LoadingWorkNode, main_menu_loading_progress(), IntoIterator, WorldLoadingWork

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - ".new"
Cohesion: 0.03
Nodes (178): AccessibilityActionRequest, ArchetypeKind, generate_world(), generate_world_with_content(), accessibility_navigation_preserves_editable_text_focus(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), append_terrain_skirt() (+170 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "setup_rendering"
Cohesion: 0.05
Nodes (73): AmbientLight, ActiveMaterialHandles, authored_post_process_stack(), building_damage_intensity(), building_damage_value(), building_material(), building_snow_strength(), BuildingMaterialInstance (+65 more)

### Community 12 - "Result"
Cohesion: 0.11
Nodes (49): AnimationFloatKeyframe, AnimationPropertyCurve, AnimationTangent, append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id() (+41 more)

### Community 13 - "NavGrid"
Cohesion: 0.17
Nodes (16): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, path_routes_around_dynamic_building() (+8 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.07
Nodes (17): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy (+9 more)

### Community 16 - "CellSpacePartitioning"
Cohesion: 0.14
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (35): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+27 more)

### Community 18 - "StableId"
Cohesion: 0.04
Nodes (209): GameConfig, GameplayConfig, BTreeMap, BuildingDef, ContentCatalog, EnemyDef, PassiveResourceContribution, ResourceReward (+201 more)

### Community 19 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 20 - "twitch.rs"
Cohesion: 0.07
Nodes (51): SecretsAuthorizationEvent, bot_and_broadcaster_tokens_use_distinct_vault_entries(), broadcaster_oauth_uses_only_the_stream_key_scope(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg() (+43 more)

### Community 21 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 22 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 23 - "Option"
Cohesion: 0.05
Nodes (159): ArchetypeDef, ArchetypeScene, PetDef, PetModelDef, PresentationCatalog, actor_material(), advance_falling_fish(), AgentCommandQueue (+151 more)

### Community 24 - "StationProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 25 - "command.rs"
Cohesion: 0.11
Nodes (37): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+29 more)

### Community 26 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 27 - "simulation.rs"
Cohesion: 0.05
Nodes (42): ObjectiveDef, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather() (+34 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "world.rs"
Cohesion: 0.15
Nodes (22): AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash(), fnv_mix(), generate_shoreline_fish(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), hash_world() (+14 more)

### Community 30 - "Res"
Cohesion: 0.03
Nodes (225): AccessibilityFocusVisualQuery, AccessibilityNode, AppExit, AudioSink, DirectBroadcastControl, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active() (+217 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (19): Action, Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable (+11 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "IRuntimeDataScriptable"
Cohesion: 0.08
Nodes (21): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, float, ParticleSystem, SortedSet, Transform (+13 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.20
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.08
Nodes (19): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+11 more)

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.06
Nodes (59): ContentError, Result, TechTree, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize (+51 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "PlayerRole"
Cohesion: 0.10
Nodes (15): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+7 more)

### Community 43 - "PlayerProcessor"
Cohesion: 0.07
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 47 - "RoleHandler"
Cohesion: 0.10
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 50 - "AudioHandler"
Cohesion: 0.07
Nodes (16): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+8 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (12): bool, double, float, int, List, long, MenuItem, string (+4 more)

### Community 52 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.15
Nodes (41): ActorCustomization, StreamUserType, should_show_actor_name(), binary_fixture(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json() (+33 more)

### Community 57 - "PlayerRoleData"
Cohesion: 0.10
Nodes (11): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+3 more)

### Community 58 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.25
Nodes (7): BinaryParser<'a>, decode_binary(), FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (15): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+7 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "RoleDataContainer"
Cohesion: 0.12
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 68 - "Objective"
Cohesion: 0.09
Nodes (10): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 69 - "UserInterface_Roles"
Cohesion: 0.14
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.05
Nodes (157): ArchetypesById, ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingModelDef, EnemyModelSetDef, EnemyRunAnimation, EnemySpawnerDef (+149 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.12
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (39): Container, ContainerBuilder, GUIDProcessor, HideInCallstack, Object, Action, bool, BoxCollider (+31 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 78 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.14
Nodes (13): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+5 more)

### Community 83 - "Handle"
Cohesion: 0.03
Nodes (100): BackgroundColor, AccessibilityMotionDefaults, apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform (+92 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerInventory"
Cohesion: 0.17
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 86 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (308): AccessibleNode, AnyResult, AssetId, accessibility_motion_preferences_preserve_authored_parameters(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_settings_selection(), accessibility_should_clear_focus(), AccessibilityActionDispatch (+300 more)

### Community 87 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 88 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "tools_ui"
Cohesion: 0.22
Nodes (19): content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), preview_grid_point(), role_i32(), role_u16(), role_u32() (+11 more)

### Community 91 - "GameEvent"
Cohesion: 0.04
Nodes (27): Transform, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+19 more)

### Community 92 - "config.rs"
Cohesion: 0.11
Nodes (26): ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet, Default, Option (+18 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (113): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, Volume (+105 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): AttackUnit, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 96 - "Resource"
Cohesion: 0.06
Nodes (12): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, Container (+4 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (27): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+19 more)

### Community 98 - "CommonEnums.cs"
Cohesion: 0.13
Nodes (13): Dictionary, List, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, Seasons (+5 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (67): AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationQuatKeyframe, AnimationStateDef, AnimationStateMachineDef, AnimationTransformTrack, AnimationVec3Keyframe (+59 more)

### Community 100 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.14
Nodes (8): LoadType, MetaData, Button, GameObject, IEnumerator, int, MainMenuManager, Inject

### Community 105 - "ResourceProcessor"
Cohesion: 0.22
Nodes (5): Dictionary, materialIndex, Matrix4x4, meshIndex, ResourceProcessor

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "Station"
Cohesion: 0.09
Nodes (10): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+2 more)

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
Cohesion: 0.13
Nodes (10): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+2 more)

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 115 - "stream_town_tools/src/main.rs"
Cohesion: 0.08
Nodes (79): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot, broadcast_encoder_label(), commit_catalog_candidate(), create_technology_group() (+71 more)

### Community 116 - "GridProcessor"
Cohesion: 0.10
Nodes (13): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 119 - "Vec"
Cohesion: 0.06
Nodes (68): AnimationClip, AnimationNodeIndex, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), advance_animation_crossfade(), animation_event_occurrences() (+60 more)

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
Cohesion: 0.12
Nodes (20): bool, float, int, string, Type, Vector3, AIPath, AstarData (+12 more)

### Community 126 - "GameStateProcessor"
Cohesion: 0.19
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "DirectBroadcastRuntime"
Cohesion: 0.20
Nodes (14): capture_direct_broadcast_frame(), configure_direct_broadcast(), DirectBroadcastPhase, DirectBroadcastRuntime, DirectBroadcastSnapshot, poll_direct_broadcast_authorization(), poll_direct_broadcast_worker(), restart_direct_broadcast() (+6 more)

### Community 129 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 130 - "AnimationHandler"
Cohesion: 0.04
Nodes (32): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+24 more)

### Community 131 - "encode_broadcast_session"
Cohesion: 0.15
Nodes (17): AtomicBool, BroadcastController, BroadcastMetrics, BroadcastMetricsSnapshot, encode_broadcast_session(), MediaInput, Arc, AtomicU64 (+9 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.11
Nodes (10): float, int, List, Transform, EnemySpawner, float, ChanceObject, float (+2 more)

### Community 134 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 136 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 137 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 138 - "String"
Cohesion: 0.10
Nodes (49): AnimationParameterDef, animator_component(), animator_reference_path(), convert_post_process(), field_f32(), field_str(), field_u64(), field_value() (+41 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "Result"
Cohesion: 0.30
Nodes (8): BroadcastEncoder, inspect_broadcast_prerequisites(), linked_ffmpeg_encodes_h264_aac_flv_without_a_subprocess(), Formatter, Result, Context, Output, Rational

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "String"
Cohesion: 0.14
Nodes (27): ActorKind, actor_prefix(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), duration_days(), entity_id(), ImportReport (+19 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TechTreeNode"
Cohesion: 0.13
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 147 - "RoleData"
Cohesion: 0.14
Nodes (14): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+6 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "Targetable"
Cohesion: 0.09
Nodes (14): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+6 more)

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "HealthModifier"
Cohesion: 0.10
Nodes (9): HealthModifier, bool, float, GameObject, HealUnit, UnitTravelToPosition, Vector3, Projectile (+1 more)

### Community 154 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 155 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (89): AnimationClipDef, MaterialDef, PrefabPresentationBinding, animation_state_id(), animation_state_machine_id(), animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses() (+81 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "ResourceTarget"
Cohesion: 0.24
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 159 - "BroadcastConfig"
Cohesion: 0.22
Nodes (14): BroadcastConfig, BroadcastEncoderPreference, AuthorizationEvent, BroadcastTarget, capture_process_audio(), direct_broadcast_stays_offline_until_operator_requests_it(), encoder_candidates(), open_audio_encoder() (+6 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - ".GenerateFromSettings"
Cohesion: 0.15
Nodes (11): HashSet, Func, HashSet, Vector2, Vector3, int, Quaternion, uint (+3 more)

### Community 163 - "EventProcessor"
Cohesion: 0.22
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "ResourceData"
Cohesion: 0.21
Nodes (6): bool, int, Matrix4x4, uint, Vector3, ResourceData

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

### Community 176 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 177 - "PassiveResourceIncrementer"
Cohesion: 0.25
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 179 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 185 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (95): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+87 more)

### Community 188 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.10
Nodes (49): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+41 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - ".new"
Cohesion: 0.27
Nodes (7): algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), generated_instance_counts_match_the_sanitized_unity_save_oracle(), observed_generation_reports_every_real_stage_without_changing_output(), positive_noise_offset(), Self, SystemRandom

### Community 195 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

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
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

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

### Community 208 - "IProcessor"
Cohesion: 0.14
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 209 - "DebugProcessor"
Cohesion: 0.16
Nodes (7): Dictionary, DebugSettings, Container, ContainerBuilder, DebugLogCategory, DebugProcessor, SerializedScriptableObject

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 211 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 212 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "SelectedBuilding"
Cohesion: 0.08
Nodes (5): SelectedBuilding, SelectedEnemyCamp, object, UnityAction, SelectedObject

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

### Community 243 - "parse_model_clip_events"
Cohesion: 0.31
Nodes (9): AnimationEventDef, AnimationObjectReference, inline_mapping_value(), parse_animation_events(), parse_model_clip_events(), parse_object_reference(), parses_normalized_animation_events_from_model_importer_clips(), parses_property_curves_and_animation_events_without_unity_types() (+1 more)

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

### Community 251 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 253 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "RoleProcessor"
Cohesion: 0.08
Nodes (9): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, List (+1 more)

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
Cohesion: 0.11
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

### Community 273 - "VideoCadence"
Cohesion: 0.43
Nodes (5): Duration, Instant, Option, video_cadence_keeps_advancing_when_the_game_stops_supplying_frames(), VideoCadence

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "TradeProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, TradeProcessor

### Community 277 - "Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones., Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "LabelDisplayProcessor.cs"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - ".default"
Cohesion: 0.15
Nodes (22): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_content_catalog() (+14 more)

### Community 285 - "UIRoleDisplay"
Cohesion: 0.24
Nodes (5): TextMeshProUGUI, UI_VoteObjectiveRow, Image, TextMeshProUGUI, UIRoleDisplay

### Community 288 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 289 - "import_save"
Cohesion: 0.29
Nodes (11): absolute_path(), backup_candidate(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), decode_legacy(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save() (+3 more)

### Community 290 - "OpenNode"
Cohesion: 0.43
Nodes (5): OpenNode, Self, Ord, Ordering, PartialOrd

### Community 291 - "Coordinator.cs"
Cohesion: 0.06
Nodes (19): CameraProcessor, GridProcessorEditor, InputButton, Vector3, UnityGraphics, Image, TextMeshProUGUI, StatusBar (+11 more)

### Community 293 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 294 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "PlayerSpawnPoint"
Cohesion: 0.05
Nodes (14): Api, PersistentScoped, Transform, PlayerSpawnPoint, TL_API, DisableOnAwake, List, SimpleEventOnStart (+6 more)

### Community 296 - "PlayerSettings"
Cohesion: 0.06
Nodes (64): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+56 more)

### Community 297 - "direct_broadcast.rs"
Cohesion: 0.13
Nodes (19): AudioFrame, bandwidth_test_url_is_constructed_without_logging_the_key(), BroadcastPrerequisites, build_ingest_url(), DirectTwitchBroadcastPlugin, draw_centered_sensitive_label(), ingest(), ingest_selection_prefers_default_or_named_region() (+11 more)

### Community 298 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 299 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 300 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 301 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 303 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, OnChatCommandReceivedArgs (+4 more)

### Community 307 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 309 - "UI_Objective"
Cohesion: 0.50
Nodes (3): Slider, TextMeshProUGUI, UI_Objective

### Community 311 - "Utils"
Cohesion: 0.04
Nodes (36): BuildCostModifier, STStateMachine.States, PlayerControls.ObjectSelection, Units, Utils, Processors, Pets.Enumerations, StreamTown.EditorTools (+28 more)

### Community 312 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 319 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **363 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+358 more)
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

- **Why does `Utils` connect `Utils` to `TownGoal.Data`, `StringUtils`, `BuildingScriptablesEditor.cs`, `EnemySpawner`, `TechTree.Elements`, `SimpleDisableAfterTime`, `.CreateEnumField`, `GenerationSettings`, `Coordinator.cs`, `EventProcessor`, `UpdateGraphBounds`, `PlayerSpawnPoint`, `RandomEnabler`, `AudioHandler`, `SimpleLookAtCamera`, `SimpleScreenShot`, `ScriptableObject`, `BuildingPlacer`, `Easings`, `FPSDisplay`, `MonoBehaviour`, `CommonEnums.cs`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `RoleProcessor`, `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `HealthHandler`, `WorldGenProcessor`, `UserInterface_Debug`, `TechTreeProcessor`, `PlayerRole`, `RoleHandler`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `NewKingVote`, `Utils`, `PlayerRoleData`, `UserInterface_Roles`, `TimeProcessor`, `ObjectPoolingProcessor`, `UserInterface_TownVote`, `BuildingPlacer`, `IProcessor`, `GameEvent`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `Query`, `.new`, `String`, `setup_rendering`, `Result`, `String`, `save.rs`, `twitch.rs`, `Option`, `command.rs`, `stream_town_migrate/src/presentation.rs`, `simulation.rs`, `Res`, `TechnologyGraphViewState`, `AnimationControllerDef`, `runtime_console.rs`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `stream_town_game/src/lib.rs`, `tools_ui`, `config.rs`, `stream_town_domain/src/presentation.rs`, `stream_town_tools/src/main.rs`, `Vec`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _363 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TownGoal.Data` be split into smaller, more focused modules?**
  _Cohesion score 0.09116809116809117 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.03508217446270544 - nodes in this community are weakly interconnected._
- **Should `PlayerSaveData` be split into smaller, more focused modules?**
  _Cohesion score 0.10526315789473684 - nodes in this community are weakly interconnected._