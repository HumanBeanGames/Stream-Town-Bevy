# Graph Report - Stream-Town-Bevy  (2026-08-25)

## Corpus Check
- 662 files · ~1,714,359 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 8547 nodes · 24714 edges · 297 communities (279 shown, 18 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 1028 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `da1a4435`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ContentCatalog
- BuildingProcessor
- GeneratedWorld
- BinarySaveCodec
- ResourceGenerationSettings
- stream_town_game/src/lib.rs
- TwitchChatProcessor
- setup_rendering
- BottomBarInterface
- Option
- SettingsProcessor
- BuildingPlacer
- ScriptablesProcessorInfrastructure
- stream_town_migrate/src/presentation.rs
- TechTreeIOUtility
- HealthHandler
- stream_town_domain/src/content.rs
- save.rs
- SelectedPlayer
- finish_world_reveal
- config.rs
- .Log
- BuildingBase
- RoleProcessor
- UnitHealthBar
- command.rs
- Query
- TechTreeGraphView
- SaveFileData
- GridPos
- Vec
- WorldGenProcessor
- UserInterface_Debug
- GameEventProcessor
- GenerationSettings
- PlayerProcessor
- ObjectiveSaveData
- SettingsData
- SeasonProcessor
- String
- ObjectSelectionProcessor
- TechTreeProcessor
- CharacterModelHandler
- StableId
- AnimationControllerRuntime
- Vec3
- STSM_Idle_Player
- RoleHandler
- ResourceDataSaveData
- .Draw
- AudioHandler
- StreamTownSessionBridge
- WorldGenSaveData
- TwitchClientProcessor
- UIProcessor
- BevyMigrationExporter
- MonoBehaviour
- SelectedObject
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
- parse_controller
- stream_town_migrate/src/content.rs
- models.rs
- Tiler
- ScriptablesEditor
- UnitTravelToPosition
- UserInterface_ObjectSelection
- Station
- UserInterface_TownVote
- tools_ui
- TwitchBotSetupWindow
- UserInterface_Roles
- WorldUtils
- Node_SO
- RenderAssets
- Access_Text
- String
- .EnsureValidCredentials
- SelectableObject
- Option
- FoliageProcessor
- DayAndNightProcessor
- RaidEvent
- TechTreeNode
- PlayerCommands
- convert_fbx_to_glb.py
- SelectedResource
- Resource
- SaveProcessor
- drive_tidal_music
- stream_town_domain/src/presentation.rs
- stream_town_migrate/src/technology_layout.rs
- StateMachine
- AnimationHandler
- TownGoalProcessor
- MainMenuManager
- ResourceProcessor
- LoadingManager
- WorldInstanceDeterminism
- GlobalAudioController
- CustomLogHandler
- LevelHandler
- GamestateJukebox
- EnemySpawner
- VfxSeagullSpawner
- Enemy
- GUIDComponent
- GridProcessor
- stream_town_migrate/src/main.rs
- Goal
- MiscCommands
- VoteEvent
- unity_color_filter
- Access_Dropdown
- SnapToGridMouseMovement
- AIPath
- TechnologyGraphLayout
- TimeProcessor
- GateController
- UpdateGraphBounds
- RotationHandler
- TransformSaveData
- TL_Secrets
- StringUtils
- twitch_tab
- ToolState
- convert
- runtime_console.rs
- STSM_StateAction
- AstarPath
- WorldGenRuntimeData
- TargetProcessor
- What You Must Do When Invoked
- RuntimeData Template
- legacy.rs
- RuntimeData Template
- Key Rules
- EnemyModelHandler
- Pet
- add_file
- CellSpacePartitioning
- ConfirmCheck
- MainMenuReferenceExporter
- Stream Town Reloaded - Architecture Documentation
- DayAndNightSettings
- load_player_settings
- Result
- LabelDisplayProcessor
- Stream Town Reloaded - Architecture Documentation
- WindController
- UserInterface_BuildingHealthBar
- CreateDefaultSettingsAssets
- ReadOnlyDrawer
- PlayerSaveData
- PlayerInputProcessor
- SeasonDataSettings
- .SetTargetType
- SimpleMusicController
- ParallelProgressReporter
- Q: There are still no animations.
- xtask/src/main.rs
- IProcessor.cs
- stream_town_tools/src/main.rs
- RandomEnabler
- UIElementWrapper
- Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?
- EditorHelpers
- Editor
- WeatherProcessor
- TechTree.Elements
- ResourceHolder
- Settings Scriptable Template
- Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?
- BuildingSettings
- Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?
- UserInterface
- Access_Toggle
- WorldGenerationReferenceExporter
- IInstaller
- ProjectCamera
- import_save
- HealthModifier
- stream_town_migrate/src/menu_scene.rs
- SavingAndLoading.Structs
- Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?
- FoliageGenerationSettings
- UserInterface_GameMenu
- Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity
- UILineRenderer
- UserInterface_DisplayUsernames
- Tree and Foliage Flicker Regression Checklist
- Bevy Migration Status
- graphify reference: extra exports and benchmark
- Key Rules
- IRuntimeDataScriptable
- Q: Why does a new Bevy town no longer spawn an enemy camp?
- Key Rules
- WorldSaveData
- Common Patterns
- StatusBar
- BuildingRuntimeData
- Requirement
- CommandDictionary
- SelectedEnemy
- GameStateRuntimeData
- Key Rules
- RuntimeData Template
- Character Animation Regression Checklist
- Easings
- ScriptKeywordProcessor
- FPSDisplay
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
- CommonEnums.cs
- Q: How does native load keep the persistent Town Hall aligned with saved state?
- Q: How do Unity materials textures renderer assignments animator controllers animation clips and runtime actor states connect, and what Bevy conversion/runtime support already exists?
- Q: Why are we vendoring Bevy Tidal and not just using the library that exists??
- Twitch setup
- Accessibility
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- EventProcessor
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
- Q: How do typed Unity resource targets, farm harvesting, shoreline fish, and native save compatibility connect in the Bevy migration?
- Coordinator
- Q: If there is more to do, keep going.
- stream_town_domain
- Q: The Bevy Tidal repo is now public, so fix the integration.
- Q: BoundsVisualizer BuildingPlacer VisualBounds collision successColor failColor prefab runtime
- Q: How does generator v3 reproduce Unity generated-resource navigation occupancy without breaking native saves?
- next_agent_goal
- Q: How does the Bevy migration preserve Unity Targetable sizes and action reach?
- Q: The characters are STILL not animated, and the trees are STILL flickering. Make individual checklists for each thing already tried that did not work and what did work, so future work does not repeat failed approaches and can narrow the successful ones.
- Q: How does Bevy new-town population now match Unity shipping startup?
- Q: Unity station TargetSensor distance range generated resource targeting parity Bevy
- Q: role level experience progression station equipment inventory skill upgrade
- music/README.md
- Stream Town external authoring suite
- record_gpu_readiness
- WorldGenerationReferenceExporter.cs
- SensorProcessor
- DebugProcessor
- UserInterface_GameMenu.cs
- RoleDataSettings
- CreateProjectScopeProcessors.cs
- settings.rs
- Utils
- VfxAnimationController
- VFXArrowPointer
- IProcessor
- ObjectSelectionProcessor.Editor.cs
- Q: Characters are still not animated. Trees still have the flickering shadows.
- Player
- UnityGraphics
- BuildPlacerData
- Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?
- Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders.
- GameConfig

## God Nodes (most connected - your core abstractions)
1. `StableId` - 363 edges
2. `WorldSimulation` - 172 edges
3. `Utils` - 159 edges
4. `Processors` - 156 edges
5. `ContentCatalog` - 151 edges
6. `ScriptablesProcessorInfrastructure` - 150 edges
7. `Player` - 142 edges
8. `RenderAssets` - 126 edges
9. `WorldGenProcessor` - 114 edges
10. `SettingsProcessor` - 107 edges

## Surprising Connections (you probably didn't know these)
- `CreditsRuntimeData` --implements--> `IRuntimeDataScriptable`  [EXTRACTED]
  Assets/Scripts/Scriptables/CreditsRuntimeData.cs → Assets/Scripts/Scriptables/IRuntimeDataScriptable.cs
- `id()` --references--> `StableId`  [EXTRACTED]
  bevy-port/crates/stream_town_domain/src/technology_layout.rs → bevy-port/crates/stream_town_domain/src/id.rs
- `main()` --calls--> `inspect_legacy_save()`  [INFERRED]
  bevy-port/crates/stream_town_migrate/src/main.rs → bevy-port/crates/stream_town_domain/src/save.rs
- `authored_assignment_penalty_spreads_farmers_across_farms()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs
- `battering_ram_targets_and_damages_buildings_from_authored_mask()` --calls--> `generate_world()`  [INFERRED]
  bevy-port/crates/stream_town_game/src/lib.rs → bevy-port/crates/stream_town_domain/src/world.rs

## Import Cycles
- None detected.

## Communities (297 total, 18 thin omitted)

### Community 0 - "ContentCatalog"
Cohesion: 0.08
Nodes (74): ContentCatalog, ActorState, RoleProgress, Default, String, action_animation_speed(), action_cooldown(), ActionPresentation (+66 more)

### Community 1 - "BuildingProcessor"
Cohesion: 0.07
Nodes (11): Container, ContainerBuilder, Dictionary, List, BuildingProcessor, BuildingType, foodCost, goldCost (+3 more)

### Community 2 - "GeneratedWorld"
Cohesion: 0.09
Nodes (57): WorldGenConfig, algorithmic_generation_matches_unity_validation_fingerprints(), authored_foliage_is_deterministic_and_respects_habitat_and_resources(), authored_grid_centre(), authored_world_to_grid(), AuthoredResourceLayer, cell_hash(), changing_seed_changes_world_hash() (+49 more)

### Community 3 - "BinarySaveCodec"
Cohesion: 0.10
Nodes (9): Action, CancellationToken, Func, int, List, UTF8Encoding, BinarySaveCodec, BinaryReader (+1 more)

### Community 4 - "ResourceGenerationSettings"
Cohesion: 0.11
Nodes (13): ContainerBuilder, ResourceGenSettingsInstaller, ContainerBuilder, WaterResourceGenSettingsInstaller, List, ResourceGenSettings, List, WaterResourceGenSettings (+5 more)

### Community 5 - "stream_town_game/src/lib.rs"
Cohesion: 0.02
Nodes (299): ArchetypeKind, generate_world(), accessibility_motion_preferences_preserve_authored_parameters(), accessibility_scope_keeps_modal_navigation_inside_the_open_surface(), AccessibilityActionDispatch, AccessibilityCandidate, AccessibilityHighContrastText, action_ranges_and_tower_acquisition_are_euclidean() (+291 more)

### Community 6 - "TwitchChatProcessor"
Cohesion: 0.06
Nodes (25): bool, float, Func, int, PlayerExistsByIDDelegate, PlayerExistsByNameDelegate, Queue, string (+17 more)

### Community 7 - "setup_rendering"
Cohesion: 0.04
Nodes (92): AmbientLight, Assets, ActiveMaterialHandles, building_damage_intensity(), building_damage_value(), building_snow_strength(), BuildingMaterialInstance, BuildingMaterialInstanced (+84 more)

### Community 8 - "BottomBarInterface"
Cohesion: 0.06
Nodes (21): bool, IEnumerator, Image, BottomBarButton, BottomBarContext, Action, bool, Button (+13 more)

### Community 9 - "Option"
Cohesion: 0.14
Nodes (29): animator_component(), animator_reference_path(), color_value(), convert_post_process(), extracts_indexed_material_properties(), field_array(), field_bool(), field_f32() (+21 more)

### Community 10 - "SettingsProcessor"
Cohesion: 0.06
Nodes (12): Autosave, List, AudioMixer, bool, Container, ContainerBuilder, UniversalRendererData, UniversalRenderPipelineAsset (+4 more)

### Community 11 - "BuildingPlacer"
Cohesion: 0.08
Nodes (16): BuildingPlacer, bool, BoxCollider, Color, int, LayerMask, List, string (+8 more)

### Community 12 - "ScriptablesProcessorInfrastructure"
Cohesion: 0.07
Nodes (8): int, AudioSettings, BuildingScriptablesEditor, CreditsRuntimeData, Reflex.Core, Data.Containers, Settings, ScriptablesProcessorInfrastructure

### Community 13 - "stream_town_migrate/src/presentation.rs"
Cohesion: 0.07
Nodes (82): animation_take_name(), array_index(), assign_clip_rigs_and_reference_poses(), collect_prefab_dependencies(), controller_id(), convert(), convert_avatar_masks(), convert_chimney_smoke() (+74 more)

### Community 14 - "TechTreeIOUtility"
Cohesion: 0.09
Nodes (19): Node_SO, NodeChildrenTechData, ChildrenSaveData, Dictionary, HashSet, List, Node_SO, NodeSaveData (+11 more)

### Community 15 - "HealthHandler"
Cohesion: 0.08
Nodes (15): BuildingDamageMaterialHandler, bool, IEnumerator, Renderer, PlayerDeathHandler, bool, float, Vector3 (+7 more)

### Community 16 - "stream_town_domain/src/content.rs"
Cohesion: 0.09
Nodes (42): ArchetypeBounds, AuthoredRecord, AuthoredValue, BuildingDef, BuildingModelDef, ContentError, EnemyDef, EnemyModelSetDef (+34 more)

### Community 17 - "save.rs"
Cohesion: 0.13
Nodes (36): actor_state(), detects_corruption_and_recovers_backup(), inspect_legacy_save(), inspects_legacy_binary_header_without_modifying_source(), LegacyMigrationMetadata, LegacySaveInfo, LegacySaveKind, load_native() (+28 more)

### Community 19 - "finish_world_reveal"
Cohesion: 0.06
Nodes (59): AccessibilityNode, AtomicU64, advance_loading_phase(), advance_loading_runtime(), advance_world_loading_cover(), AnimatedCharacterShadowReceiver, asset_root_collection_ready(), begin_world_loading_cover() (+51 more)

### Community 20 - "config.rs"
Cohesion: 0.12
Nodes (25): ConfigError, default_configuration_is_valid_and_round_trips_ron(), ease_in_out_cubic(), enabled_twitch_requires_public_configuration(), GameplayConfig, BTreeMap, BTreeSet, Default (+17 more)

### Community 21 - ".Log"
Cohesion: 0.04
Nodes (40): Container, ContainerBuilder, GUIDProcessor, Container, ContainerBuilder, GameStateProcessor, Action, bool (+32 more)

### Community 22 - "BuildingBase"
Cohesion: 0.12
Nodes (8): BuildingBase, bool, float, int, List, UnityEvent, SortBuildingByLowerLevel, IComparer

### Community 23 - "RoleProcessor"
Cohesion: 0.06
Nodes (14): RoleSlotModifier, int, RoleSlot, bool, int, Container, ContainerBuilder, int (+6 more)

### Community 24 - "UnitHealthBar"
Cohesion: 0.15
Nodes (6): bool, Camera, float, GameObject, Slider, UnitHealthBar

### Community 25 - "command.rs"
Cohesion: 0.11
Nodes (36): BuildingAction, BuildingDirection, CameraAction, CameraDirection, ChatCommand, CommandParseError, content_id(), CustomizationKind (+28 more)

### Community 26 - "Query"
Cohesion: 0.05
Nodes (192): Aabb, Added, AnimatedBy, AnimationGraphHandle, AnimationPlayer, AnimationTransitions, AudioSink, ActivePetVisual (+184 more)

### Community 27 - "TechTreeGraphView"
Cohesion: 0.07
Nodes (22): Color, float, string, TechnologyTreeGroup, Vector2, GroupSaveData, int, List (+14 more)

### Community 28 - "SaveFileData"
Cohesion: 0.09
Nodes (22): CancellationToken, int, string, Task, UTF8Encoding, BinarySaveStorage, CancellationToken, Task (+14 more)

### Community 29 - "GridPos"
Cohesion: 0.09
Nodes (35): actor_specific_exception_opens_only_the_requested_blocked_cell(), can_plan_for_three_hundred_agents(), DirtyRegion, grid(), GridPos, manhattan(), NavGrid, NavigationError (+27 more)

### Community 30 - "Vec"
Cohesion: 0.04
Nodes (86): AnimationClip, AnimationGraph, AnimationNodeIndex, AnimationTargetId, add_animation_composition(), add_animation_layer_branch(), add_rotation_curve(), add_scale_curve() (+78 more)

### Community 31 - "WorldGenProcessor"
Cohesion: 0.05
Nodes (25): Action, HashSet, Action, bool, BoxCollider, Container, ContainerBuilder, Func (+17 more)

### Community 32 - "UserInterface_Debug"
Cohesion: 0.09
Nodes (9): bool, GameObject, IEnumerator, object, TextMeshProUGUI, TMP_Dropdown, TMP_InputField, Vector3 (+1 more)

### Community 33 - "GameEventProcessor"
Cohesion: 0.06
Nodes (16): Container, ContainerBuilder, EventType, ParticleSystem, SortedSet, Transform, GameEventProcessor, EventType (+8 more)

### Community 34 - "GenerationSettings"
Cohesion: 0.08
Nodes (25): Action, IEnumerator, Vector2, Noise, AnimationCurve, bool, float, int (+17 more)

### Community 35 - "PlayerProcessor"
Cohesion: 0.07
Nodes (9): Action, Container, ContainerBuilder, List, Transform, Vector3, PlayerProcessor, EventType (+1 more)

### Community 36 - "ObjectiveSaveData"
Cohesion: 0.23
Nodes (5): Button, EnumField, ObjectiveVisualElement, ObjectiveSaveData, EnemyType

### Community 37 - "SettingsData"
Cohesion: 0.09
Nodes (14): string, GameIO, SaveFileType, bool, int, string, VideoSettingsPreset, bool (+6 more)

### Community 38 - "SeasonProcessor"
Cohesion: 0.09
Nodes (12): float, int, Material, AllSeasonSettings, SeasonProcessorEditor, Container, ContainerBuilder, SeasonProcessor (+4 more)

### Community 39 - "String"
Cohesion: 0.05
Nodes (64): AccessibleNode, AssetId, AccessibilityRuntime, active_event_text(), authored_rotating_node_names(), AuthoredCreditsElement, begin_world_loading(), building_model_node_names() (+56 more)

### Community 40 - "ObjectSelectionProcessor"
Cohesion: 0.10
Nodes (10): Camera, Container, ContainerBuilder, InputButton, List, UnityAction, Vector2, Vector3 (+2 more)

### Community 41 - "TechTreeProcessor"
Cohesion: 0.07
Nodes (11): NodeUnlockData, List, Node_SO, TechNodeData, Action, Container, ContainerBuilder, EventType (+3 more)

### Community 42 - "CharacterModelHandler"
Cohesion: 0.10
Nodes (14): SimpleToggleCarry, CharacterModelHandler, bool, int, List, Transform, RoleEquipment, bool (+6 more)

### Community 43 - "StableId"
Cohesion: 0.05
Nodes (80): ObjectiveDef, ObjectiveKind, FromStr, StableId, authored_trade_rates_clamp_to_stock_gold_and_capacity(), building_damage_and_repair_preserve_health_bounds(), BuildingState, capped_deposit_preserves_inventory_overflow() (+72 more)

### Community 44 - "AnimationControllerRuntime"
Cohesion: 0.12
Nodes (24): AnimationBlendSelection, AnimationControllerRuntime, AnimationParameterValue, AnimationRuntimeError, AnimationTransitionOutcome, AnimationTransitionPlayback, authored_state_speed_multiplies_float_parameter(), blends_between_authored_thresholds() (+16 more)

### Community 45 - "Vec3"
Cohesion: 0.07
Nodes (45): actor_combat_visual(), advance_falling_fish(), animate_healing_effects(), BuildingEffectKind, BuildingEffectParticle, chimney_particle_scale(), ChimneySmokeParticle, combat_material() (+37 more)

### Community 46 - "STSM_Idle_Player"
Cohesion: 0.05
Nodes (17): PlayerInventory, Dictionary, ResourceInventory, bool, int, int, STSM_Helper_Attack, STSM_Action_Build (+9 more)

### Community 47 - "RoleHandler"
Cohesion: 0.06
Nodes (26): PlayerRoleData, AudioClip, bool, float, int, RoleData, AudioClip, bool (+18 more)

### Community 48 - "ResourceDataSaveData"
Cohesion: 0.17
Nodes (13): Dictionary, materialIndex, meshIndex, bool, float, int, List, string (+5 more)

### Community 49 - ".Draw"
Cohesion: 0.11
Nodes (19): NodeUnlockSaveData, Port, Button, EnumField, UnlockVisualElement, Action, Button, EnumField (+11 more)

### Community 50 - "AudioHandler"
Cohesion: 0.09
Nodes (12): AudioHandler, AudioClip, AudioSource, bool, Camera, float, PlayerAudioHandler, Container (+4 more)

### Community 51 - "StreamTownSessionBridge"
Cohesion: 0.11
Nodes (13): bool, double, float, Func, int, List, long, MenuItem (+5 more)

### Community 52 - "WorldGenSaveData"
Cohesion: 0.08
Nodes (19): int, List, string, FoliageGroupSaveData, FoliageInstanceSaveData, FoliageProcessorSaveData, bool, int (+11 more)

### Community 53 - "TwitchClientProcessor"
Cohesion: 0.07
Nodes (17): Client, TwitchClientRuntimeData, Client, Container, ContainerBuilder, IEnumerator, LogType, OnChatCommandReceivedArgs (+9 more)

### Community 54 - "UIProcessor"
Cohesion: 0.09
Nodes (10): Container, ContainerBuilder, Slider, TextMeshProUGUI, UIProcessor, Color, GameObject, Slider (+2 more)

### Community 55 - "BevyMigrationExporter"
Cohesion: 0.08
Nodes (34): bool, Bounds, Color, Component, GameObject, HashSet, int, List (+26 more)

### Community 56 - "MonoBehaviour"
Cohesion: 0.01
Nodes (87): CameraProcessor, CellSpacePartitioningInstaller, ContainerBuilder, PersistentScoped, ContainerBuilder, Volume, PostProcessingInstaller, AudioMixerInstaller (+79 more)

### Community 57 - "SelectedObject"
Cohesion: 0.10
Nodes (6): SelectedEnemyCamp, object, UnityAction, SelectedObject, List, SelectedPlayerGroup

### Community 58 - "Targetable"
Cohesion: 0.10
Nodes (9): List, bool, BoxCollider, float, int, Transform, Vector3, Targetable (+1 more)

### Community 59 - "TechTreeEditorWindow"
Cohesion: 0.12
Nodes (7): bool, Button, MenuItem, string, TextField, VisualElement, TechTreeEditorWindow

### Community 60 - "Result"
Cohesion: 0.26
Nodes (6): BinaryParser<'a>, FnMut, Result, Self, T, LegacyWorldState

### Community 61 - "TechnologyGraphViewState"
Cohesion: 0.18
Nodes (25): center_world(), content_bounds(), cubic_bezier(), draw_connection(), draw_grid(), draw_minimap(), fit_bounds(), fit_handles_large_unity_coordinate_ranges() (+17 more)

### Community 62 - "CameraController"
Cohesion: 0.12
Nodes (10): bool, Camera, float, int, PlayerInput, Transform, Vector2, Vector3 (+2 more)

### Community 63 - "TargetSensor"
Cohesion: 0.14
Nodes (8): ProjectileShooter, float, int, string, bool, float, UnityEvent, TargetSensor

### Community 64 - "Access_Slider"
Cohesion: 0.07
Nodes (13): Access_AmbienceVolumeSlider, Access_BrightnessSlider, Access_EdgeScrollingSensitivitySlider, Access_FOVLevelSlider, Access_GammaSlider, Access_MasterVolumeSlider, Access_MusicVolumeSlider, Access_PanningSensitivitySlider (+5 more)

### Community 65 - "GraphicsProcessor"
Cohesion: 0.09
Nodes (16): bool, ContainerBuilder, List, UniversalRenderPipelineAsset, Volume, GraphicsProcessor, bool, ContainerBuilder (+8 more)

### Community 66 - "SerializableDictionary"
Cohesion: 0.11
Nodes (11): Dictionary, IEnumerator, List, SerializableDictionary, SerializableKeyValuePair, List, ICollection, IDictionary (+3 more)

### Community 67 - "twitch.rs"
Cohesion: 0.08
Nodes (40): channel_point_reward_tag_survives_privmsg_conversion(), CredentialVault, DeviceAuthorization, ensure_bot_identity(), envelope_from_privmsg(), message_confirms_channel_join(), OAuthClient, OAuthErrorResponse (+32 more)

### Community 68 - "Objective"
Cohesion: 0.09
Nodes (10): Action, int, Objective, Dictionary, GameObject, Image, RectTransform, TextMeshProUGUI (+2 more)

### Community 69 - "parse_controller"
Cohesion: 0.14
Nodes (22): animation_state_id(), animation_state_machine_id(), avatar_mask_id(), clip_id(), convert_clips(), inline_file_id(), parse_animation_events(), parse_blend_tree() (+14 more)

### Community 70 - "stream_town_migrate/src/content.rs"
Cohesion: 0.07
Nodes (127): ArchetypesById, aged_buildings(), animation_parameter_name(), archetype_bounds(), archetype_kind(), archetype_scenes(), asset(), authored_mask() (+119 more)

### Community 71 - "models.rs"
Cohesion: 0.22
Nodes (19): ModelBounds, ModelConversionReport, ModelEntry, ModelValidationSummary, rejects_animation_timeline_with_a_held_leading_sample(), Option, Path, PathBuf (+11 more)

### Community 72 - "Tiler"
Cohesion: 0.11
Nodes (9): TileHelper, int, Queue, Vector3, Tiler, int, string, TilerBuilding (+1 more)

### Community 73 - "ScriptablesEditor"
Cohesion: 0.07
Nodes (18): Color, List, Texture2D, EditorUtils, string, ScriptableObjectAssetData, bool, Color (+10 more)

### Community 74 - "UnitTravelToPosition"
Cohesion: 0.13
Nodes (6): Api, UnitTravelToPosition, Vector3, Projectile, TL_API, Combat

### Community 75 - "UserInterface_ObjectSelection"
Cohesion: 0.14
Nodes (13): BoxCollider, Button, GameObject, Image, List, object, Slider, TMP_Dropdown (+5 more)

### Community 76 - "Station"
Cohesion: 0.07
Nodes (17): Station, Dictionary, float, int, Queue, Transform, Container, ContainerBuilder (+9 more)

### Community 77 - "UserInterface_TownVote"
Cohesion: 0.09
Nodes (16): Button, GameObject, Image, Slider, TextMeshProUGUI, Transform, UI_TechOption, bool (+8 more)

### Community 78 - "tools_ui"
Cohesion: 0.16
Nodes (23): content_tab(), draw_world_preview(), format_runtime_frame_times(), inspector_tab(), launch_runtime_game(), migration_tab(), poll_runtime_console(), poll_tool_job_events() (+15 more)

### Community 79 - "TwitchBotSetupWindow"
Cohesion: 0.11
Nodes (19): bool, CancellationToken, CancellationTokenSource, Dictionary, int, long, MenuItem, string (+11 more)

### Community 80 - "UserInterface_Roles"
Cohesion: 0.16
Nodes (6): bool, Color32, Dictionary, GameObject, Transform, UserInterface_Roles

### Community 81 - "WorldUtils"
Cohesion: 0.23
Nodes (8): PlacementProbe, float, SurfaceType, GameObject, LayerMask, Transform, Vector3, WorldUtils

### Community 82 - "Node_SO"
Cohesion: 0.16
Nodes (10): Action, bool, Dictionary, IEnumerable, List, TechnologyTree, List, NodeChildrenTechData (+2 more)

### Community 83 - "RenderAssets"
Cohesion: 0.03
Nodes (107): BackgroundColor, AccessibilityMotionDefaults, apply_authored_ui_fonts(), authored_main_ui_image_with_ppu(), authored_ui_image(), authored_ui_image_with_corner_scale(), bottom_bar_authored_order(), bottom_bar_texture() (+99 more)

### Community 84 - "Access_Text"
Cohesion: 0.09
Nodes (11): Access_AmbienceVolumeText, Access_EdgeScrollingSensitivityText, Access_FOVLevelText, Access_MasterVolumeText, Access_MusicVolumeText, Access_PanningSensitivityText, Access_SoundEffectsVolumeText, Access_Text (+3 more)

### Community 85 - "String"
Cohesion: 0.17
Nodes (20): binary_schemas_one_through_three_decode_and_validate_trailer(), decode_binary(), decode_legacy(), ImportReport, json_pet_name(), legacy_objective_matches(), legacy_pet_name(), LegacyDecodedSave (+12 more)

### Community 86 - ".EnsureValidCredentials"
Cohesion: 0.19
Nodes (12): Action, IEnumerator, int, string, UnityWebRequest, TwitchAuthResult, TwitchAuthService, TwitchErrorResponse (+4 more)

### Community 87 - "SelectableObject"
Cohesion: 0.14
Nodes (10): InputButton, bool, List, RectTransform, UnityEvent, Vector3, ObjectSelectionRuntimeData, Selectable (+2 more)

### Community 88 - "Option"
Cohesion: 0.04
Nodes (137): ArchetypeDef, ArchetypeScene, RotatingNodeDef, PresentationCatalog, actor_detail_budget(), actor_material(), actor_scene_budget(), AgentCommand (+129 more)

### Community 89 - "FoliageProcessor"
Cohesion: 0.08
Nodes (29): Bounds, Container, ContainerBuilder, Dictionary, HashSet, List, Material, Matrix4x4 (+21 more)

### Community 90 - "DayAndNightProcessor"
Cohesion: 0.14
Nodes (6): Container, ContainerBuilder, DayAndNightProcessor, bool, float, DayAndNightRuntimeData

### Community 91 - "RaidEvent"
Cohesion: 0.08
Nodes (16): Animator, GameObject, IEnumerator, int, FishGodEvent, bool, IEnumerator, int (+8 more)

### Community 92 - "TechTreeNode"
Cohesion: 0.11
Nodes (13): Color, Foldout, List, Sprite, VisualElement, TechTreeNode, Port, Group (+5 more)

### Community 93 - "PlayerCommands"
Cohesion: 0.18
Nodes (4): OnMessageReceivedArgs, EventCommands, TwitchClientProcessor, PlayerCommands

### Community 94 - "convert_fbx_to_glb.py"
Cohesion: 0.14
Nodes (28): arguments(), bake_uniform_scale(), bounds_record(), convert(), discover_sources(), evaluated_scene_bounds(), inspect_glb(), main() (+20 more)

### Community 96 - "Resource"
Cohesion: 0.04
Nodes (33): DepositResources, BuildingResourceModelHandler, GameObject, ResourceStorageModifier, float, int, BuildingResourceModelHandlerEditor, IResourceHolder (+25 more)

### Community 97 - "SaveProcessor"
Cohesion: 0.08
Nodes (21): Action, CancellationToken, Container, ContainerBuilder, float, List, Material, materials (+13 more)

### Community 98 - "drive_tidal_music"
Cohesion: 0.19
Nodes (24): authored_gain_is_finite_bounded_and_fully_substituted(), drive_tidal_music(), effective_music_gain(), every_authored_pattern_parses_in_the_native_engine(), every_season_and_time_of_day_has_a_distinct_pattern(), music_expression(), music_template(), MusicSignature (+16 more)

### Community 99 - "stream_town_domain/src/presentation.rs"
Cohesion: 0.05
Nodes (82): AnimationClipDef, AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationEventDef, AnimationFloatKeyframe, AnimationLayerBlendMode, AnimationLayerDef (+74 more)

### Community 100 - "stream_town_migrate/src/technology_layout.rs"
Cohesion: 0.28
Nodes (15): GraphPoint, AuthoredGroup, AuthoredNode, build_layout(), checked_in_layout_exactly_matches_the_unity_graph_conversion(), convert(), parse_point(), parse_unity_graph() (+7 more)

### Community 101 - "StateMachine"
Cohesion: 0.09
Nodes (11): StateMachine, string, STSM_HelperBase, bool, List, string, uint, StateMachine (+3 more)

### Community 102 - "AnimationHandler"
Cohesion: 0.12
Nodes (10): AnimationHandler, Animator, bool, Dictionary, float, int, GameObject, int (+2 more)

### Community 103 - "TownGoalProcessor"
Cohesion: 0.19
Nodes (6): Container, ContainerBuilder, Goal, List, Objective, TownGoalProcessor

### Community 104 - "MainMenuManager"
Cohesion: 0.09
Nodes (13): LoadType, MetaData, bool, string, MainMenuRuntimeData, Button, GameObject, IEnumerator (+5 more)

### Community 105 - "ResourceProcessor"
Cohesion: 0.05
Nodes (46): Container, ContainerBuilder, Dictionary, float, int, List, Material, materialIndex (+38 more)

### Community 106 - "LoadingManager"
Cohesion: 0.08
Nodes (15): Dictionary, float, GameObject, Image, string, Task, TextMeshProUGUI, Transform (+7 more)

### Community 107 - "WorldInstanceDeterminism"
Cohesion: 0.30
Nodes (5): int, Quaternion, uint, Vector3, WorldInstanceDeterminism

### Community 108 - "GlobalAudioController"
Cohesion: 0.19
Nodes (9): GlobalAudioController, AudioSource, bool, float, IEnumerator, SeasonAudioData, AudioClip, List (+1 more)

### Community 109 - "CustomLogHandler"
Cohesion: 0.20
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

### Community 113 - "VfxSeagullSpawner"
Cohesion: 0.19
Nodes (9): AudioClip, AudioSource, float, GameObject, int, Vector3, SpawnArea, VfxSeagullSpawner (+1 more)

### Community 114 - "Enemy"
Cohesion: 0.21
Nodes (5): Action, float, Enemy, int, ActiveResourceIncrementer

### Community 115 - "GUIDComponent"
Cohesion: 0.15
Nodes (11): uint, GUIDComponent, SaveableBuilding, SaveableEnemy, SaveableEnemyCamp, string, SaveableObject, SaveablePlayer (+3 more)

### Community 116 - "GridProcessor"
Cohesion: 0.06
Nodes (22): bool, int, Vector2, GridSettings, ContainerBuilder, GridSettingsInstaller, GridProcessorEditor, int (+14 more)

### Community 117 - "stream_town_migrate/src/main.rs"
Cohesion: 0.19
Nodes (27): AssetKind, classify(), Cli, Command, destination_id(), inventory(), is_excluded(), is_yaml_kind() (+19 more)

### Community 118 - "Goal"
Cohesion: 0.15
Nodes (8): bool, Dictionary, float, int, TechTreeRuntimeData, Action, Dictionary, Goal

### Community 119 - "MiscCommands"
Cohesion: 0.17
Nodes (4): Dictionary, MiscCommands, Dictionary, MessageSender

### Community 120 - "VoteEvent"
Cohesion: 0.06
Nodes (23): List, KeepKingVote, int, List, NewKingVote, PlayerVote, Dictionary, TechVote (+15 more)

### Community 121 - "unity_color_filter"
Cohesion: 0.08
Nodes (28): init_unity_color_filter_pipeline(), App, AssetServer, Commands, Local, Option, PipelineCache, Plugin (+20 more)

### Community 122 - "Access_Dropdown"
Cohesion: 0.10
Nodes (11): Access_AODropdown, Access_AutosaveTimerDropdown, Access_DisplayBuildingDamageDropdown, Access_DisplayModeDropdown, Access_DisplayNameDropdown, Access_Dropdown, TMP_Dropdown, Access_FPSLimiterDropdown (+3 more)

### Community 123 - "SnapToGridMouseMovement"
Cohesion: 0.13
Nodes (9): Func, Vector3, MathExtended, Action, Camera, float, LayerMask, Vector3 (+1 more)

### Community 124 - "AIPath"
Cohesion: 0.17
Nodes (14): bool, float, int, Vector3, AIPath, GraphCollision, GraphNode, GraphUpdateObject (+6 more)

### Community 125 - "TechnologyGraphLayout"
Cohesion: 0.20
Nodes (14): automatic_layout_is_complete_deterministic_and_valid(), bounded_layout_index(), GraphSize, id(), reconcile_preserves_moves_and_repairs_catalog_coverage(), Default, Result, Self (+6 more)

### Community 126 - "TimeProcessor"
Cohesion: 0.16
Nodes (6): Container, ContainerBuilder, TimeProcessor, float, int, TimeRuntimeData

### Community 127 - "GateController"
Cohesion: 0.18
Nodes (7): GateController, Animator, bool, int, List, Collider, Rigidbody

### Community 128 - "UpdateGraphBounds"
Cohesion: 0.18
Nodes (6): float, SimpleDelayGraphUpdateOnce, bool, BoxCollider, int, UpdateGraphBounds

### Community 129 - "RotationHandler"
Cohesion: 0.33
Nodes (4): RotationHandler, float, Quaternion, Vector3

### Community 130 - "TransformSaveData"
Cohesion: 0.10
Nodes (16): int, List, string, uint, BuildingSaveData, int, uint, EnemyCampSaveData (+8 more)

### Community 131 - "TL_Secrets"
Cohesion: 0.35
Nodes (3): string, TL_Secrets, TwitchCredentialData

### Community 133 - "twitch_tab"
Cohesion: 0.26
Nodes (12): Duration, Sender, start_twitch_authorization(), start_twitch_clear(), start_twitch_diagnostic(), start_twitch_game_master_lookup(), start_twitch_reward_capture(), twitch_event_channel() (+4 more)

### Community 134 - "ToolState"
Cohesion: 0.13
Nodes (43): apply_foliage_draft(), apply_role_draft(), apply_technology_draft(), authoring_snapshot(), commit_catalog_candidate(), create_technology_group(), create_technology_node(), delete_selected_role() (+35 more)

### Community 135 - "convert"
Cohesion: 0.26
Nodes (12): ActorKind, actor_prefix(), checked_in_schema_one_fixture_imports_retained_terrain(), content_id(), conversion_preserves_mesh_and_relocates_invalid_positions(), convert(), duration_days(), entity_id() (+4 more)

### Community 136 - "runtime_console.rs"
Cohesion: 0.16
Nodes (20): invalid_requests_are_rejected_before_writing(), read_optional_json(), request_and_status_round_trip_through_atomic_store(), BTreeMap, Error, Into, Option, Path (+12 more)

### Community 137 - "STSM_StateAction"
Cohesion: 0.13
Nodes (7): int, STSM_Action_Attack, bool, float, int, STSM_StateAction, AnimationName

### Community 138 - "AstarPath"
Cohesion: 0.36
Nodes (5): string, Type, AstarData, AstarPath, NavGraph

### Community 139 - "WorldGenRuntimeData"
Cohesion: 0.20
Nodes (10): Action, bool, Dictionary, float, GameObject, int, List, Mesh (+2 more)

### Community 140 - "TargetProcessor"
Cohesion: 0.16
Nodes (7): Container, ContainerBuilder, List, TargetProcessor, Dictionary, List, TargetRuntimeData

### Community 141 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 142 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 143 - "legacy.rs"
Cohesion: 0.15
Nodes (42): ActorCustomization, StreamUserType, pending_stream_user_type(), binary_fixture(), BinaryParser, clamped_cell(), conversion_rejects_malformed_retained_mesh(), decode_json() (+34 more)

### Community 144 - "RuntimeData Template"
Cohesion: 0.09
Nodes (22): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **Dependencies**, 7. **No Logic Beyond State**, Architecture Overview (+14 more)

### Community 145 - "Key Rules"
Cohesion: 0.10
Nodes (20): 1. **Naming Convention**, 2. **Namespace**, 3. **File Location**, 4. **SceneScope Placement**, 5. **Interface Implementation**, 6. **SerializeField Field**, 7. **Public Property**, 8. **InstallBindings** (+12 more)

### Community 146 - "EnemyModelHandler"
Cohesion: 0.12
Nodes (9): EnemyModelHandlerEditor, bool, int, List, EnemyModelHandler, bool, float, Vector3 (+1 more)

### Community 147 - "Pet"
Cohesion: 0.11
Nodes (10): List, PetType, bool, Dictionary, float, Transform, Pet, Animator (+2 more)

### Community 148 - "add_file"
Cohesion: 0.23
Nodes (12): add_file(), package_windows(), PackageReport, portable_path(), Path, PathBuf, Result, String (+4 more)

### Community 149 - "CellSpacePartitioning"
Cohesion: 0.10
Nodes (13): Bounds, CellPartitioningEditor, bool, Vector2, BSPCell, Dictionary, float, int (+5 more)

### Community 150 - "ConfirmCheck"
Cohesion: 0.21
Nodes (5): Button, GameObject, TMP_Text, UnityAction, ConfirmCheck

### Community 151 - "MainMenuReferenceExporter"
Cohesion: 0.14
Nodes (17): bool, Color, float, GameObject, int, Quaternion, string, Vector2 (+9 more)

### Community 152 - "Stream Town Reloaded - Architecture Documentation"
Cohesion: 0.12
Nodes (15): Architecture Compliance Checklist, Benefits, Data Layer (ScriptableObjects), Exemptions, General Checks, Implementation Layer, Layer 1: Data Layer (ScriptableObjects), Layer 2: Processor Layer (+7 more)

### Community 153 - "DayAndNightSettings"
Cohesion: 0.22
Nodes (7): float, Material, Volume, DayAndNightSettings, ContainerBuilder, DayAndNightSettingsInstaller, Light

### Community 154 - "load_player_settings"
Cohesion: 0.32
Nodes (7): AnyResult, legacy_unity_settings_path(), load_player_settings(), load_runtime_config(), player_settings_path(), PathBuf, main()

### Community 155 - "Result"
Cohesion: 0.11
Nodes (51): append_vec3_keys(), convert_fireworks(), convert_healing_vfx(), f32_to_u16(), fireworks_effect_id(), json_f32(), parse_avatar_mask(), parse_conditions() (+43 more)

### Community 156 - "LabelDisplayProcessor"
Cohesion: 0.10
Nodes (12): Container, ContainerBuilder, LabelDisplayProcessor, Dictionary, GameObject, UtilDisplayRuntimeData, bool, Color (+4 more)

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

### Community 162 - "PlayerSaveData"
Cohesion: 0.07
Nodes (24): Component, Dictionary, List, Mesh, Transform, Vector3, SaveDataMapper, bool (+16 more)

### Community 163 - "PlayerInputProcessor"
Cohesion: 0.17
Nodes (5): Container, ContainerBuilder, InputButton, Vector2, PlayerInputProcessor

### Community 164 - "SeasonDataSettings"
Cohesion: 0.15
Nodes (10): Color, float, int, VisualEffect, SeasonDataSettings, bool, float, VisualEffect (+2 more)

### Community 165 - ".SetTargetType"
Cohesion: 0.23
Nodes (3): TargetableBuilding, TargetableHealth, TargetablePlayer

### Community 166 - "SimpleMusicController"
Cohesion: 0.33
Nodes (5): SimpleMusicController, AudioClip, AudioSource, float, IEnumerator

### Community 168 - "Q: There are still no animations."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: There are still no animations., Source Nodes

### Community 169 - "xtask/src/main.rs"
Cohesion: 0.19
Nodes (20): Cli, Command, glb_animation_count(), glb_document_from_bytes(), glb_node_names(), glb_node_names_from_bytes(), glb_node_validation_preserves_exact_names(), glb_with_nodes() (+12 more)

### Community 170 - "IProcessor.cs"
Cohesion: 0.16
Nodes (10): Action, CancellationToken, Exception, Task, IAsyncInitializableProcessor, IMainThreadInitializableProcessor, IPostInitializeProcessor, ProcessorStartupContext (+2 more)

### Community 171 - "stream_town_tools/src/main.rs"
Cohesion: 0.11
Nodes (38): authority_tab(), checked_in_authoring_assets_pass_headless_validation(), default_catalog_path(), default_config_path(), default_technology_layout_path(), foliage_editor_rejects_invalid_generation_values_without_mutation(), format_game_master_ids(), game_config_save_is_atomic_validated_and_round_trips() (+30 more)

### Community 172 - "RandomEnabler"
Cohesion: 0.33
Nodes (4): float, GameObject, IEnumerator, RandomEnabler

### Community 173 - "UIElementWrapper"
Cohesion: 0.08
Nodes (14): GameObject, List, PresetButtons, Access_ChannelNameInput, ContainerBuilder, Access_GOList, GameObject, List (+6 more)

### Community 174 - "Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Which reachable gameplay parity gaps remain between Unity roles, stations, enemies, events, technology, and the Bevy runtime?, Source Nodes

### Community 175 - "EditorHelpers"
Cohesion: 0.33
Nodes (3): GameObject, MenuItem, EditorHelpers

### Community 176 - "Editor"
Cohesion: 0.07
Nodes (11): BuildingModelHandler, GameObject, List, BuildingModelHandlerEditor, BuildingPlacerEditor, RoleScriptablesEditor, WindControllerEditor, GridSystemEditor (+3 more)

### Community 177 - "WeatherProcessor"
Cohesion: 0.27
Nodes (3): Container, ContainerBuilder, WeatherProcessor

### Community 178 - "TechTree.Elements"
Cohesion: 0.07
Nodes (22): Color, ErrorData, List, GroupErrorData, List, NodeErrorData, ChildrenSaveData, Vector2 (+14 more)

### Community 179 - "ResourceHolder"
Cohesion: 0.18
Nodes (6): CollectResource, AnimationCurve, bool, int, object, ResourceHolder

### Community 180 - "Settings Scriptable Template"
Cohesion: 0.18
Nodes (11): Advanced Features, Checklist for New Settings Scriptable, Conditional Display, Default Values, Field Naming, File Naming, Menu Naming, Naming Conventions (+3 more)

### Community 181 - "Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do standalone Unity animation clips flow from conversion into state-driven Bevy playback?, Source Nodes

### Community 182 - "BuildingSettings"
Cohesion: 0.09
Nodes (14): bool, Dictionary, int, BuildingSettings, Dictionary, BuildingDataContainer, int, ResourceCostData (+6 more)

### Community 183 - "Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why do Bevy generated resources now start at 100 units, and how are older saves verified?, Source Nodes

### Community 184 - "UserInterface"
Cohesion: 0.05
Nodes (19): InputButton, SharedTypes, int, ChangeTimeStamp, Slider, TextMeshProUGUI, UI_Objective, ContainerBuilder (+11 more)

### Community 185 - "Access_Toggle"
Cohesion: 0.15
Nodes (6): Access_AOToggle, Access_EdgeScrollingToggle, Access_MouseControlsToggle, Access_Toggle, Toggle, Access_VsyncToggle

### Community 186 - "WorldGenerationReferenceExporter"
Cohesion: 0.18
Nodes (13): float, IEnumerable, int, string, Vector2, GenerationReference, LayerReference, PerlinSample (+5 more)

### Community 187 - "IInstaller"
Cohesion: 0.01
Nodes (136): ContainerBuilder, InstantiationBarrier, Camera, GameObject, ProjectCameraInstaller, ContainerBuilder, AllBuildingDataSettingsInstaller, ContainerBuilder (+128 more)

### Community 188 - "ProjectCamera"
Cohesion: 0.09
Nodes (13): Camera, Quaternion, Vector3, ProjectCamera, ChannelData, string, Container, PSAccess (+5 more)

### Community 189 - "import_save"
Cohesion: 0.52
Nodes (7): absolute_path(), backup_candidate(), export_world_oracle(), import_preserves_source_and_recovers_named_backup(), import_save(), Path, PathBuf

### Community 190 - "HealthModifier"
Cohesion: 0.22
Nodes (6): AttackUnit, HealthModifier, bool, float, GameObject, HealUnit

### Community 191 - "stream_town_migrate/src/menu_scene.rs"
Cohesion: 0.11
Nodes (46): MainMenuCameraReference, MainMenuCorrectiveBake, MainMenuEmbeddedMesh, MainMenuFoliageVisual, MainMenuModelInstance, MainMenuResourceVisual, MainMenuSceneReference, Option (+38 more)

### Community 192 - "SavingAndLoading.Structs"
Cohesion: 0.08
Nodes (17): ActivityStatus, bool, float, string, UserType, TwitchUser, Color, GameUserType (+9 more)

### Community 193 - "Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does Bevy reproduce Unity ResourceProcessor zero-assignment claims?, Source Nodes

### Community 194 - "FoliageGenerationSettings"
Cohesion: 0.22
Nodes (7): List, Material, Mesh, string, Vector3, FoliageGenerationSettings, FoliageMeshSettings

### Community 196 - "Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: WorldSnapshot validate_snapshot stable ID duplicate actors map key state id world_seed simulation world_seed schema integrity, Source Nodes

### Community 197 - "UILineRenderer"
Cohesion: 0.36
Nodes (5): float, Vector2, UILineRenderer, Graphic, VertexHelper

### Community 198 - "UserInterface_DisplayUsernames"
Cohesion: 0.28
Nodes (4): bool, GameObject, UserInterface_DisplayUsernames, UsernameDisplayOption

### Community 199 - "Tree and Foliage Flicker Regression Checklist"
Cohesion: 0.22
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

### Community 203 - "IRuntimeDataScriptable"
Cohesion: 0.12
Nodes (16): Queue, AudioRuntimeData, UnityEvent, DebugRuntimeData, IRuntimeDataScriptable, bool, Dictionary, InputButton (+8 more)

### Community 204 - "Q: Why does a new Bevy town no longer spawn an enemy camp?"
Cohesion: 0.50
Nodes (3): Answer, Q: Why does a new Bevy town no longer spawn an enemy camp?, Source Nodes

### Community 205 - "Key Rules"
Cohesion: 0.25
Nodes (8): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **Serialization**, 4. **Organization**, 5. **Value Types**, 6. **No Runtime Changes**, 7. **Validation**, Key Rules

### Community 206 - "WorldSaveData"
Cohesion: 0.13
Nodes (15): bool, float, List, string, TechTreeSaveData, TechVotePlayerSaveData, TechVoteSaveData, bool (+7 more)

### Community 207 - "Common Patterns"
Cohesion: 0.25
Nodes (8): Color Settings, Common Patterns, Layer/Tag Settings, Lists of Objects, Numeric Values with Range, References, Toggles/Booleans, Vector Settings

### Community 208 - "StatusBar"
Cohesion: 0.33
Nodes (4): Image, TextMeshProUGUI, StatusBar, UserInterface.Menus

### Community 209 - "BuildingRuntimeData"
Cohesion: 0.50
Nodes (4): Dictionary, int, List, BuildingRuntimeData

### Community 210 - "Requirement"
Cohesion: 0.33
Nodes (4): RequirementType, object, Requirement, Requirements

### Community 211 - "CommandDictionary"
Cohesion: 0.19
Nodes (7): IReadOnlyList, Action, Dictionary, IReadOnlyList, List, CommandDictionary, ModeratorCommands

### Community 214 - "Key Rules"
Cohesion: 0.29
Nodes (7): 1. **Namespace and Interface**, 2. **CreateAssetMenu**, 3. **State Management**, 4. **Events**, 5. **Initialization**, 6. **No Logic Beyond State**, Key Rules

### Community 216 - "RuntimeData Template"
Cohesion: 0.33
Nodes (3): Checklist for New RuntimeData, RuntimeData Structure, RuntimeData Template

### Community 217 - "Character Animation Regression Checklist"
Cohesion: 0.22
Nodes (8): Acceptance gate, Attempt record template, Character Animation Regression Checklist, Current attempt, Do not retry unchanged, Next narrow diagnostic pass, What did not fix visible animation, What did work

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
Cohesion: 0.29
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

### Community 234 - "CommonEnums.cs"
Cohesion: 0.11
Nodes (17): List, Vector3, TargetableData, Dictionary, List, Foliage, FoliageSaveType, FoliageType (+9 more)

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
Cohesion: 0.25
Nodes (8): 1. Secure the old credentials, 2. Register the Twitch application, 3. Authorize `HumanBeanBot`, 4. Prepare the channel, 5. Bind the Fish God Channel Points reward, 6. Configure OBS, Connection controls and diagnostics, Twitch setup

### Community 240 - "Accessibility"
Cohesion: 0.29
Nodes (6): Accessibility, Automated verification, Keyboard operation, Persisted preferences, Screen-reader contract, Windows Narrator acceptance

### Community 243 - "EventProcessor"
Cohesion: 0.22
Nodes (4): Action, Container, ContainerBuilder, EventProcessor

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
Cohesion: 0.04
Nodes (134): AccessibilityActionRequest, AccessibilityFocusVisualQuery, AccumulatedMouseMotion, AccumulatedMouseScroll, AppExit, PlayerSettings, Default, accessibility_button_enabled() (+126 more)

### Community 253 - "STSM_GoToLocation"
Cohesion: 0.07
Nodes (16): STSM_HelperDeposit, float, STSM_Action_DepositResource, bool, float, GameObject, int, Transform (+8 more)

### Community 254 - "Q: How does the Bevy runtime preserve Unity world-space target range semantics?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How does the Bevy runtime preserve Unity world-space target range semantics?, Source Nodes

### Community 255 - "Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: How do Unity prefab renderer sharedMaterials flow through presentation conversion into Bevy StandardMaterial overrides on spawned GLB descendants?, Source Nodes

### Community 256 - "SelectedBuilding"
Cohesion: 0.12
Nodes (4): bool, float, PassiveResourceIncrementer, SelectedBuilding

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

### Community 264 - "Coordinator"
Cohesion: 0.07
Nodes (21): Coordinator, StartupState, Action, bool, CancellationToken, CancellationTokenSource, Container, Dictionary (+13 more)

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

### Community 273 - "next_agent_goal"
Cohesion: 0.39
Nodes (12): authored_assignment_penalty_spreads_farmers_across_farms(), battering_ram_targets_and_damages_buildings_from_authored_mask(), builder_completes_and_upgrades_authored_construction(), building_approach(), building_base_max_health(), building_region(), building_site_is_available(), farmer_harvests_unlimited_food_from_completed_farm() (+4 more)

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

### Community 281 - "Q: role level experience progression station equipment inventory skill upgrade"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: role level experience progression station equipment inventory skill upgrade, Source Nodes

### Community 283 - "Stream Town external authoring suite"
Cohesion: 0.40
Nodes (5): Authoritative files, Launch, Safe persistence, Stream Town external authoring suite, Workflows

### Community 284 - "record_gpu_readiness"
Cohesion: 0.25
Nodes (8): record_gpu_readiness(), PipelineCache, ErasedRenderAssets, GpuImage, GpuRenderAssets, PreparedMaterial, RenderMesh, RenderMeshInstances

### Community 288 - "SensorProcessor"
Cohesion: 0.12
Nodes (7): float, List, SensorRuntimeData, SensorBase, Container, ContainerBuilder, SensorProcessor

### Community 289 - "DebugProcessor"
Cohesion: 0.08
Nodes (12): Dictionary, DebugSettings, Container, ContainerBuilder, HideInCallstack, Object, DebugLogCategory, DebugProcessor (+4 more)

### Community 291 - "UserInterface_GameMenu.cs"
Cohesion: 0.25
Nodes (3): ContainerBuilder, MetaDataInstaller, MetaData

### Community 292 - "RoleDataSettings"
Cohesion: 0.07
Nodes (22): ContainerBuilder, AllRoleDataSettingsInstaller, ContainerBuilder, TerrainGenSettingsInstaller, AnimationCurve, bool, float, GameObject (+14 more)

### Community 294 - "CreateProjectScopeProcessors.cs"
Cohesion: 0.40
Nodes (3): MenuItem, CreateProjectScopeProcessors, StreamTownEditor

### Community 296 - "settings.rs"
Cohesion: 0.10
Nodes (32): AudioMixSettings, BuildingHealthDisplayMode, CameraSettings, clamp_sensitivity(), clamp_unit(), default_ui_scale_percent(), defaults_are_valid_and_round_trip(), DisplayMode (+24 more)

### Community 297 - "Utils"
Cohesion: 0.04
Nodes (33): BuildCostModifier, InputButton, STStateMachine.States, UserInterface.MainMenu, TownGoal.Enumerations, PlayerControls.ObjectSelection, Units, Utils (+25 more)

### Community 298 - "VfxAnimationController"
Cohesion: 0.12
Nodes (8): bool, float, VisualEffect, VfxAnimationController, Transform, VisualEffect, VfxParticlePosition, VFX

### Community 299 - "VFXArrowPointer"
Cohesion: 0.22
Nodes (3): float, ParticleSystem, VFXArrowPointer

### Community 302 - "IProcessor"
Cohesion: 0.18
Nodes (5): Container, IProcessor, Container, ContainerBuilder, CreditsProcessor

### Community 305 - "Q: Characters are still not animated. Trees still have the flickering shadows."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Characters are still not animated. Trees still have the flickering shadows., Source Nodes

### Community 306 - "Player"
Cohesion: 0.07
Nodes (9): Player, Dictionary, GameObject, Vector3, Vector3, BuildingCommands, GameMasterCommands, RoleCommands (+1 more)

### Community 309 - "UnityGraphics"
Cohesion: 0.33
Nodes (5): Vector3, UnityGraphics, URP, FieldInfo, ShadowResolution

### Community 311 - "BuildPlacerData"
Cohesion: 0.22
Nodes (6): BuildPlacerData, GameObject, Renderer, string, Vector2, PlacementProbeHandler

### Community 313 - "Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?"
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: Why did the converted player run animation fail to loop cleanly after visible animation started working?, Source Nodes

### Community 316 - "Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: The trees look okay, though I'd double check just to make sure their leaves' card normals are all facing the correct way for lighting purposes? I'm pretty sure the cards are doublesided within Unity (no backface culling), so be sure that's the case. Animation does not work still. One slight clue that may be a similar shadowing issue, there seems to be some flickering appearing on the characters' shoulders., Source Nodes

### Community 320 - "GameConfig"
Cohesion: 0.17
Nodes (31): GameConfig, StationDef, active_station_ids(), apply_recruit_group_order(), assigned_station(), best_station_id(), cached_station_targets(), compatible_station_ids() (+23 more)

## Knowledge Gaps
- **353 isolated node(s):** `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier`, `StartupState`, `InputButton` (+348 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

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

- **Why does `Utils` connect `Utils` to `UpdateGraphBounds`, `StringUtils`, `ResourceGenerationSettings`, `BuildingPlacer`, `ScriptablesProcessorInfrastructure`, `WorldGenerationReferenceExporter.cs`, `GenerationSettings`, `RoleDataSettings`, `SeasonDataSettings`, `SeasonProcessor`, `RandomEnabler`, `Editor`, `TechTree.Elements`, `ResourceHolder`, `UserInterface`, `MonoBehaviour`, `IInstaller`, `SavingAndLoading.Structs`, `Easings`, `FPSDisplay`, `CommonEnums.cs`, `WorldInstanceDeterminism`, `EnemySpawner`, `EventProcessor`, `SnapToGridMouseMovement`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `PlayerProcessor` connect `PlayerProcessor` to `BuildingProcessor`, `TwitchChatProcessor`, `BottomBarInterface`, `BuildingPlacer`, `SelectedPlayer`, `.Log`, `RoleProcessor`, `WorldGenProcessor`, `UserInterface_Debug`, `GameEventProcessor`, `DebugProcessor`, `Utils`, `TechTreeProcessor`, `IProcessor`, `RoleHandler`, `Player`, `StreamTownSessionBridge`, `TwitchClientProcessor`, `UIProcessor`, `MonoBehaviour`, `SelectedObject`, `IInstaller`, `UserInterface_TownVote`, `UserInterface_Roles`, `CommandDictionary`, `RaidEvent`, `PlayerCommands`, `Resource`, `SaveProcessor`, `EnemySpawner`, `Enemy`, `VoteEvent`, `TimeProcessor`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Why does `StableId` connect `StableId` to `ContentCatalog`, `GeneratedWorld`, `stream_town_game/src/lib.rs`, `ToolState`, `setup_rendering`, `runtime_console.rs`, `convert`, `Option`, `stream_town_migrate/src/presentation.rs`, `stream_town_domain/src/content.rs`, `save.rs`, `next_agent_goal`, `config.rs`, `command.rs`, `Query`, `Result`, `GridPos`, `Vec`, `String`, `stream_town_tools/src/main.rs`, `AnimationControllerRuntime`, `Vec3`, `TechnologyGraphViewState`, `stream_town_migrate/src/menu_scene.rs`, `GameConfig`, `twitch.rs`, `parse_controller`, `stream_town_migrate/src/content.rs`, `tools_ui`, `RenderAssets`, `String`, `Option`, `stream_town_domain/src/presentation.rs`, `TechnologyGraphLayout`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **What connects `StreamTown.EditorTools`, `Attributes`, `BuildCostModifier` to the rest of the system?**
  _353 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ContentCatalog` be split into smaller, more focused modules?**
  _Cohesion score 0.07552758237689744 - nodes in this community are weakly interconnected._
- **Should `BuildingProcessor` be split into smaller, more focused modules?**
  _Cohesion score 0.06641604010025062 - nodes in this community are weakly interconnected._
- **Should `GeneratedWorld` be split into smaller, more focused modules?**
  _Cohesion score 0.09011776753712238 - nodes in this community are weakly interconnected._