# Graph Report - Stream-Town-Bevy  (2026-08-14)

## Corpus Check
- 630 files · ~1,650,187 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7697 nodes · 21203 edges · 286 communities (266 shown, 20 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1012 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `dbc664f4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- drive_converted_animations
- BuildingProcessor
- stream_town_migrate/src/content.rs
- UnityAsset
- ScriptableObject
- .GenerateFromSettings
- TwitchChatProcessor
- BinaryReader
- BottomBarInterface
- Option
- SettingsProcessor
- StableId
- Targetable
- Commands
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- SelectedObject
- Station
- .CreateEnumField
- UnityAsset
- BuildingPlacer
- ResMut
- UnitHealthBar
- simulation.rs
- Res
- TechTreeGraphView
- SaveFileData
- Player
- stream_town_game/src/lib.rs
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- MeshData
- CommonEnums.cs
- CommandDictionary
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- Result
- AnimationControllerRuntime
- update_credits_fireworks
- Result
- legacy.rs
- BinarySaveCodec
- .Draw
- AudioHandler
- StreamTownSessionBridge
- Processors
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- String
- StationSensor
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- Handle
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- settings.rs
- World.Generation.Settings
- models.rs
- Tiler
- ScriptablesEditor
- SimulationError
- UserInterface_ObjectSelection
- TimeProcessor
- Goal
- AnimationHandler
- TwitchBotSetupWindow
- SelectedPlayer
- WorldUtils
- BuildingBase
- .new
- Access_Text
- runtime_console.rs
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- RoleHandler
- Access_Toggle
- command.rs
- RoleData
- convert_fbx_to_glb.py
- stream_town_migrate/src/presentation.rs
- RoleDataContainer
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- IProcessor.cs
- StateMachine
- TechTreeNode
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- Utils
- UserInterface_TownGoal
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- UIElementWrapper
- SeasonDataSettings
- world.rs
- Target
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- ResourceHolder
- unity_color_filter
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- IRuntimeDataScriptable
- config.rs
- .EnsureValidCredentials
- UpdateGraphBounds
- GlobalAudioController
- WindController
- ErrorData
- GUIDProcessor
- EnemyModelHandler
- ConfirmCheck
- STStateMachine.States
- ToolState
- GateController
- PlayerRole
- .build
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- .Log
- RuntimeData Template
- Key Rules
- SelectedBuilding
- Pet
- add_file
- Enemy
- UserInterface_GameMenu
- VfxSeagullSpawner
- Stream Town Reloaded - Architecture Documentation
- .SaveGameAsync
- stream_town_domain/src/lib.rs
- BuildingSettings
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- IProcessor
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- SelectedPlayerGroup
- .BuildMatricesDictionary
- STSM_StateAction
- GridProcessor
- ResourceProcessor
- xtask/src/main.rs
- VfxParticlePosition
- add_rotation_curve
- TL_API
- UserInterface_RulerVote
- MeshSaveData
- EditorHelpers
- TL_Secrets
- SelectedEnemy
- TechTree.Elements
- SimpleDisableAfterTime
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SelectableObject
- MonoBehaviour
- .UserIsSubscribed
- DayAndNightProcessor
- WeatherProcessor
- WorldSimulation
- Character
- NodeUnlockData
- AudioSourcesProcessor
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- DebugProcessor
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- TransformSaveData
- Editor
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- ActorKind
- Common Patterns
- DontDestroyOnLoad
- FoliageData
- Requirement
- CreditsProcessor
- GUIDComponent
- WorldGenSaveData
- Key Rules
- ScriptablesProcessorInfrastructure
- RuntimeData Template
- GameStateProcessor
- STSM_Helper_Attack
- ScriptKeywordProcessor
- FPSDisplay.cs
- KeepKingVote
- Processor Template
- Common Patterns
- PlayerSaveData
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Twitch setup
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- EquipmentHandlerEditor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- WorldSaveData
- CreateProjectScopeProcessors.cs
- UI_TechOption
- TargetProcessor
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- ResourceDataSaveData
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- .RenderFoliageType
- SimpleScreenShot
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- PlayerDeathHandler
- extraction-spec.md
- NewKingVote
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- PlayerInputRuntimeData
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- PlayerSpawnPoint
- Easings
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- TwitchClientRuntimeData
- Q: If there is more to do, keep going.
- append_vec3_keys
- RotationHandler
- GridNode
- STSM_HelperBase
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- CampGenerationSettings
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- UserInterface
- Q: role level experience progression station equipment inventory skill upgrade
- FoliageRuntimeData
- RandomEnabler
- ObjectiveSaveData
- Autosave
- PlacementProbeHandler
- FoliageGenerationSettings.cs

## God Nodes (most connected - your core abstractions)
1. `StableId` - 310 edges
2. `WorldSimulation` - 162 edges
3. `Utils` - 158 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `Player` - 142 edges
7. `ContentCatalog` - 126 edges
8. `WorldGenProcessor` - 110 edges
9. `SettingsProcessor` - 107 edges
10. `Reflex.Core` - 103 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `embedded_config_matches_shipping_starting_roster()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `native_world_compatibility_accepts_current_and_verified_upgrades_only()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `structural_footprints_clear_and_restore_foliage_visibility()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (286 total, 20 thin omitted)

### Community 0 - "drive_converted_animations"
Cohesion: 0.07
Nodes (54): AnimationGraph, AnimationNodeIndex, AnimationPlayer, ActivePetVisual, ActorAnimationDriver, add_animation_layer_branch(), advance_animation_crossfade(), animation_event_occurrences() (+46 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (12): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, STSM_HelperDeposit, BuildingType, foodCost (+4 more)

### Community 2 - "stream_town_migrate/src/content.rs"
Cohesion: 0.13
Nodes (31): PassiveResourceContribution, asset(), component(), component_at(), converts_active_catalog_references_and_round_trips_ron(), converts_building_role_slot_modifiers(), converts_disable_after_time_lifetime(), converts_enemy_combat_and_camp_spawn_data() (+23 more)

### Community 3 - "UnityAsset"
Cohesion: 0.19
Nodes (32): ArchetypesById, archetype_bounds(), archetype_kind(), building_model_definitions(), building_node_age(), component_field_value(), component_reference_name(), component_reference_names() (+24 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.02
Nodes (83): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+75 more)

### Community 5 - ".GenerateFromSettings"
Cohesion: 0.08
Nodes (29): HashSet, Func, HashSet, Vector2, Vector3, GenerateDebugPositions(), GetPositiveNoiseOffset(), OnDrawGizmosSelected() (+21 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "BinaryReader"
Cohesion: 0.15
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Option"
Cohesion: 0.05
Nodes (111): AmbientLight, Assets, ArchetypeDef, ArchetypeKind, ArchetypeScene, ChimneySmokeDef, PresentationCatalog, animation_property_value() (+103 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "StableId"
Cohesion: 0.09
Nodes (37): FromStr, StableId, BuildingState, assign_group_role(), bottom_bar_entries(), building_construction_cost(), building_cost_reduction_percent(), building_definition_id() (+29 more)

### Community 12 - "Targetable"
Cohesion: 0.09
Nodes (11): List, Dictionary, List, TargetRuntimeData, bool, BoxCollider, float, int (+3 more)

### Community 13 - "Commands"
Cohesion: 0.08
Nodes (105): actor_material(), Agent, agent_is_moving(), AgentAnimation, animate_agents(), animate_building_effects(), animate_chimney_smoke_particles(), animate_combat_effects() (+97 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.09
Nodes (12): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+4 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (32): ArchetypeBounds, AuthoredRecord, AuthoredValue, ContentError, EnemyDef, EnemyModelSetDef, EnemyRunAnimation, EnemyWeaponModelDef (+24 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 18 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

### Community 19 - "Station"
Cohesion: 0.07
Nodes (21): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+13 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "UnityAsset"
Cohesion: 0.11
Nodes (36): animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert_materials(), convert_model_materials() (+28 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.06
Nodes (25): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+17 more)

### Community 23 - "ResMut"
Cohesion: 0.04
Nodes (123): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, advance_loading_phase(), AgentCommand, AgentCommandQueue, apply_building_commands(), apply_settings_draft() (+115 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "simulation.rs"
Cohesion: 0.08
Nodes (29): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+21 more)

### Community 26 - "Res"
Cohesion: 0.05
Nodes (129): Added, AnimationGraphHandle, AnimationTransitions, BackgroundColor, ActorNameOverlay, AgentEnemyModelPresentation, animate_falling_fish(), animation_root_name() (+121 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.06
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.12
Nodes (18): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, string (+10 more)

### Community 29 - "Player"
Cohesion: 0.04
Nodes (27): Player, Dictionary, GameObject, Vector3, Vector3, Action, Container, ContainerBuilder (+19 more)

### Community 30 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (238): AudioSink, ActionPresentation, active_event_text(), actor_combat_visual(), actor_detail_budget(), actor_scene_budget(), ActorHealthFill, ActorHealthOverlay (+230 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (18): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+10 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "MeshData"
Cohesion: 0.15
Nodes (15): List, Mesh, Vector2, Vector3, MeshData, Action, AnimationCurve, GameObject (+7 more)

### Community 35 - "CommonEnums.cs"
Cohesion: 0.17
Nodes (11): TargetableData, Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, Seasons, StationUpdate (+3 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.13
Nodes (9): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary, MiscCommands (+1 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.09
Nodes (12): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+4 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.04
Nodes (166): GameConfig, BuildingDef, BuildingModelDef, ContentCatalog, EnemySpawnerDef, LoadingScreenDef, RoleDef, BTreeSet (+158 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.13
Nodes (12): SimpleToggleCarry, AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment (+4 more)

### Community 43 - "Result"
Cohesion: 0.22
Nodes (34): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), field_value(), foliage_layers(), generated_record_ids(), objective_definitions() (+26 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.12
Nodes (23): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+15 more)

### Community 45 - "update_credits_fireworks"
Cohesion: 0.15
Nodes (20): FireworksVfxDef, credits_firework_origin(), credits_fireworks_start(), credits_fireworks_use_authored_activation_and_deterministic_emission(), CreditsFireworkBurst, CreditsFireworkParticle, CreditsFireworkParticleKind, CreditsFireworksEmitter (+12 more)

### Community 46 - "Result"
Cohesion: 0.11
Nodes (36): animation_state_id(), animation_state_machine_id(), avatar_mask_id(), clip_id(), controller_id(), convert(), convert_avatar_masks(), convert_chimney_smoke() (+28 more)

### Community 47 - "legacy.rs"
Cohesion: 0.17
Nodes (37): ActorCustomization, StreamUserType, clamped_cell(), decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies() (+29 more)

### Community 48 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 49 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 50 - "AudioHandler"
Cohesion: 0.14
Nodes (7): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "Processors"
Cohesion: 0.06
Nodes (12): BuildCostModifier, ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, Level, Processors.Editor, MetaData (+4 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.07
Nodes (13): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, bool, float, PassiveResourceIncrementer (+5 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "String"
Cohesion: 0.12
Nodes (25): animation_parameter_name(), archetype_scenes(), authored_mask(), building_placements(), BuildingPlacement, child_technology_guids(), collect_model_dependencies(), ContentConversionReport (+17 more)

### Community 57 - "StationSensor"
Cohesion: 0.10
Nodes (9): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor, UnityEvent (+1 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.09
Nodes (13): bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation, bool (+5 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 61 - "Handle"
Cohesion: 0.07
Nodes (38): bottom_bar_texture(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform, cosmetic_color() (+30 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "Node_SO"
Cohesion: 0.12
Nodes (13): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+5 more)

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
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.15
Nodes (3): Action, int, Objective

### Community 69 - "settings.rs"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 70 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.06
Nodes (20): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, BuildingScriptablesEditor, bool (+12 more)

### Community 74 - "SimulationError"
Cohesion: 0.16
Nodes (6): complete_gameplay_scenario_round_trips(), BTreeMap, Result, SimulationError, TechVote, validate_trade_resource()

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 77 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "BuildingBase"
Cohesion: 0.09
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 83 - ".new"
Cohesion: 0.09
Nodes (37): absolute_path(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions() (+29 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.18
Nodes (5): Bounds, Container, ContainerBuilder, HashSet, FoliageProcessor

### Community 90 - "RoleHandler"
Cohesion: 0.07
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, RoleHandler, bool, Dictionary (+6 more)

### Community 91 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 92 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 93 - "RoleData"
Cohesion: 0.08
Nodes (20): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+12 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (77): animation_take_name(), convert_embedded_model_clips(), convert_post_process(), embedded_clip_id(), field_array(), field_bool(), field_f32(), field_str() (+69 more)

### Community 96 - "RoleDataContainer"
Cohesion: 0.10
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.09
Nodes (20): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+12 more)

### Community 98 - "Coordinator"
Cohesion: 0.09
Nodes (19): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+11 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.06
Nodes (62): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef (+54 more)

### Community 100 - "IProcessor.cs"
Cohesion: 0.20
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "TechTreeNode"
Cohesion: 0.13
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.10
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.05
Nodes (25): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+17 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "Utils"
Cohesion: 0.03
Nodes (17): RoleScriptablesEditor, DisableOnAwake, SelectionBase, List, SimpleEventOnStart, SimpleHideRendererOnAwake, GameObject, SimpleRandomModelEnabled (+9 more)

### Community 108 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 114 - "SeasonDataSettings"
Cohesion: 0.18
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 115 - "world.rs"
Cohesion: 0.07
Nodes (50): WorldGenConfig, FoliageHabitat, FoliageLayerDef, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), grid(), manhattan(), NavGrid (+42 more)

### Community 116 - "Target"
Cohesion: 0.09
Nodes (11): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Sensors, Pets, GridSystem.Partitioning, Combat (+3 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 119 - "Resource"
Cohesion: 0.03
Nodes (38): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+30 more)

### Community 120 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.17
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (15): Queue, AudioRuntimeData, Queue, AudioSourcesRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool (+7 more)

### Community 126 - "config.rs"
Cohesion: 0.14
Nodes (17): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+9 more)

### Community 127 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

### Community 132 - "GUIDProcessor"
Cohesion: 0.13
Nodes (3): Container, ContainerBuilder, GUIDProcessor

### Community 133 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (8): bool, int, List, EnemyModelHandler, bool, float, Vector3, STSM_Action_EnemyAttack

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "STStateMachine.States"
Cohesion: 0.07
Nodes (12): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+4 more)

### Community 136 - "ToolState"
Cohesion: 0.09
Nodes (65): apply_technology_draft(), bounded_ui_index(), commit_catalog_candidate(), content_tab(), create_technology_group(), create_technology_node(), default_catalog_path(), delete_selected_technology_group() (+57 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "PlayerRole"
Cohesion: 0.11
Nodes (7): RoleSlotModifier, int, Container, ContainerBuilder, int, RoleProcessor, PlayerRole

### Community 139 - ".build"
Cohesion: 0.16
Nodes (12): AnyResult, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), locate_asset_root(), player_settings_path(), production_resource_glbs_expose_unity_masks_as_color_zero(), App (+4 more)

### Community 140 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - ".Log"
Cohesion: 0.05
Nodes (35): Action, bool, BoxCollider, CancellationToken, Container, ContainerBuilder, float, int (+27 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "Enemy"
Cohesion: 0.13
Nodes (7): Action, float, Enemy, Action, Container, ContainerBuilder, EventProcessor

### Community 151 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - ".SaveGameAsync"
Cohesion: 0.22
Nodes (5): CancellationToken, Task, ISaveStorage, SaveOperationState, SaveRuntimeData

### Community 154 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 155 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 159 - "IProcessor"
Cohesion: 0.13
Nodes (7): CancellationToken, Task, Container, IPostInitializeProcessor, IProcessor, Dictionary, ParallelProgressReporter

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

### Community 164 - "SelectedPlayerGroup"
Cohesion: 0.19
Nodes (3): List, List, SelectedPlayerGroup

### Community 165 - ".BuildMatricesDictionary"
Cohesion: 0.51
Nodes (6): Dictionary, Material, Matrix4x4, Mesh, material, mesh

### Community 166 - "STSM_StateAction"
Cohesion: 0.19
Nodes (5): bool, float, int, STSM_StateAction, AnimationName

### Community 167 - "GridProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 168 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (19): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+11 more)

### Community 170 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 171 - "add_rotation_curve"
Cohesion: 0.23
Nodes (13): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), ensure_two_keyframes(), normalized_quat(), Item (+5 more)

### Community 173 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 174 - "MeshSaveData"
Cohesion: 0.18
Nodes (7): bool, int, MeshSaveData, float, Vector2SaveData, float, Vector3SaveData

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 178 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 179 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

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

### Community 185 - "MonoBehaviour"
Cohesion: 0.01
Nodes (113): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+105 more)

### Community 186 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 187 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 188 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 189 - "WorldSimulation"
Cohesion: 0.10
Nodes (16): ObjectiveDef, EnemyCampState, FishGodState, objective_increment(), ObjectiveEvent, ObjectiveProgress, RaidState, BTreeSet (+8 more)

### Community 190 - "Character"
Cohesion: 0.05
Nodes (24): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+16 more)

### Community 192 - "AudioSourcesProcessor"
Cohesion: 0.31
Nodes (3): Container, ContainerBuilder, AudioSourcesProcessor

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "DebugProcessor"
Cohesion: 0.05
Nodes (21): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+13 more)

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 200 - "Editor"
Cohesion: 0.06
Nodes (14): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, EnemyModelHandlerEditor, WindControllerEditor (+6 more)

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

### Community 206 - "ActorKind"
Cohesion: 0.60
Nodes (5): ActorKind, actor_prefix(), entity_id(), resolve_legacy_archetype(), sanitize_component()

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 209 - "FoliageData"
Cohesion: 0.24
Nodes (6): List, Material, Mesh, Quaternion, Vector3, FoliageData

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 212 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 213 - "WorldGenSaveData"
Cohesion: 0.14
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "GameStateProcessor"
Cohesion: 0.19
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 218 - "STSM_Helper_Attack"
Cohesion: 0.25
Nodes (4): int, STSM_Helper_Attack, int, STSM_Action_Attack

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "PlayerSaveData"
Cohesion: 0.07
Nodes (24): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, bool (+16 more)

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

### Community 234 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "WorldSaveData"
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 240 - "TargetProcessor"
Cohesion: 0.32
Nodes (3): Container, ContainerBuilder, TargetProcessor

### Community 243 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - ".RenderFoliageType"
Cohesion: 0.32
Nodes (6): Dictionary, int, Material, Matrix4x4, Mesh, FoliageRenderer

### Community 248 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "PlayerDeathHandler"
Cohesion: 0.29
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 253 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 259 - "Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility"
Cohesion: 0.50
Nodes (3): Answer, Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "append_vec3_keys"
Cohesion: 0.67
Nodes (3): append_vec3_keys(), Item, Iterator

### Community 268 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 269 - "GridNode"
Cohesion: 0.14
Nodes (11): int, List, Vector2, GridNode, Color, CollisionColours, CollisionType, float (+3 more)

### Community 270 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 273 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 280 - "UserInterface"
Cohesion: 0.05
Nodes (22): InputButton, SharedTypes, int, ChangeTimeStamp, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+14 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "FoliageRuntimeData"
Cohesion: 0.33
Nodes (6): Dictionary, List, Material, Matrix4x4, Mesh, FoliageRuntimeData

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 290 - "ObjectiveSaveData"
Cohesion: 0.50
Nodes (3): int, string, ObjectiveSaveData

### Community 301 - "FoliageGenerationSettings.cs"
Cohesion: 0.50
Nodes (3): Mesh, Vector3, FoliageMeshSettings

## Knowledge Gaps
- **284 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+279 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **20 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.934728199)
- `RenderAssets` (2× useful, score=1.934728199) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `GUIDProcessor`, `ScriptableObject`, `PlayerRole`, `SettingsProcessor`, `.Log`, `UserInterface_GameMenu`, `.SaveGameAsync`, `Player`, `IProcessor`, `WorldGenProcessor`, `GameEventProcessor`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `MeshSaveData`, `StreamTownSessionBridge`, `MonoBehaviour`, `DebugProcessor`, `TimeProcessor`, `WorldGenSaveData`, `FoliageProcessor`, `PlayerSaveData`, `TownGoalProcessor`, `MainMenuManager`, `WorldSaveData`, `ResourceDataSaveData`, `Target`, `Resource`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `GUIDProcessor`, `.GenerateFromSettings`, `ScriptableObject`, `TwitchChatProcessor`, `.Log`, `Player`, `WorldGenRuntimeData`, `IProcessor`, `UserInterface_Debug`, `GridProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `MonoBehaviour`, `DebugProcessor`, `CellSpacePartitioning`, `FoliageProcessor`, `GameStateProcessor`, `PlayerSaveData`, `SaveProcessor`, `Coordinator`, `GameEvent`, `EnemySpawner`, `Target`, `AIPath`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `drive_converted_animations`, `stream_town_migrate/src/content.rs`, `UnityAsset`, `ToolState`, `Option`, `Commands`, `stream_town_domain/src/content.rs`, `save.rs`, `UnityAsset`, `ResMut`, `simulation.rs`, `Res`, `stream_town_domain/src/lib.rs`, `stream_town_game/src/lib.rs`, `ContentCatalog`, `add_rotation_curve`, `AnimationControllerRuntime`, `update_credits_fireworks`, `Result`, `Result`, `String`, `WorldSimulation`, `twitch.rs`, `SimulationError`, `ActorKind`, `.new`, `runtime_console.rs`, `command.rs`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/presentation.rs`, `world.rs`, `config.rs`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _284 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `drive_converted_animations` be split into smaller, more focused modules?**
  _Cohesion score 0.06638714185883997 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06848357791754019 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/content.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1319073083778966 - nodes in this community are weakly interconnected._