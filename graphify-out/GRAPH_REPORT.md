# Graph Report - Stream-Town-Bevy  (2026-08-20)

## Corpus Check
- 637 files · ~1,660,498 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7819 nodes · 21761 edges · 263 communities (246 shown, 17 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1020 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `744d8c87`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- MonoBehaviour
- BuildingProcessor
- Result
- NavGrid
- ScriptableObject
- setup_rendering
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- ShaderRef
- SettingsProcessor
- PlayerSaveData
- DayAndNightProcessor
- Commands
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- UnityAsset
- Station
- .CreateEnumField
- Goal
- BuildingPlacer
- PlayerSettings
- UnitHealthBar
- simulation.rs
- Res
- TechTreeGraphView
- SaveFileData
- Player
- update_credits_fireworks
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- TechTreeNode
- CommandDictionary
- SettingsData
- SeasonProcessor
- StableId
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- stream_town_game/src/lib.rs
- String
- legacy.rs
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- String
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Enemy
- .SetTargetType
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- TwitchUser
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- command.rs
- IntWrapper
- models.rs
- Tiler
- ScriptablesEditor
- IProcessor.cs
- UserInterface_ObjectSelection
- GeneratedWorld
- MiscCommands
- AnimationHandler
- TwitchBotSetupWindow
- EnemyModelHandler
- WorldUtils
- BuildingBase
- Vec
- Access_Text
- RoleHandler
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- UserInterface_TownGoal
- Access_Toggle
- CommonEnums.cs
- STSM_Idle_Player
- convert_fbx_to_glb.py
- stream_town_migrate/src/presentation.rs
- PoolableObject
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- StationSensor
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- UIElementWrapper
- Resource
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- add_rotation_curve
- WorldGenSaveData
- convert
- Target
- stream_town_migrate/src/main.rs
- VoteEvent
- TownResourceProcessor
- UserInterface_Roles
- unity_color_filter
- ErrorData
- SnapToGridMouseMovement
- AIPath
- IRuntimeDataScriptable
- EnemyWeaponModel
- STSM_StateAction
- UpdateGraphBounds
- GlobalAudioController
- WindController
- Targetable
- .EnsureValidCredentials
- HealthModifier
- ConfirmCheck
- ResourceRuntimeData
- ToolState
- GateController
- PlayerRole
- CreditsProcessor
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- SelectedObject
- Pet
- add_file
- STSM_HelperBase
- SimpleCancelBuildingPlacer
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- Vec
- SelectedBuilding
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- SelectedResource
- IProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- TimeProcessor
- EquipmentHandlerEditor
- TL_Secrets
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- SelectedEnemyCamp
- RotationHandler
- BuildingScriptablesEditor.cs
- AudioMixerInstaller
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- .RenderResourceType
- SelectedEnemy
- TechTree.Elements
- ObjectiveSaveData
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SelectableObject
- ScriptablesProcessorInfrastructure
- ResourceHolder
- .SetGroupSelectionArea
- WeatherProcessor
- StringUtils
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- Utils
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- SimpleScreenShot
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- Requirement
- Key Rules
- SettingsProcessor.cs
- RuntimeData Template
- .Log
- ScriptKeywordProcessor
- FPSDisplay
- Processor Template
- Common Patterns
- TransformSaveData
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
- BuildingModelHandler
- CreateDefaultSettingsAssets.cs
- BuildingResourceModelHandler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxAnimationController
- extraction-spec.md
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- EventProcessor
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- AutosaveIntervalsInstaller
- Easings
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Q: If there is more to do, keep going.
- FoliageGenerationSettings
- stream_town_domain
- import_save
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- BuildPlacerData
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- ObjectSelectionProcessor.Editor.cs
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- RandomEnabler

## God Nodes (most connected - your core abstractions)
1. `StableId` - 327 edges
2. `WorldSimulation` - 164 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 129 edges
8. `WorldGenProcessor` - 114 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `generated_terrain_mesh_matches_navigation_grid()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (263 total, 17 thin omitted)

### Community 0 - "MonoBehaviour"
Cohesion: 0.01
Nodes (85): Api, CellSpacePartitioningInstaller, ContainerBuilder, PersistentScoped, ContainerBuilder, Volume, PostProcessingInstaller, Camera (+77 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (17): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, Container, ContainerBuilder (+9 more)

### Community 2 - "Result"
Cohesion: 0.19
Nodes (38): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), field_value(), foliage_layers(), generated_record_ids(), glb_asset_path() (+30 more)

### Community 3 - "NavGrid"
Cohesion: 0.13
Nodes (20): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid, NavigationError, OpenNode, path_routes_around_dynamic_building() (+12 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.02
Nodes (91): List, CampGenSettings, float, Material, Volume, DayAndNightSettings, List, FoliageGenSettings (+83 more)

### Community 5 - "setup_rendering"
Cohesion: 0.05
Nodes (68): AmbientLight, Assets, animate_weather_particles(), building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced (+60 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "ShaderRef"
Cohesion: 0.08
Nodes (27): BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension, CloudMaterialUniform (+19 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "PlayerSaveData"
Cohesion: 0.09
Nodes (17): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+9 more)

### Community 12 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 13 - "Commands"
Cohesion: 0.07
Nodes (122): BackgroundColor, ChimneySmokeDef, PresentationCatalog, actor_material(), advance_falling_fish(), animate_chimney_smoke_particles(), animate_falling_fish(), animate_healing_effects() (+114 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.09
Nodes (12): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+4 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (41): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+33 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "UnityAsset"
Cohesion: 0.16
Nodes (36): ArchetypesById, archetype_bounds(), archetype_kind(), building_model_definitions(), building_node_age(), building_placements(), BuildingPlacement, component_field_value() (+28 more)

### Community 19 - "Station"
Cohesion: 0.07
Nodes (22): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+14 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (18): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+10 more)

### Community 23 - "PlayerSettings"
Cohesion: 0.09
Nodes (35): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+27 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "simulation.rs"
Cohesion: 0.07
Nodes (31): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() (+23 more)

### Community 26 - "Res"
Cohesion: 0.02
Nodes (294): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimationGraphHandle, AnimationTransitions, AppExit, AudioSink, ActorHealthFill (+286 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (21): Color, float, string, TechnologyTreeGroup, Vector2, int, List, Port (+13 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (11): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+3 more)

### Community 30 - "update_credits_fireworks"
Cohesion: 0.11
Nodes (26): FireworksVfxDef, credits_firework_origin(), credits_fireworks_active(), credits_fireworks_start(), credits_fireworks_use_authored_activation_and_deterministic_emission(), CreditsFireworkBurst, CreditsFireworkParticle, CreditsFireworkParticleKind (+18 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (28): HashSet, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject, HashSet (+20 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (16): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+8 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.08
Nodes (19): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+11 more)

### Community 39 - "StableId"
Cohesion: 0.04
Nodes (192): GameConfig, GameplayConfig, BTreeMap, ContentCatalog, ObjectiveDef, StationDef, FromStr, StableId (+184 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.14
Nodes (11): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.09
Nodes (51): animation_parameter_name(), archetype_scenes(), asset(), authored_mask(), authored_value(), child_technology_guids(), collect_model_dependencies(), component() (+43 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (32): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+24 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (230): AnyResult, ArchetypeScene, generate_world(), generate_world_with_content(), actor_detail_budget(), actor_scene_budget(), ActorHealthOverlay, adjust_settings_menu() (+222 more)

### Community 46 - "String"
Cohesion: 0.11
Nodes (50): MaterialDef, animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert_materials() (+42 more)

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (39): binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), checked_in_schema_one_fixture_imports_retained_terrain(), conversion_rejects_malformed_retained_mesh(), decode_json(), decode_legacy(), json_active_goal(), json_buildings() (+31 more)

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
Cohesion: 0.10
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "String"
Cohesion: 0.05
Nodes (56): active_event_text(), authored_rotating_node_names(), AuthoredCreditsElement, building_definition_id(), building_model_node_names(), CommandFeedback, CommandOrigin, compact_technology_label() (+48 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Enemy"
Cohesion: 0.09
Nodes (16): Action, float, Enemy, int, ActiveResourceIncrementer, uint, GUIDComponent, SaveableBuilding (+8 more)

### Community 57 - ".SetTargetType"
Cohesion: 0.20
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.25
Nodes (7): BinaryParser<'a>, decode_binary(), FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.10
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "twitch.rs"
Cohesion: 0.06
Nodes (52): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet, Default, Result, Self (+44 more)

### Community 68 - "Objective"
Cohesion: 0.12
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 70 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.08
Nodes (16): Color, Texture2D, EditorUtils, BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle (+8 more)

### Community 74 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.16
Nodes (12): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+4 more)

### Community 76 - "GeneratedWorld"
Cohesion: 0.08
Nodes (51): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise(), generate_foliage(), generate_shoreline_fish() (+43 more)

### Community 77 - "MiscCommands"
Cohesion: 0.18
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 78 - "AnimationHandler"
Cohesion: 0.11
Nodes (11): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+3 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "BuildingBase"
Cohesion: 0.08
Nodes (16): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, int (+8 more)

### Community 83 - "Vec"
Cohesion: 0.05
Nodes (59): AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationLayerBlendMode, ActivePetVisual, ActorAnimationDriver, add_animation_layer_branch(), advance_animation_crossfade() (+51 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "RoleHandler"
Cohesion: 0.05
Nodes (19): PlayerRoleData, AudioClip, bool, float, int, RoleHandler, bool, Dictionary (+11 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.10
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 88 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 91 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 92 - "CommonEnums.cs"
Cohesion: 0.08
Nodes (23): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+15 more)

### Community 93 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (75): animation_take_name(), avatar_mask_id(), clip_id(), controller_id(), convert(), convert_avatar_masks(), convert_chimney_smoke(), convert_clips() (+67 more)

### Community 96 - "PoolableObject"
Cohesion: 0.08
Nodes (17): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, Dictionary, float (+9 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (24): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+16 more)

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (68): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe (+60 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.09
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "StationSensor"
Cohesion: 0.10
Nodes (9): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor, UnityEvent (+1 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.07
Nodes (13): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+5 more)

### Community 105 - "GameEvent"
Cohesion: 0.05
Nodes (25): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+17 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - "Resource"
Cohesion: 0.08
Nodes (17): PlayerInventory, Dictionary, ResourceInventory, bool, int, IResourceHolder, float, int (+9 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.13
Nodes (10): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+2 more)

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "add_rotation_curve"
Cohesion: 0.24
Nodes (12): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), Item (+4 more)

### Community 114 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 115 - "convert"
Cohesion: 0.10
Nodes (31): ActorKind, ActorCustomization, StreamUserType, actor_prefix(), BinaryParser, clamped_cell(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+23 more)

### Community 116 - "Target"
Cohesion: 0.05
Nodes (21): UnitTravelToPosition, Vector3, STStateMachine.States, UserInterface.MainMenu, PlayerControls.ObjectSelection, Units, Behaviours, Target (+13 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.06
Nodes (23): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, TechVote (+15 more)

### Community 119 - "TownResourceProcessor"
Cohesion: 0.05
Nodes (17): DepositResources, ResourceStorageModifier, float, int, Container, ContainerBuilder, Dictionary, TownResourceProcessor (+9 more)

### Community 120 - "UserInterface_Roles"
Cohesion: 0.20
Nodes (6): bool, Dictionary, GameObject, Transform, UserInterface_Roles, Color32

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.10
Nodes (21): Action, bool, float, int, string, Type, Vector3, AIPath (+13 more)

### Community 125 - "IRuntimeDataScriptable"
Cohesion: 0.11
Nodes (17): CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+9 more)

### Community 126 - "EnemyWeaponModel"
Cohesion: 0.24
Nodes (4): GameObject, int, EnemyWeaponModel, RunAnimation

### Community 127 - "STSM_StateAction"
Cohesion: 0.11
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "Targetable"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+7 more)

### Community 132 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 133 - "HealthModifier"
Cohesion: 0.22
Nodes (6): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 136 - "ToolState"
Cohesion: 0.06
Nodes (86): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+78 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "PlayerRole"
Cohesion: 0.05
Nodes (20): RoleSlotModifier, int, RoleSlot, bool, int, Dictionary, int, RoleDataContainer (+12 more)

### Community 139 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "ObjectPoolingProcessor"
Cohesion: 0.07
Nodes (20): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+12 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 154 - "Vec"
Cohesion: 0.11
Nodes (46): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), inline_file_id(), json_f32() (+38 more)

### Community 155 - "SelectedBuilding"
Cohesion: 0.13
Nodes (4): bool, float, PassiveResourceIncrementer, SelectedBuilding

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 159 - "IProcessor"
Cohesion: 0.14
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

### Community 163 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 164 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 165 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 166 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 167 - "GridProcessor"
Cohesion: 0.07
Nodes (20): bool, int, Vector2, GridSettings, GridProcessorEditor, int, List, Vector2 (+12 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.09
Nodes (24): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+16 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 171 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 173 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (21): int, ChangeTimeStamp, ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData (+13 more)

### Community 179 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

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

### Community 184 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 185 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (88): ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, AllRoleDataSettingsInstaller (+80 more)

### Community 186 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 188 - "WeatherProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, WeatherProcessor, bool, float, VisualEffect, WeatherRuntimeData

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "Utils"
Cohesion: 0.04
Nodes (27): BuildCostModifier, InputButton, TownGoal.Data, Utils, Processors, Pets.Enumerations, StreamTown.EditorTools, TownGoal (+19 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 200 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.17
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SettingsProcessor.cs"
Cohesion: 0.25
Nodes (4): CameraProcessor, ContainerBuilder, SettingsInstaller, Settings

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - ".Log"
Cohesion: 0.07
Nodes (15): Action, Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory (+7 more)

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

### Community 224 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

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
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

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
Cohesion: 0.09
Nodes (10): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, List, string, ScriptableObjectAssetData, WindControllerEditor, GridSystemEditor (+2 more)

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 238 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 240 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "EventProcessor"
Cohesion: 0.22
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 270 - "import_save"
Cohesion: 0.43
Nodes (7): absolute_path(), backup_candidate(), import_preserves_source_and_recovers_named_backup(), import_save(), ImportReport, Path, PathBuf

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

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

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

## Knowledge Gaps
- **289 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+284 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **17 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `WorldSnapshot` (3× useful, score=2.549320929)
- `RenderAssets` (3× useful, score=2.486319819) _(code changed — re-verify)_
- `load_input()` (2× useful, score=1.699300406) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (2× useful, score=1.668719777) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.667551859)
- `MaterialDef` (2× useful, score=1.667082566)
- `PresentationCatalog` (2× useful, score=1.667082566)
- `BevyMigrationExporter` (2× useful, score=1.638916163)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `MonoBehaviour`, `BuildingProcessor`, `ScriptableObject`, `PlayerRole`, `SettingsProcessor`, `ObjectPoolingProcessor`, `SaveFileData`, `IProcessor`, `WorldGenProcessor`, `GameEventProcessor`, `TimeProcessor`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `ResourceDataSaveData`, `StreamTownSessionBridge`, `ScriptablesProcessorInfrastructure`, `.Log`, `FoliageProcessor`, `PoolableObject`, `PlayerProcessor`, `TownGoalProcessor`, `MainMenuManager`, `WorldGenSaveData`, `Target`, `TownResourceProcessor`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `MonoBehaviour`, `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `ObjectPoolingProcessor`, `WorldGenRuntimeData`, `Player`, `IProcessor`, `UserInterface_Debug`, `GridProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `ScriptablesProcessorInfrastructure`, `CellSpacePartitioning`, `.Log`, `FoliageProcessor`, `PoolableObject`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `GameEvent`, `EnemySpawner`, `Target`, `AIPath`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `Result`, `setup_rendering`, `ToolState`, `Commands`, `stream_town_domain/src/content.rs`, `save.rs`, `UnityAsset`, `simulation.rs`, `Res`, `Vec`, `update_credits_fireworks`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `stream_town_game/src/lib.rs`, `String`, `String`, `twitch.rs`, `command.rs`, `GeneratedWorld`, `Vec`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/presentation.rs`, `convert`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _289 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MonoBehaviour` be split into smaller, more focused modules?**
  _Cohesion score 0.01473783656117147 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.060655737704918035 - nodes in this community are weakly interconnected._
- **Should `NavGrid` be split into smaller, more focused modules?**
  _Cohesion score 0.12605042016806722 - nodes in this community are weakly interconnected._