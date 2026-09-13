const MAX_TOWN_GOALS: usize = 2;
const AUTOMATIC_PLAYER_RESPAWN_SECONDS: f64 = 10.0 * 60.0;
const REGENERATION_TARGET_RETRY_SECONDS: f64 = 30.0;
const LOCAL_RESOURCE_SATURATION_RADIUS_CELLS: u16 = 10;
const LOCAL_RESOURCE_SATURATION_DECAY_PER_CHECK: u32 = 1;
const FORESTER_TREE_TIMER_RADIUS_CELLS: u16 = 20;
const FORESTER_TREE_TIMER_BONUS_PER_TREE: f64 = 0.01;
const FORESTER_INTERVAL_MULTIPLIER: f64 = 0.67;
const TENDER_INTERVAL_MULTIPLIER: f64 = 2.0;
const UNITY_IDLE_WANDER_INTERVAL_SECONDS: f32 = 3.0;
const FORESTER_RANDOM_FALLBACK_DENOMINATOR: u64 = 10;
const FORESTER_UTILITY_RADIUS_CELLS: i32 = 20;
const FORESTER_PREFERRED_NURSERY_DISTANCE_CELLS: f64 = 5.0;
const FORESTER_DIRECTION_MEMORY_SECONDS: f64 = 30.0 * 60.0;
const MAX_ACTIVE_CITIZEN_NIGHT_LIGHTS: usize = 32;
const MAX_ACTIVE_BUILDING_NIGHT_LIGHTS: usize = 20;
const MAX_ACTIVE_PROJECTILE_NIGHT_LIGHTS: usize = 12;
const NIGHT_LIGHT_POOL_CAPACITY: usize = MAX_ACTIVE_CITIZEN_NIGHT_LIGHTS
    + MAX_ACTIVE_BUILDING_NIGHT_LIGHTS
    + MAX_ACTIVE_PROJECTILE_NIGHT_LIGHTS;
const MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME: usize = 2;
const NIGHT_LIGHT_TRANSITION_SECONDS: f32 = 10.0;
const PROJECTILE_MAX_LIFETIME_SECONDS: f32 = 12.0;
const UNITY_AUTHORED_GRID_CELL_SIZE: f32 = 2.0;
const TWITCH_COMMAND_HELP_URL: &str = "https://github.com/HumanBeanGames/Stream-Town-Bevy/blob/codex/bevy-migration/TWITCH_COMMANDS.md";
const BUILDING_PLACEMENT_TIMEOUT_SECONDS: f32 = 60.0;
const HEALING_TARGET_EFFECT_SCALE: f32 = 0.125;
const INVALID_TWITCH_COMMAND_REPLY: &str = "Invalid Command! Type !help for the list of commands!";
const TECHNOLOGY_VOTE_DURATION_SECONDS: f32 = 60.0;
const TECHNOLOGY_VOTE_OPTION_COUNT: usize = 3;
const TECHNOLOGY_VOTE_PANEL_HEIGHT: f32 = 400.0;
const TECHNOLOGY_VOTE_TITLE_TOP: f32 = 44.0;
const TECHNOLOGY_VOTE_FIRST_ROW_TOP: f32 = 78.0;
const TECHNOLOGY_VOTE_SINGLE_LINE_ADVANCE: f32 = 76.0;
const TECHNOLOGY_VOTE_EXTRA_LINE_ADVANCE: f32 = 13.0;
const TECHNOLOGY_VOTE_LABEL_OFFSET_Y: f32 = 3.0;
const TECHNOLOGY_VOTE_ICON_TOP: f32 = 35.0;
const TECHNOLOGY_VOTE_TIMER_TOP: f32 = 330.0;
const TECHNOLOGY_VOTE_TIMER_GROUP_TOP: f32 = TECHNOLOGY_VOTE_TIMER_TOP - 2.0;
const TECHNOLOGY_VOTE_TIMER_BAR_WIDTH: f32 = 150.0;
const TECHNOLOGY_VOTE_TIMER_GAP: f32 = 9.0;
const TECHNOLOGY_VOTE_TIMER_TEXT_OFFSET_Y: f32 = 2.0;
const UNITY_NUMBERED_LABEL_SECONDS: f32 = 15.0;
const WORLD_SCENE_PATH: &str = "Assets/Scenes/Worlds/World_Town.unity";
const MAIN_MENU_SCENE_PATH: &str = "Assets/Scenes/Menu/Main_Menu_02.unity";
const CREDITS_SCENE_PATH: &str = "Assets/Scenes/Menu/Credits.unity";
const PASSIVE_RESOURCE_FIXED_POINT_DENOMINATOR: u128 = 1_000_000_000_000;
const NIGHT_ENEMY_WAVE_INTERVAL_SECONDS: f64 = 120.0;
const ENEMY_WAVE_SPAWN_SPREAD_SECONDS: f64 = 10.0;
const CHIMNEY_ALPHA_STEPS: usize = 8;
const TERRAIN_CHUNK_CELLS: u16 = 16;
const OCEAN_PADDING_CELLS: u16 = 128;
const TRAVERSAL_WEAR_DECAY_INTERVAL_SECONDS: f32 = 1.0;
const PASSIVE_BUILDING_INCOME_INTERVAL: Duration = Duration::from_millis(250);
/// Navigation alone is subdivided into thirds; authored placement, saves, terrain
/// generation, resource generation, and gameplay ranges remain in town cells.
const NAVIGATION_SUBDIVISIONS: u16 = 3;
const NAVIGATION_TERRAIN_MIN_HEIGHT_CENTIMETRES: i16 = 45;
const WORLD_DIAGNOSTIC_VIEW_SECONDS: f32 = 10.0;
const WORLD_DIAGNOSTIC_OVERLAY_LIFT: f32 = 0.14;
const WORLD_DIAGNOSTIC_TILE_SCALE: f32 = 0.94;
const DEFENDER_LOCAL_PRIORITY_RADIUS_CELLS: u16 = 5;
const WORLD_DIAGNOSTIC_DEPTH_BIAS: f32 = 64.0;
const TERRAIN_HIGH_DETAIL_RADIUS: f32 = 220.0;
const TERRAIN_MEDIUM_DETAIL_RADIUS: f32 = 440.0;
const TERRAIN_LOD_HYSTERESIS: f32 = 36.0;
// Keep every practical shipping town actor on its authored skinned rig. The
// former 64-actor ceiling was shared by citizens and enemies, so later joins
// could remain capsule placeholders and their animations could be withheld.
// Explicit benchmark mode retains its measured low-detail ceiling.
const DEFAULT_ACTOR_DETAIL_BUDGET: usize = 6_400;
const PERFORMANCE_ACTOR_DETAIL_BUDGET: usize = 16;
const HEALING_CHANNEL_RING_SCALE: f32 = 0.035;
const HEALING_BURST_RING_SCALE: f32 = 0.24;
const HEALING_MOTE_SCALE: f32 = 0.035;
const HEALING_EMISSIVE_MAX_COMPONENT: f32 = 3.5;
// Orthographic framing is retained only for deterministic close-up smoke tests.
// Shipping gameplay uses the perspective camera authored in MainCamera.prefab.
const UNITY_TOWN_CAMERA_FOV_DEGREES: f32 = 60.0;
const UNITY_TOWN_CAMERA_NEAR: f32 = 0.3;
const UNITY_TOWN_CAMERA_FAR: f32 = 1_000.0;
const UNITY_TOWN_CAMERA_OFFSET: Vec3 = Vec3::new(-33.5, 33.240_562, 0.0);
const UNITY_TOWN_CAMERA_FOCUS_BACK_SHIFT: f32 = 16.0;
const UNITY_TOWN_CAMERA_MIN_HEIGHT: f32 = 11.0;
const UNITY_TOWN_CAMERA_MAX_HEIGHT: f32 = 60.0;
const UNITY_TOWN_CAMERA_MOVE_SMOOTHNESS: f32 = 5.0;
const PATH_CENTRE_THIRD_STEP_PENALTY: u32 = 2;
const TWITCH_CAMERA_VERTICAL_PAN_DISTANCE: f32 = 12.0;
const TWITCH_CAMERA_HORIZONTAL_PAN_DISTANCE: f32 = 6.0;
const BUILD_CURSOR_RAY_DISTANCE: f32 = 2_000.0;
const AUTO_CAMERA_IDLE_SECONDS: f32 = 30.0;
const AUTO_CAMERA_TOWN_SHOT_SECONDS: f32 = 12.0;
const AUTO_CAMERA_CITIZEN_SHOT_SECONDS: f32 = 24.0;
const AUTO_CAMERA_CITIZEN_HEIGHT: f32 = 15.0;
const AUTO_CAMERA_CITIZEN_FOCUS_HEIGHT: f32 = 1.4;
const AUTO_CAMERA_BUILDING_HEIGHT: f32 = 22.0;
const AUTO_CAMERA_BUILDING_FOCUS_HEIGHT: f32 = 2.5;
const AUTO_CAMERA_ATTENTION_SECONDS: f32 = 15.0;
const AUTO_CAMERA_COMBAT_REDIRECT_COOLDOWN_SECONDS: f32 = 5.0;
const AUTO_CAMERA_TOWN_SHOT_INTERVAL: u64 = 4;
const RETREAT_HEALTH_PERCENT: i32 = 25;
const RETREAT_TOWN_HALL_RADIUS_CELLS: u16 = 10;
const HEALING_HYSTERESIS_PERCENT: u128 = 125;
const ACTOR_OVERLAY_ANCHOR_HEIGHT_CELLS: f32 = 1.15;
const ACTOR_HEALTH_OVERLAY_TOP_PX: f32 = -11.0;
const ACTOR_NAME_OVERLAY_TOP_PX: f32 = 4.0;
const BUILDING_HEALTH_OVERLAY_WIDTH_PX: f32 = 82.0;
const PATH_HEALTH_OVERLAY_WIDTH_PX: f32 = BUILDING_HEALTH_OVERLAY_WIDTH_PX / 5.0;
const FOLIAGE_CAPTURE_TIMES_SECONDS: [f32; 12] =
    [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0];
