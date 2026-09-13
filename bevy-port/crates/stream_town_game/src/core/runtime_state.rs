#[derive(Resource, Default)]
struct SettingsUiCache {
    signature: String,
}

#[derive(Resource, Default)]
struct AccessibilityRuntime {
    synthetic_pressed: Option<Entity>,
    last_announcement: String,
}

#[derive(Component)]
struct AccessibilityAnnouncement;

#[derive(Component)]
struct AccessibilityHighContrastText;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum AccessibleButtonScope {
    MainMenu,
    GoLiveConfirmation,
    GameMenu,
    Settings,
    SettingsConfirm,
    Credits,
}

#[derive(Resource, Clone, Copy)]
struct AccessibilityMotionDefaults {
    tree: Vec4,
    grass: Vec4,
    water: Vec4,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
struct AccessibilityActionDispatch;

#[derive(Resource, Default)]
struct MenuIoRequest {
    save: bool,
    save_jump_start: bool,
    load: bool,
    load_source: Option<PathBuf>,
}

/// Cross-subsystem persistence intent. UI systems may keep their local menu
/// request resource, while runtime integrations publish through this message
/// boundary instead of mutating menu-owned state directly.
#[derive(Clone, Debug, Message)]
enum TownPersistenceRequest {
    Save,
    SaveJumpStart,
    Load(Option<PathBuf>),
}

#[derive(Resource)]
struct RuntimeConsoleRuntime {
    store: RuntimeConsoleStore,
    enabled: bool,
    last_processed_sequence: u64,
    last_result: String,
    status_elapsed_seconds: f32,
}

impl Default for RuntimeConsoleRuntime {
    fn default() -> Self {
        let store = RuntimeConsoleStore::from_environment();
        let last_processed_sequence = store
            .read_request()
            .ok()
            .flatten()
            .map_or(0, |request| request.sequence);
        Self {
            store,
            enabled: std::env::var_os("STREAM_TOWN_RUNTIME_CONSOLE_DIR").is_some()
                || std::env::var_os("STREAM_TOWN_RUNTIME_CONSOLE").is_some(),
            last_processed_sequence,
            last_result: "Runtime console ready".to_owned(),
            status_elapsed_seconds: 0.0,
        }
    }
}

#[derive(Resource, Default)]
struct RuntimeCaptureRequest(bool);

#[derive(Resource, Default)]
struct CameraCommandQueue(VecDeque<CameraRequest>);

#[derive(Resource, Default)]
struct AgentCommandQueue(VecDeque<AgentCommand>);

#[derive(Resource, Default)]
struct BuildingCommandQueue(VecDeque<BuildingRuntimeCommand>);

#[derive(Clone, Debug)]
enum BuildingRuntimeCommand {
    Despawn(StableId),
}

#[derive(Clone, Debug)]
struct BuildingPlacement {
    building: StableId,
    /// `!build thickpath` still constructs ordinary path entities, but previews
    /// and independently attempts one neighbouring fine cell on each side.
    thick_path: bool,
    /// Coarse placement cell used by all authored buildings and retained as the
    /// containing cell for fine-grid paths.
    position: GridPos,
    /// Paths alone are authored directly on the one-third-cell navigation grid.
    navigation_position: Option<GridPos>,
    rotation_quarter_turns: i32,
    line_start: Option<GridPos>,
    line_end: Option<GridPos>,
    /// Ordered fine-grid cells traced after `!beginplace`, allowing paths to
    /// bend through narrow spaces instead of being constrained to one line.
    path_cells: Vec<GridPos>,
    inactivity_seconds: f32,
}

#[derive(Clone, Copy, Debug)]
struct MovementProgress {
    last_position: Vec2,
    last_path_index: usize,
    target: GridPos,
    stalled_seconds: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct NavigationFailureState {
    path_failure_seconds: f32,
    movement: Option<MovementProgress>,
}

#[derive(Resource, Default)]
struct PathFailureRuntime(BTreeMap<StableId, NavigationFailureState>);

impl PathFailureRuntime {
    fn record_failure(&mut self, actor: &StableId, delta_seconds: f32) -> bool {
        let state = self.0.entry(actor.clone()).or_default();
        state.path_failure_seconds += delta_seconds.max(0.0);
        state.path_failure_seconds >= 5.0
    }

    fn clear_path_failure(&mut self, actor: &StableId) {
        if let Some(state) = self.0.get_mut(actor) {
            state.path_failure_seconds = 0.0;
        }
    }

    fn record_movement(
        &mut self,
        actor: &StableId,
        position: Vec2,
        path_index: usize,
        target: GridPos,
        delta_seconds: f32,
        minimum_progress: f32,
    ) -> bool {
        let state = self.0.entry(actor.clone()).or_default();
        let Some(progress) = state.movement.as_mut() else {
            state.movement = Some(MovementProgress {
                last_position: position,
                last_path_index: path_index,
                target,
                stalled_seconds: 0.0,
            });
            return false;
        };
        if progress.target != target
            || progress.last_path_index != path_index
            || progress.last_position.distance_squared(position)
                >= minimum_progress * minimum_progress
        {
            *progress = MovementProgress {
                last_position: position,
                last_path_index: path_index,
                target,
                stalled_seconds: 0.0,
            };
            return false;
        }
        progress.stalled_seconds += delta_seconds.max(0.0);
        progress.stalled_seconds >= 5.0
    }

