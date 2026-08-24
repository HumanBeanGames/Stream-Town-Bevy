# Graph Report - Stream-Town-Bevy  (2026-08-24)

## Corpus Check
- 661 files · ~1,708,450 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8464 nodes · 24276 edges · 319 communities (296 shown, 23 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1026 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `871aa394`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Sensors
- BuildingProcessor
- GeneratedWorld
- BinarySaveCodec
- ScriptableObject
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- Option
- BottomBarInterface
- Target
- SettingsProcessor
- BuildingPlacer
- StableId
- stream_town_migrate/src/presentation.rs
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- World.Generation.Settings
- finish_world_reveal
- GameConfig
- .Log
- BuildingBase
- RoleDataContainer
- UnitHealthBar
- drive_tidal_music
- Query
- TechTreeGraphView
- SaveFileData
- Player
- Option
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- PlayerProcessor
- .CreateEnumField
- SettingsData
- SeasonProcessor
- RoleHandler
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- WorldSimulation
- AnimationControllerRuntime
- RenderAssets
- STSM_Idle_Player
- PlayerRoleData
- .GetResourceAssets
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ContentCatalog
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- WorldInstanceDeterminism
- String
- Targetable
- TechTreeEditorWindow
- Result
- TechnologyGraphViewState
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- twitch.rs
- Objective
- List
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- IProcessor
- UserInterface_ObjectSelection
- SensorProcessor
- UserInterface_TownVote
- tools_ui
- TwitchBotSetupWindow
- String
- WorldUtils
- Node_SO
- ShaderRef
- Access_Text
- convert
- Station
- SelectableObject
- EditorUtils
- FoliageProcessor
- TransformSaveData
- GameEvent
- TechTreeNode
- GridPos
- convert_fbx_to_glb.py
- SelectedResource
- PlayerInventory
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- string
- StateMachine
- AnimationHandler
- TownGoalProcessor
- MainMenuManager
- GUIDComponent
- LoadingManager
- BuildingModelHandler
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- VfxSeagullSpawner
- PlayerControls
- StationProcessor
- GridNode
- stream_town_migrate/src/main.rs
- Goal
- Resource
- VoteEvent
- unity_color_filter
- UnitTextDisplay
- SnapToGridMouseMovement
- AIPath
- .AddResource
- BuildingResourceModelHandler
- GateController
- UpdateGraphBounds
- STSM_StateAction
- GameStateProcessor
- RandomEnabler
- StringUtils
- twitch_tab
- ToolState
- ResourceProcessor
- runtime_console.rs
- SelectedObject
- EnemyModelHandler
- WorldGenRuntimeData
- TargetProcessor
- What You Must Do When Invoked
- RuntimeData Template
- legacy.rs
- RuntimeData Template
- Key Rules
- parse_controller
- Pet
- add_file
- CellSpacePartitioning
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- UserInterface_TownGoal
- TownResourceProcessor
- Result
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WindController
- UserInterface_BuildingHealthBar
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- setup_rendering
- PlayerInputProcessor
- ResourceStorageModifier
- .SetTargetType
- SimpleMusicController
- .InitializeAndActivateProcessorsAsync
- Q: There are still no animations.
- xtask/src/main.rs
- DontDestroyOnLoad
- stream_town_tools/src/main.rs
- Utils
- UIElementWrapper
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- Editor
- STSM_Helper_Attack
- TechTree.Elements
- ResourceHolder
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- BuildingDataSettings
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- UserInterface
- GridProcessor
- WorldGenerationReferenceExporter
- MonoBehaviour
- Access_Dropdown
- BevyMigrationExporter
- UserInterface_RulerVote
- stream_town_migrate/src/menu_scene.rs
- Character
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- TownResourceRuntimeData
- UserInterface_GameMenu
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- Access_GOList
- TerrainGenSettings
- graphify reference: extra exports and benchmark
- Key Rules
- IRuntimeDataScriptable
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- PoolableObject
- RotationHandler
- Requirement
- CommandDictionary
- SelectedEnemy
- CommonEnums.cs
- Key Rules
- BuildingDamageMaterialHandler
- RuntimeData Template
- Easings
- ScriptKeywordProcessor
- FPSDisplay
- ScriptablesProcessorInfrastructure
- Processor Template
- Common Patterns
- Q: https://github.com/HumanBeanGames/bevy-tidal/tree/codex/native-rust-pattern-engine Set up this library as the music system.
- Q: How does the Bevy migration reproduce Unity Targetable assignment scoring?
- graphify reference: query, path, explain
- TODO List
- Q: native save world seed load current config generate_world_with_content compatibility legacy imported save different seed
- Character Animation Regression Checklist
- graphify reference: add a URL and watch a folder
- Q: shader material giraffe pet skinning prefab reachable shipping presentation
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- SelectedEnemyCamp
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- STSM_HelperBase
- IntWrapper
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- PostProcessingInstaller
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- ResMut
- extraction-spec.md
- STSM_GoToLocation
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- SelectedBuilding
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- PlayerSaveData
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- .StartupSequence
- Q: If there is more to do, keep going.
- AudioMixerInstaller
- .ExportModification
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- .new
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- AutosaveIntervalsInstaller
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- NodeUnlockData
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- VfxParticlePosition
- record_gpu_readiness
- ForwardRendererInstaller
- RenderPipelineInstaller
- StreamTown.Migration
- .RefreshSceneBindingsAndTryGenerate
- DebugProcessor
- VideoSettingsPresetsInstaller
- ObjectSelectionProcessor.Editor.cs
- UI_TechOption
- TL_API
- CreateProjectScopeProcessors.cs
- EquipmentHandlerEditor
- settings.rs
- Processors
- ResourceDataSettings
- DayAndNightSettings
- KeepKingVote
- VfxAnimationController
- CreditsProcessor
- Enemy
- NewKingVote
- Q: Characters are still not animated. Trees still have the flickering shadows.
- EventCommands
- GameEventSettings
- GameEventRuntimeData
- UnityGraphics
- EventProcessor
- PlacementProbeHandler
- ScriptableObjectAssetData
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- SimpleDisableAfterTime
- player_window_mode
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- TradeProcessor
- import_save

## God Nodes (most connected - your core abstractions)
1. `StableId` - 351 edges
2. `WorldSimulation` - 165 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ScriptablesProcessorInfrastructure` - 150 edges
6. `ContentCatalog` - 143 edges
7. `Player` - 142 edges
8. `RenderAssets` - 125 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `enemy_model_node_count()` --references--> `EnemyModelSetDef`  [EXTRACTED]
  bevy-port/crates/stream_town_migrate/src/content.rs → bevy-port/crates/stream_town_domain/src/content.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `builder_completes_and_upgrades_authored_construction()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (319 total, 23 thin omitted)

### Community 0 - "Sensors"
Cohesion: 0.09
Nodes (7): STStateMachine.States, Behaviours, Animation, Sensors, STStateMachine, Pathfinding, STStateMachine.Helpers

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (16): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+8 more)