pub const PLAYER_ANIMATED_MODEL_PATH: &str = "shipping/models/Models/Characters/Characters.glb";
pub const PLAYER_ANIMATED_SOURCE_MODEL: &str = "Assets/Models/Characters/Characters.fbx";
const PING_POINTER_DURATION_SECONDS: f32 = 8.0;
const PING_POINTER_MODEL_PATH: &str = "shipping/models/Models/VFX/PointerArrow.glb";
const PING_POINTER_MATERIAL_ID: &str = "material:799ef8ce46a71414286fd24c033a98fe";
const PING_POINTER_MODEL_MIN_Y: f32 = 851.829_35;
const PING_POINTER_MODEL_HEIGHT: f32 = 314.468_26;
const PLAYER_NAME_COLOR_PRESETS: [[u8; 3]; 10] = [
    [238, 91, 91],
    [244, 157, 72],
    [239, 213, 83],
    [112, 201, 101],
    [74, 198, 191],
    [83, 163, 238],
    [132, 116, 238],
    [194, 105, 226],
    [232, 109, 171],
    [232, 218, 196],
];
const FISH_GOD_EXIT_DELAY_SECONDS: f32 = 2.5;
const RAINING_FISH_PREFAB_SUFFIX: &str = "VFX/Environment/VFX_RainingFish.prefab";
const RAINING_FISH_RENDER_BUDGET: usize = 320;
const FISH_SCHOOL_PREFAB_SUFFIX: &str = "VFX/Environment/Fish.prefab";
const FISH_SCHOOL_RENDER_BUDGET_PER_BINDING: usize = 160;
const ROLE_ACTION_AUDIO_MAX_DISTANCE: f32 = 20.0;
const SELECTION_MASK_MATERIAL_ID: &str = "material:631afbf4de8b7ad4eabc5e30e0b2c778";
#[cfg(test)]
const SELECTION_MASK_TEXTURE_PATH: &str = "Assets/Textures/SelectionMask.png";
const PLAYER_SELECTION_OUTLINE_SCALE_CELLS: f32 = 1.5 * 1.25 / 2.0;
const SELECTION_OUTLINE_COLOR: Vec4 = Vec4::new(1.0, 0.92, 0.02, 0.92);
const SELECTION_OUTLINE_DEPTH_BIAS: f32 = 8.0;
const WORLD_REVEAL_GPU_STABLE_FRAMES: u64 = 6;
// Building_Gate.prefab uses a 4 x 1 x 4 trigger centred on the gate. Converted
// building models are authored at Unity scale and then scaled by cell_size / 2.
const GATE_TRIGGER_HALF_EXTENT_UNITY_UNITS: f32 = 2.0;
// Player_Character.prefab authors 100 Unity world units for both its target
// and builder-resource sensors. Shipping terrain uses two units per cell.
const PLAYER_TARGET_SEARCH_RANGE_CELLS: u16 = 50;
const STATION_TARGET_CHECK_MILLISECONDS: f64 = 2_000.0;
const BUILDING_PLACEMENT_SUCCESS_COLOR: [f32; 3] = [0.242_250_26, 0.896_226_4, 0.054_957_304];
const BUILDING_PLACEMENT_FAIL_COLOR: [f32; 3] = [0.933_962_3, 0.0, 0.0];
const TERRAIN_SHADER_ASSET_PATH: &str = "shaders/terrain_material.wgsl";
const TERRAIN_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Terrain.mat";
const WATER_SHADER_ASSET_PATH: &str = "shaders/water_material.wgsl";
const WATER_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Water.mat";
const MAIN_MENU_BASELINE_EXPOSURE_EV: f32 = -1.5;
// Neutral now renders exactly like the former -1.0 brightness setting:
// 0.5 authored baseline + (-1.0) user adjustment = -0.5 EV.
const IN_GAME_BASELINE_EXPOSURE_EV: f32 = -0.5;
// Entity construction must yield back to Bevy regularly so the loading UI can
// be extracted and presented. Asset decoding and deterministic generation run
// on worker pools. A time budget handles models with unequal construction cost,
// while the count ceilings also bound deferred-command application.
const LOADING_SCENE_TIME_BUDGET_PER_FRAME: Duration = Duration::from_millis(3);
const MAIN_MENU_SPAWN_BUDGET_PER_FRAME: usize = 48;
const WORLD_RESOURCE_SPAWN_BUDGET_PER_FRAME: usize = 48;
const WORLD_FOLIAGE_SPAWN_BUDGET_PER_FRAME: usize = 96;
const IN_GAME_AMBIENT_LIGHT_MULTIPLIER: f32 = 1.30;
const IN_GAME_SATURATION_MULTIPLIER: f32 = 1.12;
const BUILDING_SHADER_ASSET_PATH: &str = "shaders/building_material.wgsl";
const BUILDING_MATERIAL_PATH: &str = "Assets/Materials/Building_Material.mat";
const CLOUD_SHADER_ASSET_PATH: &str = "shaders/cloud_material.wgsl";
const CLOUD_MATERIAL_PATH: &str = "Assets/Materials/VFX/Clouds.mat";
// Mean alpha of MainGameTexture_01.tga. Unity's texture importer supplied a
// mip chain for this authored noise atlas; the source TGA does not.
// The cloud shader uses this value as the correctly box-filtered limit when
// its 20x world-space noise layer is too small to resolve on screen.
const CLOUD_NOISE_ALPHA_MEAN: f32 = 67.635_544 / 255.0;
const GODRAY_SHADER_ASSET_PATH: &str = "shaders/godray_material.wgsl";
const GODRAY_MATERIAL_PATH: &str = "Assets/Materials/VFX/VFX_Godrays.mat";
const GIRAFFE_SHADER_ASSET_PATH: &str = "shaders/giraffe_material.wgsl";
const GIRAFFE_MATERIAL_PATH: &str = "Assets/Materials/Character/Giraffe.mat";
const BOUNDS_SHADER_ASSET_PATH: &str = "shaders/bounds_material.wgsl";
const BOUNDS_MATERIAL_PATH: &str = "Assets/Materials/BoundsVisualizer.mat";
const TREE_SHADER_ASSET_PATH: &str = "shaders/tree_material.wgsl";
const TREE_PREPASS_SHADER_ASSET_PATH: &str = "shaders/tree_material_prepass.wgsl";
const TREE_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Tree.mat";
const GRASS_SHADER_ASSET_PATH: &str = "shaders/grass_material.wgsl";
const GRASS_PREPASS_SHADER_ASSET_PATH: &str = "shaders/grass_material_prepass.wgsl";
const GRASS_MATERIAL_PATH: &str = "Assets/Materials/Environment/Env_Grass.mat";
const CRITTER_SHADER_ASSET_PATH: &str = "shaders/critter_material.wgsl";
const CRITTER_PREPASS_SHADER_ASSET_PATH: &str = "shaders/critter_material_prepass.wgsl";
const CRITTER_MATERIAL_PATH: &str = "Assets/Materials/Critters/Critters.mat";
const CRITTER_MATERIAL_ID: &str = "material:56cfb478fa4b9e8469bcbbf9cf077701";
const FLAG_SHADER_ASSET_PATH: &str = "shaders/flag_material.wgsl";
const FLAG_MATERIAL_PATH: &str = "Assets/Materials/Prototype/Flags.mat";
const CHARACTER_SHADER_ASSET_PATH: &str = "shaders/character_material.wgsl";
const MENU_SKY_SHADER_ASSET_PATH: &str = "shaders/menu_sky_material.wgsl";
const CHARACTER_UNITY_SHADER_PATH: &str = "Assets/Shaders/LowPolyST.shader";
// Keep the offset local to skinned character receivers. The global directional
// light bias also affects terrain and foliage, whose synchronized shadow paths
// have already been visually validated.
const CHARACTER_SHADOW_RECEIVER_NORMAL_OFFSET: f32 = 0.02;
const GAME_LOGO_TEXTURE_PATH: &str = "Assets/Sprites/Miscellaneous/Game_Logo_DropShadow.png";
const LOADING_SCREEN_TEXTURE_PATH: &str = "Assets/Sprites/LoadingScreen/UI_LoadingScreen.png";
const LOADING_OVERLAY_TEXTURE_PATH: &str = "Assets/Sprites/LoadingScreen/UI_loadingOverlay.png";
const LOADING_ICON_TEXTURE_PATH: &str = "Assets/Sprites/LoadingScreen/UI_LoadingIcon.png";
const UI_FONT_ASSET_PATH: &str = "shipping/fonts/Rubik-Bold.ttf";
const UI_DISPLAY_FONT_ASSET_PATH: &str = "shipping/fonts/Luckiest Guy.ttf";
const LOADING_SCREEN_PREFAB_SUFFIX: &str = "UserInterface/UI_LoadingScreen.prefab";
const LOADING_ICON_HIERARCHY_PATH: &str = "LoadingScreen/LoadingIcon/Anchor/Image";
const GAME_LOGO_ASPECT_RATIO: f32 = 2_048.0 / 1_227.0;
const MAIN_MENU_TEXTURE_PATHS: [&str; 4] = [
    "Assets/Sprites/Buttons/UI_Button_Unpressed.png",
    "Assets/Sprites/Buttons/UI_Button.png",
    "Assets/Sprites/Buttons/UI_Button_Disabled.png",
    "Assets/Sprites/Objectives/UI_Objectives_Background.png",
];
const GAME_MENU_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/VotingMenu/UI_VotingMenu_Background.png",
    "Assets/Sprites/Miscellaneous/UI_Cross.png",
    "Assets/Sprites/Miscellaneous/UI_Checkmark.png",
];
const SETTINGS_BACKGROUND_TEXTURE_PATH: &str = "Assets/Sprites/Settings/UI_Settings_Background.png";
const SECRETS_DISCLAIMER: &str = "By choosing Yes, you confirm that you are not presently streaming or recording this screen through OBS, Streamlabs, XSplit, browser capture, a capture card, or any other third-party application. The next screen contains sensitive Twitch account setup information. You accept any and all responsibility for keeping that information private and absolve Stream Town, its contributors, and all other parties of liability arising from violating this agreement.";
const SECRETS_PRIVACY_NOTICE: &str = "INTERNAL TWITCH VIDEO IS BLACKED OUT WHILE THIS SCREEN IS OPEN. Third-party capture software cannot be controlled by Stream Town. Never paste a Client Secret or stream key here.";
const SECRETS_INITIAL_FEEDBACK: &str =
    "Enter the public Client ID and both account logins, then authorize each account separately.";
