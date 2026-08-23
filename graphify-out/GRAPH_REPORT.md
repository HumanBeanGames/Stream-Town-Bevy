# Graph Report - Stream-Town-Bevy  (2026-08-23)

## Corpus Check
- 654 files · ~1,692,486 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8215 nodes · 23320 edges · 311 communities (286 shown, 25 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1023 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bac90ff5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Result
- BuildingProcessor
- world.rs
- BinarySaveCodec
- ScriptableObject
- .new
- TwitchChatProcessor
- Transform
- BottomBarInterface
- Commands
- SettingsProcessor
- CellSpacePartitioning
- Utils
- Buildings
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- PlayerSaveData
- stream_town_migrate/src/content.rs
- BinaryReader
- .Log
- BuildingPlacer
- simulation.rs
- UnitHealthBar
- ObjectPoolingProcessor
- Res
- TechTreeGraphView
- SaveFileData
- Player
- TechTreeProcessor.cs
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- setup_rendering
- PoolableObject
- SettingsData
- SeasonProcessor
- .default
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- StableId
- AnimationControllerDef
- String
- IInstaller
- Value
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- Option
- TwitchClientProcessor
- UIProcessor
- .PropertyValue
- config.rs
- command.rs
- Option
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
- BuildingResourceModelHandler
- UnityAsset
- models.rs
- Tiler
- ScriptablesEditor
- STSM_Idle_Player
- UserInterface_ObjectSelection
- PlayerRoleData
- SelectedObject
- stream_town_migrate/src/presentation.rs
- TwitchBotSetupWindow
- Goal
- WorldUtils
- Node_SO
- DebugSettings
- Access_Text
- RoleHandler
- Handle
- TechTreeNode
- TargetSensor
- FoliageProcessor
- Enemy
- GameEvent
- PlayerInventory
- STSM_GoToLocation
- convert_fbx_to_glb.py
- .StartupSequence
- drive_tidal_music
- SaveProcessor
- Coordinator
- stream_town_domain/src/presentation.rs
- World.Generation.Settings
- StateMachine
- stream_town_game/src/lib.rs
- TownGoalProcessor
- MainMenuManager
- IProcessor.cs
- LoadingManager
- UIElementWrapper
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- stream_town_domain/src/lib.rs
- EnemyModelHandler
- Station
- GridNode
- stream_town_migrate/src/main.rs
- SelectedPlayer
- Resource
- VoteEvent
- unity_color_filter
- UserInterface_Resources
- SnapToGridMouseMovement
- AIPath
- ResourceRuntimeData
- PlayerInputProcessor
- String
- UpdateGraphBounds
- WeatherProcessor
- AnimationHandler
- RandomEnabler
- SeasonDataSettings
- twitch_tab
- stream_town_tools/src/main.rs
- ResourceProcessor
- runtime_console.rs
- GateController
- SelectableObject
- WorldGenRuntimeData
- ErrorData
- What You Must Do When Invoked
- RuntimeData Template
- legacy.rs
- RuntimeData Template
- Key Rules
- Access_Toggle
- Pet
- add_file
- Targetable
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- tools_ui
- .InitializeAndActivateProcessorsAsync
- String
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WindController
- UserInterface_BuildingHealthBar
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- STSM_Idle
- .CreateEnumField
- UserInterface.MainMenu
- .SetTargetType
- SimpleMusicController
- .GenerateFromSettings
- InventoryEntrySaveData
- xtask/src/main.rs
- TL_API
- TransformSaveData
- STSM_StateAction
- RotationHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- .GetMissingProcessorDependencies
- WorldGenSaveData
- TechTree.Elements
- UserInterface_RulerVote
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- Access_Dropdown
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- .SerializeComponent
- ScriptablesProcessorInfrastructure
- WorldGenerationReferenceExporter
- DontDestroyOnLoad
- ProjectCamera
- UserInterface_GameMenu
- Easings
- stream_town_migrate/src/menu_scene.rs
- TwitchUser
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- .StartMusic
- NewKingVote
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- CreditsProcessor
- STSM_HelperBase
- graphify reference: extra exports and benchmark
- Key Rules
- UnityGraphics
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- ResourceStorageModifier
- BuildingSettings
- Requirement
- CommandDictionary
- SelectedEnemy
- SelectedResource
- Key Rules
- TimeProcessor
- RuntimeData Template
- UI_TechOption
- Season
- ScriptKeywordProcessor
- FPSDisplay
- BevyMigrationExporter
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
- Editor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- EditorUtils
- ObjectiveSaveData
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- StringUtils
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- PlayerDeathHandler
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- VfxSeagullSpawner
- extraction-spec.md
- STStateMachine.States
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- SelectedBuilding
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: shipping role station behavior Town Hall saved position native legacy load deposit unstuck compatibility
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- Q: 1) The pre-menu loading screen doesn't have the imagery we expect. 2) The ingame trees are flickering again. 3) The ingame ground texture seems really shiny and specular, and lacks vibrance. 4) The middle mouse-button drag movement for the camera has inverted up-down controls. Additionally, it jerks a lot. 5) The characters are not animated. 6) We can select ground cells, but not characters. 7) Farms aren't properly flattening the land they are on in the main menu. 8) Trees (And probably other things) are off-center from the cells. 9) Ingame, there are shadows flying across the ground; I think it's fish.
- StatusBar
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- .DrawDataFieldAndLabel
- Q: If there is more to do, keep going.
- .RenderResourceType
- EventProcessor
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- ResourceHolder
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- FoliageGenerationSettings
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- UnitTravelToPosition
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- ObjectSelectionProcessor.Editor.cs
- SimpleScreenShot
- Autosave
- VfxParticlePosition
- World.Generation
- MonoBehaviour
- preview_lerp_color
- GridProcessor
- BuildingModelHandler
- AllSeasonSettings
- PassiveResourceIncrementer
- CreateDefaultSettingsAssets.cs
- BuildPlacerData
- settings.rs
- PostProcessingInstaller
- AudioMixerInstaller
- AutosaveIntervalsInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- PlacementProbeHandler
- ScriptableObjectAssetData
- Q: Characters are still not animated. Trees still have the flickering shadows.
- CellSpacePartitioningInstaller
- .HandleSceneLoaded
- SelectedEnemyCamp
- IntWrapper
- WorldSimulation

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
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `begin_world_loading()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (311 total, 25 thin omitted)

### Community 0 - "Result"
Cohesion: 0.23
Nodes (32): aged_buildings(), building_cost_reductions(), building_level_caps(), convert_export(), field_value(), foliage_layers(), generated_record_ids(), objective_definitions() (+24 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, Dictionary, int, List (+7 more)

### Community 2 - "world.rs"
Cohesion: 0.06
Nodes (69): WorldGenConfig, actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError (+61 more)

### Community 3 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 4 - "ScriptableObject"
Cohesion: 0.03
Nodes (66): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, float, Material, Volume, DayAndNightSettings (+58 more)

### Community 5 - ".new"
Cohesion: 0.04
Nodes (148): ArchetypeScene, generate_world(), generate_world_with_content(), agent_facing_matches_unity_rotation_and_action_targets(), animated_pets_resolve_their_own_unity_controllers_and_rigs(), archetype_by_source(), archetype_id_by_source(), archetype_scene_for_age() (+140 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (28): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+20 more)

### Community 7 - "Transform"
Cohesion: 0.08
Nodes (38): AnimationClip, AnimationTargetId, add_rotation_curve(), add_scale_curve(), add_translation_curve(), animate_fish_school(), apply_authored_local_rotation(), camera_ground_focus() (+30 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Commands"
Cohesion: 0.06
Nodes (149): GameConfig, MainMenuSceneReference, Option, PresentationCatalog, GeneratedWorld, actor_material(), advance_falling_fish(), AgentAnimation (+141 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 12 - "Utils"
Cohesion: 0.05
Nodes (18): Utils, Processors, Pets.Enumerations, StreamTown.EditorTools, Animation, Core, World, UserInterface (+10 more)

### Community 13 - "Buildings"
Cohesion: 0.07
Nodes (20): BuildCostModifier, InputButton, PlayerControls.ObjectSelection, Units, TownGoal, Target, Utils.Pooling, Sensors (+12 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "HealthHandler"
Cohesion: 0.10
Nodes (13): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, int, STSM_Helper_Build, Action, bool (+5 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (43): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+35 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (37): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+29 more)

### Community 18 - "PlayerSaveData"
Cohesion: 0.06
Nodes (25): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, List (+17 more)

### Community 19 - "stream_town_migrate/src/content.rs"
Cohesion: 0.11
Nodes (31): asset(), authored_value(), building_placements(), BuildingPlacement, converted_rotating_axis(), converts_active_catalog_references_and_round_trips_ron(), converts_building_role_slot_modifiers(), converts_disable_after_time_lifetime() (+23 more)

### Community 20 - "BinaryReader"
Cohesion: 0.14
Nodes (6): CancellationToken, Func, List, float, Vector2SaveData, BinaryReader

### Community 21 - ".Log"
Cohesion: 0.04
Nodes (30): HideInCallstack, Object, DebugLogCategory, Container, ContainerBuilder, GameStateProcessor, Action, BoxCollider (+22 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 23 - "simulation.rs"
Cohesion: 0.08
Nodes (29): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+21 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "ObjectPoolingProcessor"
Cohesion: 0.05
Nodes (27): bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller, Container, IMainThreadInitializableProcessor, IProcessor (+19 more)

### Community 26 - "Res"
Cohesion: 0.03
Nodes (236): Aabb, AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AppExit, AudioSink, BackgroundColor, ActorNameOverlay (+228 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.06
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "Player"
Cohesion: 0.06
Nodes (12): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, TwitchClientProcessor (+4 more)

### Community 30 - "TechTreeProcessor.cs"
Cohesion: 0.07
Nodes (15): int, TechTreeSettings, InputButton, SharedTypes, int, ChangeTimeStamp, NodeGroup_SO, List (+7 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (16): Action, Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable (+8 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (8): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3, UserInterface_Debug

### Community 33 - "GameEventProcessor"
Cohesion: 0.08
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.08
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "setup_rendering"
Cohesion: 0.06
Nodes (64): AmbientLight, Assets, apply_material_overrides(), building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced (+56 more)

### Community 36 - "PoolableObject"
Cohesion: 0.09
Nodes (17): Container, ContainerBuilder, GUIDProcessor, bool, Dictionary, GUIDRuntimeData, Dictionary, float (+9 more)

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.14
Nodes (4): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor

### Community 39 - ".default"
Cohesion: 0.18
Nodes (17): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_game_config(), main() (+9 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (10): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable (+2 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "StableId"
Cohesion: 0.18
Nodes (7): FromStr, StableId, complete_gameplay_scenario_round_trips(), Result, SimulationError, validate_trade_resource(), Display

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (26): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+18 more)

### Community 45 - "String"
Cohesion: 0.03
Nodes (126): AnimatedBy, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, AnimationTransitions, active_event_text(), ActivePetVisual (+118 more)

### Community 46 - "IInstaller"
Cohesion: 0.06
Nodes (23): ContainerBuilder, AllRoleDataSettingsInstaller, ContainerBuilder, GameSettingsInstaller, List, GameSettings, Dictionary, int (+15 more)

### Community 47 - "Value"
Cohesion: 0.20
Nodes (27): ActorCustomization, StreamUserType, decode_json(), json_active_goal(), json_buildings(), json_customization(), json_enemies(), json_enemy_camps() (+19 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.18
Nodes (12): materialIndex, meshIndex, bool, float, int, List, string, uint (+4 more)

### Community 49 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 50 - "AudioHandler"
Cohesion: 0.08
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.12
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "Option"
Cohesion: 0.06
Nodes (121): ContentCatalog, GridPos, ActorState, String, action_animation_speed(), action_cooldown(), actor_accepts_resource(), actor_archetype() (+113 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.06
Nodes (21): Container, ContainerBuilder, UIProcessor, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, List (+13 more)

### Community 55 - ".PropertyValue"
Cohesion: 0.14
Nodes (8): Bounds, Color, Quaternion, Rect, SerializedProperty, Vector2, Vector3, Vector4

### Community 56 - "config.rs"
Cohesion: 0.12
Nodes (25): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet, Default (+17 more)

### Community 57 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 58 - "Option"
Cohesion: 0.10
Nodes (41): animator_component(), animator_reference_path(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert_prefab_bindings(), convert_prefab_materials() (+33 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.11
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "BuildingBase"
Cohesion: 0.09
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "PlayerProcessor"
Cohesion: 0.07
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

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
Cohesion: 0.07
Nodes (13): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+5 more)

### Community 69 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 70 - "UnityAsset"
Cohesion: 0.16
Nodes (39): ArchetypesById, archetype_bounds(), archetype_kind(), building_model_definitions(), building_node_age(), component(), component_at(), component_field_value() (+31 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.12
Nodes (11): bool, Color, Dictionary, GUIStyle, int, MenuItem, ScriptableObject, string (+3 more)

### Community 74 - "STSM_Idle_Player"
Cohesion: 0.07
Nodes (10): STSM_Action_Build, STSM_Action_GatherResource, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase, bool, float, uint (+2 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "PlayerRoleData"
Cohesion: 0.09
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 77 - "SelectedObject"
Cohesion: 0.17
Nodes (3): object, UnityAction, SelectedObject

### Community 78 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.08
Nodes (85): animation_state_id(), animation_state_machine_id(), animation_take_name(), avatar_mask_id(), clip_id(), controller_id(), convert(), convert_avatar_masks() (+77 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "Goal"
Cohesion: 0.09
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "DebugSettings"
Cohesion: 0.24
Nodes (5): Dictionary, DebugSettings, ContainerBuilder, DebugSettingsInstaller, SerializedScriptableObject

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "RoleHandler"
Cohesion: 0.03
Nodes (34): RoleSlotModifier, int, RoleData, AudioClip, bool, float, int, Sprite (+26 more)

### Community 86 - "Handle"
Cohesion: 0.05
Nodes (59): bottom_bar_texture(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform, CharacterMaterialExtension, CharacterMaterialUniform, CloudMaterialExtension (+51 more)

### Community 87 - "TechTreeNode"
Cohesion: 0.13
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

### Community 88 - "TargetSensor"
Cohesion: 0.07
Nodes (15): ProjectileShooter, float, int, string, float, List, SensorRuntimeData, SensorBase (+7 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "Enemy"
Cohesion: 0.09
Nodes (16): Action, float, Enemy, int, ActiveResourceIncrementer, uint, GUIDComponent, SaveableBuilding (+8 more)

### Community 91 - "GameEvent"
Cohesion: 0.05
Nodes (23): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+15 more)

### Community 92 - "PlayerInventory"
Cohesion: 0.16
Nodes (5): PlayerInventory, Dictionary, ResourceInventory, bool, int

### Community 93 - "STSM_GoToLocation"
Cohesion: 0.10
Nodes (11): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+3 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.15
Nodes (26): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+18 more)

### Community 96 - "drive_tidal_music"
Cohesion: 0.19
Nodes (24): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), effective_music_gain(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature (+16 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (19): Action, CancellationToken, Container, ContainerBuilder, Dictionary, float, List, Material (+11 more)

### Community 98 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (80): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+72 more)

### Community 100 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 101 - "StateMachine"
Cohesion: 0.13
Nodes (8): bool, List, string, uint, StateMachine, STStateHolder, bool, STStateBase

### Community 102 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (238): AnyResult, PlayerSettings, Default, action_ranges_and_tower_acquisition_are_euclidean(), ActionPresentation, actor_combat_visual(), actor_detail_budget(), actor_scene_budget() (+230 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.13
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.12
Nodes (11): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+3 more)

### Community 105 - "IProcessor.cs"
Cohesion: 0.16
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

### Community 106 - "LoadingManager"
Cohesion: 0.12
Nodes (10): Dictionary, float, GameObject, Image, string, TextMeshProUGUI, Transform, LoadingManager (+2 more)

### Community 107 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 108 - "GlobalAudioController"
Cohesion: 0.24
Nodes (5): GlobalAudioController, AudioSource, bool, float, IEnumerator

### Community 109 - "CustomLogHandler"
Cohesion: 0.21
Nodes (7): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.18
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, DayAndNightProcessor, Transform, float, int, List, Transform (+9 more)

### Community 113 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 114 - "EnemyModelHandler"
Cohesion: 0.10
Nodes (10): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+2 more)

### Community 115 - "Station"
Cohesion: 0.06
Nodes (23): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+15 more)

### Community 116 - "GridNode"
Cohesion: 0.11
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 119 - "Resource"
Cohesion: 0.08
Nodes (14): DepositResources, IResourceHolder, Container, ContainerBuilder, Dictionary, TownResourceProcessor, bool, float (+6 more)

### Community 120 - "VoteEvent"
Cohesion: 0.15
Nodes (10): PlayerVote, Dictionary, TechVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int (+2 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, Plugin, Res (+20 more)

### Community 122 - "UserInterface_Resources"
Cohesion: 0.21
Nodes (7): Slider, TextMeshProUGUI, Color, GameObject, Slider, TextMeshProUGUI, UserInterface_Resources

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.12
Nodes (19): bool, float, int, string, Type, Vector3, AIPath, AstarData (+11 more)

### Community 125 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 126 - "PlayerInputProcessor"
Cohesion: 0.11
Nodes (11): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor, bool, Dictionary, InputButton (+3 more)

### Community 127 - "String"
Cohesion: 0.12
Nodes (24): animation_parameter_name(), archetype_scenes(), authored_mask(), child_technology_guids(), collect_model_dependencies(), ContentConversionReport, convert(), decomposes_combined_unity_flag_values() (+16 more)

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "WeatherProcessor"
Cohesion: 0.29
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 130 - "AnimationHandler"
Cohesion: 0.11
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 131 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 132 - "SeasonDataSettings"
Cohesion: 0.17
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 133 - "twitch_tab"
Cohesion: 0.29
Nodes (11): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+3 more)

### Community 134 - "stream_town_tools/src/main.rs"
Cohesion: 0.10
Nodes (59): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), bounded_ui_index(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role() (+51 more)

### Community 135 - "ResourceProcessor"
Cohesion: 0.08
Nodes (24): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+16 more)

### Community 136 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 137 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 138 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 139 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 140 - "ErrorData"
Cohesion: 0.22
Nodes (7): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, TechTree.Data.Error

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

### Community 146 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "Targetable"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData, bool (+7 more)

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "tools_ui"
Cohesion: 0.19
Nodes (21): content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), preview_grid_point(), redo_catalog_edit(), role_i32(), role_u16() (+13 more)

### Community 154 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.21
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 155 - "String"
Cohesion: 0.10
Nodes (54): append_vec3_keys(), inline_file_id(), json_f32(), parse_blend_tree(), parse_child_references(), parse_controller(), parse_layers(), parse_model_material_remaps() (+46 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+7 more)

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

### Community 162 - "STSM_Idle"
Cohesion: 0.18
Nodes (5): bool, float, Vector3, STSM_Idle_Enemy, STSM_Idle

### Community 163 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 164 - "UserInterface.MainMenu"
Cohesion: 0.20
Nodes (3): UserInterface.MainMenu, MetaData, Settings

### Community 165 - ".SetTargetType"
Cohesion: 0.16
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 167 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (14): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+6 more)

### Community 168 - "InventoryEntrySaveData"
Cohesion: 0.40
Nodes (4): bool, int, string, InventoryEntrySaveData

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 171 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 172 - "STSM_StateAction"
Cohesion: 0.12
Nodes (8): int, STSM_Helper_Attack, int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 173 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 177 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 178 - "TechTree.Elements"
Cohesion: 0.09
Nodes (16): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+8 more)

### Community 179 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - ".SerializeComponent"
Cohesion: 0.18
Nodes (11): Component, GameObject, List, Object, NeutralAsset, NeutralAsset, NeutralField, NeutralGameObject (+3 more)

### Community 185 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.02
Nodes (59): ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder, BuildingConfigSettingsInstaller, ContainerBuilder, CampGenSettingsInstaller, ContainerBuilder, GameEventConfigSettingsInstaller (+51 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 188 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, String, Vec (+36 more)

### Community 192 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - ".StartMusic"
Cohesion: 0.40
Nodes (3): SeasonAudioData, AudioClip, List

### Community 195 - "NewKingVote"
Cohesion: 0.24
Nodes (3): int, List, NewKingVote

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "CreditsProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, CreditsProcessor

### Community 200 - "STSM_HelperBase"
Cohesion: 0.28
Nodes (3): StateMachine, string, STSM_HelperBase

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
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "ResourceStorageModifier"
Cohesion: 0.27
Nodes (3): ResourceStorageModifier, float, int

### Community 209 - "BuildingSettings"
Cohesion: 0.08
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

### Community 217 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 218 - "Season"
Cohesion: 0.29
Nodes (5): bool, float, int, SeasonRuntimeData, Season

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 220 - "FPSDisplay"
Cohesion: 0.14
Nodes (9): bool, Color, float, GUIStyle, IEnumerator, int, Rect, string (+1 more)

### Community 221 - "BevyMigrationExporter"
Cohesion: 0.15
Nodes (16): bool, HashSet, int, long, MenuItem, string, BevyMigrationExporter, NeutralComponent (+8 more)

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

### Community 234 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 235 - "Q: How does native load keep the persistent Town Hall aligned with saved state?"
Cohesion: 0.50
Nodes (3): Answer, Q: How does native load keep the persistent Town Hall aligned with saved state?, Source Nodes

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 237 - "Q: Why are we vendoring Bevy Tidal and not just using the library that exists??"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why are we vendoring Bevy Tidal and not just using the library that exists??, Source Nodes

### Community 238 - "EditorUtils"
Cohesion: 0.24
Nodes (4): Color, Texture2D, EditorUtils, DirectoryInfo

### Community 240 - "ObjectiveSaveData"
Cohesion: 0.50
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

### Community 248 - "PlayerDeathHandler"
Cohesion: 0.22
Nodes (4): PlayerDeathHandler, bool, float, Vector3

### Community 249 - "Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption, Source Nodes

### Community 251 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 253 - "STStateMachine.States"
Cohesion: 0.09
Nodes (11): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+3 more)

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

### Community 262 - "StatusBar"
Cohesion: 0.40
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 263 - "Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?, Source Nodes

### Community 264 - ".DrawDataFieldAndLabel"
Cohesion: 0.28
Nodes (3): RoleScriptablesEditor, Utils, ScriptablesEditor

### Community 266 - "Q: If there is more to do, keep going."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: If there is more to do, keep going., Source Nodes

### Community 267 - ".RenderResourceType"
Cohesion: 0.17
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 268 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

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

### Community 273 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "FoliageGenerationSettings"
Cohesion: 0.11
Nodes (15): ContainerBuilder, FoliageGenSettingsInstaller, ContainerBuilder, WaterFoliageGenSettingsInstaller, List, FoliageGenSettings, List, WaterFoliageGenSettings (+7 more)

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

### Community 284 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 286 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 287 - "World.Generation"
Cohesion: 0.07
Nodes (18): List, CampGenSettings, List, ResourceGenSettings, List, WaterResourceGenSettings, float, int (+10 more)

### Community 288 - "MonoBehaviour"
Cohesion: 0.02
Nodes (44): CameraProcessor, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped, ChannelDataInstaller, ContainerBuilder (+36 more)

### Community 289 - "preview_lerp_color"
Cohesion: 1.00
Nodes (3): preview_lerp_color(), Color32, terrain_preview_color()

### Community 290 - "GridProcessor"
Cohesion: 0.28
Nodes (3): Container, ContainerBuilder, GridProcessor

### Community 291 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 292 - "AllSeasonSettings"
Cohesion: 0.25
Nodes (6): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings

### Community 293 - "PassiveResourceIncrementer"
Cohesion: 0.29
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 294 - "CreateDefaultSettingsAssets.cs"
Cohesion: 0.33
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "BuildPlacerData"
Cohesion: 0.33
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 296 - "settings.rs"
Cohesion: 0.10
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 297 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 298 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 299 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 300 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 301 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 302 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 304 - "ScriptableObjectAssetData"
Cohesion: 0.40
Nodes (3): List, string, ScriptableObjectAssetData

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 314 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 315 - "WorldSimulation"
Cohesion: 0.07
Nodes (32): ObjectiveDef, BuildingState, EnemyCampState, FishGodState, objective_increment(), ObjectiveEvent, ObjectiveProgress, RaidState (+24 more)

## Knowledge Gaps
- **334 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+329 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `RenderAssets` (4× useful, score=3.249427768) _(code changed — re-verify)_
- `ConvertedAnimationDriver` (3× useful, score=2.546293787) _(code changed — re-verify)_
- `PresentationCatalog` (3× useful, score=2.48622158)
- `WorldSnapshot` (3× useful, score=2.374962201)
- `drive_tidal_music()` (2× useful, score=1.973582769)
- `TreeMaterialExtension` (2× useful, score=1.971762845) _(code changed — re-verify)_
- `WorldSimulation` (2× useful, score=1.723992064)
- `load_input()` (2× useful, score=1.583078139) _(code changed — re-verify)_
- `Animation` (2× useful, score=1.553501008)
- `MaterialDef` (2× useful, score=1.553063812)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Utils` connect `Utils` to `UpdateGraphBounds`, `RandomEnabler`, `ScriptableObject`, `SeasonDataSettings`, `.DrawDataFieldAndLabel`, `Buildings`, `BuildingPlacer`, `SimpleScreenShot`, `TechTreeProcessor.cs`, `World.Generation`, `MonoBehaviour`, `GenerationSettings`, `.CreateEnumField`, `AllSeasonSettings`, `BuildPlacerData`, `.GenerateFromSettings`, `TechTreeProcessor`, `IInstaller`, `TechTree.Elements`, `ScriptablesProcessorInfrastructure`, `Easings`, `.StartMusic`, `Objective`, `WorldSaveData`, `FPSDisplay`, `GamestateJukebox`, `DayAndNightProcessor`, `Station`, `StringUtils`, `SnapToGridMouseMovement`, `STStateMachine.States`?**
  _High betweenness centrality (0.058) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `Result`, `world.rs`, `.new`, `stream_town_tools/src/main.rs`, `Transform`, `runtime_console.rs`, `Commands`, `legacy.rs`, `stream_town_domain/src/content.rs`, `save.rs`, `stream_town_migrate/src/content.rs`, `simulation.rs`, `tools_ui`, `Res`, `String`, `setup_rendering`, `AnimationControllerDef`, `String`, `Option`, `config.rs`, `command.rs`, `Option`, `WorldSimulation`, `stream_town_migrate/src/menu_scene.rs`, `twitch.rs`, `UnityAsset`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/presentation.rs`, `stream_town_game/src/lib.rs`, `stream_town_domain/src/lib.rs`, `String`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `Buildings`, `BuildingPlacer`, `ObjectPoolingProcessor`, `Player`, `WorldGenProcessor`, `MonoBehaviour`, `GameEventProcessor`, `UserInterface_Debug`, `TechTreeProcessor`, `IInstaller`, `TwitchClientProcessor`, `UIProcessor`, `NewKingVote`, `PlayerRoleData`, `RoleHandler`, `TimeProcessor`, `Enemy`, `GameEvent`, `SaveProcessor`, `TownGoalProcessor`, `DayAndNightProcessor`, `SelectedPlayer`, `Resource`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _334 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.07653061224489796 - nodes in this community are weakly interconnected._
- **Should `world.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05717852684144819 - nodes in this community are weakly interconnected._
- **Should `ScriptableObject` be split into smaller, more focused modules?**
  _Cohesion score 0.027115474520804116 - nodes in this community are weakly interconnected._