    fn clear_movement(&mut self, actor: &StableId) {
        if let Some(state) = self.0.get_mut(actor) {
            state.movement = None;
        }
    }

    fn clear(&mut self, actor: &StableId) {
        self.0.remove(actor);
    }
}

#[derive(Resource, Default)]
struct SurfaceErrorRuntime {
    last_error: Option<Instant>,
    consecutive_errors: u8,
}

#[derive(Resource, Default)]
struct BuildingPlacers(BTreeMap<StableId, BuildingPlacement>);

fn expire_inactive_building_placements(time: Res<Time>, mut placers: ResMut<BuildingPlacers>) {
    let delta_seconds = time.delta_secs();
    placers
        .0
        .retain(|_, placement| building_placement_remains_active(placement, delta_seconds));
}

fn building_placement_remains_active(
    placement: &mut BuildingPlacement,
    delta_seconds: f32,
) -> bool {
    placement.inactivity_seconds += delta_seconds.max(0.0);
    placement.inactivity_seconds < BUILDING_PLACEMENT_TIMEOUT_SECONDS
}

#[derive(Clone, Debug, Default)]
struct RegenerationWorkerState {
    initialized: bool,
    next_ready_seconds: f64,
    station_visit_required: bool,
    last_station: Option<StableId>,
    prospector_step: u32,
    planting_sequence: u32,
    planting_target: Option<GridPos>,
    resource_saturation_target: Option<StableId>,
    resource_saturation_debuff: u32,
}

#[derive(Clone, Copy, Debug)]
struct RecentTreePlanting {
    nursery: GridPos,
    position: GridPos,
    planted_at_seconds: f64,
}

#[derive(Resource, Default)]
struct RegenerationRoleRuntime {
    elapsed_seconds: f64,
    next_resource_serial: u64,
    workers: BTreeMap<StableId, RegenerationWorkerState>,
    recently_fallen_trees: VecDeque<GridPos>,
    recent_tree_plantings: VecDeque<RecentTreePlanting>,
}

#[derive(SystemParam)]
struct RuntimeCommandQueues<'w> {
    injected: ResMut<'w, InjectedCommands>,
    camera: ResMut<'w, CameraCommandQueue>,
    agent: ResMut<'w, AgentCommandQueue>,
    building: ResMut<'w, BuildingCommandQueue>,
    placers: ResMut<'w, BuildingPlacers>,
}

#[derive(SystemParam)]
struct CommandResponseRuntime<'w> {
    connection: Res<'w, TwitchConnection>,
    feedback: ResMut<'w, CommandFeedback>,
    acknowledgements: ResMut<'w, CommandAcknowledgementRuntime>,
}

#[derive(SystemParam)]
struct CommandSaveRuntime<'w> {
    save: Res<'w, SaveRuntime>,
    traversal_wear: Res<'w, TraversalWearRuntime>,
}

#[derive(SystemParam)]
struct CommandWorldViewRuntime<'w, 's> {
    selected: Res<'w, SelectedCell>,
    cameras: Query<
        'w,
        's,
        (
            &'static Camera,
            &'static GlobalTransform,
            &'static TownCameraControllerRuntime,
        ),
        With<TownCamera>,
    >,
    spatial: Option<SpatialQuery<'w, 's>>,
    fine_navigation: Res<'w, FineNavigationRuntime>,
    diagnostic: ResMut<'w, WorldDiagnosticRuntime>,
}

#[derive(Clone, Debug)]
struct CameraRequest {
    reset: bool,
    actions: Vec<CameraAction>,
    follow: Option<StableId>,
    focus_building: Option<StableId>,
}

#[derive(Clone, Debug)]
enum AgentCommand {
    Teleport { actor: StableId, position: GridPos },
    Ping(StableId),
    Despawn(StableId),
}

#[derive(Resource)]
struct TwitchConnection {
    transport: Option<TwitchTransport>,
    status: TwitchStatus,
    moderation_status: TwitchModerationStatus,
    fish_god_reward_id: Option<String>,
}

