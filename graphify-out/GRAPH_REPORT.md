# Graph Report - Stream-Town-Bevy  (2026-08-27)

## Corpus Check
- 670 files · ~1,769,975 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8824 nodes · 25701 edges · 325 communities (298 shown, 27 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1035 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ebb685de`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UserInterface
- BuildingProcessor
- ScriptablesProcessorInfrastructure
- PlayerSaveData
- Res
- BuildingScriptablesEditor.cs
- TwitchChatProcessor
- Option
- BottomBarInterface
- stream_town_game/src/lib.rs
- SettingsProcessor
- setup_rendering
- stream_town_migrate/src/presentation.rs
- NavGrid
- TechTreeIOUtility
- HealthHandler
- CellSpacePartitioning
- save.rs
- ContentCatalog
- GlobalAudioController
- twitch.rs
- UIElementWrapper
- STSM_GoToLocation
- RenderAssets
- StationProcessor
- command.rs
- BinarySaveCodec
- StableId
- SaveFileData
- world.rs
- ResMut
- WorldGenProcessor
- UserInterface_Debug
- stream_town_domain/src/content.rs
- MeshData
- .SetTargetType
- BinaryWriter
- SettingsData
- SeasonProcessor
- TechnologyGraphViewState
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- SelectedPlayerGroup
- AnimationControllerDef
- runtime_console.rs
- UserInterface_RulerVote
- RoleHandler
- FoliageGenerationSettings
- update_credits_fireworks
- IRuntimeDataScriptable
- StreamTownSessionBridge
- Pet
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- legacy.rs
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
- Result
- Objective
- SaveDataMapper
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
- drive_tidal_music
- String
- Goal
- EnemyModelHandler
- FoliageProcessor
- preview_grid_point
- GameEvent
- GameConfig
- MonoBehaviour
- convert_fbx_to_glb.py
- STSM_Idle_Player
- Resource
- SaveProcessor
- convert_post_process
- stream_town_domain/src/presentation.rs
- .Draw
- StateMachine
- ResourceRuntimeData
- TownGoalProcessor
- MainMenuManager
- ResourceData
- LoadingManager
- Station
- WindController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- DayAndNightProcessor
- VfxSeagullSpawner
- SaveProcessor.cs
- stream_town_tools/src/main.rs
- GridProcessor
- .EnsureValidCredentials
- GameEventProcessor
- Vec
- VoteEvent
- unity_color_filter
- GUIDComponent
- SnapToGridMouseMovement
- AIPath
- GameStateProcessor
- GateController
- capture_direct_broadcast_frame
- IProcessor
- CommonEnums.cs
- encode_broadcast_session
- StringUtils
- EnemySpawner
- LabelDisplayProcessor
- ObjectiveSaveData
- Coordinator
- RaidEvent
- String
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- PathBuf
- RuntimeData Template
- Key Rules
- TechTreeNode
- RoleData
- xtask/src/lib.rs
- Targetable
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- Sensors
- PlayerInputProcessor
- BTreeMap
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- ResourceTarget
- BroadcastTarget
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- WorldInstanceDeterminism
- building_node_visibility
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- .ExerciseHealthBarCategory
- .RenderResourceType
- Q: There are still no animations.
- xtask/src/main.rs
- List
- RandomEnabler
- BuildingModelHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- .Update
- World.Generation
- KeepKingVote
- Access_Toggle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- NewKingVote
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- SimpleScreenShot
- TechnologyTreeGroup
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- BevyMigrationExporter
- Enemy
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- FishGodEvent
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
- .Log
- Requirement
- SimpleMusicController
- IProcessor.cs
- Easings
- Key Rules
- ResourceHolder
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
- TargetProcessor
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- TL_Secrets
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
- PlayerRole
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
- .UserIsSubscribed
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxAnimationController
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- .default
- UserInterface_TownGoal
- StreamTown.Migration
- SelectedBuilding
- UserInterface_Event
- .new
- Processors
- ResourceProcessor
- record_gpu_readiness
- CreateProjectScopeProcessors.cs
- Utils
- settings.rs
- direct_broadcast.rs
- PlayerInputRuntimeData
- DontDestroyOnLoad
- Season
- BuildingDataSettings
- BuildPlacerData
- .ValidateTokenAsync
- ObjectSelectionProcessor.Editor.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- STSM_StateAction
- NodeUnlockData
- SimpleRotateOnAxis
- CampGenerationSettings
- Buildings
- UnityGraphics
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- ResourceGenerationSettings
- .RefreshSceneBindingsAndTryGenerate
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- PlayerSpawnPoint
- WorldGenSaveData
- vcpkg.json
- .MoveCamera
- animate_loading_icon
- TwitchClientRuntimeData
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
- `battering_ram_targets_and_damages_buildings_from_authored_mask()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `builder_completes_and_upgrades_authored_construction()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `combat_goal_damages_kills_and_respawns()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (325 total, 27 thin omitted)

### Community 0 - "UserInterface"
Cohesion: 0.04
Nodes (22): InputButton, SharedTypes, int, ChangeTimeStamp, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI (+14 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.05
Nodes (16): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+8 more)

### Community 2 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 3 - "PlayerSaveData"
Cohesion: 0.08
Nodes (19): ResourceInventory, bool, int, Dictionary, bool, int, List, string (+11 more)

### Community 4 - "Res"
Cohesion: 0.04
Nodes (196): Aabb, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, ActivePetVisual (+188 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (28): MetaData, bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue (+20 more)

### Community 7 - "Option"
Cohesion: 0.03
Nodes (111): AccessibilityNode, AssetId, actor_detail_budget(), advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), animation_detail_budget(), animation_root_name() (+103 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "stream_town_game/src/lib.rs"
Cohesion: 0.01
Nodes (344): AccessibilityActionRequest, generate_world(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), accessibility_should_clear_focus(), AccessibilityActionDispatch, AccessibilityAnnouncement (+336 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.07
Nodes (10): AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset, Preset, SettingsProcessor (+2 more)

### Community 11 - "setup_rendering"
Cohesion: 0.05
Nodes (75): AmbientLight, Assets, ActiveMaterialHandles, animate_main_menu_clouds(), building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance (+67 more)

### Community 12 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (78): AnimationFloatKeyframe, AnimationTangent, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), controller_id(), convert_fireworks() (+70 more)

### Community 13 - "NavGrid"
Cohesion: 0.11
Nodes (25): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+17 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.10
Nodes (16): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+8 more)

### Community 15 - "HealthHandler"
Cohesion: 0.10
Nodes (11): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, Action, bool, float, int (+3 more)

### Community 16 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 17 - "save.rs"
Cohesion: 0.08
Nodes (63): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+55 more)

### Community 18 - "ContentCatalog"
Cohesion: 0.07
Nodes (101): ContentCatalog, GridPos, ActorState, RoleProgress, Default, String, GeneratedWorld, action_animation_speed() (+93 more)

### Community 19 - "GlobalAudioController"
Cohesion: 0.18
Nodes (8): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List

### Community 20 - "twitch.rs"
Cohesion: 0.06
Nodes (53): BTreeSet, Option, TwitchConfig, secrets_restart_requirements(), bot_and_broadcaster_tokens_use_distinct_vault_entries(), broadcaster_oauth_uses_only_the_stream_key_scope(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault (+45 more)

### Community 21 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 22 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 23 - "RenderAssets"
Cohesion: 0.05
Nodes (143): ArchetypeScene, MainMenuSceneReference, Option, PresentationCatalog, actor_material(), actor_scene_budget(), advance_falling_fish(), animate_healing_effects() (+135 more)

### Community 24 - "StationProcessor"
Cohesion: 0.06
Nodes (18): Container, ContainerBuilder, List, StationProcessor, float, List, SensorRuntimeData, Dictionary (+10 more)

### Community 25 - "command.rs"
Cohesion: 0.11
Nodes (37): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+29 more)

### Community 26 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Func, int, UTF8Encoding, BinarySaveCodec, BinaryReader

### Community 27 - "StableId"
Cohesion: 0.05
Nodes (67): ObjectiveDef, ObjectiveKind, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow() (+59 more)

### Community 28 - "SaveFileData"
Cohesion: 0.10
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "world.rs"
Cohesion: 0.09
Nodes (54): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash() (+46 more)

### Community 30 - "ResMut"
Cohesion: 0.03
Nodes (184): AccessibilityFocusVisualQuery, AppExit, BackgroundColor, BroadcastConfig, PlayerSettings, Default, DirectBroadcastControl, DirectBroadcastRuntime (+176 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (29): HashSet, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject, HashSet (+21 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.10
Nodes (7): bool, GameObject, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, UserInterface_Debug

### Community 33 - "stream_town_domain/src/content.rs"
Cohesion: 0.10
Nodes (43): ArchetypeBounds, ArchetypeDef, ArchetypeKind, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError (+35 more)

### Community 34 - "MeshData"
Cohesion: 0.11
Nodes (19): Action, IEnumerator, Vector2, Noise, List, Mesh, Vector2, Vector3 (+11 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.13
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "BinaryWriter"
Cohesion: 0.16
Nodes (4): Action, CancellationToken, List, BinaryWriter

### Community 37 - "SettingsData"
Cohesion: 0.08
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.11
Nodes (8): SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor, bool, float, int, SeasonRuntimeData

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.07
Nodes (56): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+48 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.07
Nodes (20): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+12 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.08
Nodes (9): List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, IEnumerable, List (+1 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "SelectedPlayerGroup"
Cohesion: 0.19
Nodes (3): List, List, SelectedPlayerGroup

### Community 44 - "AnimationControllerDef"
Cohesion: 0.11
Nodes (30): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+22 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "UserInterface_RulerVote"
Cohesion: 0.20
Nodes (7): TextMeshProUGUI, UI_RulerOption, Dictionary, GameObject, Slider, TextMeshProUGUI, UserInterface_RulerVote

### Community 47 - "RoleHandler"
Cohesion: 0.10
Nodes (7): PlayerInventory, Dictionary, RoleHandler, bool, Dictionary, UnityEvent, SelectedPlayer

### Community 48 - "FoliageGenerationSettings"
Cohesion: 0.10
Nodes (20): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+12 more)

### Community 49 - "update_credits_fireworks"
Cohesion: 0.08
Nodes (33): AccessibilityCandidate, animation_property_value(), AuthoredCreditsElement, credits_firework_origin(), credits_fireworks_active(), credits_fireworks_start(), credits_fireworks_use_authored_activation_and_deterministic_emission(), CreditsFade (+25 more)

### Community 50 - "IRuntimeDataScriptable"
Cohesion: 0.06
Nodes (25): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+17 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.16
Nodes (3): MenuItem, StreamTownSessionBridge, FrameCapture

### Community 52 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.07
Nodes (13): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, bool, float, PassiveResourceIncrementer (+5 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "legacy.rs"
Cohesion: 0.07
Nodes (84): ActorCustomization, StreamUserType, absolute_path(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser, BinaryParser<'a> (+76 more)

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
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (9): bool, Camera, float, int, PlayerInput, Transform, Vector2, CameraController (+1 more)

### Community 63 - "TargetSensor"
Cohesion: 0.13
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.08
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.07
Nodes (15): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, NodeGroup_SO, List, TechTree_SO (+7 more)

### Community 67 - "Result"
Cohesion: 0.18
Nodes (27): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role(), delete_selected_technology_group() (+19 more)

### Community 68 - "Objective"
Cohesion: 0.14
Nodes (5): Action, int, Objective, ObjectiveType, ObjectiveData

### Community 69 - "SaveDataMapper"
Cohesion: 0.10
Nodes (13): List, Mesh, Vector3, SaveDataMapper, bool, int, MeshSaveData, int (+5 more)

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

### Community 74 - "TimeProcessor"
Cohesion: 0.18
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "ObjectPoolingProcessor"
Cohesion: 0.04
Nodes (37): Container, ContainerBuilder, GUIDProcessor, Action, bool, BoxCollider, CancellationToken, Container (+29 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.12
Nodes (11): Dictionary, TechVote, bool, Button, GameObject, List, Slider, TextMeshProUGUI (+3 more)

### Community 78 - "TechTreeGraphView"
Cohesion: 0.08
Nodes (17): Vector2, GroupSaveData, int, List, Port, Vector2, TechTreeGraphView, List (+9 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.14
Nodes (13): bool, CancellationTokenSource, int, long, MenuItem, string, DeviceCodeResponse, ErrorResponse (+5 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "Handle"
Cohesion: 0.04
Nodes (67): AccessibilityMotionDefaults, authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform, BuildingMaterialExtension, BuildingMaterialUniform (+59 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "drive_tidal_music"
Cohesion: 0.19
Nodes (24): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), effective_music_gain(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature (+16 more)

### Community 86 - "String"
Cohesion: 0.05
Nodes (62): AccessibleNode, active_event_text(), authored_rotating_node_names(), building_definition_id(), building_model_node_names(), ChimneySmokeEmitterRuntime, ChimneySmokeEmitters, CommandFeedback (+54 more)

### Community 87 - "Goal"
Cohesion: 0.11
Nodes (9): bool, Dictionary, float, int, TechTreeRuntimeData, EventType, Action, Dictionary (+1 more)

### Community 88 - "EnemyModelHandler"
Cohesion: 0.11
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "preview_grid_point"
Cohesion: 0.67
Nodes (3): preview_grid_point(), Pos2, Rect

### Community 91 - "GameEvent"
Cohesion: 0.12
Nodes (7): Action, bool, double, object, EventType, GameEvent, SortGameEventStartTime

### Community 92 - "GameConfig"
Cohesion: 0.09
Nodes (42): ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameConfig, GameplayConfig, BTreeMap (+34 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (110): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+102 more)

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
Cohesion: 0.07
Nodes (20): Action, CancellationToken, Component, Container, ContainerBuilder, float, List, Material (+12 more)

### Community 98 - "convert_post_process"
Cohesion: 0.15
Nodes (22): AnimationClipDef, animation_take_name(), clip_id(), convert_clips(), convert_embedded_model_clips(), convert_post_process(), embedded_clip_id(), field_bool() (+14 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (69): AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+61 more)

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
Cohesion: 0.16
Nodes (8): List, TownGoalRuntimeData, Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.11
Nodes (10): LoadType, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator, int (+2 more)

### Community 105 - "ResourceData"
Cohesion: 0.18
Nodes (10): Dictionary, materialIndex, Matrix4x4, meshIndex, bool, int, Matrix4x4, uint (+2 more)

### Community 106 - "LoadingManager"
Cohesion: 0.07
Nodes (19): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+11 more)

### Community 107 - "Station"
Cohesion: 0.07
Nodes (15): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+7 more)

### Community 108 - "WindController"
Cohesion: 0.24
Nodes (4): float, Material, Vector2, WindController

### Community 109 - "CustomLogHandler"
Cohesion: 0.18
Nodes (8): CustomLogHandler, Exception, HideInCallstack, LogType, Object, string, Type, ILogHandler

### Community 110 - "LevelHandler"
Cohesion: 0.16
Nodes (5): BuildingLevelHandler, int, UnityEvent, LevelHandler, RoleLevelHandler

### Community 111 - "GamestateJukebox"
Cohesion: 0.21
Nodes (8): AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string, GamestateJukebox

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "SaveProcessor.cs"
Cohesion: 0.09
Nodes (16): Api, ActivityStatus, TL_API, bool, float, string, UserType, TwitchUser (+8 more)

### Community 115 - "stream_town_tools/src/main.rs"
Cohesion: 0.08
Nodes (70): authoring_snapshot(), AuthoringSnapshot, broadcast_encoder_label(), content_tab(), draw_world_preview(), format_game_master_ids(), format_runtime_frame_times(), inject_runtime_command() (+62 more)

### Community 116 - "GridProcessor"
Cohesion: 0.09
Nodes (14): int, List, Vector2, GridNode, Container, ContainerBuilder, GridProcessor, Color (+6 more)

### Community 117 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 118 - "GameEventProcessor"
Cohesion: 0.10
Nodes (9): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+1 more)

### Community 119 - "Vec"
Cohesion: 0.04
Nodes (91): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+83 more)

### Community 120 - "VoteEvent"
Cohesion: 0.19
Nodes (9): PlayerVote, Dictionary, float, IReadOnlyDictionary, VoteEvent, int, object, string (+1 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "GUIDComponent"
Cohesion: 0.17
Nodes (10): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+2 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 126 - "GameStateProcessor"
Cohesion: 0.16
Nodes (5): Container, ContainerBuilder, GameStateProcessor, bool, GameStateRuntimeData

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "capture_direct_broadcast_frame"
Cohesion: 0.16
Nodes (13): AudioFrame, BroadcastPrerequisites, capture_direct_broadcast_frame(), poll_direct_broadcast_authorization(), poll_direct_broadcast_worker(), Commands, Image, Res (+5 more)

### Community 129 - "IProcessor"
Cohesion: 0.08
Nodes (13): Container, IMainThreadInitializableProcessor, IProcessor, Container, ContainerBuilder, CreditsProcessor, Action, Container (+5 more)

### Community 130 - "CommonEnums.cs"
Cohesion: 0.05
Nodes (25): AnimationHandler, Animator, bool, Dictionary, float, int, PlayerDeathHandler, bool (+17 more)

### Community 131 - "encode_broadcast_session"
Cohesion: 0.21
Nodes (14): AtomicBool, BroadcastController, BroadcastMetrics, BroadcastMetricsSnapshot, encode_broadcast_session(), MediaInput, Arc, AtomicU64 (+6 more)

### Community 133 - "EnemySpawner"
Cohesion: 0.10
Nodes (11): Transform, float, int, List, Transform, EnemySpawner, float, ChanceObject (+3 more)

### Community 134 - "LabelDisplayProcessor"
Cohesion: 0.07
Nodes (17): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+9 more)

### Community 135 - "ObjectiveSaveData"
Cohesion: 0.08
Nodes (18): ChildrenSaveData, Vector2, GroupSaveData, List, Vector2, NodeSaveData, List, TechTreeSaveData_SO (+10 more)

### Community 136 - "Coordinator"
Cohesion: 0.09
Nodes (16): Coordinator, StartupState, Action, bool, CancellationTokenSource, Container, Dictionary, GameObject (+8 more)

### Community 137 - "RaidEvent"
Cohesion: 0.15
Nodes (6): bool, IEnumerator, int, List, string, RaidEvent

### Community 138 - "String"
Cohesion: 0.11
Nodes (45): AnimationParameterDef, animator_component(), animator_reference_path(), convert_prefab_bindings(), fixture_asset(), glb_animation_names(), infer_missing_parameters(), inline_file_id() (+37 more)

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

### Community 143 - "PathBuf"
Cohesion: 0.19
Nodes (14): AnyResult, FoliageAcceptanceCapture, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), open_twitch_verification_uri(), open_twitch_verification_uri_with(), player_settings_path() (+6 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TechTreeNode"
Cohesion: 0.14
Nodes (9): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Group, ContextualMenuPopulateEvent (+1 more)

### Community 147 - "RoleData"
Cohesion: 0.14
Nodes (14): RoleData, AudioClip, bool, float, int, Sprite, string, AudioClip (+6 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "Targetable"
Cohesion: 0.13
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
Cohesion: 0.09
Nodes (11): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours (+3 more)

### Community 154 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 155 - "BTreeMap"
Cohesion: 0.10
Nodes (48): MaterialDef, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), color_value(), convert(), convert_avatar_masks() (+40 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.13
Nodes (11): NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement, Button, EnumField, UnlockVisualElement, EnumField (+3 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - "ResourceTarget"
Cohesion: 0.39
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 159 - "BroadcastTarget"
Cohesion: 0.27
Nodes (10): BroadcastEncoderPreference, AuthorizationEvent, BroadcastTarget, encoder_candidates(), open_audio_encoder(), open_video_encoder(), Debug, String (+2 more)

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "WorldInstanceDeterminism"
Cohesion: 0.31
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 163 - "building_node_visibility"
Cohesion: 0.15
Nodes (15): apply_passive_building_income(), authored_building_nodes_follow_construction_age_and_storage_fill(), building_construction_cost(), building_cost_reduction_percent(), building_node_visibility(), building_upgrade_cost(), model_contains_node(), model_node_visible() (+7 more)

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.16
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - ".ExerciseHealthBarCategory"
Cohesion: 0.18
Nodes (11): bool, double, float, Func, int, IReadOnlyList, List, long (+3 more)

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

### Community 185 - "TechnologyTreeGroup"
Cohesion: 0.12
Nodes (12): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, Color, float (+4 more)

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.02
Nodes (88): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+80 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "Enemy"
Cohesion: 0.09
Nodes (11): Action, float, Enemy, Action, Dictionary, IReadOnlyList, List, CommandDictionary (+3 more)

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (44): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, String, Vec (+36 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "FishGodEvent"
Cohesion: 0.19
Nodes (6): Animator, GameObject, IEnumerator, int, FishGodEvent, GameEvent

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
Cohesion: 0.15
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.17
Nodes (4): CancellationToken, Task, Dictionary, ParallelProgressReporter

### Community 209 - ".Log"
Cohesion: 0.09
Nodes (12): Action, Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory (+4 more)

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 212 - "IProcessor.cs"
Cohesion: 0.18
Nodes (9): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext, ProcessorStartupReport (+1 more)

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

### Community 218 - "SelectedObject"
Cohesion: 0.14
Nodes (4): SelectedEnemyCamp, object, UnityAction, SelectedObject

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

### Community 234 - "TargetProcessor"
Cohesion: 0.18
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

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

### Community 243 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

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

### Community 256 - "PlayerRole"
Cohesion: 0.05
Nodes (23): RoleSlotModifier, int, RoleSlot, bool, int, Dictionary, int, RoleDataContainer (+15 more)

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

### Community 273 - "VideoCadence"
Cohesion: 0.22
Nodes (7): DirectBroadcastPhase, DirectBroadcastSnapshot, Duration, Instant, Option, video_cadence_keeps_advancing_when_the_game_stops_supplying_frames(), VideoCadence

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

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

### Community 284 - ".default"
Cohesion: 0.16
Nodes (21): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), game_config_save_is_atomic_validated_and_round_trips(), load_content_catalog() (+13 more)

### Community 285 - "UserInterface_TownGoal"
Cohesion: 0.19
Nodes (7): Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI, Transform, UserInterface_TownGoal

### Community 289 - "UserInterface_Event"
Cohesion: 0.24
Nodes (6): OnMessageReceivedArgs, EventCommands, GameObject, Slider, TextMeshProUGUI, UserInterface_Event

### Community 290 - ".new"
Cohesion: 0.27
Nodes (7): apply_direct_broadcast_control(), capture_process_audio(), configure_direct_broadcast(), direct_broadcast_stays_offline_until_operator_requests_it(), operator_stop_cancels_an_in_flight_session_without_restarting_it(), Self, SyncSender

### Community 291 - "Processors"
Cohesion: 0.05
Nodes (13): GridProcessorEditor, InputButton, UserInterface.MainMenu, Processors, World, MetaData, GameEventSystem, Audio (+5 more)

### Community 292 - "ResourceProcessor"
Cohesion: 0.15
Nodes (3): Container, ContainerBuilder, ResourceProcessor

### Community 293 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "Utils"
Cohesion: 0.06
Nodes (14): DisableOnAwake, SelectionBase, SimpleHideRendererOnAwake, GameObject, SimpleRandomModelEnabled, Utils, Animation, Core (+6 more)

### Community 296 - "settings.rs"
Cohesion: 0.09
Nodes (34): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+26 more)

### Community 297 - "direct_broadcast.rs"
Cohesion: 0.16
Nodes (15): bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), DirectTwitchBroadcastPlugin, draw_centered_sensitive_label(), ingest(), ingest_selection_prefers_default_or_named_region(), resolve_broadcast_target(), App (+7 more)

### Community 298 - "PlayerInputRuntimeData"
Cohesion: 0.33
Nodes (6): bool, Dictionary, InputButton, PlayerInput, Vector2, PlayerInputRuntimeData

### Community 299 - "DontDestroyOnLoad"
Cohesion: 0.18
Nodes (5): DontDestroyOnLoad, Camera, ContainerBuilder, GameObject, ProjectCameraInstaller

### Community 300 - "Season"
Cohesion: 0.22
Nodes (7): Color, float, int, VisualEffect, SeasonDataSettings, Season, Gradient

### Community 301 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 302 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 303 - ".ValidateTokenAsync"
Cohesion: 0.33
Nodes (6): CancellationToken, Dictionary, Task, UnityWebRequest, TokenValidationResponse, WebResponse

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.04
Nodes (21): Player, Dictionary, GameObject, Vector3, Vector3, Action, Container, ContainerBuilder (+13 more)

### Community 307 - "STSM_StateAction"
Cohesion: 0.09
Nodes (12): RotationHandler, float, Quaternion, Vector3, int, STSM_Helper_Attack, int, STSM_Action_Attack (+4 more)

### Community 309 - "SimpleRotateOnAxis"
Cohesion: 0.19
Nodes (5): List, SimpleEventOnStart, float, Vector3, SimpleRotateOnAxis

### Community 310 - "CampGenerationSettings"
Cohesion: 0.29
Nodes (5): float, int, string, Vector2, CampGenerationSettings

### Community 311 - "Buildings"
Cohesion: 0.07
Nodes (13): BuildCostModifier, SaveablFoliage, PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, Level, GridSystem.Partitioning (+5 more)

### Community 312 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 314 - "ResourceGenerationSettings"
Cohesion: 0.33
Nodes (5): AnimationCurve, bool, int, List, ResourceGenerationSettings

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 319 - "WorldGenSaveData"
Cohesion: 0.15
Nodes (12): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, List, SaveGameData (+4 more)

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

### Community 322 - "animate_loading_icon"
Cohesion: 0.67
Nodes (4): animate_loading_icon(), apply_loading_icon_rotation(), LoadingIconSpinner, UiTransform

## Knowledge Gaps
- **363 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+358 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **27 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `UserInterface`, `ScriptablesProcessorInfrastructure`, `CommonEnums.cs`, `StringUtils`, `BuildingScriptablesEditor.cs`, `EnemySpawner`, `ObjectiveSaveData`, `SimpleDisableAfterTime`, `LabelDisplayProcessor`, `Sensors`, `.CreateEnumField`, `MeshData`, `Processors`, `.SetTargetType`, `UpdateGraphBounds`, `RandomEnabler`, `World.Generation`, `SimpleRotateOnAxis`, `Buildings`, `SimpleScreenShot`, `BuildingPlacer`, `Easings`, `FPSDisplay`, `Station`, `SaveProcessor.cs`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `PlayerRole`, `IProcessor`, `BuildingProcessor`, `EnemySpawner`, `TwitchChatProcessor`, `BottomBarInterface`, `RaidEvent`, `WorldGenProcessor`, `UserInterface_Debug`, `UserInterface_Event`, `TechTreeProcessor`, `SelectedPlayerGroup`, `RoleHandler`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `NewKingVote`, `Buildings`, `PlayerRoleData`, `Enemy`, `FishGodEvent`, `TimeProcessor`, `ObjectPoolingProcessor`, `UserInterface_TownVote`, `BuildingPlacer`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `Res`, `Option`, `stream_town_game/src/lib.rs`, `String`, `setup_rendering`, `stream_town_migrate/src/presentation.rs`, `save.rs`, `ContentCatalog`, `twitch.rs`, `RenderAssets`, `command.rs`, `BTreeMap`, `world.rs`, `stream_town_domain/src/content.rs`, `building_node_visibility`, `TechnologyGraphViewState`, `AnimationControllerDef`, `runtime_console.rs`, `update_credits_fireworks`, `legacy.rs`, `stream_town_migrate/src/menu_scene.rs`, `Result`, `stream_town_migrate/src/content.rs`, `String`, `GameConfig`, `convert_post_process`, `stream_town_domain/src/presentation.rs`, `stream_town_tools/src/main.rs`, `Vec`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _363 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `UserInterface` be split into smaller, more focused modules?**
  _Cohesion score 0.04406779661016949 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.05285592497868713 - nodes in this community are weakly interconnected._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07847082494969819 - nodes in this community are weakly interconnected._