const SETTINGS_PANEL_CORNER_SCALE: f32 = 1.5;
const TOP_BAR_TEXTURE_PATHS: [&str; 10] = [
    "Assets/Sprites/TopBar/UI_TopBar_Background.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Food.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Gold.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Ore.png",
    "Assets/Sprites/TopBar/UI_TopBar_Resources_Wood.png",
    "Assets/Sprites/TopBar/UI_TopBar_Stats_Players.png",
    "Assets/Sprites/TopBar/UI_TopBar_Stats_Buildings.png",
    "Assets/Sprites/TopBar/UI_TopBar_Stats_PlayTime.png",
    "Assets/Sprites/TopBar/UI_TopBar_SeasonGauge.png",
    "Assets/Sprites/TopBar/UI_TopBar_SeasonGauge_Meter.png",
];
const SELECTION_PANEL_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/SelectionWindow/UI_SelectionWindow_Slider_Unfilled.png",
    "Assets/Sprites/SelectionWindow/UI_SelectionWindow_GreenSlider_Filled.png",
    "Assets/Sprites/SelectionWindow/UI_SelectionWindow_RedSlider_Filled.png",
];
const VOTE_TEXTURE_PATHS: [&str; 9] = [
    "Assets/Sprites/VotingMenu/UI_VotingMenu_Background.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_TimerSlider_Filled.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_TimerSlider_Unfilled.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_Timer_Icon.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_VotePrompt.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_VoteSlider_Filled.png",
    "Assets/Sprites/VotingMenu/UI_VotingMenu_VoteSlider_Unfilled.png",
    "Assets/Sprites/RulerVotingMenu/UI_RulerVotingMenu_Icon.png",
    "Assets/Sprites/RulerVotingMenu/UI_RulerVotingMenu_TimerSlider_Filled.png",
];
const RULER_VOTE_TIMER_UNFILLED_PATH: &str =
    "Assets/Sprites/RulerVotingMenu/UI_RulerVotingMenu_TimerSlider_Unfilled.png";
