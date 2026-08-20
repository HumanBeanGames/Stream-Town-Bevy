# Graph Report - Stream-Town-Bevy  (2026-08-21)

## Corpus Check
- 638 files · ~1,667,268 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 7873 nodes · 21982 edges · 267 communities (246 shown, 21 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 1020 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `af3cccb4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- World.Generation.Settings
- BuildingProcessor
- GeneratedWorld
- GridPos
- ScriptableObject
- embedded_content
- TwitchChatProcessor
- BinarySaveCodec
- BottomBarInterface
- Option
- SettingsProcessor
- SaveDataMapper
- .Log
- Commands
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- Result
- Station
- ObjectiveSaveData
- Goal
- BuildingPlacer
- Vec4
- UnitHealthBar
- runtime_console.rs
- Res
- TechTreeGraphView
- SaveFileData
- Player
- StableId
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- TechTreeNode
- CommandDictionary
- SettingsData
- SeasonProcessor
- ContentCatalog
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- stream_town_migrate/src/content.rs
- AnimationControllerDef
- stream_town_game/src/lib.rs
- stream_town_migrate/src/presentation.rs
- Value
- ResourceDataSaveData
- .CreateEnumField
- IRuntimeDataScriptable
- StreamTownSessionBridge
- .default
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- Utils
- .SetTargetType
- STSM_GoToLocation
- TechTreeEditorWindow
- Result
- UserInterface
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
- StationSensor
- UserInterface_ObjectSelection
- process_injected_commands
- update_credits_fireworks
- AnimationHandler
- TwitchBotSetupWindow
- CommonEnums.cs
- WorldUtils
- config.rs
- Vec
- Access_Text
- PlayerRoleData
- CellSpacePartitioning
- UserInterface_TownVote
- TargetSensor
- FoliageProcessor
- UserInterface_TownGoal
- Enemy
- RoleHandler
- STSM_Idle_Player
- convert_fbx_to_glb.py
- RoleDataContainer
- technology_tab
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- PlayerProcessor
- StateMachine
- IProcessor.cs
- TownGoalProcessor
- MainMenuManager
- GameEvent
- LoadingManager
- UIElementWrapper
- PlayerInventory
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- UserInterface_RulerVote
- WorldGenSaveData
- legacy.rs
- Target
- stream_town_migrate/src/main.rs
- VoteEvent
- Resource
- EditorUtils
- unity_color_filter
- World.Generation
- SnapToGridMouseMovement
- AIPath
- TwitchUser
- SimpleDisableAfterTime
- KeepKingVote
- UpdateGraphBounds
- GlobalAudioController
- WindController
- GUIDComponent
- UI_TechOption
- SensorBase
- ConfirmCheck
- .GetAvailableContainers
- ToolState
- GateController
- RoleProcessor
- IProcessor
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- SelectedBuilding
- Pet
- add_file
- ResourceDataSettings
- DontDestroyOnLoad
- GameConfig
- Stream Town Reloaded - Architecture Documentation
- WorldGenRuntimeData
- .RefreshSceneBindingsAndTryGenerate
- DebugSettings
- UnitTextDisplay
- Stream Town Reloaded - Architecture Documentation
- NewKingVote
- .InitializeAndActivateProcessorsAsync
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SensorProcessor
- TimeProcessor
- Access_Toggle
- TechTree_SO
- GridNode
- ResourceProcessor
- xtask/src/main.rs
- SelectedPlayer
- RotationHandler
- .DrawDataFieldAndLabel
- TL_API
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- .CreatePort
- SelectedEnemy
- TechTree.Elements
- CellPartitioningEditor
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- parse_property_curves
- MonoBehaviour
- ResourceHolder
- GridProcessor
- setup_camera
- UserInterface_GameMenu
- StringUtils
- Targetable
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- SelectedResource
- Character
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
- TerrainGenSettings
- Key Rules
- ScriptablesProcessorInfrastructure
- RuntimeData Template
- ScriptKeywordProcessor
- FPSDisplay
- DayAndNightSettings
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
- Editor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- BuildingModelHandler
- CreateProjectScopeProcessors.cs
- TechNodeData
- VfxParticlePosition
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxSeagullSpawner
- extraction-spec.md
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Q: If there is more to do, keep going.
- FoliageGenerationSettings
- VFXArrowPointer
- stream_town_domain
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Processors
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Autosave
- Q: role level experience progression station equipment inventory skill upgrade
- PostProcessingInstaller
- RandomEnabler
- RenderPipelineInstaller

## God Nodes (most connected - your core abstractions)
1. `StableId` - 330 edges
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
- `embedded_config_matches_shipping_starting_roster()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `falling_fish_uses_authored_gravity_terrain_bounce_and_lifetime_loss()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generated_terrain_chunks_cover_the_grid_with_watertight_seams()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (267 total, 21 thin omitted)

### Community 0 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.04
Nodes (36): BuildingBase, bool, float, int, List, UnityEvent, TilerBuilding, bool (+28 more)

### Community 2 - "GeneratedWorld"
Cohesion: 0.14
Nodes (38): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), fnv_mix(), generate_authored_resources() (+30 more)

### Community 3 - "GridPos"
Cohesion: 0.09
Nodes (34): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError (+26 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.03
Nodes (67): int, AudioSettings, List, CampGenSettings, List, FoliageGenSettings, List, GameSettings (+59 more)

### Community 5 - "embedded_content"
Cohesion: 0.07
Nodes (65): ArchetypeKind, changing_seed_changes_world_hash(), generate_world(), generated_resources_preserve_unity_target_types_and_reachable_fish(), generation_is_deterministic(), actor_combat_visual(), agent_action_facing_grid(), AgentGoal (+57 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (26): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+18 more)

### Community 7 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Option"
Cohesion: 0.03
Nodes (134): AmbientLight, Assets, ArchetypeScene, actor_detail_budget(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), animation_event_occurrences(), apply_material_overrides(), archetype_by_source() (+126 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "SaveDataMapper"
Cohesion: 0.10
Nodes (17): Dictionary, Mesh, Vector3, SaveDataMapper, bool, int, List, string (+9 more)

### Community 12 - ".Log"
Cohesion: 0.09
Nodes (9): Action, Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, Container, ContainerBuilder (+1 more)

### Community 13 - "Commands"
Cohesion: 0.07
Nodes (113): PresentationCatalog, actor_material(), actor_scene_budget(), advance_falling_fish(), animate_chimney_smoke_particles(), animate_healing_effects(), bottom_bar_texture(), building_effect_material() (+105 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.06
Nodes (20): HealthModifier, bool, float, GameObject, HealUnit, BuildingDamageMaterialHandler, bool, IEnumerator (+12 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.07
Nodes (53): ArchetypeBounds, ArchetypeDef, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, EnemyDef (+45 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "Result"
Cohesion: 0.09
Nodes (60): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), convert_fireworks(), convert_healing_vfx(), f32_to_u16() (+52 more)

### Community 19 - "Station"
Cohesion: 0.09
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 20 - "ObjectiveSaveData"
Cohesion: 0.18
Nodes (7): Button, EnumField, ObjectiveVisualElement, ObjectiveSaveData, ObjectiveType, EnemyType, TownGoal.Enumerations

### Community 21 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.06
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 23 - "Vec4"
Cohesion: 0.08
Nodes (27): BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension, CloudMaterialUniform (+19 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "runtime_console.rs"
Cohesion: 0.15
Nodes (21): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+13 more)

### Community 26 - "Res"
Cohesion: 0.05
Nodes (173): Aabb, AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AnimationTransitions, AppExit, AudioSink, ActorNameOverlay (+165 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (20): Color, float, string, TechnologyTreeGroup, int, List, Port, Vector2 (+12 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (13): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, ModeratorCommands (+5 more)

### Community 30 - "StableId"
Cohesion: 0.05
Nodes (70): ObjectiveDef, ObjectiveKind, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow() (+62 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (28): HashSet, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject, HashSet (+20 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, ParticleSystem, SortedSet, Transform, GameEventProcessor, bool, ParticleSystem (+9 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - "TechTreeNode"
Cohesion: 0.09
Nodes (15): NodeUnlockSaveData, Color, Foldout, List, Port, Sprite, Vector2, VisualElement (+7 more)

### Community 36 - "CommandDictionary"
Cohesion: 0.15
Nodes (8): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary, MiscCommands

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.05
Nodes (32): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator, float, int, Material (+24 more)

### Community 39 - "ContentCatalog"
Cohesion: 0.08
Nodes (63): ContentCatalog, ActorState, RoleProgress, Default, String, action_animation_speed(), action_cooldown(), actor_accepts_resource() (+55 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.07
Nodes (20): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+12 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.09
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (125): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+117 more)

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (24): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+16 more)

### Community 45 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (224): AnyResult, active_event_text(), ActorHealthFill, ActorHealthOverlay, advance_loading_runtime(), agent_action_animation(), AgentCommandQueue, AgentEnemyModelPresentation (+216 more)

### Community 46 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (113): MaterialAlphaMode, MaterialDef, animation_take_name(), animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies() (+105 more)

### Community 47 - "Value"
Cohesion: 0.26
Nodes (23): decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps(), json_f32_default(), json_foliage() (+15 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".CreateEnumField"
Cohesion: 0.13
Nodes (15): Button, EnumField, UnlockVisualElement, EnumField, Foldout, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "IRuntimeDataScriptable"
Cohesion: 0.05
Nodes (28): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+20 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - ".default"
Cohesion: 0.03
Nodes (92): PlayerSettings, Default, adjust_settings_menu(), agent_facing_matches_unity_rotation_and_action_targets(), apply_authored_local_rotation(), apply_settings_draft(), bottom_bar_input(), bottom_bar_main_buttons() (+84 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.07
Nodes (35): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+27 more)

### Community 56 - "Utils"
Cohesion: 0.07
Nodes (5): BuildCostModifier, Utils, Level, Buildings, GameResources

### Community 57 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "Result"
Cohesion: 0.32
Nodes (3): BinaryParser<'a>, Result, LegacyWorldState

### Community 61 - "UserInterface"
Cohesion: 0.06
Nodes (13): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, UserInterface (+5 more)

### Community 62 - "CameraController"
Cohesion: 0.08
Nodes (12): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+4 more)

### Community 63 - "Node_SO"
Cohesion: 0.15
Nodes (11): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+3 more)

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (10): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, ICollection, IDictionary, ISerializationCallbackReceiver (+2 more)

### Community 67 - "twitch.rs"
Cohesion: 0.09
Nodes (38): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, envelope_from_privmsg(), OAuthClient, OAuthErrorResponse (+30 more)

### Community 68 - "Objective"
Cohesion: 0.15
Nodes (3): Action, int, Objective

### Community 69 - "command.rs"
Cohesion: 0.06
Nodes (63): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+55 more)

### Community 70 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.13
Nodes (8): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerGate

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (11): BuildingScriptablesEditor, bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject (+3 more)

### Community 74 - "StationSensor"
Cohesion: 0.16
Nodes (8): List, Dictionary, List, Queue, StationRuntimeData, UnityEvent, StationSensor, StationMask

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "process_injected_commands"
Cohesion: 0.11
Nodes (30): StreamUserType, bottom_bar_authored_order(), bottom_bar_entries(), BottomBarAction, building_definition_id(), building_icon_path(), CommandOrigin, eligible_technology_ids() (+22 more)

### Community 77 - "update_credits_fireworks"
Cohesion: 0.11
Nodes (26): BackgroundColor, animation_property_value(), credits_firework_origin(), credits_fireworks_active(), credits_fireworks_start(), credits_fireworks_use_authored_activation_and_deterministic_emission(), CreditsFireworkBurst, CreditsFireworkParticle (+18 more)

### Community 78 - "AnimationHandler"
Cohesion: 0.06
Nodes (20): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+12 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "CommonEnums.cs"
Cohesion: 0.07
Nodes (19): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+11 more)

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "config.rs"
Cohesion: 0.14
Nodes (19): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+11 more)

### Community 83 - "Vec"
Cohesion: 0.04
Nodes (94): AnimationClip, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, AnimationTargetId, ActivePetVisual, ActorAnimationDriver (+86 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "PlayerRoleData"
Cohesion: 0.09
Nodes (14): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, List (+6 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.14
Nodes (13): Bounds, bool, List, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "TargetSensor"
Cohesion: 0.13
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 91 - "Enemy"
Cohesion: 0.06
Nodes (18): Action, float, Enemy, Animator, GameObject, IEnumerator, int, FishGodEvent (+10 more)

### Community 92 - "RoleHandler"
Cohesion: 0.06
Nodes (25): RoleData, AudioClip, bool, float, int, Sprite, string, RoleHandler (+17 more)

### Community 93 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): AttackUnit, STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 95 - "RoleDataContainer"
Cohesion: 0.12
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 96 - "technology_tab"
Cohesion: 0.31
Nodes (16): apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_technology_group(), delete_selected_technology_node(), refresh_technology_draft(), Option (+8 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (21): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+13 more)

### Community 98 - "Coordinator"
Cohesion: 0.10
Nodes (15): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, IEnumerable (+7 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (80): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef (+72 more)

### Community 100 - "PlayerProcessor"
Cohesion: 0.07
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 101 - "StateMachine"
Cohesion: 0.08
Nodes (13): int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase, bool, List, string (+5 more)

### Community 102 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GameEvent"
Cohesion: 0.07
Nodes (10): SortBuildingByLowerLevel, EventType, Action, bool, double, object, EventType, GameEvent (+2 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - "PlayerInventory"
Cohesion: 0.22
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 109 - "CustomLogHandler"
Cohesion: 0.19
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, DebugLogCategory, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.15
Nodes (4): int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 113 - "UserInterface_RulerVote"
Cohesion: 0.15
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 114 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 115 - "legacy.rs"
Cohesion: 0.09
Nodes (54): ActorKind, ActorCustomization, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser (+46 more)

### Community 116 - "Target"
Cohesion: 0.06
Nodes (16): STStateMachine.States, PlayerControls.ObjectSelection, Units, Behaviours, Target, Animation, Utils.Pooling, Sensors (+8 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "VoteEvent"
Cohesion: 0.18
Nodes (10): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+2 more)

### Community 119 - "Resource"
Cohesion: 0.04
Nodes (33): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer (+25 more)

### Community 120 - "EditorUtils"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - "TwitchUser"
Cohesion: 0.18
Nodes (9): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+1 more)

### Community 126 - "SimpleDisableAfterTime"
Cohesion: 0.05
Nodes (16): PersistentScoped, Transform, PlayerSpawnPoint, Image, TextMeshProUGUI, UIRoleDisplay, float, GameObject (+8 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 130 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 131 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 132 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 133 - "SensorBase"
Cohesion: 0.15
Nodes (4): float, List, SensorRuntimeData, SensorBase

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (46): bounded_ui_index(), content_tab(), default_catalog_path(), draw_world_preview(), inject_runtime_command(), inspector_tab(), launch_runtime_game(), main() (+38 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "RoleProcessor"
Cohesion: 0.08
Nodes (9): RoleSlotModifier, int, Container, ContainerBuilder, int, List, RoleProcessor, List (+1 more)

### Community 139 - "IProcessor"
Cohesion: 0.05
Nodes (21): Container, ContainerBuilder, LabelDisplayProcessor, Container, IProcessor, Container, ContainerBuilder, CreditsProcessor (+13 more)

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
Cohesion: 0.04
Nodes (38): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+30 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "SelectedBuilding"
Cohesion: 0.08
Nodes (5): SelectedBuilding, SelectedEnemyCamp, object, UnityAction, SelectedObject

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 151 - "GameConfig"
Cohesion: 0.13
Nodes (34): GameConfig, RoleDef, BTreeSet, StationDef, actor_resource_storage_has_room(), apply_recruit_group_order(), assigned_station(), best_station_id() (+26 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 155 - "DebugSettings"
Cohesion: 0.48
Nodes (3): Dictionary, DebugSettings, SerializedScriptableObject

### Community 156 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 159 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.21
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 163 - "SensorProcessor"
Cohesion: 0.38
Nodes (3): Container, ContainerBuilder, SensorProcessor

### Community 164 - "TimeProcessor"
Cohesion: 0.20
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 165 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 166 - "TechTree_SO"
Cohesion: 0.33
Nodes (3): NodeGroup_SO, List, TechTree_SO

### Community 167 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 168 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 171 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 172 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - ".CreatePort"
Cohesion: 0.40
Nodes (4): Port, Capacity, Direction, Orientation

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

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

### Community 184 - "parse_property_curves"
Cohesion: 0.67
Nodes (4): parse_animation_events(), parse_property_curves(), parses_property_curves_and_animation_events_without_unity_types(), unity_scalar()

### Community 185 - "MonoBehaviour"
Cohesion: 0.02
Nodes (105): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, Camera (+97 more)

### Community 186 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 187 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 192 - "Targetable"
Cohesion: 0.10
Nodes (8): List, bool, BoxCollider, float, int, Transform, Vector3, Targetable

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "Character"
Cohesion: 0.08
Nodes (13): Pets.Enumerations, TownGoal, Character.Enumerations, Core, Pets, GameEventSystem, GameEventSystem.Events, SavingAndLoading (+5 more)

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
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 213 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 224 - "PlayerSaveData"
Cohesion: 0.06
Nodes (27): Component, Transform, int, List, string, uint, BuildingSaveData, int (+19 more)

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
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "TechNodeData"
Cohesion: 0.25
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

### Community 240 - "VfxParticlePosition"
Cohesion: 0.22
Nodes (4): Transform, VisualEffect, VfxParticlePosition, VFX

### Community 244 - "Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?, Source Nodes

### Community 246 - "Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?, Source Nodes

### Community 247 - "Q: How are shipping visual and audio parity implemented in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How are shipping visual and audio parity implemented in the Bevy migration?, Source Nodes

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

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

### Community 267 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 268 - "VFXArrowPointer"
Cohesion: 0.33
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 269 - "stream_town_domain"
Cohesion: 0.50
Nodes (5): stream_town_domain, stream_town_game, stream_town_migrate, stream_town_tools, xtask

### Community 271 - "Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime, Source Nodes

### Community 272 - "Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?, Source Nodes

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 277 - "Processors"
Cohesion: 0.07
Nodes (11): ObjectSelectionProcessor, InputButton, UserInterface.MainMenu, Processors, World, Processors.Editor, MetaData, Audio (+3 more)

### Community 278 - "Q: How does Bevy new-town population now match Unity shipping startup?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does Bevy new-town population now match Unity shipping startup?, Source Nodes

### Community 279 - "Q: Unity station TargetSensor distance range generated resource targeting parity Bevy"
Cohesion: 0.50
Nodes (3): Answer, Q: Unity station TargetSensor distance range generated resource targeting parity Bevy, Source Nodes

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 285 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 286 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

## Knowledge Gaps
- **293 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+288 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `RenderAssets` (4× useful, score=3.481802588) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.66401759)
- `WorldSnapshot` (3× useful, score=2.54480177)
- `WorldSimulation` (2× useful, score=1.847279109)
- `load_input()` (2× useful, score=1.696288071) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (2× useful, score=1.665761652) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.664595804)
- `MaterialDef` (2× useful, score=1.664127343)
- `BevyMigrationExporter` (2× useful, score=1.63601087)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `ScriptableObject`, `RoleProcessor`, `SaveDataMapper`, `IProcessor`, `.Log`, `SettingsProcessor`, `ObjectPoolingProcessor`, `SaveFileData`, `WorldGenProcessor`, `GameEventProcessor`, `TimeProcessor`, `SeasonProcessor`, `ResourceProcessor`, `TechTreeProcessor`, `ResourceDataSaveData`, `StreamTownSessionBridge`, `MonoBehaviour`, `UserInterface_GameMenu`, `Character`, `WorldSaveData`, `FoliageProcessor`, `PlayerSaveData`, `PlayerProcessor`, `TownGoalProcessor`, `MainMenuManager`, `Resource`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `ScriptableObject`, `TwitchChatProcessor`, `IProcessor`, `.Log`, `ObjectPoolingProcessor`, `WorldGenRuntimeData`, `Player`, `UserInterface_Debug`, `ResourceProcessor`, `TwitchClientProcessor`, `Access_Dropdown`, `MonoBehaviour`, `GridProcessor`, `TerrainGenSettings`, `CellSpacePartitioning`, `FoliageProcessor`, `Enemy`, `PlayerSaveData`, `SaveProcessor`, `Coordinator`, `PlayerProcessor`, `DayAndNightProcessor`, `Target`, `AIPath`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `GeneratedWorld`, `GridPos`, `embedded_content`, `ToolState`, `Option`, `Commands`, `stream_town_domain/src/content.rs`, `save.rs`, `Result`, `GameConfig`, `runtime_console.rs`, `Res`, `ContentCatalog`, `stream_town_migrate/src/content.rs`, `AnimationControllerDef`, `stream_town_game/src/lib.rs`, `stream_town_migrate/src/presentation.rs`, `twitch.rs`, `command.rs`, `process_injected_commands`, `update_credits_fireworks`, `config.rs`, `Vec`, `technology_tab`, `stream_town_domain/src/presentation.rs`, `legacy.rs`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _293 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World.Generation.Settings` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.035998615437867774 - nodes in this community are weakly interconnected._
- **Should `GeneratedWorld` be split into smaller, more focused modules?**
  _Cohesion score 0.13510520487264674 - nodes in this community are weakly interconnected._