const OPERATOR_CHAT_HISTORY_CAPACITY: usize = 200;

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperatorChatBadges {
    broadcaster: bool,
    moderator: bool,
    subscriber: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperatorChatLine {
    line_id: u64,
    user_id: String,
    login: String,
    display_name: String,
    message: String,
    badges: OperatorChatBadges,
    is_system: bool,
}

#[derive(Resource, Default)]
struct OperatorChatRuntime {
    lines: VecDeque<OperatorChatLine>,
    seen_message_ids: VecDeque<String>,
    next_line_id: u64,
    selected_line: Option<u64>,
    scroll_from_latest: usize,
    draft: String,
    input_focused: bool,
    selected_user: Option<(String, String)>,
    feedback: String,
}

impl OperatorChatRuntime {
    fn push(&mut self, mut line: OperatorChatLine) {
        self.next_line_id = self.next_line_id.wrapping_add(1).max(1);
        line.line_id = self.next_line_id;
        if self.scroll_from_latest > 0 {
            self.scroll_from_latest = self.scroll_from_latest.saturating_add(1);
        }
        self.lines.push_back(line);
        while self.lines.len() > OPERATOR_CHAT_HISTORY_CAPACITY {
            if let Some(removed) = self.lines.pop_front()
                && self.selected_line == Some(removed.line_id)
            {
                self.selected_line = None;
                self.selected_user = None;
            }
        }
    }

    fn push_chat(&mut self, message: &twitch::TwitchChatEnvelope) -> bool {
        if let Some(message_id) = message.message_id.as_ref() {
            if self.seen_message_ids.contains(message_id) {
                return false;
            }
            self.seen_message_ids.push_back(message_id.clone());
            while self.seen_message_ids.len() > OPERATOR_CHAT_HISTORY_CAPACITY {
                self.seen_message_ids.pop_front();
            }
        }
        self.push(OperatorChatLine {
            line_id: 0,
            user_id: message.user_id.clone(),
            login: message.login.clone(),
            display_name: message.display_name.clone(),
            message: message.message.clone(),
            badges: OperatorChatBadges {
                broadcaster: message.is_broadcaster,
                moderator: message.is_moderator,
                subscriber: message.is_subscriber,
            },
            is_system: false,
        });
        true
    }

    fn maximum_scroll(&self, visible_rows: usize) -> usize {
        self.lines.len().saturating_sub(visible_rows)
    }

    fn scroll_older(&mut self, rows: usize, visible_rows: usize) {
        self.scroll_from_latest = self
            .scroll_from_latest
            .saturating_add(rows)
            .min(self.maximum_scroll(visible_rows));
    }

    fn scroll_newer(&mut self, rows: usize) {
        self.scroll_from_latest = self.scroll_from_latest.saturating_sub(rows);
    }

    fn visible_lines(&self, visible_rows: usize) -> Vec<&OperatorChatLine> {
        let end = self.lines.len().saturating_sub(
            self.scroll_from_latest
                .min(self.maximum_scroll(visible_rows)),
        );
        let start = end.saturating_sub(visible_rows);
        self.lines.iter().skip(start).take(end - start).collect()
    }

    fn push_system(&mut self, message: impl Into<String>) {
        self.push(OperatorChatLine {
            line_id: 0,
            user_id: String::new(),
            login: "stream_town".to_owned(),
            display_name: "Stream Town".to_owned(),
            message: message.into(),
            badges: OperatorChatBadges {
                broadcaster: false,
                moderator: false,
                subscriber: false,
            },
            is_system: true,
        });
    }
}

impl Default for TwitchConnection {
    fn default() -> Self {
        Self {
            transport: None,
            status: TwitchStatus::Disabled,
            moderation_status: TwitchModerationStatus::Disabled,
            fish_god_reward_id: None,
        }
    }
}

#[derive(Resource, Default)]
struct SelectedCell(Option<GridPos>);

#[derive(Resource, Default)]
struct SelectedActor(Option<StableId>);

/// Opt-in marker for the retained pointer-to-object resolver. Shipping gameplay
/// never inserts this resource: text commands own interaction, while the
/// selection model stays available for future automatic camera targeting.
#[derive(Resource)]
struct PointerObjectSelectionEnabled;

#[derive(Resource, Default)]
struct EnvironmentPresentation {
    environment: Option<(Season, Weather)>,
    daylight_bits: Option<u32>,
    season_blend_bits: Option<u32>,
}

#[derive(Resource, Default)]
struct PostProcessPresentation {
    applied: Option<(GameState, u32, i32, i32)>,
}

#[derive(Resource, Default)]
struct BuildingMaterialInstances(BTreeMap<StableId, BuildingMaterialInstance>);

#[derive(Resource, Default)]
struct BuildingMaterialUpdateRuntime {
    order: Vec<StableId>,
    next_index: usize,
}

fn round_robin_indices(total: usize, start: usize) -> impl Iterator<Item = usize> {
    (0..total).map(move |offset| (start + offset) % total)
}

struct BuildingMaterialInstance {
    handle: Handle<BuildingMaterial>,
    applied_health: i32,
    applied_season: Season,
    applied_season_blend_bits: u32,
    applied_time_cycle: BuildingTimeCycleSignature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BuildingTimeCycleSignature {
    seconds_per_day: u32,
    daylight_per_thousand: u16,
    transition_seconds: u32,
    max_building_emission_milli: u16,
}

impl From<&stream_town_domain::TimeCycleConfig> for BuildingTimeCycleSignature {
    fn from(config: &stream_town_domain::TimeCycleConfig) -> Self {
        Self {
            seconds_per_day: config.seconds_per_day,
            daylight_per_thousand: config.daylight_per_thousand,
            transition_seconds: config.transition_seconds,
            max_building_emission_milli: config.max_building_emission_milli,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect, ShaderType)]
struct TerrainMaterialUniform {
    sand_color_a: Vec4,
    sand_color_b: Vec4,
    grass_color_a: Vec4,
    grass_color_b: Vec4,
    season_tint: Vec4,
    texture_uv_blend_tint: Vec4,
    grid_scale_offset: Vec4,
    selection_center_extent: Vec4,
    selection_color: Vec4,
    traversal_grid: Vec4,
    path_grid: Vec4,
    traversal_dirt_color: Vec4,
    constructed_path_color: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct TerrainMaterialExtension {
    #[uniform(100)]
    parameters: TerrainMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    grid_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    traversal_wear_texture: Option<Handle<Image>>,
    #[texture(105)]
    #[sampler(106)]
    path_surface_texture: Option<Handle<Image>>,
}

impl MaterialExtension for TerrainMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        TERRAIN_SHADER_ASSET_PATH.into()
    }
}

type TerrainMaterial = ExtendedMaterial<StandardMaterial, TerrainMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct WaterMaterialUniform {
    surface_color: Vec4,
    deep_color: Vec4,
    foam_color: Vec4,
    ice_color: Vec4,
    wind_speed_noise_alpha: Vec4,
    scale_foam_ice: Vec4,
    season_tint: Vec4,
    main_scale_offset: Vec4,
    noise_scale_offset: Vec4,
    depth_foam_controls: Vec4,
    opacity_controls: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct WaterMaterialExtension {
    #[uniform(100)]
    parameters: WaterMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for WaterMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        WATER_SHADER_ASSET_PATH.into()
    }
}

type WaterMaterial = ExtendedMaterial<StandardMaterial, WaterMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct BuildingMaterialUniform {
    detail_color: Vec4,
    emissive_color: Vec4,
    ambient_occlusion: Vec4,
    surface_controls: Vec4,
    snow_damage: Vec4,
    main_scale_offset: Vec4,
    tint_color_strength: Vec4,
    time_cycle: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct BuildingMaterialExtension {
    #[uniform(100)]
    parameters: BuildingMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
}

impl MaterialExtension for BuildingMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        BUILDING_SHADER_ASSET_PATH.into()
    }
}

type BuildingMaterial = ExtendedMaterial<StandardMaterial, BuildingMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct CloudMaterialUniform {
    noise_controls: Vec4,
    surface_transform: Vec4,
    filter_controls: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct CloudMaterialExtension {
    #[uniform(100)]
    parameters: CloudMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for CloudMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        CLOUD_SHADER_ASSET_PATH.into()
    }
}

type CloudMaterial = ExtendedMaterial<StandardMaterial, CloudMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct MenuSkyMaterialUniform {
    horizon_color: Vec4,
    zenith_color: Vec4,
    haze_color: Vec4,
    sun_direction_strength: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct MenuSkyMaterialExtension {
    #[uniform(100)]
    parameters: MenuSkyMaterialUniform,
}

impl MaterialExtension for MenuSkyMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        MENU_SKY_SHADER_ASSET_PATH.into()
    }
}

