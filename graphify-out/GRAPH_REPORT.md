# Graph Report - Stream-Town-Bevy  (2026-08-26)

## Corpus Check
- 670 files · ~1,759,398 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8751 nodes · 25308 edges · 321 communities (301 shown, 20 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1034 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bbe800ec`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- TechTreeProcessor.cs
- BuildingProcessor
- world.rs
- SaveDataMapper
- ScriptablesProcessorInfrastructure
- GlobalAudioController
- TwitchChatProcessor
- MenuRuntime
- BottomBarInterface
- .new
- SettingsProcessor
- ErrorData
- advance_world_loading_cover
- IInstaller
- TechTreeIOUtility
- HealthHandler
- STStateMachine.States
- save.rs
- ContentCatalog
- TwitchUser
- twitch.rs
- UIElementWrapper
- STSM_GoToLocation
- Option
- AnimationHandler
- command.rs
- BinarySaveCodec
- StableId
- SaveFileData
- .SetGeneratedResources
- Res
- WorldGenProcessor
- UserInterface_Debug
- secrets_buttons
- GenerationSettings
- BuildingBase
- stream_town_domain/src/content.rs
- SettingsData
- SeasonProcessor
- TechnologyGraphViewState
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- PlayerProcessor
- AnimationControllerDef
- runtime_console.rs
- UserInterface_RulerVote
- PlayerRoleData
- ResourceDataSaveData
- BroadcastController
- AudioHandler
- StreamTownSessionBridge
- PlayerSaveData
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- Value
- simulation.rs
- Targetable
- TechTreeEditorWindow
- Result
- SelectableObject
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Enemy
- GameEventProcessor
- Coordinator
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- Objective
- UserInterface_ObjectSelection
- .Log
- UserInterface_TownVote
- TechTreeGraphView
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- Node_SO
- RenderAssets
- Access_Text
- String
- stream_town_game/src/lib.rs
- Goal
- IProcessor
- FoliageProcessor
- String
- RaidEvent
- config.rs
- MonoBehaviour
- convert_fbx_to_glb.py
- STSM_Idle_Player
- Resource
- SaveProcessor
- TechTreeNode
- stream_town_domain/src/presentation.rs
- .Draw
- StateMachine
- TechnologyGraphLayout
- TownGoalProcessor
- MainMenuManager
- ResourceProcessor
- LoadingManager
- Station
- .SetTargetType
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- VfxSeagullSpawner
- Character
- ToolState
- GridProcessor
- stream_town_migrate/src/main.rs
- GameEvent
- DirectBroadcastRuntime
- VoteEvent
- unity_color_filter
- PoolableObject
- SnapToGridMouseMovement
- PathfindingMigrationStubs.cs
- Option
- GateController
- direct_broadcast.rs
- SeasonDataSettings
- BuildingModelHandler
- Result
- StringUtils
- EnemySpawner
- LabelDisplayProcessor
- TechTree.Elements
- RoleDataSettings
- World.Generation
- RoleHandler
- UnitHealthBar
- Easings
- What You Must Do When Invoked
- RuntimeData Template
- legacy.rs
- RuntimeData Template
- Key Rules
- TimeProcessor
- RoleData
- xtask/src/lib.rs
- CellSpacePartitioning
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- Processors
- IRuntimeDataScriptable
- stream_town_migrate/src/presentation.rs
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- WindController
- Result
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- WorldInstanceDeterminism
- PlayerInputProcessor
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- SimpleMusicController
- stream_town_domain/src/lib.rs
- Q: There are still no animations.
- xtask/src/main.rs
- List
- MiscCommands
- RandomEnabler
- Season
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- BuildingSettings
- SelectedBuilding
- tools_ui
- Access_Toggle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- ResourceRuntimeData
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SimpleScreenShot
- UserInterface_TownGoal
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- BevyMigrationExporter
- CommandDictionary
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- stream_town_migrate/src/technology_layout.rs
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
- .InitializeAndActivateProcessorsAsync
- DebugProcessor
- Requirement
- IProcessor.cs
- UI_TechOption
- Key Rules
- RotationHandler
- RuntimeData Template
- Character Animation Regression Checklist
- KeepKingVote
- ScriptKeywordProcessor
- FPSDisplay
- BuildingConfigSettings
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
- BuildPlacerData
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- GameStateProcessor
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- WeatherProcessor
- extraction-spec.md
- Globals
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- PlayerRole
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- Editor
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- DontDestroyOnLoad
- Q: If there is more to do, keep going.
- AllBuildingDataSettings
- BroadcastConfig
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- technology_tab
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Autosave
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- StatusBar
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- stream_town_tools/src/main.rs
- TL_API
- StreamTown.Migration
- TransformSaveData
- ObjectPoolingRuntimeData
- NewKingVote
- SimpleDisableAfterTime
- record_gpu_readiness
- CreateProjectScopeProcessors.cs
- VfxAnimationController
- PlayerSettings
- runtime_tab
- IntWrapper
- format_game_master_ids
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- ObjectSelectionProcessor.Editor.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- UnityGraphics
- .RefreshSceneBindingsAndTryGenerate
- Utils
- VfxParticlePosition
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- ObjectiveDef
- VFX
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- preview_lerp_color
- FoliageGenerationSettings
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 354 edges
2. `WorldSimulation` - 163 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `ContentCatalog` - 148 edges
7. `Player` - 142 edges
8. `RenderAssets` - 129 edges
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

## Communities (321 total, 20 thin omitted)

### Community 0 - "TechTreeProcessor.cs"
Cohesion: 0.09
Nodes (10): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, TechTree.Data, TechTree.ScriptableObjects (+2 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.08
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "world.rs"
Cohesion: 0.05
Nodes (74): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+66 more)

### Community 3 - "SaveDataMapper"
Cohesion: 0.06
Nodes (23): ResourceInventory, bool, int, Dictionary, List, Mesh, Vector3, SaveDataMapper (+15 more)

### Community 4 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.06
Nodes (9): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, MetaData, Settings (+1 more)

### Community 5 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "MenuRuntime"
Cohesion: 0.06
Nodes (80): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AppExit, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibilityAnnouncement (+72 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - ".new"
Cohesion: 0.03
Nodes (168): generate_world(), generate_world_with_content(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_id_by_source() (+160 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 12 - "advance_world_loading_cover"
Cohesion: 0.05
Nodes (69): AccessibilityNode, AssetId, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), begin_world_loading(), begin_world_loading_cover() (+61 more)

### Community 13 - "IInstaller"
Cohesion: 0.07
Nodes (19): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, MetaDataInstaller, ContainerBuilder, GameSettingsInstaller, List, GameSettings (+11 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (12): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+4 more)

### Community 16 - "STStateMachine.States"
Cohesion: 0.10
Nodes (9): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+1 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "ContentCatalog"
Cohesion: 0.04
Nodes (172): GameConfig, ContentCatalog, GridPos, ActorState, String, GeneratedWorld, action_animation_speed(), action_cooldown() (+164 more)

### Community 19 - "TwitchUser"
Cohesion: 0.22
Nodes (7): ActivityStatus, bool, float, string, UserType, TwitchUser, Character.Enumerations

### Community 20 - "twitch.rs"
Cohesion: 0.07
Nodes (49): bot_and_broadcaster_tokens_use_distinct_vault_entries(), broadcaster_oauth_uses_only_the_stream_key_scope(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), IngestsResponse (+41 more)

### Community 21 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 22 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 23 - "Option"
Cohesion: 0.04
Nodes (153): AmbientLight, ArchetypeScene, PresentationCatalog, ActiveMaterialHandles, animation_property_value(), apply_authored_main_menu_camera(), apply_material_overrides(), archetype_scene_for_age() (+145 more)

### Community 24 - "AnimationHandler"
Cohesion: 0.04
Nodes (32): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+24 more)

### Community 25 - "command.rs"
Cohesion: 0.19
Nodes (29): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+21 more)

### Community 26 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 27 - "StableId"
Cohesion: 0.09
Nodes (28): FromStr, StableId, BuildingState, complete_gameplay_scenario_round_trips(), EnemyCampState, RaidState, BTreeMap, BTreeSet (+20 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - ".SetGeneratedResources"
Cohesion: 0.16
Nodes (16): List, Material, materials, Mesh, meshes, Dictionary, int, List (+8 more)

### Community 30 - "Res"
Cohesion: 0.04
Nodes (227): Aabb, AccumulatedMouseScroll, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, Assets (+219 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "secrets_buttons"
Cohesion: 0.14
Nodes (23): AnyResult, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), open_twitch_verification_uri(), open_twitch_verification_uri_with(), player_settings_path(), FnOnce (+15 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "BuildingBase"
Cohesion: 0.09
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 36 - "stream_town_domain/src/content.rs"
Cohesion: 0.11
Nodes (40): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, EnemyDef (+32 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.14
Nodes (4): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.18
Nodes (25): center_world(), content_bounds(), cubic_bezier(), draw_connection(), draw_grid(), draw_minimap(), fit_bounds(), fit_handles_large_unity_coordinate_ranges() (+17 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "PlayerProcessor"
Cohesion: 0.08
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (31): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+23 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 47 - "PlayerRoleData"
Cohesion: 0.10
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - "BroadcastController"
Cohesion: 0.17
Nodes (18): AtomicBool, BroadcastController, BroadcastMetrics, BroadcastMetricsSnapshot, BroadcastTarget, capture_process_audio(), encode_broadcast_session(), MediaInput (+10 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "PlayerSaveData"
Cohesion: 0.07
Nodes (19): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+11 more)

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
Cohesion: 0.20
Nodes (27): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+19 more)

### Community 57 - "simulation.rs"
Cohesion: 0.07
Nodes (34): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+26 more)

### Community 58 - "Targetable"
Cohesion: 0.05
Nodes (30): List, Vector3, Container, ContainerBuilder, List, TargetProcessor, Dictionary, List (+22 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Enemy"
Cohesion: 0.08
Nodes (19): Action, float, Enemy, AnimationCurve, bool, int, object, ResourceHolder (+11 more)

### Community 68 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 69 - "Coordinator"
Cohesion: 0.09
Nodes (16): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+8 more)

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (129): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+121 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.10
Nodes (14): SelectedResource, BoxCollider, Button, GameObject, Image, List, object, Slider (+6 more)

### Community 76 - ".Log"
Cohesion: 0.07
Nodes (22): HideInCallstack, Object, Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder (+14 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 78 - "TechTreeGraphView"
Cohesion: 0.06
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, List (+14 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (20): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+12 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "RenderAssets"
Cohesion: 0.04
Nodes (107): BackgroundColor, AccessibilityMotionDefaults, apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform (+99 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "String"
Cohesion: 0.11
Nodes (39): AnimationEventDef, AnimationObjectReference, AnimationParameterDef, infer_missing_parameters(), inline_file_id(), inline_mapping_value(), parse_animation_events(), parse_blend_tree() (+31 more)

### Community 86 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (280): accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, action_ranges_and_tower_acquisition_are_euclidean(), ActionPresentation, actor_combat_visual(), actor_detail_budget() (+272 more)

### Community 87 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 88 - "IProcessor"
Cohesion: 0.17
Nodes (6): Container, IProcessor, Action, Container, ContainerBuilder, EventProcessor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "String"
Cohesion: 0.03
Nodes (122): AccessibleNode, AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, accessibility_settings_selection(), active_event_text(), add_animation_composition() (+114 more)

### Community 91 - "RaidEvent"
Cohesion: 0.07
Nodes (17): Transform, Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator (+9 more)

### Community 92 - "config.rs"
Cohesion: 0.11
Nodes (27): ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet (+19 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.02
Nodes (64): CameraProcessor, ContainerBuilder, InstantiationBarrier, AIPath, PersistentScoped, ChannelDataInstaller, ContainerBuilder, ContainerBuilder (+56 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "STSM_Idle_Player"
Cohesion: 0.08
Nodes (10): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 96 - "Resource"
Cohesion: 0.04
Nodes (33): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer (+25 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (21): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+13 more)

### Community 98 - "TechTreeNode"
Cohesion: 0.13
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (74): AnimationClipDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+66 more)

### Community 100 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "TechnologyGraphLayout"
Cohesion: 0.15
Nodes (19): ContentError, Result, TechTree, valid_asset_path(), automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphSize, id() (+11 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.09
Nodes (19): Container, ContainerBuilder, Dictionary, float, int, materialIndex, Matrix4x4, meshIndex (+11 more)

### Community 106 - "LoadingManager"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+10 more)

### Community 107 - "Station"
Cohesion: 0.07
Nodes (18): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+10 more)

### Community 108 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 109 - "CustomLogHandler"
Cohesion: 0.18
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Type, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.13
Nodes (7): Container, ContainerBuilder, DayAndNightProcessor, IPostInitializeProcessor, bool, float, DayAndNightRuntimeData

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "Character"
Cohesion: 0.07
Nodes (14): Color, GameUserType, UserColours, Pets.Enumerations, StreamTown.EditorTools, Core, Pets, UserInterface (+6 more)

### Community 115 - "ToolState"
Cohesion: 0.15
Nodes (23): Arc, Default, Duration, Mutex, Receiver, Sender, Vec, start_twitch_authorization() (+15 more)

### Community 116 - "GridProcessor"
Cohesion: 0.06
Nodes (22): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, GridProcessorEditor, int (+14 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "GameEvent"
Cohesion: 0.13
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 119 - "DirectBroadcastRuntime"
Cohesion: 0.18
Nodes (16): AuthorizationEvent, begin_direct_broadcast(), configure_direct_broadcast(), DirectBroadcastControl, DirectBroadcastPhase, DirectBroadcastRuntime, DirectBroadcastSnapshot, poll_direct_broadcast_authorization() (+8 more)

### Community 120 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "PoolableObject"
Cohesion: 0.07
Nodes (12): Container, ContainerBuilder, GUIDProcessor, Component, Transform, bool, Dictionary, GUIDRuntimeData (+4 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "PathfindingMigrationStubs.cs"
Cohesion: 0.10
Nodes (20): Action, bool, float, int, string, Type, Vector3, AstarData (+12 more)

### Community 126 - "Option"
Cohesion: 0.08
Nodes (52): PrefabPresentationBinding, animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert_materials() (+44 more)

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "direct_broadcast.rs"
Cohesion: 0.11
Nodes (21): AudioFrame, bandwidth_test_url_is_constructed_without_logging_the_key(), black_rgba_frame(), BroadcastPrerequisites, build_ingest_url(), capture_direct_broadcast_frame(), DirectTwitchBroadcastPlugin, ingest() (+13 more)

### Community 129 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 130 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 131 - "Result"
Cohesion: 0.36
Nodes (16): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role(), delete_selected_technology_group() (+8 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.11
Nodes (10): float, int, List, Transform, EnemySpawner, float, ChanceObject, float (+2 more)

### Community 134 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 136 - "RoleDataSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings (+7 more)

### Community 137 - "World.Generation"
Cohesion: 0.04
Nodes (34): ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller (+26 more)

### Community 138 - "RoleHandler"
Cohesion: 0.10
Nodes (8): CollectResource, PlayerInventory, Dictionary, RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "legacy.rs"
Cohesion: 0.10
Nodes (54): ActorKind, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, checked_in_schema_one_fixture_imports_retained_terrain() (+46 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 147 - "RoleData"
Cohesion: 0.12
Nodes (13): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+5 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.28
Nodes (14): add_file(), add_tree(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result (+6 more)

### Community 149 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Processors"
Cohesion: 0.05
Nodes (22): BuildCostModifier, InputButton, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Processors, TownGoal, Target (+14 more)

### Community 154 - "IRuntimeDataScriptable"
Cohesion: 0.11
Nodes (16): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+8 more)

### Community 155 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (97): animation_state_id(), animation_state_machine_id(), animation_take_name(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id(), convert() (+89 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "Result"
Cohesion: 0.30
Nodes (8): BroadcastEncoder, inspect_broadcast_prerequisites(), linked_ffmpeg_encodes_h264_aac_flv_without_a_subprocess(), Formatter, Result, Context, Output, Rational

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 163 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.13
Nodes (36): AudioBaselineManifest, Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names() (+28 more)

### Community 170 - "List"
Cohesion: 0.19
Nodes (8): GameObject, List, MenuItem, NeutralAsset, NeutralScene, NeutralAsset, NeutralGameObject, NeutralScene

### Community 171 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 172 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 173 - "Season"
Cohesion: 0.16
Nodes (11): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, bool, float (+3 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "BuildingSettings"
Cohesion: 0.08
Nodes (16): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingSettingsInstaller (+8 more)

### Community 177 - "SelectedBuilding"
Cohesion: 0.07
Nodes (6): SelectedBuilding, SelectedEnemy, SelectedEnemyCamp, object, UnityAction, SelectedObject

### Community 178 - "tools_ui"
Cohesion: 0.19
Nodes (21): content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), poll_tool_job_events(), preview_grid_point(), role_i32(), role_u16() (+13 more)

### Community 179 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 185 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (89): ContainerBuilder, GameEventConfigSettingsInstaller, ContainerBuilder, ResourceDataSettingsInstaller, ContainerBuilder, TimeDataSettingsInstaller, ContainerBuilder, WorldGenBehaviorSettingsInstaller (+81 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "stream_town_migrate/src/technology_layout.rs"
Cohesion: 0.28
Nodes (15): GraphPoint, AuthoredGroup, AuthoredNode, build_layout(), checked_in_layout_exactly_matches_the_unity_graph_conversion(), convert(), parse_point(), parse_unity_graph() (+7 more)

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
Cohesion: 0.20
Nodes (5): Color, List, Texture2D, EditorUtils, DirectoryInfo

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.10
Nodes (21): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+13 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.19
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 209 - "DebugProcessor"
Cohesion: 0.13
Nodes (9): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, DebugLogCategory, DebugProcessor (+1 more)

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 212 - "IProcessor.cs"
Cohesion: 0.20
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 213 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

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

### Community 221 - "BuildingConfigSettings"
Cohesion: 0.29
Nodes (5): ContainerBuilder, BuildingConfigSettingsInstaller, Dictionary, int, BuildingConfigSettings

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

### Community 234 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

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

### Community 243 - "GameStateProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GameStateProcessor

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

### Community 251 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 253 - "Globals"
Cohesion: 0.20
Nodes (4): BuildingPlacerEditor, string, ScriptableObjectAssetData, Globals

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "PlayerRole"
Cohesion: 0.05
Nodes (17): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+9 more)

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
Cohesion: 0.20
Nodes (4): BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Editor

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "AllBuildingDataSettings"
Cohesion: 0.22
Nodes (5): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingDataContainerInstaller, AllBuildingDataSettings

### Community 268 - "BroadcastConfig"
Cohesion: 0.43
Nodes (7): BroadcastConfig, BroadcastEncoderPreference, encoder_candidates(), open_audio_encoder(), open_video_encoder(), Codec, Encoder

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

### Community 273 - "technology_tab"
Cohesion: 0.36
Nodes (9): authoring_snapshot(), AuthoringSnapshot, push_authoring_undo(), redo_authoring_edit(), refresh_catalog_drafts(), refresh_foliage_draft(), reload_content_catalog(), technology_tab() (+1 more)

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

### Community 280 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "stream_town_tools/src/main.rs"
Cohesion: 0.12
Nodes (36): authority_tab(), broadcast_encoder_label(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips() (+28 more)

### Community 288 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 289 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 291 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 292 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 293 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "VfxAnimationController"
Cohesion: 0.29
Nodes (4): bool, float, VisualEffect, VfxAnimationController

### Community 296 - "PlayerSettings"
Cohesion: 0.07
Nodes (58): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+50 more)

### Community 297 - "runtime_tab"
Cohesion: 0.33
Nodes (7): format_runtime_frame_times(), inject_runtime_command(), launch_runtime_game(), poll_runtime_console(), runtime_actions_sequence_after_latest_acknowledgement(), runtime_tab(), send_runtime_action()

### Community 298 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 299 - "format_game_master_ids"
Cohesion: 0.40
Nodes (6): format_game_master_ids(), parse_game_master_ids(), poll_twitch_tool_events(), BTreeSet, sync_twitch_tool_fields(), twitch_tool_normalizes_game_master_ids_and_reward_field()

### Community 300 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 301 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 302 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 303 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.06
Nodes (13): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, OnChatCommandReceivedArgs (+5 more)

### Community 307 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 308 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 309 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 311 - "Utils"
Cohesion: 0.06
Nodes (8): RoleScriptablesEditor, Utils, Animation, ScriptablesEditor, STStateMachine, Audio, Environment, SavingAndLoading.Structs

### Community 312 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "ObjectiveDef"
Cohesion: 0.60
Nodes (4): ObjectiveDef, ObjectiveKind, objective_increment(), ObjectiveEvent

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 318 - "preview_lerp_color"
Cohesion: 1.00
Nodes (3): preview_lerp_color(), Color32, terrain_preview_color()

### Community 319 - "FoliageGenerationSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, List, FoliageGenSettings, List, WaterFoliageGenSettings (+7 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **362 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+357 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **20 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `TechTreeProcessor.cs`, `ScriptablesProcessorInfrastructure`, `EnemySpawner`, `LabelDisplayProcessor`, `TechTree.Elements`, `StringUtils`, `World.Generation`, `AllBuildingDataSettings`, `Easings`, `STStateMachine.States`, `Processors`, `.CreateEnumField`, `GenerationSettings`, `UpdateGraphBounds`, `SimpleDisableAfterTime`, `RandomEnabler`, `SimpleScreenShot`, `Targetable`, `ScriptableObject`, `BuildingPlacer`, `FPSDisplay`, `MonoBehaviour`, `BuildingConfigSettings`, `Character`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `world.rs`, `Result`, `.new`, `legacy.rs`, `save.rs`, `ContentCatalog`, `twitch.rs`, `Option`, `command.rs`, `stream_town_migrate/src/presentation.rs`, `stream_town_tools/src/main.rs`, `Res`, `stream_town_domain/src/content.rs`, `stream_town_domain/src/lib.rs`, `TechnologyGraphViewState`, `AnimationControllerDef`, `runtime_console.rs`, `tools_ui`, `simulation.rs`, `ObjectiveDef`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `RenderAssets`, `String`, `stream_town_game/src/lib.rs`, `String`, `config.rs`, `stream_town_domain/src/presentation.rs`, `TechnologyGraphLayout`, `ToolState`, `Option`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `PlayerRole`, `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `RoleHandler`, `IInstaller`, `TimeProcessor`, `RoleData`, `Processors`, `WorldGenProcessor`, `UserInterface_Debug`, `NewKingVote`, `TechTreeProcessor`, `PlayerRoleData`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `Enemy`, `GameEventProcessor`, `.Log`, `UserInterface_TownVote`, `BuildingPlacer`, `IProcessor`, `RaidEvent`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `PoolableObject`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _362 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TechTreeProcessor.cs` be split into smaller, more focused modules?**
  _Cohesion score 0.08994708994708994 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.08233117483811286 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.053975363941769314 - nodes in this community are weakly interconnected._