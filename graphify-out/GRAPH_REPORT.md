# Graph Report - Stream-Town-Bevy  (2026-08-27)

## Corpus Check
- 670 files · ~1,773,923 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8905 nodes · 25976 edges · 306 communities (277 shown, 29 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1040 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `47c5c870`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UserInterface
- BuildingProcessor
- ScriptablesProcessorInfrastructure
- SaveDataMapper
- Res
- setup_rendering
- TwitchChatProcessor
- String
- BottomBarInterface
- stream_town_game/src/lib.rs
- SettingsProcessor
- UserInterface_Debug
- stream_town_migrate/src/presentation.rs
- RoleProcessor
- TechTreeIOUtility
- HealthHandler
- CellSpacePartitioning
- save.rs
- ContentCatalog
- command.rs
- twitch.rs
- .EnsureValidCredentials
- STSM_Idle_Player
- Option
- StationSensor
- PlayerSettings
- BinaryReader
- StableId
- SaveFileData
- world.rs
- ResMut
- WorldGenProcessor
- .Log
- stream_town_domain/src/content.rs
- GenerationSettings
- .SetTargetType
- BinarySaveCodec
- SettingsData
- SeasonProcessor
- TechnologyGraphViewState
- IRuntimeDataScriptable
- TechTreeProcessor
- CharacterModelHandler
- Dictionary
- AnimationControllerDef
- runtime_console.rs
- legacy.rs
- SelectedPlayer
- ResourceDataSaveData
- UserInterface_Roles
- AudioHandler
- StreamTownSessionBridge
- Pet
- TwitchClientProcessor
- UIProcessor
- .SerializeComponent
- NavGrid
- .RecalculateStats
- World.Generation.Settings
- TechTreeEditorWindow
- BuildingBase
- TransformSaveData
- CameraController
- TargetSensor
- Access_Slider
- GraphicsProcessor
- SerializableDictionary
- ResourceData
- Objective
- save_secrets_fields
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- ResourceInventory
- UserInterface_ObjectSelection
- PoolableObject
- UserInterface_TownVote
- TechTreeSearchWindow
- TwitchBotSetupWindow
- BuildingPlacer
- WorldUtils
- TwitchUser
- RenderAssets
- Access_Text
- SelectedBuilding
- sync_stream_only_capture
- .GetMissingDataScriptableDependencies
- .StartupSequence
- FoliageProcessor
- tools_ui
- RoleSlot
- config.rs
- MonoBehaviour
- convert_fbx_to_glb.py
- STSM_Helper_Build
- Resource
- .LoadGameAsync
- ResourceTarget
- stream_town_domain/src/presentation.rs
- .Draw
- StateMachine
- ResourceRuntimeData
- SaveProcessor
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
- Character
- ToolState
- GridNode
- TL_Secrets
- GameEventProcessor
- Vec
- VoteEvent
- unity_color_filter
- ResourceHolder
- SnapToGridMouseMovement
- AIPath
- .UserIsSubscribed
- GateController
- DirectBroadcastRuntime
- stream_town_domain/src/lib.rs
- AnimationHandler
- encode_broadcast_session
- StringUtils
- ChanceObjectList
- UnitTextDisplay
- TechTree.Elements
- Coordinator
- Enemy
- Option
- UnitHealthBar
- Result
- What You Must Do When Invoked
- RuntimeData Template
- World.Generation
- RuntimeData Template
- Key Rules
- TechTreeNode
- FrameCapture
- xtask/src/lib.rs
- Targetable
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- Sensors
- .ResolveSaveProcessor
- String
- .CreateEnumField
- Stream Town Reloaded - Architecture Documentation
- .Update
- TechnologyTreeGroup
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- BuildingDamageMaterialHandler
- BroadcastTarget
- UpdateGraphBounds
- UserInterface_DisplayUsernames
- UserInterface_GameMenu
- .RenderResourceType
- Q: There are still no animations.
- xtask/src/main.rs
- List
- RoleSlotModifier
- RandomEnabler
- BuildingModelHandler
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- ParallelProgressReporter
- BuildingScriptablesEditor.cs
- CommonEnums.cs
- Access_Toggle
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- CombatVisualKind
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- upgrade_building_instance
- PlayerSpawnPoint
- WorldGenerationReferenceExporter
- ScriptableObject
- Access_Dropdown
- BevyMigrationExporter
- SimpleRotateOnAxis
- stream_town_migrate/src/menu_scene.rs
- UserInterface_BuildingHealthBar
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- ObjectSelectionProcessor.Editor.cs
- Target
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
- ObjectPoolingProcessor
- Requirement
- .GetCompatiblePorts
- SensorProcessor
- SimpleEventOnStart
- Key Rules
- FoliageGenerationSettings
- RuntimeData Template
- Character Animation Regression Checklist
- SelectedObject
- ScriptKeywordProcessor
- FPSDisplay
- SimpleRandomModelEnabled
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
- animate_loading_icon
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- .OnStopped
- Q: What reachable Unity custom shaders and VFX still lack Bevy WGSL/ECS parity according to current code/status, and which has complete authored assets and runtime state for next bounded milestone?
- AGENTS.md
- Q: How does native actor restoration preserve completed-gate semantics and authoritative positions?
- Q: How are shipping visual and audio parity implemented in the Bevy migration?
- Q: Why would retargeted Unity character animation curves rotate the entire Bevy character model instead of deforming individual limbs, and where is native clip selection decided?
- Q: NativeSaveStore validate_snapshot load_input building enemy camp semantic validation destructive despawn partial load corruption
- CustomLogger
- ProcessorStartupReport
- extraction-spec.md
- TwitchClientRuntimeData
- Q: How does the Bevy runtime preserve Unity world-space target range semantics?
- Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?
- RoleHandler
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
- SimpleHideRendererOnAwake
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- stream_user_color
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- UIElementWrapper
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- VfxAnimationController
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- stream_town_tools/src/main.rs
- main
- StreamTown.Migration
- PlayerControls
- record_gpu_readiness
- CreateProjectScopeProcessors.cs
- Utils
- direct_broadcast.rs
- BuildingDataSettings
- TL_API
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- SimpleScreenShot
- BuildPlacerData
- UnityGraphics
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- .ExportModification
- VideoCadence
- vcpkg.json
- FFmpeg runtime and relinking

## God Nodes (most connected - your core abstractions)
1. `StableId` - 360 edges
2. `WorldSimulation` - 166 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ContentCatalog` - 151 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 134 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `handle_twitch_event()` --calls--> `unity_command_usage()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/command.rs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `combat_goal_damages_kills_and_respawns()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `convert()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/legacy.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `agent_facing_matches_unity_rotation_and_action_targets()` --calls--> `generate_world_with_content()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (306 total, 29 thin omitted)

### Community 0 - "UserInterface"
Cohesion: 0.06
Nodes (17): InputButton, SharedTypes, Slider, TextMeshProUGUI, UI_Objective, TextMeshProUGUI, UI_VoteObjectiveRow, Image (+9 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (15): bool, Dictionary, int, BuildingSettings, Container, ContainerBuilder, Dictionary, List (+7 more)

### Community 2 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.08
Nodes (3): Reflex.Core, Data.Containers, ScriptablesProcessorInfrastructure

### Community 3 - "SaveDataMapper"
Cohesion: 0.08
Nodes (15): List, Component, Mesh, Transform, Vector3, SaveDataMapper, bool, int (+7 more)

### Community 4 - "Res"
Cohesion: 0.04
Nodes (224): Aabb, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, ActivePetVisual (+216 more)

### Community 5 - "setup_rendering"
Cohesion: 0.04
Nodes (87): AmbientLight, ActiveMaterialHandles, authored_post_process_stack(), building_material(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstances, character_material() (+79 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.05
Nodes (26): MetaData, float, Func, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string, TwitchChatRuntimeData (+18 more)

### Community 7 - "String"
Cohesion: 0.03
Nodes (104): AccessibilityNode, AccessibleNode, AssetId, accessibility_settings_selection(), active_event_text(), advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover() (+96 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (322): generate_world(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_navigation_preserves_editable_text_focus(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, action_ranges_and_tower_acquisition_are_euclidean() (+314 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.06
Nodes (12): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+4 more)

### Community 11 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 12 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.06
Nodes (97): AnimationFloatKeyframe, AnimationTangent, animation_state_id(), animation_state_machine_id(), append_vec3_keys(), avatar_mask_id(), clip_id(), controller_id() (+89 more)

### Community 13 - "RoleProcessor"
Cohesion: 0.10
Nodes (14): Dictionary, int, RoleDataContainer, Container, ContainerBuilder, int, RoleProcessor, AllRoleDataSettings (+6 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (18): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+10 more)

### Community 15 - "HealthHandler"
Cohesion: 0.12
Nodes (8): Func, STSM_Action_Heal, Action, bool, float, int, UnityEvent, HealthHandler

### Community 16 - "CellSpacePartitioning"
Cohesion: 0.11
Nodes (14): Bounds, CellPartitioningEditor, bool, List, Vector2, BSPCell, Dictionary, float (+6 more)

### Community 17 - "save.rs"
Cohesion: 0.08
Nodes (63): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+55 more)

### Community 18 - "ContentCatalog"
Cohesion: 0.06
Nodes (116): GameConfig, ContentCatalog, GridPos, ActorState, EnemyCampState, String, GeneratedFoliage, GeneratedWorld (+108 more)

### Community 19 - "command.rs"
Cohesion: 0.19
Nodes (29): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+21 more)

### Community 20 - "twitch.rs"
Cohesion: 0.07
Nodes (50): bot_and_broadcaster_tokens_use_distinct_vault_entries(), broadcaster_oauth_uses_only_the_stream_key_scope(), channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), IngestsResponse (+42 more)

### Community 21 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 22 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (21): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+13 more)

### Community 23 - "Option"
Cohesion: 0.03
Nodes (185): PresentationCatalog, actor_detail_budget(), actor_material(), actor_scene_budget(), advance_falling_fish(), animation_detail_budget(), animation_property_value(), animation_root_name() (+177 more)

### Community 24 - "StationSensor"
Cohesion: 0.08
Nodes (12): List, float, List, SensorRuntimeData, Dictionary, List, Queue, StationRuntimeData (+4 more)

### Community 25 - "PlayerSettings"
Cohesion: 0.06
Nodes (62): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+54 more)

### Community 26 - "BinaryReader"
Cohesion: 0.16
Nodes (4): CancellationToken, Func, List, BinaryReader

### Community 27 - "StableId"
Cohesion: 0.05
Nodes (65): GameplayConfig, BTreeMap, ObjectiveDef, ObjectiveKind, Display, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity() (+57 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (21): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+13 more)

### Community 29 - "world.rs"
Cohesion: 0.09
Nodes (57): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, avalanche_instance_hash(), cell_hash() (+49 more)

### Community 30 - "ResMut"
Cohesion: 0.03
Nodes (169): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AppExit, accessibility_button_enabled(), accessibility_input(), accessibility_scope_active(), accessibility_should_clear_focus(), AccessibilityAnnouncement (+161 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.06
Nodes (24): HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func, GameObject (+16 more)

### Community 32 - ".Log"
Cohesion: 0.09
Nodes (10): Action, ProcessorStartupContext, HideInCallstack, Object, DebugLogCategory, Action, CancellationToken, LoadSceneMode (+2 more)

### Community 33 - "stream_town_domain/src/content.rs"
Cohesion: 0.08
Nodes (52): ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef (+44 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.06
Nodes (35): Action, IEnumerator, Vector2, Noise, float, int, string, Vector2 (+27 more)

### Community 35 - ".SetTargetType"
Cohesion: 0.11
Nodes (4): Projectile, TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 36 - "BinarySaveCodec"
Cohesion: 0.17
Nodes (5): Action, int, UTF8Encoding, BinarySaveCodec, BinaryWriter

### Community 37 - "SettingsData"
Cohesion: 0.07
Nodes (17): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+9 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.05
Nodes (27): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+19 more)

### Community 39 - "TechnologyGraphViewState"
Cohesion: 0.07
Nodes (56): TechTree, automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphPoint, GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), BTreeMap (+48 more)

### Community 40 - "IRuntimeDataScriptable"
Cohesion: 0.04
Nodes (37): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+29 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.04
Nodes (33): bool, Dictionary, float, int, TechTreeRuntimeData, NodeUnlockData, List, Node_SO (+25 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "Dictionary"
Cohesion: 0.32
Nodes (4): Dictionary, materialIndex, Matrix4x4, meshIndex

### Community 44 - "AnimationControllerDef"
Cohesion: 0.10
Nodes (33): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+25 more)

### Community 45 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 46 - "legacy.rs"
Cohesion: 0.07
Nodes (85): ActorCustomization, StreamUserType, absolute_path(), actor_prefix(), backup_candidate(), binary_fixture(), binary_schemas_one_through_three_decode_and_validate_trailer(), BinaryParser (+77 more)

### Community 47 - "SelectedPlayer"
Cohesion: 0.09
Nodes (4): List, SelectedPlayer, List, SelectedPlayerGroup

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.13
Nodes (16): bool, float, int, List, string, uint, ResourceDataSaveData, ResourceGroupSaveData (+8 more)

### Community 49 - "UserInterface_Roles"
Cohesion: 0.14
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (14): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+6 more)

### Community 52 - "Pet"
Cohesion: 0.12
Nodes (9): PetType, bool, Dictionary, float, Transform, Pet, Animator, int (+1 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.12
Nodes (9): Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs, OnMessageReceivedArgs, TwitchClientProcessor (+1 more)

### Community 54 - "UIProcessor"
Cohesion: 0.10
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - ".SerializeComponent"
Cohesion: 0.13
Nodes (11): Bounds, Color, Component, Object, Quaternion, Rect, SerializedProperty, Vector2 (+3 more)

### Community 56 - "NavGrid"
Cohesion: 0.11
Nodes (25): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), manhattan(), NavGrid, NavigationError, OpenNode (+17 more)

### Community 57 - ".RecalculateStats"
Cohesion: 0.17
Nodes (8): StatModifiers, Dictionary, Dictionary, List, Queue, Transform, PlayerRuntimeData, StatType

### Community 58 - "World.Generation.Settings"
Cohesion: 0.05
Nodes (30): CampGenerationSettings, List, CampGenerationSettingsContainer, FoliageGenerationSettings, List, FoliageGenerationSettingsContainer, List, ResourceGenerationSettings (+22 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.10
Nodes (8): TechTreeGraphView, bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "BuildingBase"
Cohesion: 0.09
Nodes (12): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, Dictionary (+4 more)

### Community 61 - "TransformSaveData"
Cohesion: 0.10
Nodes (18): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+10 more)

### Community 62 - "CameraController"
Cohesion: 0.07
Nodes (16): bool, Camera, float, IEnumerator, int, PlayerInput, Transform, Vector2 (+8 more)

### Community 63 - "TargetSensor"
Cohesion: 0.13
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (15): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+7 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.12
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "ResourceData"
Cohesion: 0.14
Nodes (9): Dictionary, materialIndex, meshIndex, bool, int, Matrix4x4, uint, Vector3 (+1 more)

### Community 68 - "Objective"
Cohesion: 0.07
Nodes (13): int, string, ObjectiveSaveData, Action, int, Objective, Dictionary, GameObject (+5 more)

### Community 69 - "save_secrets_fields"
Cohesion: 0.21
Nodes (15): AnyResult, FoliageAcceptanceCapture, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), open_twitch_verification_uri(), open_twitch_verification_uri_with(), player_settings_path() (+7 more)

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

### Community 74 - "ResourceInventory"
Cohesion: 0.15
Nodes (10): ResourceInventory, bool, int, Dictionary, bool, int, List, string (+2 more)

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "PoolableObject"
Cohesion: 0.07
Nodes (23): BoxCollider, List, Quaternion, Vector3, bool, Dictionary, GUIDRuntimeData, Dictionary (+15 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 78 - "TechTreeSearchWindow"
Cohesion: 0.16
Nodes (9): GroupSaveData, Vector2, List, Texture2D, TechTreeSearchWindow, IManipulator, ISearchWindowProvider, SearchTreeEntry (+1 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "BuildingPlacer"
Cohesion: 0.07
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 81 - "WorldUtils"
Cohesion: 0.16
Nodes (9): PlacementProbe, float, PlacementProbeHandler, SurfaceType, GameObject, LayerMask, Transform, Vector3 (+1 more)

### Community 82 - "TwitchUser"
Cohesion: 0.16
Nodes (10): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+2 more)

### Community 83 - "RenderAssets"
Cohesion: 0.04
Nodes (109): BackgroundColor, AccessibilityMotionDefaults, apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), BoundsMaterialExtension, BoundsMaterialUniform (+101 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 86 - "sync_stream_only_capture"
Cohesion: 0.18
Nodes (16): camera_targets_primary_window(), Assets, Commands, Entity, Handle, HashMap, Image, PrimaryWindow (+8 more)

### Community 87 - ".GetMissingDataScriptableDependencies"
Cohesion: 0.22
Nodes (3): Container, IEnumerable, Type

### Community 88 - ".StartupSequence"
Cohesion: 0.22
Nodes (3): IEnumerator, LoadSceneMode, Scene

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "tools_ui"
Cohesion: 0.15
Nodes (26): authority_tab(), content_tab(), draw_world_preview(), inspector_tab(), migration_tab(), poll_tool_job_events(), preview_grid_point(), preview_lerp_color() (+18 more)

### Community 91 - "RoleSlot"
Cohesion: 0.18
Nodes (7): RoleSlot, bool, int, bool, Dictionary, UnityEvent, RoleRuntimeData

### Community 92 - "config.rs"
Cohesion: 0.11
Nodes (27): broadcast_render_mode_default(), BroadcastRenderMode, ConfigError, default_configuration_is_valid_and_round_trips_ron(), direct_broadcast_settings_are_strictly_validated(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), BTreeSet (+19 more)

### Community 93 - "MonoBehaviour"
Cohesion: 0.01
Nodes (119): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, ContainerBuilder, InstantiationBarrier, ContainerBuilder, MetaDataInstaller, PersistentScoped (+111 more)

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 95 - "STSM_Helper_Build"
Cohesion: 0.07
Nodes (11): AttackUnit, int, STSM_Helper_Attack, int, STSM_Helper_Build, StateMachine, string, STSM_HelperBase (+3 more)

### Community 96 - "Resource"
Cohesion: 0.03
Nodes (39): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, PlayerInventory, Dictionary (+31 more)

### Community 97 - ".LoadGameAsync"
Cohesion: 0.10
Nodes (21): Action, CancellationToken, List, Task, int, string, uint, EnemySaveData (+13 more)

### Community 98 - "ResourceTarget"
Cohesion: 0.24
Nodes (6): float, int, Resource, uint, Vector3, ResourceTarget

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (68): AnimationEventDef, AnimationLayerBlendMode, AnimationLayerDef, AnimationMotionDef, AnimationObjectReference, AnimationPropertyCurve, AnimationQuatKeyframe, AnimationStateDef (+60 more)

### Community 100 - ".Draw"
Cohesion: 0.14
Nodes (15): Port, Action, Button, Foldout, Port, TextField, Toggle, TechTreeUtilities (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (12): PlayerDeathHandler, bool, float, Vector3, bool, List, string, uint (+4 more)

### Community 102 - "ResourceRuntimeData"
Cohesion: 0.32
Nodes (11): Dictionary, float, List, Material, materialIndex, materials, Matrix4x4, Mesh (+3 more)

### Community 103 - "SaveProcessor"
Cohesion: 0.06
Nodes (17): Container, ContainerBuilder, GUIDProcessor, Container, ContainerBuilder, float, Material, materials (+9 more)

### Community 104 - "MainMenuManager"
Cohesion: 0.13
Nodes (10): LoadType, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator, int (+2 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.15
Nodes (3): Container, ContainerBuilder, ResourceProcessor

### Community 106 - "LoadingManager"
Cohesion: 0.05
Nodes (22): Container, ContainerBuilder, CreditsProcessor, Dictionary, float, GameObject, Image, string (+14 more)

### Community 107 - "Station"
Cohesion: 0.08
Nodes (14): Station, Dictionary, float, int, List, Queue, Transform, Vector3 (+6 more)

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
Cohesion: 0.06
Nodes (28): DontDestroyOnLoad, AudioClip, AudioSource, bool, Dictionary, float, IEnumerator, string (+20 more)

### Community 112 - "DayAndNightProcessor"
Cohesion: 0.15
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "Character"
Cohesion: 0.09
Nodes (12): Pets.Enumerations, TownGoal, Core, Pets, GameEventSystem, GameEventSystem.Events, SavingAndLoading, Twitch.Commands (+4 more)

### Community 115 - "ToolState"
Cohesion: 0.12
Nodes (41): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), AuthoringSnapshot, commit_catalog_candidate(), create_technology_group(), create_technology_node() (+33 more)

### Community 116 - "GridNode"
Cohesion: 0.10
Nodes (13): GridProcessorEditor, int, List, Vector2, GridNode, Color, CollisionColours, CollisionType (+5 more)

### Community 117 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 118 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 119 - "Vec"
Cohesion: 0.04
Nodes (91): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+83 more)

### Community 120 - "VoteEvent"
Cohesion: 0.06
Nodes (23): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, TechVote (+15 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "ResourceHolder"
Cohesion: 0.09
Nodes (16): AnimationCurve, bool, int, object, ResourceHolder, uint, GUIDComponent, SaveableBuilding (+8 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.09
Nodes (23): Action, bool, float, int, string, Type, Vector3, AIPath (+15 more)

### Community 126 - ".UserIsSubscribed"
Cohesion: 0.15
Nodes (6): OnCommunitySubscriptionArgs, OnContinuedGiftedSubscriptionArgs, OnGiftedSubscriptionArgs, OnNewSubscriberArgs, OnPrimePaidSubscriberArgs, OnReSubscriberArgs

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, Collider, int, List, Rigidbody

### Community 128 - "DirectBroadcastRuntime"
Cohesion: 0.08
Nodes (30): apply_direct_broadcast_control(), BroadcastMetricsSnapshot, capture_direct_broadcast_frame(), configure_direct_broadcast(), DirectBroadcastControl, DirectBroadcastPhase, DirectBroadcastRuntime, DirectBroadcastSnapshot (+22 more)

### Community 129 - "stream_town_domain/src/lib.rs"
Cohesion: 0.17
Nodes (8): round_trips_through_serde(), Err, Formatter, Into, Result, Self, String, StableIdError

### Community 130 - "AnimationHandler"
Cohesion: 0.04
Nodes (30): AnimationHandler, Animator, bool, Dictionary, float, int, RotationHandler, float (+22 more)

### Community 131 - "encode_broadcast_session"
Cohesion: 0.21
Nodes (27): AtomicBool, AudioFrame, AudioInput, BroadcastController, BroadcastMetrics, capture_process_audio(), discard_pending_audio(), encode_broadcast_session() (+19 more)

### Community 133 - "ChanceObjectList"
Cohesion: 0.27
Nodes (5): float, ChanceObject, float, List, ChanceObjectList

### Community 134 - "UnitTextDisplay"
Cohesion: 0.10
Nodes (11): Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color, float, string, UnitTextDisplay (+3 more)

### Community 135 - "TechTree.Elements"
Cohesion: 0.06
Nodes (24): int, ChangeTimeStamp, Color, ErrorData, List, GroupErrorData, List, NodeErrorData (+16 more)

### Community 136 - "Coordinator"
Cohesion: 0.12
Nodes (13): Coordinator, StartupState, Action, bool, CancellationTokenSource, Dictionary, GameObject, int (+5 more)

### Community 137 - "Enemy"
Cohesion: 0.04
Nodes (30): Transform, Action, float, Enemy, float, int, List, Transform (+22 more)

### Community 138 - "Option"
Cohesion: 0.09
Nodes (42): AnimationClipDef, animation_take_name(), animator_component(), animator_reference_path(), color_value(), convert_clips(), convert_embedded_model_clips(), convert_post_process() (+34 more)

### Community 139 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 140 - "Result"
Cohesion: 0.10
Nodes (29): BroadcastConfig, BroadcastEncoderPreference, BroadcastEncoder, configured_1080p60_encoder_sustains_realtime_output(), controller_counts_replaced_video_without_rejecting_the_newest_frame(), copy_packed_video_frame(), direct_broadcast_stays_offline_until_operator_requests_it(), encoder_candidates() (+21 more)

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "TechTreeNode"
Cohesion: 0.09
Nodes (13): Color, Foldout, List, Sprite, Vector2, TechTreeNode, Group, int (+5 more)

### Community 147 - "FrameCapture"
Cohesion: 0.22
Nodes (10): bool, double, float, int, IReadOnlyList, List, long, string (+2 more)

### Community 148 - "xtask/src/lib.rs"
Cohesion: 0.27
Nodes (15): add_file(), add_tree(), ffmpeg_link_metadata_is_stale(), package_windows(), PackageReport, portable_path(), Path, PathBuf (+7 more)

### Community 149 - "Targetable"
Cohesion: 0.10
Nodes (11): List, Dictionary, List, TargetRuntimeData, bool, BoxCollider, float, int (+3 more)

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
Cohesion: 0.08
Nodes (12): HealthModifier, bool, float, GameObject, HealUnit, STStateMachine.States, Behaviours, Animation (+4 more)

### Community 155 - "String"
Cohesion: 0.10
Nodes (52): MaterialDef, PrefabPresentationBinding, TextureDef, array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), convert(), convert_avatar_masks() (+44 more)

### Community 156 - ".CreateEnumField"
Cohesion: 0.10
Nodes (18): ChildrenSaveData, List, Vector2, NodeSaveData, NodeUnlockSaveData, Button, EnumField, ObjectiveVisualElement (+10 more)

### Community 157 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.13
Nodes (14): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+6 more)

### Community 158 - ".Update"
Cohesion: 0.55
Nodes (5): List, Material, materials, Mesh, meshes

### Community 159 - "TechnologyTreeGroup"
Cohesion: 0.25
Nodes (5): Color, float, string, TechnologyTreeGroup, Group

### Community 160 - "CreateDefaultSettingsAssets"
Cohesion: 0.29
Nodes (5): Dictionary, MenuItem, ScriptableObject, Type, CreateDefaultSettingsAssets

### Community 161 - "ReadOnlyDrawer"
Cohesion: 0.22
Nodes (8): ReadOnlyAttribute, ReadOnlyDrawer, Rect, SerializedProperty, Attributes, GUIContent, PropertyAttribute, PropertyDrawer

### Community 162 - "BuildingDamageMaterialHandler"
Cohesion: 0.29
Nodes (5): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, MaterialPropertyBlock

### Community 163 - "BroadcastTarget"
Cohesion: 0.27
Nodes (8): AuthorizationEvent, BroadcastPrerequisites, BroadcastTarget, Debug, Display, Formatter, String, VideoEncoderSelection

### Community 164 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 165 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 166 - "UserInterface_GameMenu"
Cohesion: 0.16
Nodes (3): GameObject, UserInterface_GameMenu, SettingsPanel

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
Cohesion: 0.18
Nodes (4): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 178 - "CommonEnums.cs"
Cohesion: 0.18
Nodes (10): Foliage, FoliageSaveType, FoliageType, PLayerActivityStatus, ResourceType, SaveItem, Seasons, TimeOfDay (+2 more)

### Community 179 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "CombatVisualKind"
Cohesion: 0.47
Nodes (6): ActionPresentation, actor_combat_visual(), CombatProjectile, CombatVisualKind, ProjectileSource, ProjectileSpawn

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "upgrade_building_instance"
Cohesion: 0.40
Nodes (6): building_construction_cost(), building_cost_reduction_percent(), building_upgrade_cost(), maximum_building_level(), percentage_reduced(), upgrade_building_instance()

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "ScriptableObject"
Cohesion: 0.03
Nodes (78): int, AudioSettings, List, CampGenSettings, float, Material, Volume, DayAndNightSettings (+70 more)

### Community 188 - "Access_Dropdown"
Cohesion: 0.05
Nodes (24): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+16 more)

### Community 189 - "BevyMigrationExporter"
Cohesion: 0.22
Nodes (13): bool, HashSet, int, long, string, BevyMigrationExporter, NeutralComponent, NeutralExport (+5 more)

### Community 190 - "SimpleRotateOnAxis"
Cohesion: 0.40
Nodes (3): float, Vector3, SimpleRotateOnAxis

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "UserInterface_BuildingHealthBar"
Cohesion: 0.25
Nodes (5): bool, GameObject, Slider, BuildingHealthDisplayOption, UserInterface_BuildingHealthBar

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 195 - "Target"
Cohesion: 0.09
Nodes (9): PlayerControls.ObjectSelection, Units, Target, Utils.Pooling, GridSystem.Partitioning, Combat, SavingAndLoading.SavableObjects, Enemies (+1 more)

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
Cohesion: 0.16
Nodes (7): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, DirectoryInfo

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

### Community 208 - ".InitializeAndActivateProcessorsAsync"
Cohesion: 0.18
Nodes (4): CancellationToken, Task, CancellationToken, Task

### Community 209 - "ObjectPoolingProcessor"
Cohesion: 0.03
Nodes (42): CollectResource, Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, DebugSettings, bool, List (+34 more)

### Community 210 - "Requirement"
Cohesion: 0.25
Nodes (6): RequirementType, object, Requirement, List, RequirementsData, Requirements

### Community 211 - ".GetCompatiblePorts"
Cohesion: 0.50
Nodes (3): List, Port, NodeAdapter

### Community 212 - "SensorProcessor"
Cohesion: 0.31
Nodes (3): Container, ContainerBuilder, SensorProcessor

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 215 - "FoliageGenerationSettings"
Cohesion: 0.14
Nodes (12): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings, int (+4 more)

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.25
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

### Community 218 - "SelectedObject"
Cohesion: 0.10
Nodes (5): SelectedEnemyCamp, object, UnityAction, SelectedObject, SelectedResource

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

### Community 234 - "animate_loading_icon"
Cohesion: 0.67
Nodes (4): animate_loading_icon(), apply_loading_icon_rotation(), LoadingIconSpinner, UiTransform

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

### Community 251 - "ProcessorStartupReport"
Cohesion: 0.67
Nodes (3): Exception, ProcessorStartupReport, ProcessorStartupStage

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "RoleHandler"
Cohesion: 0.06
Nodes (21): PlayerRoleData, AudioClip, bool, float, int, RoleData, AudioClip, bool (+13 more)

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
Cohesion: 0.12
Nodes (6): BuildingPlacerEditor, BuildingResourceModelHandlerEditor, WindControllerEditor, GridSystemEditor, Globals, Editor

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

### Community 273 - "stream_user_color"
Cohesion: 0.67
Nodes (3): building_health_color(), Color, stream_user_color()

### Community 275 - "Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?, Source Nodes

### Community 276 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

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

### Community 284 - "stream_town_tools/src/main.rs"
Cohesion: 0.09
Nodes (53): broadcast_encoder_label(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), format_game_master_ids(), format_runtime_frame_times() (+45 more)

### Community 291 - "PlayerControls"
Cohesion: 0.12
Nodes (5): InputButton, UserInterface.MainMenu, MetaData, Settings, PlayerControls

### Community 293 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 295 - "Utils"
Cohesion: 0.05
Nodes (11): BuildCostModifier, DisableOnAwake, SelectionBase, Utils, Processors, World, Level, Buildings (+3 more)

### Community 297 - "direct_broadcast.rs"
Cohesion: 0.10
Nodes (24): average_milliseconds(), bandwidth_test_url_is_constructed_without_logging_the_key(), build_ingest_url(), DirectTwitchBroadcastPlugin, draw_centered_sensitive_label(), gpu_readback_padding_is_removed_without_corrupting_rows(), ingest(), ingest_selection_prefers_default_or_named_region() (+16 more)

### Community 301 - "BuildingDataSettings"
Cohesion: 0.18
Nodes (9): Dictionary, BuildingDataContainer, int, ResourceCostData, bool, float, Sprite, string (+1 more)

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.04
Nodes (26): Player, Dictionary, GameObject, Vector3, Vector3, Action, Container, ContainerBuilder (+18 more)

### Community 309 - "SimpleScreenShot"
Cohesion: 0.40
Nodes (3): int, string, SimpleScreenShot

### Community 311 - "BuildPlacerData"
Cohesion: 0.40
Nodes (5): BuildPlacerData, GameObject, Renderer, string, Vector2

### Community 312 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 318 - "VideoCadence"
Cohesion: 0.21
Nodes (8): CadenceTick, duration_as_micros(), Duration, Instant, video_cadence_skips_stale_slots_instead_of_bursting_after_a_stall(), VideoCadence, CapturedWindowFrame, InternalCaptureControl

### Community 320 - "vcpkg.json"
Cohesion: 0.33
Nodes (5): builtin-baseline, dependencies, name, $schema, version-string

## Knowledge Gaps
- **365 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+360 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **29 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `UserInterface`, `ScriptablesProcessorInfrastructure`, `StringUtils`, `ChanceObjectList`, `UnitTextDisplay`, `TechTree.Elements`, `SimpleDisableAfterTime`, `SimpleHideRendererOnAwake`, `World.Generation`, `Sensors`, `.CreateEnumField`, `GenerationSettings`, `PlayerControls`, `UpdateGraphBounds`, `RandomEnabler`, `BuildingScriptablesEditor.cs`, `CommonEnums.cs`, `SimpleScreenShot`, `CameraController`, `SimpleRotateOnAxis`, `Target`, `BuildingPlacer`, `SimpleEventOnStart`, `FPSDisplay`, `SimpleRandomModelEnabled`, `Character`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `Player` to `RoleHandler`, `BuildingProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `Enemy`, `UserInterface_Debug`, `RoleProcessor`, `WorldGenProcessor`, `SeasonProcessor`, `TechTreeProcessor`, `SelectedPlayer`, `UserInterface_Roles`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `.RecalculateStats`, `Target`, `UserInterface_TownVote`, `BuildingPlacer`, `ObjectPoolingProcessor`, `MonoBehaviour`, `Resource`, `SaveProcessor`, `GameEventProcessor`, `VoteEvent`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `stream_town_domain/src/lib.rs`, `Res`, `setup_rendering`, `String`, `stream_town_game/src/lib.rs`, `Option`, `stream_town_migrate/src/presentation.rs`, `save.rs`, `ContentCatalog`, `command.rs`, `twitch.rs`, `Option`, `String`, `world.rs`, `ResMut`, `stream_town_domain/src/content.rs`, `TechnologyGraphViewState`, `AnimationControllerDef`, `runtime_console.rs`, `legacy.rs`, `CombatVisualKind`, `upgrade_building_instance`, `stream_town_migrate/src/menu_scene.rs`, `stream_town_migrate/src/content.rs`, `RenderAssets`, `tools_ui`, `config.rs`, `stream_town_domain/src/presentation.rs`, `ToolState`, `Vec`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _365 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `UserInterface` be split into smaller, more focused modules?**
  _Cohesion score 0.05507246376811594 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06708595387840671 - nodes in this community are weakly interconnected._
- **Should `ScriptablesProcessorInfrastructure` be split into smaller, more focused modules?**
  _Cohesion score 0.07991718426501035 - nodes in this community are weakly interconnected._