const OBJECTIVE_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/Objectives/UI_Objectives_Background.png",
    "Assets/Sprites/Objectives/UI_Objectives_Slider_Unfilled.png",
    "Assets/Sprites/Objectives/UI_Objectives_Slider_Filled.png",
];
const CURRENT_EVENT_TEXTURE_PATHS: [&str; 3] = [
    "Assets/Sprites/CurrentEvent/UI_CurrentEvent_Background.png",
    "Assets/Sprites/CurrentEvent/UI_CurrentEvent_Slider_Unfilled.png",
    "Assets/Sprites/CurrentEvent/UI_CurrentEvent_Slider_Filled.png",
];
const FOLIAGE_VISIBILITY_MIN_RANGE: f32 = 96.0;
const FOLIAGE_VISIBILITY_MAX_RANGE: f32 = 192.0;
const FOLIAGE_VISIBILITY_FADE: f32 = 18.0;
// Unity groups generated foliage by mesh/material. Bevy performs the matching
// GPU instancing automatically; stable spatial groups make that contract
// auditable without duplicating source geometry or changing generation.
const FOLIAGE_BATCH_CHUNK_CELLS: u16 = 32;
const CROWD_SEPARATION_RADIUS_CELLS: f32 = 0.42;
const CROWD_SEPARATION_MAX_CELLS: f32 = 0.28;
const CROWD_PREDICTION_RADIUS_CELLS: f32 = 0.46;
const CROWD_PREDICTION_HORIZON_SECONDS: f32 = 0.85;
const CROWD_MINIMUM_YIELD_SPEED: f32 = 0.18;
const CHARACTER_HIT_SECONDS: f32 = 0.25;
const TOWER_TRAIL_SECONDS: f32 = 2.0;
const TOWER_TRAIL_WIDTH: f32 = 0.1;
const FIREBALL_SIZE: f32 = 0.4;
const FIREBALL_TRAIL_SIZE: f32 = 0.3;
const BUILDING_HIT_SECONDS: f32 = 0.5;
const BUILDING_HIT_SMOKE_SPEED: f32 = 3.0;
const BUILDING_HIT_SPARK_SPEED: f32 = 12.0;
const SETTINGS_STREAMING_FIRST_INDEX: usize = 25;
const SETTINGS_STREAMING_LAST_INDEX: usize = 32;
const SETTINGS_APPLY_INDEX: usize = 33;
const SETTINGS_DEFAULTS_INDEX: usize = 34;
const SETTINGS_BACK_INDEX: usize = 35;
const SETTINGS_MENU_ITEM_COUNT: usize = 36;
const AMBIENCE_GAIN: f32 = 0.16;
const SEAGULL_GAIN: f32 = 0.28;
const SEAGULL_FLIGHT_SECONDS: f32 = 32.0;
const SEAGULL_HEIGHT: f32 = 15.0;
const SEAGULL_MAX_AUDIO_DISTANCE: f32 = 50.0;
const SEAGULL_MODEL_PATH: &str = "shipping/models/Models/Critters/Critter_Seagull_01.glb";
const RAIN_CAMERA_CULL_DISTANCE: f32 = 7.5;
const WORLD_UI_SAFE_TOP: f32 = 78.0;
const WORLD_UI_SAFE_BOTTOM: f32 = 0.0;
const WORLD_UI_OVERLAY_MARGIN: f32 = 36.0;
const UNITY_MAIN_UI_REFERENCE_HEIGHT: f32 = 2_160.0;
const UNITY_SETTINGS_UI_REFERENCE_HEIGHT: f32 = 1_080.0;
const MAIN_MENU_CLOUD_COLUMNS: usize = 11;
const MAIN_MENU_CLOUD_ROWS: usize = 5;
const PROCEDURAL_AUDIO_SAMPLE_RATE: u32 = 16_000;
const BUILDING_HIT_SMOKE_SIZE: f32 = 0.5;
const BUILDING_HIT_SPARK_SIZE: f32 = 0.25;
const BUILDING_LEVEL_UP_SECONDS: f32 = 1.5;
const BUILDING_LEVEL_UP_ARROW_SIZE: f32 = 0.5;
const BUILDING_LEVEL_UP_TILE_SIZE: f32 = 4.0;
const BUILDING_DAMAGED_RADIUS: f32 = 1.403_639_8;
const BUILDING_DAMAGED_FIRE_AMOUNT: u16 = 128;
const BUILDING_DAMAGED_SMOKE_AMOUNT: u16 = 200;
const AGENT_ROTATION_SPEED: f32 = 5.0;
const LOCOMOTION_REFERENCE_SPEED_CELLS_PER_SECOND: f32 = 5.0;
const LOCOMOTION_STOP_GRACE_SECONDS: f32 = 0.12;
const EYE_NODES: [&str; 10] = [
    "Eyes_Angry",
    "Eyes_Annoyed",
    "Eyes_Cool",
    "Eyes_Happy",
    "Eyes_MissingEye",
    "Eyes_Normal",
    "Eyes_Pain",
    "Eyes_Sad",
    "Eyes_Wink",
    "Eyes_Worried",
];
const HAIR_NODES: [&str; 7] = [
    "Hair_Long_Ponytail",
    "Hair_Long_Sidebraids",
    "Hair_Medium_Side",
    "Hair_Short_Bowlcut",
    "Hair_Short_Normal",
    "Hair_Short_Pushup",
    "Hair_Short_Sideswept",
];
const FACIAL_HAIR_NODES: [&str; 2] = ["FacialHair_Long_Beard", "FacialHair_Medium_Beard"];
const HAIR_COLORS: [[f32; 3]; 6] = [
    [0.019_607_844, 0.019_607_844, 0.019_607_844],
    [0.575_471_7, 0.176_441_8, 0.176_441_8],
    [0.839_622_6, 0.678_662_4, 0.099_012_084],
    [0.5, 0.287_401_7, 0.134_433_95],
    [0.127_714_48, 0.660_377_4, 0.216_843_2],
    [0.129_411_74, 0.599_905_3, 0.658_823_55],
];
const EYE_COLORS: [[f32; 3]; 5] = [
    [0.133_333_34, 0.098_039_22, 0.098_039_22],
    [0.191_883_25, 0.543_265_76, 0.830_188_7],
    [0.402_439_36, 0.726_415_1, 0.085_662_15],
    [0.743_859, 0.792_452_8, 0.790_356_93],
    [0.801_886_8, 0.693_535_1, 0.086_997_17],
];
const SKIN_COLORS: [[f32; 3]; 5] = [
    [0.850_980_4, 0.678_431_4, 0.509_803_95],
    [0.811_320_8, 0.697_127_04, 0.585_528_7],
    [0.556_603_8, 0.387_968_9, 0.223_166_61],
    [0.358_490_6, 0.222_511_4, 0.089_622_65],
    [0.850_980_4, 0.745_124_1, 0.509_803_95],
];
use twitch::{
    CredentialVault, DeviceAuthorization, OAuthClient, TokenValidation, TwitchControl, TwitchEvent,
    TwitchModerationStatus, TwitchStatus, TwitchTransport,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Boot,
    MainMenu,
    WorldLoading,
    InGame,
    Credits,
}