### Community 2 - "GeneratedWorld"
Cohesion: 0.10
Nodes (53): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash() (+45 more)

### Community 3 - "BinarySaveCodec"
Cohesion: 0.09
Nodes (11): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, int (+3 more)

### Community 4 - "ScriptableObject"
Cohesion: 0.04
Nodes (68): int, AudioSettings, List, CampGenSettings, List, FoliageGenSettings, List, GameSettings (+60 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (197): AnyResult, action_ranges_and_tower_acquisition_are_euclidean(), ActorHealthFill, ActorHealthOverlay, advance_falling_fish(), AgentEnemyModelPresentation, AgentEquipmentPresentation, AmbienceAudio (+189 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (27): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+19 more)

### Community 7 - "Option"
Cohesion: 0.05
Nodes (107): ArchetypeScene, PresentationCatalog, actor_detail_budget(), actor_scene_budget(), animation_event_occurrences(), animation_root_name(), animation_selection_duration(), archetype_by_source() (+99 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Target"
Cohesion: 0.17
Nodes (6): Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, GUIDSystem

### Community 10 - "SettingsProcessor"
Cohesion: 0.06
Nodes (12): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+4 more)

### Community 11 - "BuildingPlacer"
Cohesion: 0.07
Nodes (21): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+13 more)