type MenuSkyMaterial = ExtendedMaterial<StandardMaterial, MenuSkyMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct GodrayMaterialUniform {
    emission_alpha: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct GodrayMaterialExtension {
    #[uniform(100)]
    parameters: GodrayMaterialUniform,
}

impl MaterialExtension for GodrayMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        GODRAY_SHADER_ASSET_PATH.into()
    }
}

type GodrayMaterial = ExtendedMaterial<StandardMaterial, GodrayMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct GiraffeMaterialUniform {
    animation_controls: Vec4,
    mask_controls: Vec4,
    rotation_controls: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct GiraffeMaterialExtension {
    #[uniform(100)]
    parameters: GiraffeMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
}

impl MaterialExtension for GiraffeMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        GIRAFFE_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        GIRAFFE_SHADER_ASSET_PATH.into()
    }
}

type GiraffeMaterial = ExtendedMaterial<StandardMaterial, GiraffeMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct BoundsMaterialUniform {
    color_alpha: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct BoundsMaterialExtension {
    #[uniform(100)]
    parameters: BoundsMaterialUniform,
}

impl MaterialExtension for BoundsMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        BOUNDS_SHADER_ASSET_PATH.into()
    }
}

type BoundsMaterial = ExtendedMaterial<StandardMaterial, BoundsMaterialExtension>;

#[derive(SystemParam)]
struct SpecialtyMaterialAssets<'w> {
    giraffe: Option<ResMut<'w, Assets<GiraffeMaterial>>>,
    bounds: Option<ResMut<'w, Assets<BoundsMaterial>>>,
    character: Option<ResMut<'w, Assets<CharacterMaterial>>>,
}

#[derive(SystemParam)]
struct SceneMaterialAssets<'w> {
    cloud: Option<ResMut<'w, Assets<CloudMaterial>>>,
    menu_sky: Option<ResMut<'w, Assets<MenuSkyMaterial>>>,
    godray: Option<ResMut<'w, Assets<GodrayMaterial>>>,
}

