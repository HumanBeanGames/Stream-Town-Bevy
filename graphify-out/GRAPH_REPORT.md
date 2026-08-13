# Graph Report - Stream-Town-Bevy  (2026-08-13)

## Corpus Check
- 608 files · ~1,599,202 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 6921 nodes · 18074 edges · 263 communities (241 shown, 22 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 994 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `eaabdc66`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- world.rs
- BuildingProcessor
- Transform
- stream_town_migrate/src/presentation.rs
- Target
- EnemyModelHandler
- TwitchChatProcessor
- TechTreeGraphView
- BottomBarInterface
- settings.rs
- SettingsProcessor
- WorldGenProcessor
- Player
- simulation.rs
- TechTreeIOUtility
- World.Generation.Settings
- Character
- Commands
- Processors
- process_injected_commands
- .CreateEnumField
- TechTree.Elements
- BuildingPlacer
- PlayerProcessor
- HealthHandler
- PlayerRoleData
- Goal
- Access_Dropdown
- SaveFileData
- GameEventProcessor
- ContentCatalog
- BuildingResourceModelHandler
- PlayerInventory
- ResourceData
- Station
- UserInterface_Debug
- MiscCommands
- SettingsData
- AnimationHandler
- .SerializeComponent
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- BinarySaveCodec
- AnimationControllerDef
- legacy.rs
- SeasonProcessor
- .Log
- NavGrid
- .Draw
- AudioHandler
- StreamTownSessionBridge
- ResourceProcessor
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- RoleHandler
- StableId
- STSM_GoToLocation
- TechTreeEditorWindow
- VfxSeagullSpawner
- StateMachine
- CameraController
- Node_SO
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- Result
- Objective
- TransformSaveData
- GenerationSettings
- models.rs
- Tiler
- ScriptablesEditor
- BTreeMap
- UserInterface_ObjectSelection
- STSM_Action_PlayerBase
- ProjectCamera
- GridProcessor
- TwitchBotSetupWindow
- GlobalAudioController
- WorldUtils
- SaveDataMapper
- TreeMaterialExtension
- Access_Text
- stream_town_game/src/lib.rs
- CellSpacePartitioning
- UserInterface_TownVote
- CommonEnums.cs
- FoliageProcessor
- Option
- PlayerRole
- GameEvent
- GameStateProcessor
- convert_fbx_to_glb.py
- STSM_StateAction
- stream_town_migrate/src/content.rs
- BinaryReader
- Coordinator
- stream_town_domain/src/presentation.rs
- Editor
- TargetProcessor
- twitch.rs
- TownGoalProcessor
- MainMenuManager
- UnitHealthBar
- LoadingManager
- ResourceRuntimeData
- ResourceHolder
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- SelectedObject
- UserInterface_RulerVote
- .OnGUI
- RotationHandler
- IRuntimeDataScriptable
- VoteEvent
- Resource
- .new
- Res
- .SetTargetType
- SnapToGridMouseMovement
- AIPath
- Targetable
- BuildingBase
- CommandDictionary
- UpdateGraphBounds
- SelectableObject
- stream_town_migrate/src/main.rs
- .GenerateFromSettings
- SelectedResource
- .RenderResourceType
- ConfirmCheck
- config.rs
- ToolState
- LabelDisplayProcessor
- SensorProcessor
- DontDestroyOnLoad
- PlayerInputProcessor
- What You Must Do When Invoked
- RuntimeData Template
- ObjectPoolingProcessor
- RuntimeData Template
- Key Rules
- DayAndNightProcessor
- Pet
- add_file
- UI_TechOption
- Requirement
- ObjectPoolingSettings
- Stream Town Reloaded - Architecture Documentation
- Access_Toggle
- WeatherProcessor
- DataStructures
- ResourceDataSaveData
- Stream Town Reloaded - Architecture Documentation
- WindController
- TechTreeNode
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- UserInterface_BuildingHealthBar
- SimpleMusicController
- GateController
- FoliageGenerationSettings
- AnimationName
- CampGenerationSettings
- Easings
- TerrainGenSettings
- DayAndNightSettings
- TimeProcessor
- SelectedPlayerGroup
- Access_GOList
- SelectedBuilding
- EditorHelpers
- EventProcessor
- IProcessor
- ObjectPoolingRuntimeData
- VfxAnimationController
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- PoolableObject
- WorldGenRuntimeData
- StringUtils
- TargetSettings
- SelectedEnemyCamp
- PassiveResourceIncrementer
- Access_TextInput
- ObjectiveDef
- BuildingDataSettings
- command.rs
- .StartGoalFromNode
- Sensors
- TL_API
- Node_SO.cs
- TechNodeData
- UILineRenderer
- UserInterface_DisplayUsernames
- CellPartitioningEditor
- BuildingModelHandler
- graphify reference: extra exports and benchmark
- Key Rules
- ObjectSelectionProcessor.Editor.cs
- .GetCompatiblePorts
- Key Rules
- SelectedEnemy
- Common Patterns
- Utils
- VfxParticlePosition
- .InjectRuntimeData
- FPSDisplay.cs
- NodeGroup_SO
- Key Rules
- SimpleScreenShot
- RuntimeData Template
- ScriptKeywordProcessor
- xtask/src/main.rs
- Processor Template
- Common Patterns
- WorldSaveData
- graphify reference: query, path, explain
- TODO List
- TechTree_SO
- Twitch setup
- graphify reference: add a URL and watch a folder
- PostProcessingInstaller
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- UnityGraphics
- SimpleDisableAfterTime
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- CreateProjectScopeProcessors.cs
- BuildPlacerData
- RandomEnabler
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- SaveProcessor
- AGENTS.md
- ResourceGenerationSettings
- CustomLogger
- extraction-spec.md
- AllRoleDataSettings
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- TechTreeNodeType.cs
- PoolablePlayer.cs
- Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?
- TargetSensor
- EquipmentHandlerEditor
- AudioMixerInstaller
- ForwardRendererInstaller
- RenderPipelineInstaller
- VideoSettingsPresetsInstaller
- ScriptablesProcessorInfrastructure
- IntWrapper
- MonoBehaviour
- ScriptableObject
- Q: role level experience progression station equipment inventory skill upgrade
- AutosaveIntervalsInstaller
- PlayerSaveData
- Autosave

## God Nodes (most connected - your core abstractions)
1. `StableId` - 251 edges
2. `Utils` - 158 edges
3. `Processors` - 156 edges
4. `ScriptablesProcessorInfrastructure` - 150 edges
5. `Player` - 142 edges
6. `WorldSimulation` - 129 edges
7. `WorldGenProcessor` - 110 edges
8. `SettingsProcessor` - 107 edges
9. `Reflex.Core` - 103 edges
10. `SaveProcessor` - 88 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `generate_and_spawn_world()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `load_input()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `stress()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/xtask/src/main.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (263 total, 22 thin omitted)

### Community 0 - "world.rs"
Cohesion: 0.18
Nodes (20): WorldGenConfig, authored_foliage_is_deterministic_and_respects_habitat_and_resources(), cell_hash(), changing_seed_changes_world_hash(), foliage_hash(), foliage_noise(), generate_foliage(), generate_world_from_layers() (+12 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.06
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "Transform"
Cohesion: 0.13
Nodes (28): AnimationClip, AnimationGraph, AnimationTargetId, add_animation_layer_branch(), add_rotation_curve(), add_scale_curve(), add_translation_curve(), authored_layer_weight_and_mask_configure_bevy_graph_branch() (+20 more)

### Community 3 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.10
Nodes (63): animation_state_id(), animation_state_machine_id(), append_vec3_keys(), clip_id(), controller_id(), convert_clips(), convert_controllers(), convert_prefab_renderer_materials() (+55 more)

### Community 4 - "Target"
Cohesion: 0.09
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

### Community 5 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (23): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+15 more)

### Community 7 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (19): Color, float, string, TechnologyTreeGroup, Vector2, Group, int, Vector2 (+11 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "settings.rs"
Cohesion: 0.12
Nodes (29): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), defaults_are_valid_and_round_trip(), DisplayMode, imports_unity_json_indices_and_clamps_values() (+21 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "WorldGenProcessor"
Cohesion: 0.07
Nodes (15): Action, bool, BoxCollider, Container, ContainerBuilder, GameObject, IEnumerable, int (+7 more)

### Community 12 - "Player"
Cohesion: 0.05
Nodes (15): Player, Dictionary, GameObject, Vector3, List, GameSettings, Vector3, BuildingCommands (+7 more)

### Community 13 - "simulation.rs"
Cohesion: 0.07
Nodes (30): authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), capped_deposit_preserves_inventory_overflow(), default_ruler_vote_cooldown(), deterministic_fish_god_value(), deterministic_weather(), enemy_camps_and_raid_progress_round_trip_with_stable_archetypes(), fish_god_progress_rewards_food_unlocks_pet_and_expires() (+22 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (17): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, GroupSaveData, HashSet, List, Node_SO (+9 more)

### Community 15 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 16 - "Character"
Cohesion: 0.05
Nodes (26): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+18 more)

### Community 17 - "Commands"
Cohesion: 0.11
Nodes (51): ActorCustomization, ActionPresentation, actor_combat_visual(), actor_material(), animate_agents(), building_effect_material(), BuildingEffectKind, BuildingEffectParticle (+43 more)

### Community 18 - "Processors"
Cohesion: 0.05
Nodes (15): BuildCostModifier, GridProcessorEditor, InputButton, UserInterface.MainMenu, Processors, World, Level, MetaData (+7 more)

### Community 19 - "process_injected_commands"
Cohesion: 0.09
Nodes (37): StreamUserType, active_event_text(), building_age(), building_definition_id(), building_model_node_names(), CommandOrigin, CreditsTimeline, eligible_technology_ids() (+29 more)

### Community 20 - ".CreateEnumField"
Cohesion: 0.12
Nodes (13): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+5 more)

### Community 21 - "TechTree.Elements"
Cohesion: 0.07
Nodes (23): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+15 more)

### Community 22 - "BuildingPlacer"
Cohesion: 0.07
Nodes (17): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+9 more)

### Community 23 - "PlayerProcessor"
Cohesion: 0.09
Nodes (11): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, OnMessageReceivedArgs (+3 more)

### Community 24 - "HealthHandler"
Cohesion: 0.08
Nodes (15): Func, BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, float, Enemy (+7 more)

### Community 25 - "PlayerRoleData"
Cohesion: 0.08
Nodes (13): PlayerRoleData, AudioClip, bool, float, int, StatModifiers, Dictionary, Dictionary (+5 more)

### Community 26 - "Goal"
Cohesion: 0.12
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 27 - "Access_Dropdown"
Cohesion: 0.07
Nodes (17): GameObject, List, PresetButtons, Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown (+9 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "GameEventProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+2 more)

### Community 30 - "ContentCatalog"
Cohesion: 0.06
Nodes (106): GameConfig, ContentCatalog, GridPos, ActorState, String, generate_world(), GeneratedWorld, action_animation_speed() (+98 more)

### Community 31 - "BuildingResourceModelHandler"
Cohesion: 0.11
Nodes (12): BuildingResourceModelHandler, GameObject, float, int, Queue, ResourceRateOfChange, UnityEvent, Dictionary (+4 more)

### Community 32 - "PlayerInventory"
Cohesion: 0.11
Nodes (8): PlayerInventory, Dictionary, ResourceInventory, bool, int, Dictionary, float, STSM_Action_DepositResource

### Community 33 - "ResourceData"
Cohesion: 0.18
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 34 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 35 - "UserInterface_Debug"
Cohesion: 0.07
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 36 - "MiscCommands"
Cohesion: 0.16
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (18): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+10 more)

### Community 38 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 39 - ".SerializeComponent"
Cohesion: 0.11
Nodes (13): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+5 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.11
Nodes (9): Camera, Container, ContainerBuilder, InputButton, UnityAction, Vector2, Vector3, ObjectSelectionProcessor (+1 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.12
Nodes (6): NodeUnlockData, Action, Container, ContainerBuilder, List, TechTreeProcessor

### Community 42 - "CharacterModelHandler"
Cohesion: 0.17
Nodes (11): AddEquipmentSet(), CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+3 more)

### Community 43 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 44 - "AnimationControllerDef"
Cohesion: 0.12
Nodes (27): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+19 more)

### Community 45 - "legacy.rs"
Cohesion: 0.05
Nodes (111): ActorKind, detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+103 more)

### Community 46 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 47 - ".Log"
Cohesion: 0.08
Nodes (15): Container, ContainerBuilder, HideInCallstack, Object, DebugProcessor, int, STSM_Helper_Build, STSM_Action_Build (+7 more)

### Community 48 - "NavGrid"
Cohesion: 0.13
Nodes (21): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+13 more)

### Community 49 - ".Draw"
Cohesion: 0.19
Nodes (11): Port, Action, Button, Foldout, TextField, Toggle, TechTreeUtilities, ChangeEvent (+3 more)

### Community 50 - "AudioHandler"
Cohesion: 0.10
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, int, IReadOnlyList, List, long, MenuItem (+5 more)

### Community 52 - "ResourceProcessor"
Cohesion: 0.12
Nodes (13): Container, float, int, List, Material, materials, Mesh, meshes (+5 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.12
Nodes (22): bool, GameObject, HashSet, int, List, long, MenuItem, string (+14 more)

### Community 56 - "RoleHandler"
Cohesion: 0.10
Nodes (6): RoleHandler, bool, Dictionary, UnityEvent, List, SelectedPlayer

### Community 57 - "StableId"
Cohesion: 0.11
Nodes (19): FromStr, StableId, BuildingState, complete_gameplay_scenario_round_trips(), EnemyCampState, RaidState, BTreeMap, BTreeSet (+11 more)

### Community 58 - "STSM_GoToLocation"
Cohesion: 0.08
Nodes (14): STSM_HelperDeposit, bool, float, GameObject, int, Transform, Vector3, STSM_GoToLocation (+6 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 61 - "StateMachine"
Cohesion: 0.07
Nodes (15): PlayerDeathHandler, bool, float, Vector3, StateMachine, string, STSM_HelperBase, bool (+7 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

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
Cohesion: 0.10
Nodes (12): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, CollectionUtility, ICollection (+4 more)

### Community 67 - "Result"
Cohesion: 0.14
Nodes (18): BTreeSet, TwitchConfig, CredentialVault, DeviceAuthorization, OAuthClient, OAuthErrorResponse, Client, Formatter (+10 more)

### Community 68 - "Objective"
Cohesion: 0.08
Nodes (13): Action, int, Objective, ObjectiveType, ObjectiveData, Dictionary, GameObject, Image (+5 more)

### Community 69 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 70 - "GenerationSettings"
Cohesion: 0.09
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 71 - "models.rs"
Cohesion: 0.23
Nodes (17): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, Option, Path, PathBuf, Result (+9 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.13
Nodes (12): BuildingScriptablesEditor, bool, Color, Dictionary, int, MenuItem, ScriptableObject, string (+4 more)

### Community 74 - "BTreeMap"
Cohesion: 0.10
Nodes (41): PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), avatar_mask_id(), collect_prefab_dependencies(), color_value(), convert() (+33 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "STSM_Action_PlayerBase"
Cohesion: 0.12
Nodes (5): int, STSM_Helper_Attack, STSM_Action_Heal, STSM_Action_PlayerAttack, STSM_Action_PlayerBase

### Community 77 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 78 - "GridProcessor"
Cohesion: 0.09
Nodes (14): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+6 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.06
Nodes (34): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+26 more)

### Community 80 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 81 - "WorldUtils"
Cohesion: 0.21
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "SaveDataMapper"
Cohesion: 0.09
Nodes (14): Component, List, Mesh, Transform, SaveDataMapper, bool, int, MeshSaveData (+6 more)

### Community 83 - "TreeMaterialExtension"
Cohesion: 0.16
Nodes (15): BuildingMaterialExtension, BuildingMaterialUniform, CloudMaterialExtension, CloudMaterialUniform, Image, TerrainMaterialExtension, TerrainMaterialUniform, TreeMaterialExtension (+7 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "stream_town_game/src/lib.rs"
Cohesion: 0.03
Nodes (114): AnimationNodeIndex, AnimationPlayer, AnyResult, ActivePetVisual, ActorAnimationDriver, advance_animation_crossfade(), AgentCommand, AgentCommandQueue (+106 more)

### Community 86 - "CellSpacePartitioning"
Cohesion: 0.15
Nodes (12): bool, List, Vector2, BSPCell, Dictionary, float, int, List (+4 more)

### Community 87 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 88 - "CommonEnums.cs"
Cohesion: 0.06
Nodes (32): List, Vector3, RoleData, AudioClip, bool, float, int, Sprite (+24 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "Option"
Cohesion: 0.10
Nodes (64): AssetServer, ArchetypeDef, ArchetypeKind, ArchetypeScene, PresentationCatalog, actor_detail_budget(), actor_scene_budget(), animation_property_value() (+56 more)

### Community 91 - "PlayerRole"
Cohesion: 0.05
Nodes (24): RoleSlotModifier, int, RoleSlot, bool, int, Dictionary, int, RoleDataContainer (+16 more)

### Community 92 - "GameEvent"
Cohesion: 0.05
Nodes (26): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+18 more)

### Community 93 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.18
Nodes (22): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+14 more)

### Community 95 - "STSM_StateAction"
Cohesion: 0.14
Nodes (6): int, STSM_Action_Attack, bool, float, int, STSM_StateAction

### Community 96 - "stream_town_migrate/src/content.rs"
Cohesion: 0.06
Nodes (139): ArchetypesById, ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, EnemyDef (+131 more)

### Community 97 - "BinaryReader"
Cohesion: 0.15
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 98 - "Coordinator"
Cohesion: 0.08
Nodes (20): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+12 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.08
Nodes (45): AnimationClipDef, AnimationConditionMode, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference (+37 more)

### Community 100 - "Editor"
Cohesion: 0.11
Nodes (7): BuildingModelHandlerEditor, BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

### Community 101 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 102 - "twitch.rs"
Cohesion: 0.14
Nodes (19): channel_point_reward_tag_survives_privmsg_conversion(), envelope_from_privmsg(), Arc, Mutex, Option, Receiver, Vec, token_from_response() (+11 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.08
Nodes (13): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+5 more)

### Community 105 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 108 - "ResourceHolder"
Cohesion: 0.22
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 109 - "CustomLogHandler"
Cohesion: 0.12
Nodes (12): CustomLogHandler, Exception, HideInCallstack, LogType, Object, Dictionary, DebugSettings, ContainerBuilder (+4 more)

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.19
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 113 - "SelectedObject"
Cohesion: 0.18
Nodes (3): object, UnityAction, SelectedObject

### Community 114 - "UserInterface_RulerVote"
Cohesion: 0.12
Nodes (9): List, KeepKingVote, TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI (+1 more)

### Community 115 - ".OnGUI"
Cohesion: 0.14
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

### Community 116 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 117 - "IRuntimeDataScriptable"
Cohesion: 0.13
Nodes (14): Queue, AudioRuntimeData, CreditsRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, Dictionary (+6 more)

### Community 118 - "VoteEvent"
Cohesion: 0.12
Nodes (12): int, List, NewKingVote, PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent (+4 more)

### Community 119 - "Resource"
Cohesion: 0.06
Nodes (21): DepositResources, ResourceStorageModifier, float, int, int, ActiveResourceIncrementer, IResourceHolder, Container (+13 more)

### Community 120 - ".new"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 121 - "Res"
Cohesion: 0.04
Nodes (157): AccumulatedMouseMotion, AccumulatedMouseScroll, Added, AmbientLight, AnimationGraphHandle, App, AppExit, Assets (+149 more)

### Community 122 - ".SetTargetType"
Cohesion: 0.12
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (22): Action, bool, float, int, string, Type, Vector3, AIPath (+14 more)

### Community 125 - "Targetable"
Cohesion: 0.08
Nodes (17): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+9 more)

### Community 126 - "BuildingBase"
Cohesion: 0.10
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 127 - "CommandDictionary"
Cohesion: 0.31
Nodes (5): Action, Dictionary, IReadOnlyList, List, CommandDictionary

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 130 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 131 - ".GenerateFromSettings"
Cohesion: 0.12
Nodes (17): HashSet, Func, HashSet, List, Material, Resource, Vector2, Vector3 (+9 more)

### Community 133 - ".RenderResourceType"
Cohesion: 0.18
Nodes (11): Dictionary, int, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 134 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 135 - "config.rs"
Cohesion: 0.14
Nodes (17): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, Default, Result (+9 more)

### Community 136 - "ToolState"
Cohesion: 0.11
Nodes (48): apply_technology_draft(), bounded_ui_index(), content_tab(), draw_world_preview(), inspector_tab(), main(), migration_tab(), poll_tool_job_events() (+40 more)

### Community 137 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 138 - "SensorProcessor"
Cohesion: 0.13
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 139 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

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
Cohesion: 0.09
Nodes (13): Action, ProcessorStartupContext, Action, bool, CancellationToken, Container, ContainerBuilder, float (+5 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "UI_TechOption"
Cohesion: 0.22
Nodes (7): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption

### Community 150 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 151 - "ObjectPoolingSettings"
Cohesion: 0.18
Nodes (9): bool, List, ObjectPoolingSettings, ContainerBuilder, ObjectPoolingSettingsInstaller, GameObject, int, string (+1 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 154 - "WeatherProcessor"
Cohesion: 0.08
Nodes (19): ContainerBuilder, AllSeasonsSettingsInstaller, float, int, Material, AllSeasonSettings, Container, ContainerBuilder (+11 more)

### Community 155 - "DataStructures"
Cohesion: 0.22
Nodes (4): int, ChangeTimeStamp, DataStructures, DateTime

### Community 156 - "ResourceDataSaveData"
Cohesion: 0.10
Nodes (19): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+11 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 159 - "TechTreeNode"
Cohesion: 0.14
Nodes (12): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Capacity (+4 more)

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

### Community 164 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 165 - "FoliageGenerationSettings"
Cohesion: 0.10
Nodes (16): Vector3, int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, float (+8 more)

### Community 167 - "CampGenerationSettings"
Cohesion: 0.17
Nodes (9): ContainerBuilder, CampGenSettingsInstaller, List, CampGenSettings, float, int, string, Vector2 (+1 more)

### Community 169 - "TerrainGenSettings"
Cohesion: 0.22
Nodes (8): ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject, Material, TerrainGenSettings

### Community 170 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 171 - "TimeProcessor"
Cohesion: 0.17
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 173 - "Access_GOList"
Cohesion: 0.22
Nodes (5): Access_GOList, GameObject, List, Access_SettingsMenus, Access_SettingsTabs

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "EventProcessor"
Cohesion: 0.25
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

### Community 177 - "IProcessor"
Cohesion: 0.10
Nodes (14): CancellationToken, Task, CancellationToken, Container, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor (+6 more)

### Community 178 - "ObjectPoolingRuntimeData"
Cohesion: 0.43
Nodes (7): Dictionary, float, GameObject, List, Queue, ObjectPoolingRuntimeData, TimeSpan

### Community 179 - "VfxAnimationController"
Cohesion: 0.07
Nodes (15): Slider, TextMeshProUGUI, UI_Objective, Image, TextMeshProUGUI, UIRoleDisplay, GameObject, SimpleRandomModelEnabled (+7 more)

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "PoolableObject"
Cohesion: 0.09
Nodes (16): Container, ContainerBuilder, GUIDProcessor, BoxCollider, List, Quaternion, Vector3, SaveablFoliage (+8 more)

### Community 183 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 185 - "TargetSettings"
Cohesion: 0.29
Nodes (4): TargetSettings, ContainerBuilder, TargetSettingsInstaller, TargetableData

### Community 187 - "PassiveResourceIncrementer"
Cohesion: 0.24
Nodes (3): bool, float, PassiveResourceIncrementer

### Community 188 - "Access_TextInput"
Cohesion: 0.29
Nodes (3): Access_ChannelNameInput, Access_TextInput, TMP_InputField

### Community 189 - "ObjectiveDef"
Cohesion: 0.32
Nodes (6): ObjectiveDef, objective_increment(), ObjectiveEvent, ObjectiveProgress, Vec, TownGoalState

### Community 190 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 191 - "command.rs"
Cohesion: 0.21
Nodes (28): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+20 more)

### Community 193 - "Sensors"
Cohesion: 0.08
Nodes (13): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+5 more)

### Community 195 - "Node_SO.cs"
Cohesion: 0.40
Nodes (3): InputButton, SharedTypes, Data

### Community 196 - "TechNodeData"
Cohesion: 0.25
Nodes (4): List, Node_SO, TechNodeData, IEnumerable

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 200 - "BuildingModelHandler"
Cohesion: 0.25
Nodes (3): BuildingModelHandler, GameObject, List

### Community 201 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 202 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **No Fields Other Than Injected Dependencies**, 2. **No Awake/Start Methods**, 3. **RuntimeData Pattern**, 4. **ProjectScope Only**, 5. **Data Retrieval Sections**, 6. **No Coroutines in Processors**, 7. **Temporary Legacy Exclusion Policy**, Key Rules

### Community 204 - ".GetCompatiblePorts"
Cohesion: 0.50
Nodes (3): List, Port, NodeAdapter

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "Utils"
Cohesion: 0.05
Nodes (7): RoleScriptablesEditor, Utils, ScriptablesEditor, SavingAndLoading, SavingAndLoading.Structs, GameResources, World.Generation

### Community 209 - "VfxParticlePosition"
Cohesion: 0.40
Nodes (3): Transform, VisualEffect, VfxParticlePosition

### Community 211 - "FPSDisplay.cs"
Cohesion: 0.29
Nodes (4): IEnumerator, FPS(), Start(), FPSDisplay

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 219 - "ScriptKeywordProcessor"
Cohesion: 0.33
Nodes (4): AssetModificationProcessor, List, ScriptKeywordProcessor, char

### Community 221 - "xtask/src/main.rs"
Cohesion: 0.33
Nodes (8): Cli, Command, main(), Command, PathBuf, Result, stress(), validate()

### Community 222 - "Processor Template"
Cohesion: 0.33
Nodes (6): Architecture Overview, Checklist for New Processors, Processor Structure, Processor Template, RuntimeData, Settings Scriptable

### Community 223 - "Common Patterns"
Cohesion: 0.33
Nodes (6): Boolean Flags, Collections, Common Patterns, Complex State Objects, Counters, Events with Parameters

### Community 225 - "WorldSaveData"
Cohesion: 0.14
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 226 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 227 - "TODO List"
Cohesion: 0.40
Nodes (4): Excluded Processor Files (serialized scene/UI refs), High Priority, Medium Priority, TODO List

### Community 228 - "TechTree_SO"
Cohesion: 0.40
Nodes (4): int, TechTreeSettings, List, TechTree_SO

### Community 229 - "Twitch setup"
Cohesion: 0.09
Nodes (18): Audio provenance, Binaries, Commands, Stream Town Bevy, Bevy Migration Status, Delivered in this milestone, Milestone interpretation, Not yet at parity (+10 more)

### Community 230 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 231 - "PostProcessingInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, Volume, PostProcessingInstaller

### Community 232 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 233 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 234 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 235 - "SimpleDisableAfterTime"
Cohesion: 0.25
Nodes (3): float, GameObject, SimpleDisableAfterTime

### Community 236 - "Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?, Source Nodes

### Community 238 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 239 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 240 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 243 - "SaveProcessor"
Cohesion: 0.08
Nodes (21): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+13 more)

### Community 249 - "ResourceGenerationSettings"
Cohesion: 0.18
Nodes (9): ContainerBuilder, WaterResourceGenSettingsInstaller, List, WaterResourceGenSettings, AnimationCurve, bool, int, List (+1 more)

### Community 254 - "AllRoleDataSettings"
Cohesion: 0.25
Nodes (5): ContainerBuilder, AllRoleDataSettingsInstaller, ContainerBuilder, RoleDataContainerInstaller, AllRoleDataSettings

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 260 - "Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the translated Unity Animator controller schema execute and drive weighted Bevy playback?, Source Nodes

### Community 261 - "TargetSensor"
Cohesion: 0.12
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 263 - "EquipmentHandlerEditor"
Cohesion: 0.40
Nodes (3): GameObject, List, EquipmentHandlerEditor

### Community 266 - "AudioMixerInstaller"
Cohesion: 0.40
Nodes (3): AudioMixerInstaller, AudioMixer, ContainerBuilder

### Community 267 - "ForwardRendererInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRendererData, ForwardRendererInstaller

### Community 269 - "RenderPipelineInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, UniversalRenderPipelineAsset, RenderPipelineInstaller

### Community 270 - "VideoSettingsPresetsInstaller"
Cohesion: 0.40
Nodes (3): ContainerBuilder, List, VideoSettingsPresetsInstaller

### Community 276 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (7): ContainerBuilder, AudioSettingsInstaller, int, AudioSettings, Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 277 - "IntWrapper"
Cohesion: 0.33
Nodes (3): Access_Preset, ContainerBuilder, IntWrapper

### Community 279 - "MonoBehaviour"
Cohesion: 0.02
Nodes (76): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+68 more)

### Community 280 - "ScriptableObject"
Cohesion: 0.03
Nodes (56): List, FoliageGenSettings, bool, ParticleSystem, Transform, GameEventSettings, int, Vector2 (+48 more)

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "AutosaveIntervalsInstaller"
Cohesion: 0.40
Nodes (3): AutosaveIntervalsInstaller, ContainerBuilder, List

### Community 285 - "PlayerSaveData"
Cohesion: 0.14
Nodes (13): bool, int, List, string, InventoryEntrySaveData, InventorySaveData, bool, int (+5 more)

## Knowledge Gaps
- **226 isolated node(s):** `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState` (+221 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Work-memory lessons

**Preferred sources** — corroborated by past sessions; start here.
- `BevyMigrationExporter` (2× useful, score=1.985812412) _(code changed — re-verify)_
- `RenderAssets` (2× useful, score=1.985812412) _(code changed — re-verify)_

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SaveProcessor` connect `SaveProcessor` to `BuildingProcessor`, `SettingsProcessor`, `WorldGenProcessor`, `Player`, `ObjectPoolingProcessor`, `Character`, `PlayerProcessor`, `ScriptableObject`, `PlayerRoleData`, `MonoBehaviour`, `ResourceDataSaveData`, `GameEventProcessor`, `SaveFileData`, `FoliageGenerationSettings`, `TechTreeProcessor`, `TimeProcessor`, `SeasonProcessor`, `.Log`, `IProcessor`, `StreamTownSessionBridge`, `ResourceProcessor`, `PoolableObject`, `SaveDataMapper`, `FoliageProcessor`, `PlayerRole`, `WorldSaveData`, `TownGoalProcessor`, `MainMenuManager`, `Resource`, `ResourceGenerationSettings`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **Why does `WorldGenProcessor` connect `WorldGenProcessor` to `BuildingProcessor`, `.GenerateFromSettings`, `Target`, `TwitchChatProcessor`, `Player`, `ObjectPoolingProcessor`, `PlayerProcessor`, `ScriptableObject`, `MonoBehaviour`, `ResourceData`, `UserInterface_Debug`, `CampGenerationSettings`, `TerrainGenSettings`, `.Log`, `IProcessor`, `ResourceProcessor`, `TwitchClientProcessor`, `PoolableObject`, `WorldGenRuntimeData`, `ProjectCamera`, `GridProcessor`, `CellSpacePartitioning`, `FoliageProcessor`, `GameEvent`, `GameStateProcessor`, `EnemySpawner`, `SaveProcessor`, `ResourceGenerationSettings`, `AIPath`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `SettingsProcessor` connect `SettingsProcessor` to `Autosave`, `GraphicsProcessor`, `Access_Slider`, `SettingsData`, `MainMenuManager`, `ProjectCamera`, `Access_GOList`, `IProcessor`, `SaveProcessor`, `ScriptablesProcessorInfrastructure`, `Access_Text`, `MonoBehaviour`, `Access_Toggle`, `Access_Dropdown`, `Access_TextInput`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **What connects `StreamTown.Migration`, `StreamTown.EditorTools`, `Attributes` to the rest of the system?**
  _226 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.059887005649717516 - nodes in this community are weakly interconnected._
- **Should `Transform` be split into smaller, more focused modules?**
  _Cohesion score 0.12698412698412698 - nodes in this community are weakly interconnected._
- **Should `stream_town_migrate/src/presentation.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10367063492063493 - nodes in this community are weakly interconnected._