/// Present only after the recursive loading overlay and its dedicated camera
/// have actually left the ECS world. Systems in this set must not advance
/// gameplay, presentation clocks, or world audio without this marker.
#[derive(Resource)]
struct GameplayReady;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct GameplaySimulationSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct AgentSimulationSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct NightLightSyncSet;

const STREAM_ONLY_AGENT_SIMULATION_HZ: u32 = 60;
const STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ: u32 = 30;

#[derive(Resource, Default)]
struct AgentSimulationCadence(AtomicU64);

impl AgentSimulationCadence {
    fn set_delta(&self, delta: Duration) {
        self.0.store(
            u64::try_from(delta.as_nanos()).unwrap_or(u64::MAX),
            AtomicOrdering::Relaxed,
        );
    }

    fn delta(&self) -> Duration {
        Duration::from_nanos(self.0.load(AtomicOrdering::Relaxed))
    }
}

#[derive(Default)]
struct AgentSimulationCadenceState {
    accumulated: Duration,
    delta: Duration,
}

#[derive(Resource)]
pub struct RuntimeConfig(pub GameConfig);

#[derive(Resource, Clone)]
pub struct RuntimePlayerSettings(pub PlayerSettings);

#[derive(Resource)]
struct PlayerSettingsRuntime {
    autosave_elapsed_seconds: f32,
}

#[derive(Resource)]
pub struct RuntimeContent(pub ContentCatalog);

#[derive(Resource)]
pub struct RuntimePresentation(pub PresentationCatalog);

#[derive(Resource)]
struct RuntimeAssetRoot(PathBuf);

#[derive(Resource)]
struct WorldRuntime {
    generated: GeneratedWorld,
}

#[derive(Resource, Default)]
struct FineNavigationRuntime {
    grid: Option<stream_town_domain::NavGrid>,
    applied_signature: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorldDiagnosticMode {
    Pathfinding,
    Floorplan,
}

#[derive(Resource, Default)]
struct WorldDiagnosticRuntime {
    mode: Option<WorldDiagnosticMode>,
    remaining_seconds: f32,
    black_material: Option<Handle<StandardMaterial>>,
}

impl WorldDiagnosticRuntime {
    fn activate(&mut self, mode: WorldDiagnosticMode) {
        self.mode = Some(mode);
        self.remaining_seconds = WORLD_DIAGNOSTIC_VIEW_SECONDS;
    }

    fn refresh_if_active(&mut self) {
        if self.mode.is_some() {
            self.remaining_seconds = WORLD_DIAGNOSTIC_VIEW_SECONDS;
        }
    }
}

/// Sparse traffic history. A point is added only when a citizen completes a
/// move into a different grid cell, and is discarded once exponential decay
/// makes it visually irrelevant.
#[derive(Resource)]
struct TraversalWearRuntime {
    cells: HashMap<GridPos, TraversalWearCell>,
    decay_elapsed_seconds: f32,
    texture_dirty: bool,
}

#[derive(Resource, Default)]
struct PathSurfaceRuntime {
    applied_signature: u64,
    initialized: bool,
    levels: HashMap<GridPos, u16>,
}

#[derive(Clone, Copy, Debug, Default)]
struct TraversalWearCell {
    score: f32,
    decay_pause_seconds: f32,
}

impl Default for TraversalWearRuntime {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            decay_elapsed_seconds: 0.0,
            texture_dirty: true,
        }
    }
}

impl TraversalWearRuntime {
    fn record(
        &mut self,
        position: GridPos,
        settings: &stream_town_domain::TerrainAppearanceConfig,
    ) {
        let cell = self.cells.entry(position).or_default();
        cell.score = (cell.score + 1.0)
            .min(traversal_score_for_rate(
                settings.traversal_full_tint_per_minute,
                settings.traversal_half_life_seconds,
            ))
            .min(MAX_TRAVERSAL_WEAR_SCORE);
        cell.decay_pause_seconds = settings.traversal_decay_pause_seconds;
        self.texture_dirty = true;
    }

    fn decay(
        &mut self,
        delta_seconds: f32,
        settings: &stream_town_domain::TerrainAppearanceConfig,
    ) {
        self.decay_elapsed_seconds += delta_seconds.max(0.0);
        if self.decay_elapsed_seconds < TRAVERSAL_WEAR_DECAY_INTERVAL_SECONDS {
            return;
        }
        if self.cells.is_empty() {
            self.decay_elapsed_seconds = 0.0;
            return;
        }
        let elapsed = std::mem::take(&mut self.decay_elapsed_seconds);
        self.cells.retain(|_, cell| {
            let active_decay_seconds = (elapsed - cell.decay_pause_seconds).max(0.0);
            cell.decay_pause_seconds = (cell.decay_pause_seconds - elapsed).max(0.0);
            if active_decay_seconds > 0.0 {
                cell.score *=
                    0.5_f32.powf(active_decay_seconds / settings.traversal_half_life_seconds);
            }
            cell.score >= settings.traversal_prune_score
        });
        self.texture_dirty = true;
    }

    fn restore(
        &mut self,
        cells: &BTreeMap<GridPos, f32>,
        width: u16,
        height: u16,
        settings: &stream_town_domain::TerrainAppearanceConfig,
    ) {
        self.cells = cells
            .iter()
            .filter(|(position, score)| {
                position.x < width
                    && position.z < height
                    && score.is_finite()
                    && **score >= settings.traversal_prune_score
            })
            .map(|(position, score)| {
                (
                    *position,
                    TraversalWearCell {
                        score: score
                            .min(traversal_score_for_rate(
                                settings.traversal_full_tint_per_minute,
                                settings.traversal_half_life_seconds,
                            ))
                            .min(MAX_TRAVERSAL_WEAR_SCORE),
                        decay_pause_seconds: 0.0,
                    },
                )
            })
            .collect();
        self.decay_elapsed_seconds = 0.0;
        self.texture_dirty = true;
    }

    fn saved_cells(&self) -> BTreeMap<GridPos, f32> {
        self.cells
            .iter()
            .filter(|(_, cell)| cell.score.is_finite() && cell.score > 0.0)
            .map(|(position, cell)| (*position, cell.score.min(MAX_TRAVERSAL_WEAR_SCORE)))
            .collect()
    }
}