#[derive(SystemParam)]
struct CoreRenderAssets<'w> {
    images: Option<ResMut<'w, Assets<Image>>>,
    meshes: Option<ResMut<'w, Assets<Mesh>>>,
    materials: Option<ResMut<'w, Assets<StandardMaterial>>>,
}

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct TreeMaterialUniform {
    wind_direction_smoothness: Vec4,
    wind_controls: Vec4,
    season_controls: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct TreeMaterialExtension {
    #[uniform(100)]
    parameters: TreeMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for TreeMaterialExtension {
    fn enable_shadows() -> bool {
        true
    }

    fn vertex_shader() -> ShaderRef {
        TREE_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        TREE_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        TREE_PREPASS_SHADER_ASSET_PATH.into()
    }
}

type TreeMaterial = ExtendedMaterial<StandardMaterial, TreeMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct GrassMaterialUniform {
    grid_color_1: Vec4,
    grid_color_2: Vec4,
    wind_color: Vec4,
    wind_direction_smoothness: Vec4,
    wind_controls: Vec4,
    surface_controls: Vec4,
    world_strength_transform: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct GrassMaterialExtension {
    #[uniform(100)]
    parameters: GrassMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for GrassMaterialExtension {
    fn enable_shadows() -> bool {
        true
    }

    fn vertex_shader() -> ShaderRef {
        GRASS_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        GRASS_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        GRASS_PREPASS_SHADER_ASSET_PATH.into()
    }
}

type GrassMaterial = ExtendedMaterial<StandardMaterial, GrassMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct CritterMaterialUniform {
    animation_controls: Vec4,
    main_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct CritterMaterialExtension {
    #[uniform(100)]
    parameters: CritterMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    main_texture: Option<Handle<Image>>,
}

impl MaterialExtension for CritterMaterialExtension {
    fn enable_shadows() -> bool {
        true
    }

    fn vertex_shader() -> ShaderRef {
        CRITTER_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        CRITTER_PREPASS_SHADER_ASSET_PATH.into()
    }
}

type CritterMaterial = ExtendedMaterial<StandardMaterial, CritterMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct FlagMaterialUniform {
    colour_1: Vec4,
    colour_2: Vec4,
    controls: Vec4,
    noise_scale_offset: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct FlagMaterialExtension {
    #[uniform(100)]
    parameters: FlagMaterialUniform,
    #[texture(101)]
    #[sampler(102)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for FlagMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        FLAG_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        FLAG_SHADER_ASSET_PATH.into()
    }
}

type FlagMaterial = ExtendedMaterial<StandardMaterial, FlagMaterialExtension>;

#[derive(Clone, Copy, Debug, Reflect, ShaderType)]
struct CharacterMaterialUniform {
    albedo_color: Vec4,
    shadow_controls: Vec4,
}

#[derive(Asset, AsBindGroup, Clone, Debug, Reflect)]
struct CharacterMaterialExtension {
    #[uniform(100)]
    parameters: CharacterMaterialUniform,
}

impl MaterialExtension for CharacterMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        CHARACTER_SHADER_ASSET_PATH.into()
    }
}

type CharacterMaterial = ExtendedMaterial<StandardMaterial, CharacterMaterialExtension>;

#[derive(Clone)]
enum ResolvedMaterialHandle {
    Standard(Handle<StandardMaterial>),
    Building(Handle<BuildingMaterial>),
    Cloud(Handle<CloudMaterial>),
    Godray(Handle<GodrayMaterial>),
    Giraffe(Handle<GiraffeMaterial>),
    Bounds(Handle<BoundsMaterial>),
    Tree(Handle<TreeMaterial>),
    Grass(Handle<GrassMaterial>),
    Critter(Handle<CritterMaterial>),
    Flag(Handle<FlagMaterial>),
    Character(Handle<CharacterMaterial>),
}

impl ResolvedMaterialHandle {
    fn untyped_id(&self) -> UntypedAssetId {
        match self {
            Self::Standard(handle) => handle.id().untyped(),
            Self::Building(handle) => handle.id().untyped(),
            Self::Cloud(handle) => handle.id().untyped(),
            Self::Godray(handle) => handle.id().untyped(),
            Self::Giraffe(handle) => handle.id().untyped(),
            Self::Bounds(handle) => handle.id().untyped(),
            Self::Tree(handle) => handle.id().untyped(),
            Self::Grass(handle) => handle.id().untyped(),
            Self::Critter(handle) => handle.id().untyped(),
            Self::Flag(handle) => handle.id().untyped(),
            Self::Character(handle) => handle.id().untyped(),
        }
    }
}

#[derive(Resource, Default)]
struct RenderAssets {
    cube: Handle<Mesh>,
    chimney_particle: Handle<Mesh>,
    actor_lod: Handle<Mesh>,
    menu_sky_mesh: Handle<Mesh>,
    cloud_plane: Handle<Mesh>,
    healing_ring: Handle<Mesh>,
    healing_plus: Option<Handle<Mesh>>,
    fish_school_mesh: Option<Handle<Mesh>>,
    fish_school_material: Handle<CritterMaterial>,
    projectile_arrow_scene: Option<Handle<bevy::world_serialization::WorldAsset>>,
    ground: Handle<TerrainMaterial>,
    traversal_wear: Handle<Image>,
    path_surface: Handle<Image>,
    water: Handle<WaterMaterial>,
    menu_water: Handle<WaterMaterial>,
    menu_sky: Handle<MenuSkyMaterial>,
    wood: Handle<StandardMaterial>,
    ore: Handle<StandardMaterial>,
    food: Handle<StandardMaterial>,
    building: Handle<StandardMaterial>,
    construction: Handle<StandardMaterial>,
    streetlight_lamp: Handle<StandardMaterial>,
    regeneration_buildings: BTreeMap<String, Handle<StandardMaterial>>,
    placement_valid: Handle<BoundsMaterial>,
    placement_invalid: Handle<BoundsMaterial>,
    enemy_idle: Handle<StandardMaterial>,
    enemy_moving: Handle<StandardMaterial>,
    player_idle: Handle<StandardMaterial>,
    player_moving: Handle<StandardMaterial>,
    selection: Handle<StandardMaterial>,
    rain: Handle<StandardMaterial>,
    snow: Handle<StandardMaterial>,
    projectile: Handle<StandardMaterial>,
    projectile_arrow: Handle<StandardMaterial>,
    projectile_necrotic: Handle<StandardMaterial>,
    impact_physical: Handle<StandardMaterial>,
    building_smoke: Handle<StandardMaterial>,
    chimney_smoke: BTreeMap<StableId, [Vec<Handle<StandardMaterial>>; 2]>,
    building_spark: Handle<StandardMaterial>,
    building_fire: Handle<StandardMaterial>,
    building_upgrade: Handle<StandardMaterial>,
    healing_green: Handle<StandardMaterial>,
    healing_channel: BTreeMap<StableId, Handle<StandardMaterial>>,
    healing_plus_materials: BTreeMap<StableId, Vec<Handle<StandardMaterial>>>,
    healing_disc_materials: BTreeMap<StableId, Vec<Handle<StandardMaterial>>>,
    authored_building: Handle<BuildingMaterial>,
    clouds: Handle<CloudMaterial>,
    menu_cloud: Handle<StandardMaterial>,
    menu_ocean_floor: Handle<StandardMaterial>,
    tree: Handle<TreeMaterial>,
    menu_tree: Handle<StandardMaterial>,
    grass: Handle<GrassMaterial>,
    game_logo: Option<Handle<Image>>,
    loading_screen: Option<Handle<Image>>,
    loading_overlay: Option<Handle<Image>>,
    loading_icon: Option<Handle<Image>>,
    ui_font: Option<Handle<Font>>,
    ui_display_font: Option<Handle<Font>>,
    main_menu_textures: BTreeMap<String, Handle<Image>>,
    top_bar_textures: BTreeMap<String, Handle<Image>>,
    selection_panel_textures: BTreeMap<String, Handle<Image>>,
    vote_textures: BTreeMap<String, Handle<Image>>,
    objective_textures: BTreeMap<String, Handle<Image>>,
    current_event_textures: BTreeMap<String, Handle<Image>>,
    ui_slicers: BTreeMap<String, TextureSlicer>,
    main_ui_scale: f32,
    settings_ui_scale: f32,
    presentation_materials: BTreeMap<StableId, ResolvedMaterialHandle>,
    presentation_materials_by_source_path: BTreeMap<String, ResolvedMaterialHandle>,
}

impl RenderAssets {
    fn world_gpu_mesh_ids(&self) -> BTreeSet<AssetId<Mesh>> {
        [self.cube.id(), self.actor_lod.id(), self.cloud_plane.id()]
            .into_iter()
            .collect()
    }

    fn world_gpu_material_ids(&self) -> BTreeSet<UntypedAssetId> {
        let mut ids = self
            .presentation_materials
            .values()
            .map(ResolvedMaterialHandle::untyped_id)
            .collect::<BTreeSet<_>>();
        ids.extend([
            self.ground.id().untyped(),
            self.water.id().untyped(),
            self.wood.id().untyped(),
            self.ore.id().untyped(),
            self.food.id().untyped(),
            self.building.id().untyped(),
            self.construction.id().untyped(),
            self.enemy_idle.id().untyped(),
            self.enemy_moving.id().untyped(),
            self.player_idle.id().untyped(),
            self.player_moving.id().untyped(),
            self.selection.id().untyped(),
            self.authored_building.id().untyped(),
            self.tree.id().untyped(),
            self.grass.id().untyped(),
            self.fish_school_material.id().untyped(),
        ]);
        ids
    }
}

#[derive(SystemParam)]
struct ActiveMaterialHandles<'w, 's> {
    standard: Query<'w, 's, &'static MeshMaterial3d<StandardMaterial>>,
    terrain: Query<'w, 's, &'static MeshMaterial3d<TerrainMaterial>>,
    water: Query<'w, 's, &'static MeshMaterial3d<WaterMaterial>>,
    building: Query<'w, 's, &'static MeshMaterial3d<BuildingMaterial>>,
    cloud: Query<'w, 's, &'static MeshMaterial3d<CloudMaterial>>,
    godray: Query<'w, 's, &'static MeshMaterial3d<GodrayMaterial>>,
    giraffe: Query<'w, 's, &'static MeshMaterial3d<GiraffeMaterial>>,
    bounds: Query<'w, 's, &'static MeshMaterial3d<BoundsMaterial>>,
    tree: Query<'w, 's, &'static MeshMaterial3d<TreeMaterial>>,
    grass: Query<'w, 's, &'static MeshMaterial3d<GrassMaterial>>,
    critter: Query<'w, 's, &'static MeshMaterial3d<CritterMaterial>>,
    flag: Query<'w, 's, &'static MeshMaterial3d<FlagMaterial>>,
    character: Query<'w, 's, &'static MeshMaterial3d<CharacterMaterial>>,
}

impl ActiveMaterialHandles<'_, '_> {
    fn ids(&self) -> BTreeSet<UntypedAssetId> {
        let mut ids = BTreeSet::new();
        ids.extend(
            self.standard
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(
            self.terrain
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.water.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.building
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.cloud.iter().map(|material| material.0.id().untyped()));
        ids.extend(self.godray.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.giraffe
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.bounds.iter().map(|material| material.0.id().untyped()));
        ids.extend(self.tree.iter().map(|material| material.0.id().untyped()));
        ids.extend(self.grass.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.critter
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids.extend(self.flag.iter().map(|material| material.0.id().untyped()));
        ids.extend(
            self.character
                .iter()
                .map(|material| material.0.id().untyped()),
        );
        ids
    }
}

#[derive(Component)]
struct StateEntity;

#[derive(Component)]
struct UiDisplayFont;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum MainMenuAction {
    NewGame,
    LoadGame,
    Settings,
    Secrets,
    Credits,
    Quit,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum GoLiveConfirmationAction {
    No,
    Yes,
}

#[derive(Component)]
struct GoLiveConfirmationRoot;

#[derive(Component)]
struct GoLiveConfirmationBody;

#[derive(Component)]
struct TownDialogRoot(MenuPage);

#[derive(Component)]
struct TownNameField;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum TownDialogAction {
    Create,
    Back,
}

#[derive(Component)]
struct TownLoadChoice {
    name: String,
    path: PathBuf,
    protected: bool,
}

#[derive(Component)]
struct TownDialogFeedback;

#[derive(Component)]
struct TownLoadList;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum GameMenuAction {
    SaveGame,
    SaveJumpStart,
    LoadGame,
    Settings,
    GoLive,
    ExitGame,
    Close,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SettingsAction {
    Apply,
    Defaults,
    Back,
    ConfirmApply,
    ConfirmDiscard,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsTabButton(SettingsTab);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsValueButton {
    index: usize,
    direction: i8,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsValueRow(usize);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
struct SettingsValueText(usize);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SecretsAction {
    DisclaimerYes,
    DisclaimerNo,
    ToggleBot,
    ToggleBroadcast,
    ToggleBandwidthTest,
    Save,
    AuthorizeBot,
    AuthorizeBroadcaster,
    Back,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SecretsField {
    ClientId,
    BotLogin,
    ChannelLogin,
}

#[derive(Component)]
struct SecretsDynamicLabel(SecretsAction);

#[derive(Component)]
struct SecretsDisclaimerRoot;

#[derive(Component)]
struct SecretsRoot;

#[derive(Component)]
struct SecretsStatusText;

#[derive(Component)]
struct SecretsDeviceText;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum SecretsConnectionKind {
    Bot,
    Broadcast,
}

#[derive(Component)]
struct SecretsConnectionText(SecretsConnectionKind);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SecretsStatusTone {
    Good,
    Pending,
    Inactive,
    Error,
}

#[derive(Component)]
struct SettingsRoot;

#[derive(Component)]
struct SettingsPanel;

#[derive(Component)]
struct SettingsRows;

#[derive(Component)]
struct SettingsConfirmModal;

#[derive(Component)]
struct SettingsFeedbackText;

#[derive(Component)]
struct GameMenuRoot;

#[derive(Component)]
struct GameMenuActionLabel(GameMenuAction);

#[derive(Component)]
struct LoadingScreenEntity;

#[derive(Component)]
struct LoadingUiCamera;

#[derive(Component)]
struct LoadingIconSpinner {
    radians_per_second: f32,
}

#[derive(Component)]
struct LoadingStatusText;

#[derive(Component)]
struct LoadingSubstatusText;

#[derive(Component)]
struct LoadingPercentText;

#[derive(Component)]
struct LoadingProgressFill;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WorldLoadingPhase {
    #[default]
    Presenting,
    Loading,
    Spawning,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum LoadingWork {
    #[default]
    Pending,
    Count {
        completed: u64,
        total: u64,
    },
}

impl LoadingWork {
    fn count(completed: usize, total: usize) -> Self {
        Self::Count {
            completed: u64::try_from(completed).expect("loading work count fits u64"),
            total: u64::try_from(total).expect("loading work total fits u64"),
        }
    }

    fn boolean(completed: bool) -> Self {
        Self::Count {
            completed: u64::from(completed),
            total: 1,
        }
    }

    fn fraction(self) -> f32 {
        match self {
            Self::Pending => 0.0,
            Self::Count { total: 0, .. } => 1.0,
            Self::Count { completed, total } => {
                let completed =
                    u16::try_from(completed.min(total)).expect("loading leaf count fits u16");
                let total = u16::try_from(total).expect("loading leaf total fits u16");
                f32::from(completed) / f32::from(total)
            }
        }
    }
}

#[derive(Clone, Debug)]
enum LoadingWorkNode {
    Leaf(LoadingWork),
    Group(Vec<Self>),
}

impl LoadingWorkNode {
    fn leaf(work: LoadingWork) -> Self {
        Self::Leaf(work)
    }

    fn group(children: impl IntoIterator<Item = Self>) -> Self {
        Self::Group(children.into_iter().collect())
    }

    fn fraction(&self) -> f32 {
        match self {
            Self::Leaf(work) => work.fraction(),
            Self::Group(children) if children.is_empty() => 1.0,
            Self::Group(children) => {
                let child_count =
                    u16::try_from(children.len()).expect("loading group size fits u16");
                children.iter().map(Self::fraction).sum::<f32>() / f32::from(child_count)
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct WorldLoadingWork {
    cover_entities: LoadingWork,
    cover_artwork: LoadingWork,
    cover_gpu_images: LoadingWork,
    cover_pipelines: LoadingWork,
    cover_presented_frames: LoadingWork,
    world_assets: LoadingWork,
    generation_completed: BTreeSet<WorldGenerationStage>,
    terrain_entities: LoadingWork,
    resource_entities: LoadingWork,
    foliage_entities: LoadingWork,
    actor_entities: LoadingWork,
    gameplay_setup: LoadingWork,
    scene_roots: LoadingWork,
    scene_stable_frames: LoadingWork,
    material_overrides: LoadingWork,
    animation_receivers: LoadingWork,
    lighting_receivers: LoadingWork,
    gpu_images: LoadingWork,
    gpu_meshes: LoadingWork,
    gpu_materials: LoadingWork,
    gpu_pipelines: LoadingWork,
    selection_draw: LoadingWork,
    gpu_stable_frames: LoadingWork,
}

impl WorldLoadingWork {
    fn mark_cover_complete(&mut self) {
        self.cover_entities = LoadingWork::boolean(true);
        self.cover_artwork = LoadingWork::boolean(true);
        self.cover_gpu_images = LoadingWork::boolean(true);
        self.cover_pipelines = LoadingWork::boolean(true);
        self.cover_presented_frames = LoadingWork::boolean(true);
    }

    fn generation_node(&self) -> LoadingWorkNode {
        LoadingWorkNode::group(WorldGenerationStage::ALL.into_iter().map(|stage| {
            LoadingWorkNode::leaf(LoadingWork::boolean(
                self.generation_completed.contains(&stage),
            ))
        }))
    }

    fn root(&self) -> LoadingWorkNode {
        let leaf = LoadingWorkNode::leaf;
        LoadingWorkNode::group([
            LoadingWorkNode::group([
                leaf(self.cover_entities),
                leaf(self.cover_artwork),
                leaf(self.cover_gpu_images),
                leaf(self.cover_pipelines),
                leaf(self.cover_presented_frames),
            ]),
            LoadingWorkNode::group([leaf(self.world_assets), self.generation_node()]),
            LoadingWorkNode::group([
                LoadingWorkNode::group([
                    leaf(self.terrain_entities),
                    leaf(self.resource_entities),
                    leaf(self.foliage_entities),
                    leaf(self.actor_entities),
                    leaf(self.gameplay_setup),
                ]),
                leaf(self.scene_roots),
                leaf(self.scene_stable_frames),
            ]),
            LoadingWorkNode::group([
                leaf(self.material_overrides),
                leaf(self.animation_receivers),
                leaf(self.lighting_receivers),
            ]),
            LoadingWorkNode::group([
                leaf(self.gpu_images),
                leaf(self.gpu_meshes),
                leaf(self.gpu_materials),
                leaf(self.gpu_pipelines),
                leaf(self.selection_draw),
                leaf(self.gpu_stable_frames),
            ]),
        ])
    }

    fn progress(&self) -> f32 {
        self.root().fraction().clamp(0.0, 1.0)
    }
}

#[derive(Resource, Default)]
struct WorldLoadingRuntime {
    phase: WorldLoadingPhase,
    progress: f32,
    status: String,
    substatus: String,
    asset_handles: Vec<UntypedHandle>,
    loaded_assets: usize,
    failed_assets: usize,
    prepared_world: Option<PreparedWorld>,
    spawn_runtime: Option<WorldSpawnRuntime>,
    presented_frames: Option<PresentedRenderFrames>,
    scene_ready_frames: u8,
    completion_remaining_seconds: f32,
    work: WorldLoadingWork,
}

struct PreparedWorld {
    world: GeneratedWorld,
    terrain_mesh: Option<Mesh>,
    terrain_collider: Option<Collider>,
    water_mesh: Option<Mesh>,
}
