# Graph Report - Stream-Town-Bevy  (2026-08-23)

## Corpus Check
- 656 files · ~1,704,413 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8431 nodes · 24035 edges · 294 communities (271 shown, 23 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1025 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `962e9d09`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UnityAsset
- BuildingProcessor
- world.rs
- BinarySaveCodec
- ScriptableObject
- .new
- TwitchChatProcessor
- BinaryWriter
- BottomBarInterface
- Res
- SettingsProcessor
- Targetable
- Utils
- audio.rs
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- pattern.rs
- backend.rs
- Station
- ObjectPoolingProcessor
- BuildingPlacer
- simulation.rs
- UnitHealthBar
- PlayerRole
- Query
- TechTreeGraphView
- SaveFileData
- Player
- Character
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- DayAndNightProcessor
- ResMut
- SettingsData
- SeasonProcessor
- PlayerSettings
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerRuntime
- stream_town_game/src/lib.rs
- component_field_value
- legacy.rs
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- StableId
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- AudioSettings
- Handle
- String
- TechTreeEditorWindow
- Result
- BuildingBase
- CameraController
- PlayerProcessor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- bevy_tidal/src/main.rs
- Enemy
- models.rs
- Tiler
- ScriptablesEditor
- DebugProcessor
- UserInterface_ObjectSelection
- PlayerRoleData
- SelectedObject
- BTreeMap
- TwitchBotSetupWindow
- Goal
- WorldUtils
- Node_SO
- PlayerCommands
- Access_Text
- RoleDataSettings
- update_environment_presentation
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- String
- RaidEvent
- ResourceRuntimeData
- STSM_GoToLocation
- convert_fbx_to_glb.py
- TechTreeProcessor.cs
- drive_tidal_music
- SaveProcessor
- Coordinator
- PresentationCatalog
- MonoBehaviour
- StateMachine
- parse_program
- TownGoalProcessor
- MainMenuManager
- ResourceTarget
- LoadingManager
- UIElementWrapper
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- .SetGeneratedResources
- STSM_StateAction
- TidalController
- GridNode
- stream_town_migrate/src/main.rs
- .OnGUI
- Resource
- VoteEvent
- unity_color_filter
- PlayerInventory
- SnapToGridMouseMovement
- AIPath
- LabelDisplayProcessor
- PlayerInputProcessor
- TechTreeNode
- UpdateGraphBounds
- RoleData
- AnimationHandler
- STSM_Idle
- WeatherProcessor
- twitch_tab
- stream_town_tools/src/main.rs
- ResourceData
- runtime_console.rs
- GateController
- SelectableObject
- WorldGenRuntimeData
- .RenderResourceType
- What You Must Do When Invoked
- RuntimeData Template
- String
- RuntimeData Template
- Key Rules
- BuildingResourceModelHandler
- Pet
- add_file
- RoleHandler
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- ToolState
- IProcessor.cs
- stream_town_migrate/src/presentation.rs
- IRuntimeDataScriptable
- Stream Town Reloaded - Architecture Documentation
- WindController
- IProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- .CreateEnumField
- BevyMigrationExporter
- .SetTargetType
- SimpleMusicController
- FoliageGenerationSettings
- Target
- xtask/src/main.rs
- MiscCommands
- TransformSaveData
- ResourceProcessor
- RotationHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- UserInterface.MainMenu
- PlayerSaveData
- TechTree.Elements
- TradeProcessor
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- List
- ScriptablesProcessorInfrastructure
- WorldGenerationReferenceExporter
- ResourceStorageModifier
- Bevy Tidal
- UserInterface_GameMenu
- Easings
- stream_town_migrate/src/menu_scene.rs
- TownResourceRuntimeData
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SaveDataMapper
- .DrawDataFieldAndLabel
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- UserInterface_RulerVote
- GridProcessor
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- BuildingModelHandler
- BuildingSettings
- Requirement
- CommandDictionary
- SelectedEnemy
- SelectedResource
- Key Rules
- TimeProcessor
- RuntimeData Template
- PassiveResourceIncrementer
- PlayerDeathHandler
- ScriptKeywordProcessor
- FPSDisplay
- string
- Processor Template
- Common Patterns
- STSM_HelperBase
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- Editor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- ResourceInventory
- CreateDefaultSettingsAssets.cs
- .RestoreObjectiveProgress
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- StringUtils
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- GameEventRuntimeData
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxSeagullSpawner
- extraction-spec.md
- Access_GOList
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- StatusBar
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- TL_API
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- UnitTravelToPosition
- Q: If there is more to do, keep going.
- Units
- EventProcessor
- stream_town_domain
- ObjectSelectionProcessor.Editor.cs
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- BuildingRuntimeData
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- ActiveResourceIncrementer
- .InjectRuntimeData
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- import_save
- SimpleScreenShot
- RandomEnabler
- LabelDisplayProcessor.cs
- StreamTown.Migration
- IInstaller
- PlayerInputRuntimeData
- .ExportModification
- UPSTREAM.md
- command.rs
- PlacementProbeHandler

## God Nodes (most connected - your core abstractions)
1. `StableId` - 341 edges
2. `WorldSimulation` - 164 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 138 edges
8. `WorldGenProcessor` - 114 edges
9. `SettingsProcessor` - 107 edges
10. `RenderAssets` - 105 edges

## Surprising Connections (you probably didn't know these)
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `begin_world_loading()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- 2-file cycle: `bevy-port/vendor/bevy_tidal/src/audio.rs -> bevy-port/vendor/bevy_tidal/src/lib.rs -> bevy-port/vendor/bevy_tidal/src/audio.rs`
- 2-file cycle: `bevy-port/vendor/bevy_tidal/src/backend.rs -> bevy-port/vendor/bevy_tidal/src/lib.rs -> bevy-port/vendor/bevy_tidal/src/backend.rs`
- 3-file cycle: `bevy-port/vendor/bevy_tidal/src/audio.rs -> bevy-port/vendor/bevy_tidal/src/lib.rs -> bevy-port/vendor/bevy_tidal/src/backend.rs -> bevy-port/vendor/bevy_tidal/src/audio.rs`
- 4-file cycle: `bevy-port/vendor/bevy_tidal/src/audio.rs -> bevy-port/vendor/bevy_tidal/src/lib.rs -> bevy-port/vendor/bevy_tidal/src/backend.rs -> bevy-port/vendor/bevy_tidal/src/pattern.rs -> bevy-port/vendor/bevy_tidal/src/audio.rs`

## Communities (294 total, 23 thin omitted)

### Community 0 - "UnityAsset"
Cohesion: 0.18
Nodes (44): aged_buildings(), archetype_kind(), building_cost_reductions(), building_level_caps(), building_node_age(), convert_export(), field_value(), foliage_layers() (+36 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "world.rs"
Cohesion: 0.06
Nodes (67): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+59 more)

### Community 3 - "BinarySaveCodec"
Cohesion: 0.15
Nodes (7): Func, int, UTF8Encoding, BinarySaveCodec, int, PlayerRoleSaveData, BinaryReader

### Community 4 - "ScriptableObject"
Cohesion: 0.02
Nodes (80): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+72 more)

### Community 5 - ".new"
Cohesion: 0.03
Nodes (173): ArchetypeScene, generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_id_by_source(), archetype_scene_for_age() (+165 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Res"
Cohesion: 0.05
Nodes (160): AnimationTransitions, MainMenuSceneReference, Option, actor_material(), advance_falling_fish(), AgentAnimation, animate_agents(), animate_building_effects() (+152 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "Targetable"
Cohesion: 0.04
Nodes (43): Vector3, Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary (+35 more)

### Community 12 - "Utils"
Cohesion: 0.06
Nodes (12): BuildCostModifier, InputButton, Utils, Processors, World, Level, Buildings, Audio (+4 more)

### Community 13 - "audio.rs"
Cohesion: 0.06
Nodes (63): AtomicBool, absolute_path(), AudioStatusInner, build_stream(), built_in_voice_and_effects_produce_finite_audio(), documented_synth_families_and_controls_render_without_sidecars(), load_wav(), Mixer (+55 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (20): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+12 more)

### Community 15 - "HealthHandler"
Cohesion: 0.09
Nodes (13): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, int, STSM_Helper_Attack, Action, bool (+5 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (41): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+33 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "pattern.rs"
Cohesion: 0.10
Nodes (69): alternation_and_note_names_are_native(), apply_event_transforms(), apply_hit_transforms(), apply_sound(), arc_after(), chord_intervals(), contains_function(), Controls (+61 more)

### Community 19 - "backend.rs"
Cohesion: 0.08
Nodes (40): NativeAudioSender, ActiveTrack, apply_commands(), applying_hush_removes_every_track(), BackendCommand, BackendReceiver, BackendStatusInner, BackendThread (+32 more)

### Community 20 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, List, Queue, Container, ContainerBuilder (+9 more)

### Community 21 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (37): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+29 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "simulation.rs"
Cohesion: 0.07
Nodes (31): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+23 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "PlayerRole"
Cohesion: 0.09
Nodes (8): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, PlayerRole

### Community 26 - "Query"
Cohesion: 0.03
Nodes (161): Aabb, Added, AnimationGraphHandle, Assets, AudioSink, ActorAnimationDriver, ActorHealthFill, ActorHealthOverlay (+153 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.07
Nodes (10): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+2 more)

### Community 30 - "Character"
Cohesion: 0.05
Nodes (24): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+16 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (26): Action, HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func (+18 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 33 - "GameEventProcessor"
Cohesion: 0.06
Nodes (15): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, GameEventProcessor, EventType, EventTester (+7 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 36 - "ResMut"
Cohesion: 0.04
Nodes (117): AccumulatedMouseMotion, AccumulatedMouseScroll, advance_loading_phase(), advance_loading_runtime(), apply_settings_draft(), authored_color_grading(), autosave_game(), BootDestination (+109 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.07
Nodes (21): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container (+13 more)

### Community 39 - "PlayerSettings"
Cohesion: 0.10
Nodes (32): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+24 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.11
Nodes (13): SimpleToggleCarry, CharacterModelHandler, bool, int, List, RoleEquipment, bool, GameObject (+5 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.10
Nodes (38): asset(), authored_value(), component(), component_at(), ContentConversionReport, convert(), converted_rotating_axis(), converts_active_catalog_references_and_round_trips_ron() (+30 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.11
Nodes (24): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+16 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (271): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, AnyResult, action_ranges_and_tower_acquisition_are_euclidean(), active_event_text() (+263 more)

### Community 46 - "component_field_value"
Cohesion: 0.19
Nodes (28): ArchetypesById, archetype_bounds(), building_model_definitions(), building_placements(), BuildingPlacement, component_field_value(), component_reference_name(), component_reference_names() (+20 more)

### Community 47 - "legacy.rs"
Cohesion: 0.15
Nodes (41): ActorCustomization, StreamUserType, should_show_actor_name(), binary_fixture(), BinaryParser, conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal() (+33 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "StableId"
Cohesion: 0.03
Nodes (231): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, ObjectiveDef, FromStr, StableId, GridPos (+223 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "AudioSettings"
Cohesion: 0.33
Nodes (4): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings

### Community 57 - "Handle"
Cohesion: 0.04
Nodes (65): BackgroundColor, bottom_bar_texture(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform (+57 more)

### Community 58 - "String"
Cohesion: 0.09
Nodes (56): animator_component(), animator_reference_path(), clip_id(), convert_clips(), convert_post_process(), field_bool(), field_f32(), field_str() (+48 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "BuildingBase"
Cohesion: 0.11
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 62 - "CameraController"
Cohesion: 0.13
Nodes (9): bool, Camera, float, int, PlayerInput, Vector2, Vector3, CameraController (+1 more)

### Community 63 - "PlayerProcessor"
Cohesion: 0.08
Nodes (13): Action, Container, ContainerBuilder, List, Vector3, PlayerProcessor, Dictionary, List (+5 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "twitch.rs"
Cohesion: 0.08
Nodes (41): channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join(), OAuthClient, OAuthErrorResponse (+33 more)

### Community 68 - "Objective"
Cohesion: 0.09
Nodes (9): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+1 more)

### Community 69 - "bevy_tidal/src/main.rs"
Cohesion: 0.14
Nodes (27): IntegrationRun, load_buffer(), log_tidal_events(), main(), normalized_tidal_filename(), requested_test_file(), AppExit, Commands (+19 more)

### Community 70 - "Enemy"
Cohesion: 0.07
Nodes (21): CollectResource, Action, float, Enemy, AnimationCurve, bool, int, object (+13 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+4 more)

### Community 74 - "DebugProcessor"
Cohesion: 0.04
Nodes (24): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, Container, ContainerBuilder, HideInCallstack, Object (+16 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.08
Nodes (14): SelectedBuilding, BoxCollider, Button, GameObject, Image, List, object, Slider (+6 more)

### Community 76 - "PlayerRoleData"
Cohesion: 0.10
Nodes (9): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+1 more)

### Community 77 - "SelectedObject"
Cohesion: 0.10
Nodes (6): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup

### Community 78 - "BTreeMap"
Cohesion: 0.10
Nodes (46): animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert(), convert_avatar_masks(), convert_controllers() (+38 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (7): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "PlayerCommands"
Cohesion: 0.12
Nodes (5): OnMessageReceivedArgs, EventCommands, OnChatCommandReceivedArgs, TwitchClientProcessor, PlayerCommands

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "RoleDataSettings"
Cohesion: 0.06
Nodes (22): RoleSlot, bool, int, ContainerBuilder, AllRoleDataSettingsInstaller, Dictionary, int, RoleDataContainer (+14 more)

### Community 86 - "update_environment_presentation"
Cohesion: 0.13
Nodes (24): AmbientLight, building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced, BuildingMaterialInstances, environment_palette() (+16 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (14): Button, GameObject, Image, Slider, TextMeshProUGUI, UI_TechOption, bool, Button (+6 more)

### Community 88 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "String"
Cohesion: 0.20
Nodes (16): animation_parameter_name(), archetype_scenes(), authored_mask(), child_technology_guids(), collect_model_dependencies(), decomposes_combined_unity_flag_values(), glb_asset_path(), mask_ids() (+8 more)

### Community 91 - "RaidEvent"
Cohesion: 0.08
Nodes (16): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+8 more)

### Community 92 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 93 - "STSM_GoToLocation"
Cohesion: 0.11
Nodes (10): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Vector3 (+2 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "TechTreeProcessor.cs"
Cohesion: 0.07
Nodes (16): int, TechTreeSettings, InputButton, SharedTypes, int, ChangeTimeStamp, NodeGroup_SO, List (+8 more)

### Community 96 - "drive_tidal_music"
Cohesion: 0.23
Nodes (20): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), effective_music_gain(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature (+12 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (18): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+10 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "PresentationCatalog"
Cohesion: 0.05
Nodes (90): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef (+82 more)

### Community 100 - "MonoBehaviour"
Cohesion: 0.02
Nodes (65): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, MetaDataInstaller, PersistentScoped, PlayerSpawnPoint, Slider (+57 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "parse_program"
Cohesion: 0.12
Nodes (18): all_documented_scales_map_degrees_across_octaves(), concatenation_generators_and_runtime_tempo_parse_natively(), documented_chords_voicings_and_rolls_expand_to_native_events(), documented_compression_echo_and_hurry_are_native(), documented_cycle_time_and_stereo_transforms_are_native(), documented_structure_slicing_and_signal_sampling_are_native(), hash_pattern_name(), is_statement_start() (+10 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+9 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.07
Nodes (15): GameObject, List, PresetButtons, Access_AOToggle, Access_ChannelNameInput, ContainerBuilder, Access_EdgeScrollingToggle, Access_MouseControlsToggle (+7 more)

### Community 108 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.11
Nodes (13): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+5 more)

### Community 112 - "EnemySpawner"
Cohesion: 0.11
Nodes (9): float, int, List, EnemySpawner, float, ChanceObject, float, List (+1 more)

### Community 113 - ".SetGeneratedResources"
Cohesion: 0.40
Nodes (5): List, Material, materials, Mesh, meshes

### Community 114 - "STSM_StateAction"
Cohesion: 0.07
Nodes (16): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, int, STSM_Action_Attack, bool (+8 more)

### Community 115 - "TidalController"
Cohesion: 0.27
Nodes (9): AsRef, controller_reports_parse_errors_synchronously(), App, Plugin, Result, Sender, String, TidalBackendPlugin (+1 more)

### Community 116 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - ".OnGUI"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 119 - "Resource"
Cohesion: 0.13
Nodes (5): Container, Dictionary, TownResourceProcessor, Resource, ReviveType

### Community 120 - "VoteEvent"
Cohesion: 0.10
Nodes (13): List, KeepKingVote, PlayerVote, Dictionary, TechVote, Dictionary, float, IReadOnlyDictionary (+5 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "PlayerInventory"
Cohesion: 0.14
Nodes (4): DepositResources, PlayerInventory, Dictionary, IResourceHolder

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.12
Nodes (19): bool, float, int, string, Type, Vector3, AIPath, AstarData (+11 more)

### Community 125 - "LabelDisplayProcessor"
Cohesion: 0.14
Nodes (6): Container, ContainerBuilder, LabelDisplayProcessor, float, ParticleSystem, VFXArrowPointer

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 127 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "RoleData"
Cohesion: 0.12
Nodes (12): RoleData, AudioClip, bool, float, int, Sprite, string, bool (+4 more)

### Community 130 - "AnimationHandler"
Cohesion: 0.11
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 131 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 132 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 133 - "twitch_tab"
Cohesion: 0.29
Nodes (11): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+3 more)

### Community 134 - "stream_town_tools/src/main.rs"
Cohesion: 0.08
Nodes (60): authority_tab(), bounded_ui_index(), checked_in_authoring_assets_pass_headless_validation(), content_tab(), default_catalog_path(), default_config_path(), draw_world_preview(), foliage_editor_rejects_invalid_generation_values_without_mutation() (+52 more)

### Community 135 - "ResourceData"
Cohesion: 0.18
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 136 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectableObject"
Cohesion: 0.12
Nodes (12): InputButton, UnityEvent, DebugRuntimeData, bool, List, RectTransform, UnityEvent, Vector3 (+4 more)

### Community 139 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 140 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "String"
Cohesion: 0.10
Nodes (35): ActorKind, actor_prefix(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert() (+27 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

### Community 147 - "Pet"
Cohesion: 0.12
Nodes (9): List, PetType, bool, Dictionary, float, Pet, Animator, int (+1 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "RoleHandler"
Cohesion: 0.12
Nodes (5): RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "ToolState"
Cohesion: 0.14
Nodes (40): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role(), delete_selected_technology_group() (+32 more)

### Community 154 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 155 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (76): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert_chimney_smoke(), convert_fireworks(), convert_fish_schools() (+68 more)

### Community 156 - "IRuntimeDataScriptable"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, GameStateProcessor, CreditsRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, Dictionary (+10 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "IProcessor"
Cohesion: 0.15
Nodes (6): CancellationToken, Task, Container, IProcessor, Dictionary, ParallelProgressReporter

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 163 - ".CreateEnumField"
Cohesion: 0.13
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 164 - "BevyMigrationExporter"
Cohesion: 0.29
Nodes (4): HashSet, MenuItem, BevyMigrationExporter, NeutralAsset

### Community 165 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - "FoliageGenerationSettings"
Cohesion: 0.15
Nodes (12): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings, int (+4 more)

### Community 168 - "Target"
Cohesion: 0.15
Nodes (8): TargetSettings, TargetableData, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, GUIDSystem

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 170 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 171 - "TransformSaveData"
Cohesion: 0.08
Nodes (22): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+14 more)

### Community 172 - "ResourceProcessor"
Cohesion: 0.16
Nodes (3): Container, ContainerBuilder, ResourceProcessor

### Community 173 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "UserInterface.MainMenu"
Cohesion: 0.20
Nodes (3): UserInterface.MainMenu, MetaData, Settings

### Community 177 - "PlayerSaveData"
Cohesion: 0.14
Nodes (13): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, bool, int (+5 more)

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 179 - "TradeProcessor"
Cohesion: 0.15
Nodes (8): Dictionary, float, TradeSettings, ContainerBuilder, TradeSettingsInstaller, Container, ContainerBuilder, TradeProcessor

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "List"
Cohesion: 0.31
Nodes (6): GameObject, List, NeutralAsset, NeutralScene, NeutralGameObject, NeutralScene

### Community 185 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (63): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, FoliageGenSettingsInstaller (+55 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ResourceStorageModifier"
Cohesion: 0.24
Nodes (3): ResourceStorageModifier, float, int

### Community 188 - "Bevy Tidal"
Cohesion: 0.17
Nodes (10): Bevy Tidal, Configuration, Native pattern language, Use it in a game, Verify the complete path, Implemented in the native engine, Intentionally not emulated, Native Tidal documentation coverage (+2 more)

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, String, Vec (+36 more)

### Community 192 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "SaveDataMapper"
Cohesion: 0.08
Nodes (17): Component, Mesh, Vector3, SaveDataMapper, bool, int, MeshSaveData, int (+9 more)

### Community 195 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "UserInterface_RulerVote"
Cohesion: 0.10
Nodes (13): int, List, NewKingVote, Slider, TextMeshProUGUI, UIRuntimeData, TextMeshProUGUI, UI_RulerOption (+5 more)

### Community 200 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "UnityGraphics"
Cohesion: 0.40
Nodes (4): Vector3, UnityGraphics, FieldInfo, ShadowResolution

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.12
Nodes (17): List, SaveGameData, bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData (+9 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 209 - "BuildingSettings"
Cohesion: 0.09
Nodes (16): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, ContainerBuilder, BuildingSettingsInstaller (+8 more)

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 211 - "CommandDictionary"
Cohesion: 0.26
Nodes (6): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 218 - "PlayerDeathHandler"
Cohesion: 0.22
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "string"
Cohesion: 0.22
Nodes (11): bool, int, long, string, NeutralComponent, NeutralExport, NeutralField, NeutralGameObject (+3 more)

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

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

### Community 229 - "Twitch setup"
Cohesion: 0.07
Nodes (24): Audio provenance, Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows, Binaries, Commands (+16 more)

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

### Community 234 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "ResourceInventory"
Cohesion: 0.29
Nodes (4): ResourceInventory, bool, int, Dictionary

### Community 238 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - ".RestoreObjectiveProgress"
Cohesion: 0.33
Nodes (3): int, string, ObjectiveSaveData

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Q: How are shipping visual and audio parity implemented in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How are shipping visual and audio parity implemented in the Bevy migration?, Source Nodes

### Community 248 - "GameEventRuntimeData"
Cohesion: 0.40
Nodes (5): bool, float, ParticleSystem, SortedSet, GameEventRuntimeData

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 253 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

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

### Community 267 - "Units"
Cohesion: 0.05
Nodes (15): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, PlayerControls.ObjectSelection (+7 more)

### Community 268 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 269 - "stream_town_domain"
Cohesion: 0.40
Nodes (6): bevy_tidal, stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 284 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 286 - "LabelDisplayProcessor.cs"
Cohesion: 0.22
Nodes (3): VisualEffect, VfxParticlePosition, VFX

### Community 288 - "IInstaller"
Cohesion: 0.03
Nodes (34): ContainerBuilder, InstantiationBarrier, ContainerBuilder, Volume, PostProcessingInstaller, AudioMixerInstaller, AudioMixer, ContainerBuilder (+26 more)

### Community 289 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 296 - "command.rs"
Cohesion: 0.06
Nodes (59): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+51 more)

## Knowledge Gaps
- **314 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+309 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `RenderAssets` (4× useful, score=3.31547271) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.53675428)
- `WorldSnapshot` (3× useful, score=2.423233543)
- `WorldSimulation` (2× useful, score=1.759032374)
- `load_input()` (2× useful, score=1.615254359) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (2× useful, score=1.586186223) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.58507607)
- `MaterialDef` (2× useful, score=1.584629988)
- `BevyMigrationExporter` (2× useful, score=1.557856672)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `UpdateGraphBounds`, `ScriptableObject`, `Units`, `Targetable`, `BuildingPlacer`, `IRuntimeDataScriptable`, `RandomEnabler`, `Character`, `SimpleScreenShot`, `GenerationSettings`, `.CreateEnumField`, `Target`, `TechTree.Elements`, `ScriptablesProcessorInfrastructure`, `Easings`, `.DrawDataFieldAndLabel`, `FPSDisplay`, `TechTreeProcessor.cs`, `MonoBehaviour`, `EnemySpawner`, `StringUtils`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.052) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `ResourceData`, `WorldGenRuntimeData`, `Targetable`, `ObjectPoolingProcessor`, `IRuntimeDataScriptable`, `Player`, `IProcessor`, `IInstaller`, `UserInterface_Debug`, `Target`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `PlayerProcessor`, `SaveDataMapper`, `GridProcessor`, `DebugProcessor`, `FoliageProcessor`, `RaidEvent`, `SaveProcessor`, `Coordinator`, `MonoBehaviour`, `EnemySpawner`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `UnityAsset`, `.new`, `stream_town_tools/src/main.rs`, `runtime_console.rs`, `Res`, `String`, `stream_town_domain/src/content.rs`, `save.rs`, `simulation.rs`, `ToolState`, `Query`, `stream_town_migrate/src/presentation.rs`, `command.rs`, `AnimationControllerRuntime`, `stream_town_game/src/lib.rs`, `component_field_value`, `String`, `stream_town_migrate/src/menu_scene.rs`, `twitch.rs`, `BTreeMap`, `update_environment_presentation`, `String`, `PresentationCatalog`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _314 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06801346801346801 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0577324973876698 - nodes in this community are weakly interconnected._
- **Should `BinarySaveCodec` be split into smaller, more focused modules?**
  _Cohesion score 0.14982578397212543 - nodes in this community are weakly interconnected._