fn traversal_wear_fraction(
    score: f32,
    settings: &stream_town_domain::TerrainAppearanceConfig,
) -> f32 {
    let fade_start = traversal_score_for_rate(
        settings.traversal_fade_start_per_minute,
        settings.traversal_half_life_seconds,
    );
    let full_tint = traversal_score_for_rate(
        settings.traversal_full_tint_per_minute,
        settings.traversal_half_life_seconds,
    );
    ((score - fade_start) / (full_tint - fade_start)).clamp(0.0, 1.0)
}

/// Maps an author-facing sustained crossing rate to the equilibrium of the
/// exponentially decaying sparse score: rate / decay constant.
fn traversal_score_for_rate(rate_per_minute: f32, half_life_seconds: f32) -> f32 {
    rate_per_minute.max(0.0) * half_life_seconds.max(f32::EPSILON) / (60.0 * std::f32::consts::LN_2)
}

#[derive(Resource)]
struct SaveRuntime {
    store: NativeSaveStore,
}

/// Holds the startup resume request after process initialization. Keeping the
/// path in a resource makes the loading and broadcast gates independent of any
/// later environment changes, and `applied` prevents a failed resume from ever
/// exposing or streaming the freshly generated fallback world.
#[derive(Resource, Clone, Debug, Default)]
struct AutomaticResumeRuntime {
    path: Option<PathBuf>,
    applied: bool,
}

impl AutomaticResumeRuntime {
    fn new(path: Option<PathBuf>) -> Self {
        Self {
            path,
            applied: false,
        }
    }

    fn pending(&self) -> bool {
        self.path.is_some() && !self.applied
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TownSaveEntry {
    name: String,
    path: PathBuf,
    protected: bool,
}

#[derive(Resource)]
struct TownSaveCatalogRuntime {
    directory: PathBuf,
    fixed_path: Option<PathBuf>,
    active_town: Option<String>,
}

fn automatic_resume_save_path() -> Option<PathBuf> {
    std::env::var_os("STREAM_TOWN_AUTO_RESUME_PATH").map(PathBuf::from)
}

fn automatic_resume_world_seed(path: Option<&Path>) -> Option<u64> {
    let path = path?;
    NativeSaveStore::new(path)
        .load()
        .ok()
        .map(|snapshot| snapshot.world_seed)
}

fn startup_save_path(resume_path: Option<PathBuf>, fixed_path: Option<PathBuf>) -> PathBuf {
    resume_path
        .or(fixed_path)
        .unwrap_or_else(|| PathBuf::from(".stream-town").join("StreamTownSave.stbevy"))
}

impl TownSaveCatalogRuntime {
    fn from_startup_paths(fixed_path: Option<PathBuf>, resume_path: Option<PathBuf>) -> Self {
        let active_town = resume_path
            .as_deref()
            .or(fixed_path.as_deref())
            .and_then(|path| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(str::to_owned)
            });
        Self {
            directory: PathBuf::from(".stream-town").join("saves"),
            fixed_path,
            active_town,
        }
    }

    fn entries(&self) -> Vec<TownSaveEntry> {
        if let Some(path) = &self.fixed_path {
            return path
                .is_file()
                .then(|| town_save_entry(path))
                .into_iter()
                .collect();
        }
        let mut entries = fs::read_dir(&self.directory)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("stbevy"))
            })
            .map(|path| town_save_entry(&path))
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| left.path.cmp(&right.path))
        });
        entries
    }

    fn new_town_path(&self, name: &str) -> PathBuf {
        self.fixed_path.clone().unwrap_or_else(|| {
            self.directory
                .join(format!("{}.stbevy", safe_town_filename(name)))
        })
    }
}

#[derive(Resource, Default)]
struct TownRestartRuntime {
    retained_players: Option<Vec<ActorState>>,
    suppress_exit_save: bool,
}

#[derive(SystemParam)]
struct LoadRenderParams<'w, 's> {
    presentation: Res<'w, RuntimePresentation>,
    render: Res<'w, RenderAssets>,
    meshes: Option<ResMut<'w, Assets<Mesh>>>,
    asset_server: Option<Res<'w, AssetServer>>,
    asset_root: Res<'w, RuntimeAssetRoot>,
    terrain_surfaces: Query<'w, 's, Entity, With<TerrainSurface>>,
    water_surfaces: Query<'w, 's, Entity, With<WaterSurface>>,
}