### Community 12 - "StableId"
Cohesion: 0.07
Nodes (54): FromStr, StableId, StreamUserType, actor_combat_visual(), assign_group_role(), bottom_bar_authored_order(), bottom_bar_entries(), BottomBarAction (+46 more)

### Community 13 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (82): animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), controller_id(), convert(), convert_avatar_masks(), convert_chimney_smoke() (+74 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.12
Nodes (7): Action, bool, float, int, UnityEvent, HealthHandler, ReviveType

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (43): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+35 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 19 - "finish_world_reveal"
Cohesion: 0.06
Nodes (55): AssetId, AtomicU64, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), asset_root_collection_ready(), begin_world_loading(), begin_world_loading_cover() (+47 more)

### Community 20 - "GameConfig"
Cohesion: 0.08
Nodes (46): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, BTreeMap, Default (+38 more)

### Community 21 - ".Log"
Cohesion: 0.06
Nodes (25): Action, CancellationToken, Task, IAsyncInitializableProcessor, ProcessorStartupContext, Action, bool, BoxCollider (+17 more)

### Community 22 - "BuildingBase"
Cohesion: 0.11
Nodes (11): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+3 more)

### Community 23 - "RoleDataContainer"
Cohesion: 0.12
Nodes (10): RoleSlot, bool, int, Dictionary, int, RoleDataContainer, bool, Dictionary (+2 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "drive_tidal_music"
Cohesion: 0.19
Nodes (24): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), effective_music_gain(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature (+16 more)

### Community 26 - "Query"
Cohesion: 0.05
Nodes (172): Aabb, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, ActivePetVisual, ActorAnimationDriver (+164 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.09
Nodes (16): Vector2, int, List, Port, Vector2, TechTreeGraphView, List, Texture2D (+8 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (11): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+3 more)

### Community 30 - "Option"
Cohesion: 0.14
Nodes (29): animator_component(), animator_reference_path(), color_value(), convert_post_process(), extracts_indexed_material_properties(), field_array(), field_bool(), field_f32() (+21 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (23): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+15 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - "PlayerProcessor"
Cohesion: 0.07
Nodes (14): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, Dictionary (+6 more)

### Community 36 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.06
Nodes (26): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+18 more)

### Community 39 - "RoleHandler"
Cohesion: 0.04
Nodes (28): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+20 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.14
Nodes (11): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "WorldSimulation"
Cohesion: 0.05
Nodes (49): ObjectiveDef, ObjectiveKind, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow(), complete_gameplay_scenario_round_trips(), default_ruler_vote_cooldown() (+41 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.12
Nodes (24): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+16 more)

### Community 45 - "RenderAssets"
Cohesion: 0.05
Nodes (121): Added, AmbientLight, BackgroundColor, ActiveMaterialHandles, actor_material(), apply_authored_main_menu_camera(), apply_authored_ui_fonts(), authored_main_ui_image_with_ppu() (+113 more)

### Community 46 - "STSM_Idle_Player"
Cohesion: 0.06
Nodes (16): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STSM_Action_Build, STSM_Action_GatherResource (+8 more)

### Community 47 - "PlayerRoleData"
Cohesion: 0.08
Nodes (15): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, AudioClip (+7 more)

### Community 48 - ".GetResourceAssets"
Cohesion: 0.12
Nodes (17): Dictionary, Material, materialIndex, materials, Mesh, meshes, meshIndex, bool (+9 more)

### Community 49 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.10
Nodes (14): bool, double, float, Func, int, IReadOnlyList, List, long (+6 more)

### Community 52 - "ContentCatalog"
Cohesion: 0.09
Nodes (67): ContentCatalog, ActorState, RoleProgress, Default, String, action_animation_speed(), action_cooldown(), actor_accepts_resource() (+59 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "WorldInstanceDeterminism"
Cohesion: 0.33
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 57 - "String"
Cohesion: 0.03
Nodes (124): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, active_event_text(), add_animation_composition(), add_animation_layer_branch(), add_rotation_curve() (+116 more)

### Community 58 - "Targetable"
Cohesion: 0.10
Nodes (12): List, Dictionary, List, TargetRuntimeData, bool, BoxCollider, float, int (+4 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (9): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow (+1 more)

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "TechnologyGraphViewState"
Cohesion: 0.07
Nodes (55): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap, Default (+47 more)

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
Cohesion: 0.08
Nodes (14): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+6 more)

### Community 67 - "twitch.rs"
Cohesion: 0.08
Nodes (43): BTreeSet, TwitchConfig, channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join() (+35 more)

### Community 68 - "Objective"
Cohesion: 0.13
Nodes (6): Action, int, Objective, ObjectiveType, ObjectiveData, EnemyType

### Community 69 - "List"
Cohesion: 0.31
Nodes (6): GameObject, List, NeutralAsset, NeutralScene, NeutralGameObject, NeutralScene

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (126): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+118 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (10): bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject, string (+2 more)

### Community 74 - "IProcessor"
Cohesion: 0.07
Nodes (16): Container, ContainerBuilder, DayAndNightProcessor, Container, Exception, IMainThreadInitializableProcessor, IPostInitializeProcessor, IProcessor (+8 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 78 - "tools_ui"
Cohesion: 0.16
Nodes (25): content_tab(), draw_world_preview(), inject_runtime_command(), inspector_tab(), migration_tab(), poll_runtime_console(), poll_tool_job_events(), preview_grid_point() (+17 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "String"
Cohesion: 0.23
Nodes (15): ImportReport, json_pet_name(), legacy_objective_matches(), legacy_pet_name(), LegacyGoal, LegacyObjective, objective_target_matches(), restore_legacy_goal() (+7 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "ShaderRef"
Cohesion: 0.05
Nodes (39): BoundsMaterialExtension, BoundsMaterialUniform, building_material(), BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension (+31 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "convert"
Cohesion: 0.20
Nodes (16): ActorKind, actor_prefix(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), decode_binary(), decode_legacy() (+8 more)

### Community 86 - "Station"
Cohesion: 0.09
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

### Community 87 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 88 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "TransformSaveData"
Cohesion: 0.05
Nodes (31): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+23 more)

### Community 91 - "GameEvent"
Cohesion: 0.05
Nodes (24): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+16 more)

### Community 92 - "TechTreeNode"
Cohesion: 0.10
Nodes (14): Color, float, string, TechnologyTreeGroup, Color, Foldout, List, Sprite (+6 more)

### Community 93 - "GridPos"
Cohesion: 0.08
Nodes (45): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError (+37 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 96 - "PlayerInventory"
Cohesion: 0.12
Nodes (8): DepositResources, PlayerInventory, Dictionary, ResourceInventory, bool, int, float, STSM_Action_DepositResource

### Community 97 - "SaveProcessor"
Cohesion: 0.06
Nodes (32): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Task (+24 more)

### Community 98 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (82): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef (+74 more)

### Community 100 - "string"
Cohesion: 0.22
Nodes (11): bool, int, long, string, NeutralComponent, NeutralExport, NeutralField, NeutralGameObject (+3 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "AnimationHandler"
Cohesion: 0.08
Nodes (14): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+6 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "BuildingModelHandler"
Cohesion: 0.17
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 108 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 109 - "CustomLogHandler"
Cohesion: 0.20
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

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.17
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "PlayerControls"
Cohesion: 0.09
Nodes (12): CameraProcessor, InputButton, bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData (+4 more)

### Community 115 - "StationProcessor"
Cohesion: 0.12
Nodes (11): Container, ContainerBuilder, List, StationProcessor, Dictionary, List, Queue, StationRuntimeData (+3 more)

### Community 116 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "Goal"
Cohesion: 0.12
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 120 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "UnitTextDisplay"
Cohesion: 0.12
Nodes (8): bool, Color, float, string, UnitTextDisplay, Camera, SimpleLookAtCamera, TextMeshPro

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 125 - ".AddResource"
Cohesion: 0.12
Nodes (5): int, ActiveResourceIncrementer, bool, float, PassiveResourceIncrementer

### Community 126 - "BuildingResourceModelHandler"
Cohesion: 0.26
Nodes (4): BuildingResourceModelHandler, GameObject, UnityEvent, StorageStatus

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "STSM_StateAction"
Cohesion: 0.19
Nodes (5): bool, float, int, STSM_StateAction, AnimationName

### Community 130 - "GameStateProcessor"
Cohesion: 0.21
Nodes (3): Container, ContainerBuilder, GameStateProcessor

### Community 131 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 133 - "twitch_tab"
Cohesion: 0.29
Nodes (11): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+3 more)

### Community 134 - "ToolState"
Cohesion: 0.15
Nodes (39): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot, commit_catalog_candidate(), create_technology_group(), create_technology_node() (+31 more)

### Community 135 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 136 - "runtime_console.rs"
Cohesion: 0.14
Nodes (22): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+14 more)

### Community 137 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 138 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 139 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 140 - "TargetProcessor"
Cohesion: 0.20
Nodes (6): TargetSettings, ContainerBuilder, TargetSettingsInstaller, Container, ContainerBuilder, TargetProcessor

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "legacy.rs"
Cohesion: 0.15
Nodes (42): ActorCustomization, binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json(), json_active_goal() (+34 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "parse_controller"
Cohesion: 0.14
Nodes (22): animation_state_id(), animation_state_machine_id(), avatar_mask_id(), clip_id(), convert_clips(), inline_file_id(), parse_animation_events(), parse_blend_tree() (+14 more)

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

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

### Community 153 - "UserInterface_TownGoal"
Cohesion: 0.15
Nodes (10): Slider, TextMeshProUGUI, UIRuntimeData, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 154 - "TownResourceProcessor"
Cohesion: 0.16
Nodes (4): Container, ContainerBuilder, Dictionary, TownResourceProcessor

### Community 155 - "Result"
Cohesion: 0.11
Nodes (51): append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id(), json_f32(), parse_avatar_mask(), parse_conditions() (+43 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.14
Nodes (9): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, float, ParticleSystem (+1 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "setup_rendering"
Cohesion: 0.09
Nodes (41): Assets, animate_healing_effects(), character_material(), character_material_from_standard(), character_material_preserves_authored_albedo_and_cosmetic_contract(), chimney_alpha_progress(), cosmetic_color(), cosmetic_node_visible() (+33 more)

### Community 163 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 164 - "ResourceStorageModifier"
Cohesion: 0.22
Nodes (3): ResourceStorageModifier, float, int

### Community 165 - ".SetTargetType"
Cohesion: 0.11
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.19
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 170 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 171 - "stream_town_tools/src/main.rs"
Cohesion: 0.11
Nodes (39): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), format_game_master_ids(), game_config_save_is_atomic_validated_and_round_trips() (+31 more)

### Community 172 - "Utils"
Cohesion: 0.06
Nodes (8): BuildingScriptablesEditor, RoleScriptablesEditor, TownGoal.Enumerations, Utils, ScriptablesEditor, SavingAndLoading.Structs, World.Generation, TownGoal.Data.Save

### Community 173 - "UIElementWrapper"
Cohesion: 0.07
Nodes (15): GameObject, List, PresetButtons, Access_AOToggle, Access_ChannelNameInput, ContainerBuilder, Access_EdgeScrollingToggle, Access_MouseControlsToggle (+7 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "Editor"
Cohesion: 0.13
Nodes (6): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 177 - "STSM_Helper_Attack"
Cohesion: 0.18
Nodes (4): int, STSM_Helper_Attack, int, STSM_Action_Attack

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (22): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+14 more)

### Community 179 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "UserInterface"
Cohesion: 0.06
Nodes (13): InputButton, SharedTypes, int, ChangeTimeStamp, DataStructures, TownGoal.Data, StreamTown.EditorTools, UserInterface (+5 more)

### Community 185 - "GridProcessor"
Cohesion: 0.24
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "MonoBehaviour"
Cohesion: 0.02
Nodes (89): CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped, ChannelDataInstaller (+81 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.29
Nodes (4): HashSet, MenuItem, BevyMigrationExporter, NeutralAsset

### Community 190 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+36 more)

### Community 192 - "Character"
Cohesion: 0.07
Nodes (22): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+14 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "TownResourceRuntimeData"
Cohesion: 0.18
Nodes (8): float, int, Queue, ResourceRateOfChange, Dictionary, float, UnityEvent, TownResourceRuntimeData

### Community 195 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 200 - "TerrainGenSettings"
Cohesion: 0.20
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 203 - "IRuntimeDataScriptable"
Cohesion: 0.08
Nodes (12): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, bool, GameStateRuntimeData, IRuntimeDataScriptable (+4 more)

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.07
Nodes (21): int, string, ObjectiveSaveData, bool, float, List, string, TechTreeSaveData (+13 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "PoolableObject"
Cohesion: 0.10
Nodes (17): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, Dictionary, float (+9 more)

### Community 209 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 211 - "CommandDictionary"
Cohesion: 0.17
Nodes (7): Action, Dictionary, IReadOnlyList, List, CommandDictionary, Dictionary, MiscCommands

### Community 213 - "CommonEnums.cs"
Cohesion: 0.20
Nodes (9): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, Seasons, TimeOfDay, WallType (+1 more)

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.06
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

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

### Community 229 - "Character Animation Regression Checklist"
Cohesion: 0.04
Nodes (40): Audio provenance, Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows, Acceptance gate, Attempt record template (+32 more)

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

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "Q: Why are we vendoring Bevy Tidal and not just using the library that exists??"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why are we vendoring Bevy Tidal and not just using the library that exists??, Source Nodes

### Community 238 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

### Community 240 - "IntWrapper"
Cohesion: 0.40
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 243 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

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

### Community 251 - "ResMut"
Cohesion: 0.03
Nodes (144): AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, PlayerSettings, Default, adjust_settings_menu(), AgentCommandQueue, apply_building_commands() (+136 more)

### Community 253 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

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

### Community 261 - "Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish., Source Nodes

### Community 262 - "PlayerSaveData"
Cohesion: 0.12
Nodes (15): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, int, PlayerCustomizationSaveData (+7 more)

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - ".StartupSequence"
Cohesion: 0.16
Nodes (3): Container, IEnumerable, Type

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

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

### Community 273 - ".new"
Cohesion: 0.03
Nodes (145): generate_world(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), append_terrain_skirt(), archetype_id_by_source(), authored_assignment_penalty_spreads_farmers_across_farms(), authored_enemies_drive_damage_range_cadence_and_weighted_spawning(), authored_level_curves_drive_effective_role_stats() (+137 more)

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

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

### Community 283 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 284 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 285 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 286 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 288 - ".RefreshSceneBindingsAndTryGenerate"
Cohesion: 0.36
Nodes (3): IEnumerator, LoadSceneMode, Scene

### Community 289 - "DebugProcessor"
Cohesion: 0.09
Nodes (12): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+4 more)

### Community 290 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 292 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "EquipmentHandlerEditor"
Cohesion: 0.29
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 296 - "settings.rs"
Cohesion: 0.06
Nodes (63): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+55 more)

### Community 297 - "Processors"
Cohesion: 0.08
Nodes (9): BuildCostModifier, PlayerControls.ObjectSelection, Units, Processors, World, Level, Buildings, Enemies (+1 more)

### Community 298 - "ResourceDataSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, ResourceDataSettingsInstaller, bool, int, Vector3, ResourceDataSettings

### Community 299 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 301 - "VfxAnimationController"
Cohesion: 0.06
Nodes (14): Transform, PlayerSpawnPoint, List, SimpleEventOnStart, float, Vector3, SimpleRotateOnAxis, int (+6 more)

### Community 302 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 303 - "Enemy"
Cohesion: 0.38
Nodes (3): Action, float, Enemy

### Community 304 - "NewKingVote"
Cohesion: 0.43
Nodes (3): int, List, NewKingVote

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 307 - "GameEventSettings"
Cohesion: 0.25
Nodes (6): bool, ParticleSystem, Transform, GameEventSettings, ContainerBuilder, GameEventSettingsInstaller

### Community 308 - "GameEventRuntimeData"
Cohesion: 0.33
Nodes (6): bool, float, ParticleSystem, SortedSet, Transform, GameEventRuntimeData

### Community 309 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 310 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 312 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 315 - "player_window_mode"
Cohesion: 0.67
Nodes (4): DisplayMode, player_window_mode(), startup_window_mode(), WindowMode

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 317 - "TradeProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, TradeProcessor

### Community 318 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

## Knowledge Gaps
- **347 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+342 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `ConvertedAnimationDriver` (5× useful, score=4.53424798) _(code changed — re-verify)_
- `RenderAssets` (4× useful, score=3.237941093) _(code changed — re-verify)_
- `TreeMaterialExtension` (3× useful, score=2.962292656) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.477432826) _(code changed — re-verify)_
- `WorldSnapshot` (3× useful, score=2.366566747)
- `SkinnedMesh` (2× useful, score=1.997632118)
- `drive_tidal_music()` (2× useful, score=1.966606185)
- `WorldSimulation` (2× useful, score=1.71789778)
- `load_input()` (2× useful, score=1.577481983) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.548009406)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `Sensors`, `UpdateGraphBounds`, `RandomEnabler`, `StringUtils`, `Target`, `BuildingPlacer`, `GenerationSettings`, `.CreateEnumField`, `SeasonProcessor`, `Processors`, `VfxAnimationController`, `PlayerRoleData`, `TechTree.Elements`, `UserInterface`, `SimpleDisableAfterTime`, `MonoBehaviour`, `Character`, `IRuntimeDataScriptable`, `CommonEnums.cs`, `Easings`, `FPSDisplay`, `ScriptablesProcessorInfrastructure`, `EnemySpawner`, `PlayerControls`, `UnitTextDisplay`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.056) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `Target`, `BuildingPlacer`, `.Log`, `TownResourceProcessor`, `Player`, `WorldGenProcessor`, `UserInterface_Debug`, `GameEventProcessor`, `RoleHandler`, `TechTreeProcessor`, `PlayerRoleData`, `Enemy`, `NewKingVote`, `EventCommands`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `IProcessor`, `UserInterface_TownVote`, `GameEvent`, `SaveProcessor`, `EnemySpawner`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `GeneratedWorld`, `stream_town_game/src/lib.rs`, `ToolState`, `Option`, `runtime_console.rs`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/content.rs`, `save.rs`, `.new`, `parse_controller`, `GameConfig`, `Query`, `Result`, `Option`, `setup_rendering`, `settings.rs`, `WorldSimulation`, `AnimationControllerRuntime`, `RenderAssets`, `stream_town_tools/src/main.rs`, `ContentCatalog`, `String`, `TechnologyGraphViewState`, `stream_town_migrate/src/menu_scene.rs`, `twitch.rs`, `stream_town_migrate/src/content.rs`, `tools_ui`, `String`, `convert`, `GridPos`, `stream_town_domain/src/presentation.rs`, `ResMut`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _347 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Sensors` be split into smaller, more focused modules?**
  _Cohesion score 0.09041835357624832 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.060109289617486336 - nodes in this community are weakly interconnected._
- **Should `GeneratedWorld` be split into smaller, more focused modules?**
  _Cohesion score 0.09818819403857393 - nodes in this community are weakly interconnected._