#[derive(SystemParam)]
struct LoadWorldEntities<'w, 's> {
    agents: Query<
        'w,
        's,
        (
            Entity,
            &'static mut Agent,
            &'static mut GridLocation,
            &'static AgentAnimation,
            &'static mut Transform,
        ),
        Without<TownHall>,
    >,
    runtime_buildings: Query<'w, 's, (Entity, &'static RuntimeBuilding), Without<TownHall>>,
    town_halls: Query<
        'w,
        's,
        (
            &'static mut GridLocation,
            &'static mut BuildingPresentation,
            &'static mut Transform,
        ),
        With<TownHall>,
    >,
    enemy_camps: Query<'w, 's, (Entity, &'static EnemyCamp)>,
    surface_visuals: Query<'w, 's, Entity, With<RuntimeTerrainGrounding>>,
    world_fish_schools: Query<'w, 's, Entity, (With<FishSchoolParticle>, With<WorldEntity>)>,
}

#[derive(Resource)]
struct SimulationRuntime(WorldSimulation);

#[derive(Resource, Default)]
struct NightEnemyWaveRuntime {
    night_day: Option<u32>,
    remaining_seconds: f64,
    wave_index: u32,
    pending_spawns: VecDeque<PendingEnemySpawn>,
    spawn_interval_seconds: f64,
    spawn_remaining_seconds: f64,
}

#[derive(Clone, Debug)]
struct PendingEnemySpawn {
    camp: StableId,
    archetype: StableId,
    serial: u64,
    final_raid_wave: bool,
}

#[derive(Clone, Debug)]
struct EnemyRouteBuilding {
    id: StableId,
    town_hall_distance: u64,
    approaches: Vec<GridPos>,
}

#[derive(Clone, Debug)]
struct EnemyNavigationField {
    signature: u64,
    width: u16,
    component_by_cell: Vec<u32>,
    target_by_component: Vec<Option<StableId>>,
    next_by_cell: Vec<Option<GridPos>>,
    goal_by_cell: Vec<Option<GridPos>>,
    cluster_edges: BTreeMap<EnemyClusterNode, Vec<EnemyClusterNode>>,
}

// Preserve the original eight-town-cell HPA* cluster size on the navigation-only
// grid, which has three samples per town cell.
const ENEMY_NAVIGATION_CLUSTER_SIZE: u16 = 8 * NAVIGATION_SUBDIVISIONS;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct EnemyClusterNode {
    x: u16,
    z: u16,
    component: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EnemyPathOpenNode {
    position: GridPos,
    estimated_total: u32,
    cost: u32,
}

impl Ord for EnemyPathOpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_total
            .cmp(&self.estimated_total)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for EnemyPathOpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl EnemyNavigationField {
    fn index(&self, position: GridPos) -> Option<usize> {
        (position.x < self.width)
            .then(|| usize::from(position.z) * usize::from(self.width) + usize::from(position.x))
            .filter(|index| *index < self.component_by_cell.len())
    }

    fn destination(&self, start: GridPos) -> Option<(&StableId, GridPos)> {
        let index = self.index(start)?;
        let component = *self.component_by_cell.get(index)?;
        let component = usize::try_from(component).ok()?;
        Some((
            self.target_by_component.get(component)?.as_ref()?,
            self.goal_by_cell.get(index).copied().flatten()?,
        ))
    }

    fn component_at(&self, position: GridPos) -> Option<u32> {
        let component = *self.component_by_cell.get(self.index(position)?)?;
        (component != u32::MAX).then_some(component)
    }

    fn cluster_node(&self, position: GridPos) -> Option<EnemyClusterNode> {
        Some(EnemyClusterNode {
            x: position.x / ENEMY_NAVIGATION_CLUSTER_SIZE,
            z: position.z / ENEMY_NAVIGATION_CLUSTER_SIZE,
            component: self.component_at(position)?,
        })
    }

    fn cluster_path(&self, start: GridPos, goal: GridPos) -> Option<Vec<EnemyClusterNode>> {
        let start = self.cluster_node(start)?;
        let goal = self.cluster_node(goal)?;
        if start.component != goal.component {
            return None;
        }
        if start == goal {
            return Some(vec![start]);
        }
        let mut frontier = VecDeque::from([start]);
        let mut previous = BTreeMap::<EnemyClusterNode, EnemyClusterNode>::new();
        let mut visited = BTreeSet::from([start]);
        while let Some(current) = frontier.pop_front() {
            for neighbour in self.cluster_edges.get(&current).into_iter().flatten() {
                if !visited.insert(*neighbour) {
                    continue;
                }
                previous.insert(*neighbour, current);
                if *neighbour == goal {
                    let mut route = vec![goal];
                    let mut cursor = goal;
                    while cursor != start {
                        cursor = previous[&cursor];
                        route.push(cursor);
                    }
                    route.reverse();
                    return Some(route);
                }
                frontier.push_back(*neighbour);
            }
        }
        None
    }

    fn hierarchical_path(
        &self,
        navigation: &stream_town_domain::NavGrid,
        start: GridPos,
        goal: GridPos,
    ) -> Option<Vec<GridPos>> {
        let cluster_route = self.cluster_path(start, goal)?;
        let component = self.component_at(start)?;
        let cluster_width = self.width.div_ceil(ENEMY_NAVIGATION_CLUSTER_SIZE);
        let height = u16::try_from(self.component_by_cell.len() / usize::from(self.width)).ok()?;
        let cluster_height = height.div_ceil(ENEMY_NAVIGATION_CLUSTER_SIZE);
        let mut corridor = BTreeSet::new();
        for cluster in cluster_route {
            for dz in -1_i32..=1 {
                for dx in -1_i32..=1 {
                    let x = i32::from(cluster.x) + dx;
                    let z = i32::from(cluster.z) + dz;
                    if x >= 0
                        && z >= 0
                        && x < i32::from(cluster_width)
                        && z < i32::from(cluster_height)
                    {
                        corridor.insert((u16::try_from(x).ok()?, u16::try_from(z).ok()?));
                    }
                }
            }
        }
        self.path_within_cluster_corridor(navigation, start, goal, component, &corridor)
    }

    fn path_within_cluster_corridor(
        &self,
        navigation: &stream_town_domain::NavGrid,
        start: GridPos,
        goal: GridPos,
        component: u32,
        corridor: &BTreeSet<(u16, u16)>,
    ) -> Option<Vec<GridPos>> {
        if start == goal {
            return Some(vec![start]);
        }
        let allowed = |position: GridPos| {
            self.component_at(position) == Some(component)
                && corridor.contains(&(
                    position.x / ENEMY_NAVIGATION_CLUSTER_SIZE,
                    position.z / ENEMY_NAVIGATION_CLUSTER_SIZE,
                ))
        };
        if !allowed(start) || !allowed(goal) {
            return None;
        }
        let mut open = BinaryHeap::from([EnemyPathOpenNode {
            position: start,
            estimated_total: grid_octile_distance(start, goal),
            cost: 0,
        }]);
        let mut previous = HashMap::<GridPos, GridPos>::new();
        let mut costs = HashMap::<GridPos, u32>::from([(start, 0)]);
        while let Some(current) = open.pop() {
            if current.position == goal {
                let mut route = vec![goal];
                let mut cursor = goal;
                while cursor != start {
                    cursor = previous[&cursor];
                    route.push(cursor);
                }
                route.reverse();
                return Some(route);
            }
            if current.cost > costs[&current.position] {
                continue;
            }
            for (neighbour, step_cost) in navigation
                .walkable_neighbours(current.position)
                .into_iter()
                .flatten()
                .filter(|(position, _)| allowed(*position))
            {
                let next_cost = current.cost.saturating_add(step_cost);
                if next_cost < *costs.get(&neighbour).unwrap_or(&u32::MAX) {
                    costs.insert(neighbour, next_cost);
                    previous.insert(neighbour, current.position);
                    open.push(EnemyPathOpenNode {
                        position: neighbour,
                        estimated_total: next_cost
                            .saturating_add(grid_octile_distance(neighbour, goal)),
                        cost: next_cost,
                    });
                }
            }
        }
        None
    }

    fn path_to(&self, start: GridPos, building: &StableId, goal: GridPos) -> Option<Vec<GridPos>> {
        let (field_building, field_goal) = self.destination(start)?;
        if field_building != building || field_goal != goal {
            return None;
        }
        let mut path = vec![start];
        let mut current = start;
        for _ in 0..self.component_by_cell.len() {
            if current == goal {
                return Some(path);
            }
            let next = self.next_by_cell.get(self.index(current)?)?.as_ref()?;
            if *next == current {
                return None;
            }
            current = *next;
            path.push(current);
        }
        None
    }
}

struct EnemyNavigationTask {
    signature: u64,
    task: Task<EnemyNavigationField>,
}

#[derive(Resource, Default)]
struct EnemyNavigationRuntime {
    field: Option<EnemyNavigationField>,
    task: Option<EnemyNavigationTask>,
}

#[derive(Resource, Default)]
struct SessionStats {
    elapsed_seconds: f64,
    paths_completed: u64,
    commands_processed: u64,
}

#[derive(SystemParam)]
struct MovementStats<'w, 's> {
    session: ResMut<'w, SessionStats>,
    traversal_wear: ResMut<'w, TraversalWearRuntime>,
    path_surfaces: Res<'w, PathSurfaceRuntime>,
    fine_navigation: Res<'w, FineNavigationRuntime>,
    passive_income_elapsed: Local<'s, Duration>,
}

fn advance_agent_simulation_cadence(
    cadence: &mut AgentSimulationCadenceState,
    frame_delta: Duration,
    stream_only: bool,
    active: bool,
) -> bool {
    advance_stream_only_cadence(
        cadence,
        frame_delta,
        stream_only,
        active,
        STREAM_ONLY_AGENT_SIMULATION_HZ,
    )
}

fn advance_stream_only_cadence(
    cadence: &mut AgentSimulationCadenceState,
    frame_delta: Duration,
    stream_only: bool,
    active: bool,
    frequency_hz: u32,
) -> bool {
    if !active {
        *cadence = AgentSimulationCadenceState::default();
        return false;
    }
    if !stream_only {
        cadence.accumulated = Duration::ZERO;
        cadence.delta = frame_delta;
        return true;
    }

    cadence.accumulated = cadence.accumulated.saturating_add(frame_delta);
    let interval = Duration::from_secs_f64(1.0 / f64::from(frequency_hz.max(1)));
    if cadence.accumulated < interval {
        return false;
    }
    cadence.delta = std::mem::take(&mut cadence.accumulated);
    true
}

fn night_light_sync_due(
    time: Res<Time>,
    state: Res<State<GameState>>,
    gameplay_ready: Option<Res<GameplayReady>>,
    config: Res<RuntimeConfig>,
    mut local: Local<AgentSimulationCadenceState>,
) -> bool {
    advance_stream_only_cadence(
        &mut local,
        time.delta(),
        config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly,
        *state.get() == GameState::InGame && gameplay_ready.is_some(),
        STREAM_ONLY_NIGHT_LIGHT_SYNC_HZ,
    )
}

fn agent_simulation_due(
    time: Res<Time>,
    state: Res<State<GameState>>,
    gameplay_ready: Option<Res<GameplayReady>>,
    config: Res<RuntimeConfig>,
    cadence: Res<AgentSimulationCadence>,
    mut local: Local<AgentSimulationCadenceState>,
) -> bool {
    let due = advance_agent_simulation_cadence(
        &mut local,
        time.delta(),
        config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly,
        *state.get() == GameState::InGame && gameplay_ready.is_some(),
    );
    if due {
        cadence.set_delta(local.delta);
    }
    due
}

#[derive(Resource, Default)]
struct WorldRenderStats {
    terrain_high_chunks: usize,
    terrain_medium_chunks: usize,
    terrain_low_chunks: usize,
    foliage_instances: usize,
    foliage_visible_instances: usize,
    foliage_batches: usize,
    foliage_spatial_groups: usize,
    foliage_unbatched_instances: usize,
    crowd_adjusted_agents: usize,
    crowd_yielding_agents: usize,
}

#[derive(Resource, Default)]
struct CrowdSeparationRuntime {
    applied_offsets: HashMap<Entity, Vec3>,
}

#[derive(Clone, Debug, Default)]
struct CachedStationTargets {
    refresh_elapsed_milliseconds: f64,
    target_check_elapsed_milliseconds: f64,
    targets: BTreeMap<StableId, Vec<StableId>>,
    reachability_queue: VecDeque<(StableId, StableId)>,
}

#[derive(Resource, Default)]
struct StationTargetRuntime {
    stations: BTreeMap<StableId, CachedStationTargets>,
    refresh_queue: VecDeque<StableId>,
}

#[derive(Resource, Default)]
struct StationResourceTargetIndex {
    resource_count: usize,
    first_resource: Option<StableId>,
    last_resource: Option<StableId>,
    by_id: HashMap<StableId, usize>,
    by_kind: BTreeMap<StableId, Vec<usize>>,
}

impl StationResourceTargetIndex {
    fn sync(&mut self, world: &GeneratedWorld) {
        let first_resource = world.resources.first().map(|resource| resource.id.clone());
        let last_resource = world.resources.last().map(|resource| resource.id.clone());
        if self.resource_count == world.resources.len()
            && self.first_resource == first_resource
            && self.last_resource == last_resource
        {
            return;
        }
        self.resource_count = world.resources.len();
        self.first_resource = first_resource;
        self.last_resource = last_resource;
        self.by_id.clear();
        self.by_kind.clear();
        self.by_id.reserve(world.resources.len());
        for (index, resource) in world.resources.iter().enumerate() {
            self.by_id.insert(resource.id.clone(), index);
            self.by_kind
                .entry(resource.target_kind.clone())
                .or_default()
                .push(index);
        }
    }
}

#[derive(Clone, Debug)]
struct PendingChatCommand {
    actor_id: StableId,
    login_name: String,
    display_name: String,
    command: ChatCommand,
    is_broadcaster: bool,
    is_moderator: bool,
    is_subscriber: bool,
    origin: CommandOrigin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandOrigin {
    Twitch,
    /// Local tool/keyboard injection mirrors Unity's debug bridge permission bypass.
    LocalDebug,
}

#[derive(Resource, Default)]
struct InjectedCommands(VecDeque<PendingChatCommand>);

#[derive(Resource, Default)]
struct CommandFeedback(String);

#[derive(Resource, Default)]
struct CommandAcknowledgementRuntime {
    sequence: u64,
}

impl CommandAcknowledgementRuntime {
    fn acknowledge(&mut self) {
        self.sequence = self.sequence.saturating_add(1);
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum MenuPage {
    #[default]
    Closed,
    Game,
    GoLiveConfirmation,
    NewTown,
    LoadTown,
    Settings,
    SecretsDisclaimer,
    Secrets,
}

#[derive(Resource, Default)]
pub(crate) struct SensitiveScreenActive(pub bool);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub(crate) struct SensitiveScreenUpdateSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SecretsAuthorizationKind {
    Bot,
    Broadcaster,
}

impl SecretsAuthorizationKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Bot => "bot",
            Self::Broadcaster => "broadcaster",
        }
    }
}

enum SecretsAuthorizationEvent {
    Device {
        kind: SecretsAuthorizationKind,
        authorization: DeviceAuthorization,
    },
    Authorized {
        kind: SecretsAuthorizationKind,
        validation: TokenValidation,
    },
    Error {
        kind: SecretsAuthorizationKind,
        message: String,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum SecretsCredentialState {
    #[default]
    Unknown,
    NotConfigured,
    Missing,
    Stored,
    Error(String),
}

#[derive(Resource, Default)]
struct SecretsRuntime {
    authorization_events: Option<Arc<Mutex<mpsc::Receiver<SecretsAuthorizationEvent>>>>,
    active_authorization: Option<SecretsAuthorizationKind>,
    device: Option<DeviceAuthorization>,
    feedback: String,
    credential_signature: Option<(String, String, String)>,
    bot_credential: SecretsCredentialState,
    broadcaster_credential: SecretsCredentialState,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum SettingsTab {
    #[default]
    Video,
    Audio,
    Gameplay,
    Accessibility,
    Streaming,
    Connection,
}

#[derive(Resource)]
struct MenuRuntime {
    page: MenuPage,
    return_page: MenuPage,
    selected: usize,
    settings_tab: SettingsTab,
    confirm_settings_close: bool,
    draft: PlayerSettings,
    streaming_draft: BroadcastConfig,
    feedback: String,
    pending_town_start: Option<PendingTownStart>,
}

impl Default for MenuRuntime {
    fn default() -> Self {
        Self {
            page: MenuPage::Closed,
            return_page: MenuPage::Closed,
            selected: 0,
            settings_tab: SettingsTab::Video,
            confirm_settings_close: false,
            draft: PlayerSettings::default(),
            streaming_draft: BroadcastConfig::default(),
            feedback: String::new(),
            pending_town_start: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PendingTownStart {
    NewGame,
    LoadGame { source: PathBuf },
}
