#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorldSpawnPhase {
    Resources,
    Foliage,
    Gameplay,
}

struct WorldSpawnRuntime {
    phase: WorldSpawnPhase,
    resource_cursor: usize,
    foliage_cursor: usize,
    resource_total: usize,
    foliage_total: usize,
    foliage_gpu_batches: BTreeSet<(StableId, u16)>,
    foliage_spatial_groups: BTreeSet<FoliageBatchKey>,
    update_count: u32,
    starting_render_frame: u64,
}

#[derive(Resource)]
struct WorldLoadingCoverRuntime {
    started_at: Instant,
    initialized: bool,
    starting_render_frame: u64,
    fallback_updates: u8,
    ready_updates: u8,
    transition_queued: bool,
}

impl Default for WorldLoadingCoverRuntime {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            initialized: false,
            starting_render_frame: 0,
            fallback_updates: 0,
            ready_updates: 0,
            transition_queued: false,
        }
    }
}

#[derive(Resource)]
struct WorldGenerationTask {
    task: Task<PreparedWorld>,
    completed: Arc<Mutex<BTreeSet<WorldGenerationStage>>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BootDestination {
    MainMenu,
    WorldLoading,
    Credits,
}

#[derive(Resource)]
struct MenuLoadingRuntime {
    started_at: Instant,
    destination: BootDestination,
    progress: f32,
    status: String,
    substatus: String,
    asset_handles: Vec<UntypedHandle>,
    loaded_assets: usize,
    failed_assets: usize,
    ready_presented_frames: u8,
}

#[derive(Resource)]
struct MenuRevealRuntime {
    started_at: Instant,
    starting_render_frame: u64,
    ready_starting_render_frame: Option<u64>,
    fallback_updates: u16,
    scene_ready_frames: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MainMenuSpawnPhase {
    Models,
    Resources,
    Foliage,
    Complete,
}

#[derive(Resource)]
struct MainMenuSpawnRuntime {
    phase: MainMenuSpawnPhase,
    model_indices: Vec<usize>,
    resource_indices: Vec<usize>,
    foliage_indices: Vec<usize>,
    cursor: usize,
    completed: usize,
    total: usize,
    update_count: u32,
    starting_render_frame: u64,
}

#[derive(Resource)]
struct WorldRevealRuntime {
    started_at: Instant,
    starting_render_frame: u64,
    ready_starting_render_frame: Option<u64>,
    completion_starting_render_frame: Option<u64>,
    fallback_updates: u16,
    ready_frames: u8,
    simulation_elapsed_seconds: f64,
    session_elapsed_seconds: f64,
}

impl Default for WorldRevealRuntime {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            starting_render_frame: 0,
            ready_starting_render_frame: None,
            completion_starting_render_frame: None,
            fallback_updates: 0,
            ready_frames: 0,
            simulation_elapsed_seconds: 0.0,
            session_elapsed_seconds: 0.0,
        }
    }
}

impl Default for MenuRevealRuntime {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            starting_render_frame: 0,
            ready_starting_render_frame: None,
            fallback_updates: 0,
            scene_ready_frames: 0,
        }
    }
}

#[derive(Resource, Clone)]
struct PresentedRenderFrames {
    count: Arc<AtomicU64>,
    render_schedule_available: bool,
}

impl PresentedRenderFrames {
    fn new(render_schedule_available: bool) -> Self {
        Self {
            count: Arc::new(AtomicU64::new(0)),
            render_schedule_available,
        }
    }

    fn current(&self) -> u64 {
        self.count.load(AtomicOrdering::Acquire)
    }
}

fn record_presented_render_frame(frames: Res<PresentedRenderFrames>) {
    frames.count.fetch_add(1, AtomicOrdering::Release);
}

#[derive(Clone, Default)]
struct GpuReadinessExpected {
    active: bool,
    epoch: u64,
    images: BTreeSet<AssetId<Image>>,
    meshes: BTreeSet<AssetId<Mesh>>,
    materials: BTreeSet<UntypedAssetId>,
    selection_entity: Option<Entity>,
}

#[derive(Clone, Copy, Debug, Default)]
struct GpuReadinessSnapshot {
    epoch: u64,
    expected_images: usize,
    ready_images: usize,
    expected_meshes: usize,
    ready_meshes: usize,
    expected_materials: usize,
    ready_materials: usize,
    pending_pipelines: usize,
    failed_pipelines: usize,
    selection_expected: bool,
    selection_draw_ready: bool,
}

impl GpuReadinessSnapshot {
    fn is_ready(self) -> bool {
        self.epoch != 0
            && self.ready_images == self.expected_images
            && self.ready_meshes == self.expected_meshes
            && self.ready_materials == self.expected_materials
            && self.pending_pipelines == 0
            && self.failed_pipelines == 0
            && (!self.selection_expected || self.selection_draw_ready)
    }
}

#[derive(Default)]
struct GpuReadinessShared {
    expected: GpuReadinessExpected,
    snapshot: GpuReadinessSnapshot,
}

#[derive(Clone, Resource, Default)]
struct GpuReadinessProbe(Arc<Mutex<GpuReadinessShared>>);

impl GpuReadinessProbe {
    fn begin_world(
        &self,
        images: BTreeSet<AssetId<Image>>,
        meshes: BTreeSet<AssetId<Mesh>>,
        materials: BTreeSet<UntypedAssetId>,
    ) {
        let mut shared = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let epoch = shared.expected.epoch.wrapping_add(1).max(1);
        shared.expected = GpuReadinessExpected {
            active: true,
            epoch,
            images,
            meshes,
            materials,
            selection_entity: None,
        };
        shared.snapshot = GpuReadinessSnapshot::default();
    }

    fn merge_world_content(
        &self,
        meshes: impl IntoIterator<Item = AssetId<Mesh>>,
        materials: impl IntoIterator<Item = UntypedAssetId>,
        selection_entity: Option<Entity>,
    ) {
        let mut shared = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !shared.expected.active {
            return;
        }
        let mut changed = false;
        for mesh in meshes {
            changed |= shared.expected.meshes.insert(mesh);
        }
        for material in materials {
            changed |= shared.expected.materials.insert(material);
        }
        if selection_entity.is_some() && shared.expected.selection_entity != selection_entity {
            shared.expected.selection_entity = selection_entity;
            changed = true;
        }
        if changed {
            shared.expected.epoch = shared.expected.epoch.wrapping_add(1).max(1);
            shared.snapshot = GpuReadinessSnapshot::default();
        }
    }

    fn snapshot(&self) -> GpuReadinessSnapshot {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .snapshot
    }

    fn clear(&self) {
        let mut shared = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        shared.expected.active = false;
    }
}

fn record_gpu_readiness(
    probe: Res<GpuReadinessProbe>,
    gpu_images: Res<GpuRenderAssets<GpuImage>>,
    gpu_meshes: Res<GpuRenderAssets<RenderMesh>>,
    gpu_materials: Res<ErasedRenderAssets<PreparedMaterial>>,
    pipeline_cache: Res<PipelineCache>,
    render_mesh_instances: Res<RenderMeshInstances>,
) {
    let (expected, selection_previously_ready) = {
        let shared = probe.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !shared.expected.active {
            return;
        }
        (
            shared.expected.clone(),
            shared.snapshot.epoch == shared.expected.epoch && shared.snapshot.selection_draw_ready,
        )
    };
    let waiting_pipelines = pipeline_cache.waiting_pipelines().count();
    let queued_or_creating = pipeline_cache
        .pipelines()
        .filter(|pipeline| {
            matches!(
                pipeline.state,
                CachedPipelineState::Queued | CachedPipelineState::Creating(_)
            )
        })
        .count();
    let snapshot = GpuReadinessSnapshot {
        epoch: expected.epoch,
        expected_images: expected.images.len(),
        ready_images: expected
            .images
            .iter()
            .filter(|id| gpu_images.get(**id).is_some())
            .count(),
        expected_meshes: expected.meshes.len(),
        ready_meshes: expected
            .meshes
            .iter()
            .filter(|id| gpu_meshes.get(**id).is_some())
            .count(),
        expected_materials: expected.materials.len(),
        ready_materials: expected
            .materials
            .iter()
            .filter(|id| gpu_materials.get(**id).is_some())
            .count(),
        pending_pipelines: waiting_pipelines.max(queued_or_creating),
        failed_pipelines: pipeline_cache
            .pipelines()
            .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Err(_)))
            .count(),
        selection_expected: expected.selection_entity.is_some(),
        selection_draw_ready: selection_previously_ready
            || expected.selection_entity.is_none_or(|entity| {
                render_mesh_instances
                    .render_mesh_queue_data(MainEntity::from(entity))
                    .is_some()
            }),
    };
    let mut shared = probe.0.lock().unwrap_or_else(PoisonError::into_inner);
    if shared.expected.active && shared.expected.epoch == snapshot.epoch {
        shared.snapshot = snapshot;
    }
}

#[derive(Component)]
struct AuthoredCreditsElement {
    target_path: String,
}

#[derive(Component)]
struct CreditsSkipButton;

#[derive(Component)]
struct CreditsFade;

#[derive(Component)]
struct CreditsFireworksEmitter {
    target_path: String,
    effect: StableId,
    emitter_index: u8,
    origin_percent: Vec2,
    next_launch_index: u32,
    active_rockets: u16,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum CreditsFireworkParticleKind {
    Rocket,
    BurstFlash,
    Spark,
}

#[derive(Component)]
struct CreditsFireworkParticle {
    kind: CreditsFireworkParticleKind,
    effect: StableId,
    emitter_index: u8,
    sequence: u32,
    position_percent: Vec2,
    velocity_percent_per_second: Vec2,
    age_seconds: f32,
    lifetime_seconds: f32,
    color_index: usize,
}

#[derive(Component)]
struct CreditsFireworkBurst {
    effect: StableId,
    emitter_index: u8,
    sequence: u32,
    position_percent: Vec2,
    color_index: usize,
    remaining: u16,
    delay_seconds: f32,
}

#[derive(Resource)]
struct CreditsTimeline {
    elapsed_seconds: f32,
}

impl Default for CreditsTimeline {
    fn default() -> Self {
        Self {
            elapsed_seconds: std::env::var("STREAM_TOWN_DEBUG_CREDITS_TIME")
                .ok()
                .and_then(|value| value.parse().ok())
                .filter(|value: &f32| value.is_finite() && *value >= 0.0)
                .unwrap_or(0.0),
        }
    }
}

#[derive(Component)]
struct LevelUpToast;

#[derive(Resource, Default)]
struct LevelUpPresentation {
    actor_levels: BTreeMap<StableId, u16>,
    elapsed_seconds: Option<f32>,
}

#[derive(Component)]
struct WorldEntity;

#[derive(Component)]
struct PingPointer {
    actor: StableId,
    elapsed_seconds: f32,
    base_scale: f32,
}

#[derive(Component)]
struct SeagullFlight {
    start: Vec3,
    end: Vec3,
    elapsed_seconds: f32,
    leg_serial: u64,
    call_elapsed_seconds: f32,
    call_wait_seconds: f32,
    call_serial: u64,
    world_seed: u64,
}

#[derive(Component)]
struct MainMenuCloudPrism {
    drift_per_second: Vec3,
    wrap_min_x: f32,
    wrap_max_x: f32,
    fade_delay_seconds: f32,
    fade_elapsed_seconds: f32,
    entrance_fade_distance: f32,
    target_alpha: f32,
}

#[derive(Component)]
struct TerrainSurface;

#[derive(Component)]
struct WaterSurface;

#[derive(Component)]
struct WorldDiagnosticOverlay {
    mode: WorldDiagnosticMode,
}

#[derive(Component, Clone, Copy)]
struct WorldDiagnosticVisibilityBackup(Visibility);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum TerrainLodLevel {
    #[default]
    High,
    Medium,
    Low,
}

#[derive(Component)]
struct TerrainChunkLod {
    centre: Vec2,
    high: Handle<Mesh>,
    medium: Handle<Mesh>,
    low: Handle<Mesh>,
    current: TerrainLodLevel,
}

#[derive(Component)]
struct Agent {
    id: StableId,
    kind: ActorKind,
    archetype: StableId,
    goal: AgentGoal,
    spawn: GridPos,
    origin: GridPos,
    /// Current location in the navigation-only grid (three units per town cell).
    navigation_position: GridPos,
    path: Vec<GridPos>,
    path_index: usize,
    target: GridPos,
    action_cooldown_seconds: f32,
    action_started: bool,
    repath_remaining_seconds: f32,
    health_regen_accumulator: f64,
    wander_sequence: u64,
    previous_wander_origin: Option<GridPos>,
}

#[derive(Resource, Default)]
struct RulerVoteAnnouncementRuntime {
    initialized: bool,
    active_kind: Option<RulerVoteKind>,
    ruler_before_vote: Option<StableId>,
}

#[derive(Resource, Default)]
struct CitizenDeathAnnouncementRuntime {
    initialized: bool,
    dead: BTreeSet<StableId>,
}

#[derive(Component, Default)]
struct AgentLocomotion {
    previous_position: Vec3,
    normalized_speed: f32,
    stop_grace_seconds: f32,
    initialized: bool,
}

#[derive(Component)]
struct PlayerRigAxisCorrected;

#[derive(Component)]
struct PlayerRigAxisCorrectionRequired;

#[derive(Component)]
struct PlayerAnimatedRig;

/// Marks an animated-player renderer using the character-specific shadow
/// receiver path.
#[derive(Component)]
struct AnimatedCharacterShadowReceiver;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum AgentGoal {
    #[default]
    Wander,
    Gather(StableId),
    HarvestFarm(StableId),
    Deposit,
    WaitForStorage,
    Attack(StableId),
    AttackBuilding(StableId),
    Heal(StableId),
    Construct(StableId),
    VisitRegenerationStation(StableId),
    PlantTree(GridPos),
    Prospect {
        cell: GridPos,
        sequence: u32,
    },
    PlantBush(GridPos),
}

#[derive(Component, Clone, Copy)]
struct GridLocation(GridPos);

#[derive(Component)]
struct ResourceNode {
    id: StableId,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
struct ResourceVisual {
    mesh_index: usize,
}

#[derive(Component)]
struct FoliageVisual(StableId);

#[derive(Component)]
struct PendingSurfaceGrounding {
    surface_height: f32,
}

/// Gameplay visuals must wait for the authoritative terrain runtime before
/// they are grounded or habitat-tested. They are queued while the loading
/// screen is still constructing that runtime.
#[derive(Component)]
struct RuntimeTerrainGrounding;

#[derive(Component, Clone, Copy)]
struct SurfaceFoliageHabitat(FoliageHabitat);

#[derive(Component)]
struct FoliageRenderBatch(FoliageBatchKey);

#[derive(Component, Clone, Copy)]
struct FoliageNavigationLocation(GridPos);

type FoliageClearanceQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static GridLocation,
        Option<&'static FoliageNavigationLocation>,
        Option<&'static FoliageRenderBatch>,
        Option<&'static PendingSurfaceGrounding>,
        &'static mut Visibility,
    ),
    With<FoliageVisual>,
>;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudCommandGuidance;

#[derive(Component)]
struct HudResourceStrip;

#[derive(Component)]
struct HudStatsStrip;

#[derive(Component)]
struct HudMetricRow;

#[derive(Component)]
struct HudTopBar;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum HudTechnologyTextKind {
    Title,
    Requirement,
}

#[derive(Component)]
struct HudTechnologyObjectivePanel;

#[derive(Component)]
struct HudTechnologyProgressFill;

type HudMetricTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static HudMetric, &'static mut Text),
    (Without<Hud>, Without<HudTechnologyTextKind>),
>;
type HudMetricMaximumTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static HudMetricMaximum, &'static mut Text),
    (
        Without<Hud>,
        Without<HudMetric>,
        Without<HudTechnologyTextKind>,
    ),
>;
type HudTechnologyTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static HudTechnologyTextKind, &'static mut Text),
    (Without<Hud>, Without<HudMetric>),
>;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum HudMetric {
    Food,
    Gold,
    Ore,
    Wood,
    Players,
    Npcs,
    Buildings,
    PlayTime,
}

impl HudMetric {
    const fn shows_maximum(self) -> bool {
        matches!(
            self,
            Self::Food | Self::Gold | Self::Ore | Self::Wood | Self::Npcs
        )
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
struct HudMetricMaximum(HudMetric);

#[derive(Component)]
struct SeasonMeter;

#[derive(Component)]
struct SelectionPanel;

#[derive(Component)]
struct SelectionPanelSlider(SelectionPanelBar);

#[derive(Component)]
struct SelectionPanelSliderTrack(SelectionPanelBar);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SelectionPanelBar {
    Health,
    Experience,
}

struct SelectionPanelDetails {
    description: String,
    health_progress: Option<f32>,
    experience_progress: Option<f32>,
}

type SelectionPanelTextQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Text, &'static mut Visibility),
    (With<SelectionPanel>, Without<SelectionPanelSliderTrack>),
>;
type SelectionPanelTrackQuery<'w, 's> = Query<
    'w,
    's,
    (&'static SelectionPanelSliderTrack, &'static mut Visibility),
    (With<SelectionPanelSliderTrack>, Without<SelectionPanel>),
>;

impl SelectionPanelDetails {
    const fn progress(&self, bar: SelectionPanelBar) -> Option<f32> {
        match bar {
            SelectionPanelBar::Health => self.health_progress,
            SelectionPanelBar::Experience => self.experience_progress,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum VotePanelKind {
    Technology,
    Ruler,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum VoteTextKind {
    TechnologyTitle,
    TechnologyTimer,
    TechnologyOptionTitle(u8),
    TechnologyOptionDepthTag(u8),
    TechnologyOptionRequirements(u8),
    RulerTitle,
    RulerDescription,
    RulerTimer,
}

#[derive(Component)]
struct RulerOptionsContainer;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum VoteFillKind {
    TechnologyTimer,
    TechnologyOption(u8),
    RulerTimer,
}

#[derive(Component, Clone, Copy)]
struct TechnologyVoteIcon(u8);

#[derive(Component, Clone, Copy)]
struct TechnologyVoteOptionRow(u8);

#[derive(Component, Clone, Copy)]
struct TechnologyVoteTitleBar;

#[derive(Component, Clone, Copy)]
struct TechnologyVoteDepthBadge(u8);

#[derive(Component)]
struct TechnologyVoteTimerTrack;

#[derive(Component)]
struct TechnologyVoteTimerLabel;

#[derive(Component)]
struct TechnologyVoteTimerGroup;

#[derive(Component)]
struct TechnologyVoteTimerBar;

#[derive(Component)]
struct TechnologyVoteTimerIcon;

#[derive(Component)]
struct TechnologyVoteTimerGap;

#[derive(Component)]
struct CurrentEventPanel;

#[derive(Component, Clone, Copy)]
enum CurrentEventText {
    Title,
    Description,
    Progress,
}

#[derive(Component)]
struct CurrentEventFill;

type TechnologyVoteIconQuery<'w, 's> =
    Query<'w, 's, (&'static TechnologyVoteIcon, &'static mut ImageNode)>;
type TechnologyVoteRowQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static TechnologyVoteOptionRow,
        &'static mut Visibility,
        &'static mut Node,
    ),
    (
        Without<VotePanelKind>,
        Without<VoteFillKind>,
        Without<TechnologyVoteDepthBadge>,
    ),
>;
type TechnologyVoteDepthBadgeQuery<'w, 's> = Query<
    'w,
    's,
    (&'static TechnologyVoteDepthBadge, &'static mut Visibility),
    (Without<VotePanelKind>, Without<TechnologyVoteOptionRow>),
>;
type TownCameraMutQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Camera,
        &'static mut Projection,
        &'static mut Transform,
        &'static mut AmbientLight,
    ),
    (With<TownCamera>, Without<TownSun>),
>;
type TownSunMutQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut DirectionalLight, &'static mut Transform),
    (With<TownSun>, Without<TownCamera>),
>;
type TownPostProcessQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        Option<&'static mut ColorGrading>,
        Has<Hdr>,
        Has<Bloom>,
        Has<Vignette>,
        Has<MotionBlur>,
        Option<&'static mut Tonemapping>,
    ),
    With<TownCamera>,
>;
type LoadingSubstatusQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<LoadingSubstatusText>,
        Without<LoadingStatusText>,
        Without<LoadingPercentText>,
    ),
>;
type LoadingPercentQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<LoadingPercentText>,
        Without<LoadingStatusText>,
        Without<LoadingSubstatusText>,
    ),
>;
type LoadingCoverEntityQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, Option<&'static mut Visibility>),
    Or<(With<LoadingScreenEntity>, With<LoadingUiCamera>)>,
>;
type LoadingRetirementQuery<'w, 's> =
    Query<'w, 's, (), Or<(With<LoadingScreenEntity>, With<LoadingUiCamera>)>>;
#[derive(SystemParam)]
struct WorldRevealReadinessQueries<'w, 's> {
    material_specs: Query<'w, 's, (), With<MaterialOverrideSpec>>,
    mesh_overrides: Query<'w, 's, (Entity, Option<&'static MaterialOverrideApplied>), With<Mesh3d>>,
    animated_player_rigs: Query<'w, 's, (), With<PlayerAnimatedRig>>,
    character_receivers: Query<'w, 's, (), With<AnimatedCharacterShadowReceiver>>,
    suppressed_receivers: Query<'w, 's, (), With<bevy::light::NotShadowReceiver>>,
    converted_animation_roots:
        Query<'w, 's, Option<&'static ConvertedAnimationApplied>, With<ConvertedAnimationSpec>>,
    native_animation_roots: Query<'w, 's, Entity, With<NativeAnimationSpec>>,
    native_animation_drivers: Query<'w, 's, &'static ActorAnimationDriver>,
    parents: Query<'w, 's, &'static ChildOf>,
}
type MenuWorldAssetRootQuery<'w, 's> = Query<
    'w,
    's,
    (Option<&'static Children>, Option<&'static WorldInstance>),
    (
        With<StateEntity>,
        With<WorldAssetRoot>,
        Without<WorldEntity>,
    ),
>;
type WorldAssetRootQuery<'w, 's> = Query<
    'w,
    's,
    (Option<&'static Children>, Option<&'static WorldInstance>),
    (With<WorldEntity>, With<WorldAssetRoot>),
>;

#[derive(Component)]
struct MenuOverlay;

type MenuOverlayEntityQuery<'w, 's> = Query<
    'w,
    's,
    Entity,
    Or<(
        With<MenuOverlay>,
        With<GameMenuRoot>,
        With<SettingsRoot>,
        With<SecretsDisclaimerRoot>,
        With<SecretsRoot>,
        With<GoLiveConfirmationRoot>,
        With<TownDialogRoot>,
    )>,
>;

#[derive(Component)]
struct TownHall;

#[derive(Component)]
struct RuntimeBuilding {
    id: StableId,
}

#[derive(Component)]
struct NightPointLightPoolSlot;

#[derive(Component)]
struct ChimneySmokeEmitters {
    prefab_guid: String,
    age: u8,
    emitters: Vec<ChimneySmokeEmitterRuntime>,
}

struct ChimneySmokeEmitterRuntime {
    effect: StableId,
    local_position: Vec3,
    emission_accumulator: f32,
    sequence: u32,
}

#[derive(Component)]
struct ChimneySmokeParticle {
    effect: StableId,
    elapsed_seconds: f32,
    duration_seconds: f32,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    size_over_lifetime: [f32; 2],
    color_variant: usize,
}

#[derive(Component)]
struct ActorNameOverlay {
    actor: StableId,
}

#[derive(Component)]
struct ActorHealthOverlay {
    actor: StableId,
    last_health: i32,
    was_damaged: bool,
    hide_remaining_seconds: f32,
}

#[derive(Component)]
struct ActorHealthFill;

#[derive(Component)]
struct TemporaryWorldLabel {
    target: StableId,
    remaining_seconds: f32,
}

#[derive(Component)]
struct BuildingHealthOverlay {
    building: StableId,
}

#[derive(Component)]
struct BuildingHealthFill;

#[derive(Component)]
struct BuildingPlacementVisual {
    owner: StableId,
    cell: GridPos,
}

#[derive(Component)]
struct BuildingPlacementGhost {
    owner: StableId,
    cell: GridPos,
    building: StableId,
    scene_asset_path: String,
}

#[derive(Component)]
struct BuildingPlacementGhostMesh {
    owner: StableId,
}

#[derive(Component)]
struct BuildingPlacementGhostNodeProcessed;

#[derive(Component)]
struct BuildingPlacementGhostHiddenNodes(BTreeSet<String>);

#[derive(Component)]
struct BuildingPlacementOwnerOverlay {
    owner: StableId,
}

#[derive(Component)]
struct EnemyCamp {
    id: StableId,
}

#[derive(Component)]
struct FishGodPresentation;

#[derive(Component)]
struct FishGodAnimation;

#[derive(Component)]
struct FishGodExitTimer {
    remaining_seconds: f32,
}

#[derive(Component)]
struct FishGodExitTriggerSent;

#[derive(Component)]
struct FallingFish {
    effect: StableId,
    sequence: u32,
    velocity: Vec3,
    angular_velocity: Vec3,
    age_seconds: f32,
    lifetime_seconds: f32,
    base_scale: f32,
    collision_count: u8,
}

#[derive(Component)]
struct FallingFishEmitter {
    effect: StableId,
    emission_remainder: f32,
    sequence: u32,
}

#[derive(Component)]
struct FishSchoolParticle {
    base_position: Vec3,
    noise_amplitude: Vec3,
    phase: Vec3,
    frequency: f32,
    scroll_speed: f32,
    octaves: u8,
    octave_multiplier: f32,
    octave_scale: f32,
    align_to_velocity: bool,
}

#[derive(Component)]
struct TowerShooter {
    cooldown_seconds: f32,
}

#[derive(Component)]
struct CombatProjectile {
    source: ProjectileSource,
    target: StableId,
    damage: u32,
    speed_cells_per_second: f32,
    visual: CombatVisualKind,
    trail_cooldown_seconds: f32,
    remaining_seconds: f32,
}

#[derive(Clone)]
enum ProjectileSource {
    Actor(StableId),
    Building(StableId),
}

#[derive(Clone)]
struct ProjectileSpawn {
    source: ProjectileSource,
    target: StableId,
    damage: u32,
    speed_cells_per_second: f32,
    visual: CombatVisualKind,
}

#[derive(Clone)]
enum ActionPresentation {
    Projectile(ProjectileSpawn),
    Impact {
        target: GridPos,
        visual: CombatVisualKind,
    },
    Healing {
        source: StableId,
        target: StableId,
    },
    BuildingWork {
        target: GridPos,
        sparks: bool,
    },
    BuildingDestroyed {
        building: StableId,
        target: GridPos,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CombatVisualKind {
    Physical,
    Arrow,
    Fireball,
    Necrotic,
}

#[derive(Component)]
struct CombatTrailSegment {
    elapsed_seconds: f32,
    duration_seconds: f32,
    base_scale: Vec3,
}

#[derive(Component)]
struct CombatImpactParticle {
    elapsed_seconds: f32,
    duration_seconds: f32,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BuildingEffectKind {
    WorkSmoke,
    WorkSpark,
    LevelArrow,
    DamageSmoke,
    DamageFire,
}

#[derive(Component)]
struct BuildingEffectParticle {
    kind: BuildingEffectKind,
    elapsed_seconds: f32,
    duration_seconds: f32,
    origin: Vec3,
    velocity: Vec3,
    base_scale: Vec3,
    phase: f32,
}

#[derive(Component, Default)]
struct BuildingDamageEmitter {
    cooldown_seconds: f32,
    sequence: u32,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
enum HealingEffectKind {
    Channel,
    Burst,
    Revive,
}

#[derive(Component)]
struct HealingRingEffect {
    kind: HealingEffectKind,
    effect: StableId,
    origin: Vec3,
    elapsed_seconds: f32,
    duration_seconds: f32,
    base_scale: f32,
    follow_target: Option<StableId>,
}

#[derive(Component)]
struct HealingMoteEffect {
    kind: HealingEffectKind,
    effect: StableId,
    origin: Vec3,
    elapsed_seconds: f32,
    duration_seconds: f32,
    angle_radians: f32,
    phase: f32,
    base_scale: Vec3,
    distance_scale: f32,
    size_multiplier: f32,
    follow_target: Option<StableId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct HealingEffectSample {
    ring_scale: f32,
    mote_scale: f32,
    radial_distance: f32,
    rise: f32,
    rotation_radians: f32,
}

#[derive(Component)]
struct BuildingPresentation {
    base_translation: Vec3,
    base_scale: Vec3,
    base_height_offset: f32,
    applied_stage: u8,
    applied_level: u16,
    applied_age: u8,
    applied_scene: Option<String>,
}

#[derive(Component)]
struct TownCamera;

#[derive(Component, Clone)]
struct TownCameraControllerRuntime {
    home: Transform,
    move_target: Vec3,
    zoom_target_height: f32,
    seconds_since_acknowledgement: f32,
    observed_command_acknowledgements: u64,
    auto_shot: AutoCameraShot,
    auto_shot_elapsed_seconds: f32,
    auto_sequence: u64,
    temporary_focus: Option<TemporaryCameraFocus>,
    observed_damage_sequence: u64,
    combat_redirect_cooldown_seconds: f32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum AutoCameraShot {
    #[default]
    Inactive,
    Town,
    Citizen(StableId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CameraFocusTarget {
    Citizen(StableId),
    Building(StableId),
}

#[derive(Clone, Debug)]
struct TemporaryCameraFocus {
    target: CameraFocusTarget,
    remaining_seconds: f32,
    return_to_auto_camera: bool,
}

#[derive(Resource, Default)]
struct CameraDamageRuntime {
    actor_health: BTreeMap<StableId, i32>,
    building_health: BTreeMap<StableId, i32>,
    latest_target: Option<CameraFocusTarget>,
    sequence: u64,
    initialized: bool,
}

#[derive(Resource, Default)]
struct RetreatingCitizens(BTreeSet<StableId>);

#[derive(SystemParam)]
struct AgentRecoveryRuntime<'w> {
    path_failures: ResMut<'w, PathFailureRuntime>,
    retreating: ResMut<'w, RetreatingCitizens>,
}

#[derive(Default)]
struct FoliageAcceptanceCapture {
    initialized: bool,
    elapsed_seconds: f32,
    next_capture: usize,
    written_captures: usize,
    capture_cooldown_seconds: f32,
    starting_camera: Option<Transform>,
    output_directory: Option<PathBuf>,
    renderer_count: usize,
    shadow_caster_count: usize,
    shadow_receiver_count: usize,
    pending_grounding_count: usize,
    habitat_violation_count: usize,
    configured_seed: u64,
    generated_seed: u64,
    duplicate_group_count: usize,
    duplicate_groups: Vec<serde_json::Value>,
    frames: Vec<serde_json::Value>,
    completion_delay_seconds: Option<f32>,
}

#[derive(Component)]
struct FoliageAcceptanceScreenshot;

impl TownCameraControllerRuntime {
    fn new(home: Transform) -> Self {
        Self {
            move_target: home.translation,
            zoom_target_height: home.translation.y,
            seconds_since_acknowledgement: 0.0,
            observed_command_acknowledgements: 0,
            auto_shot: AutoCameraShot::Inactive,
            auto_shot_elapsed_seconds: 0.0,
            auto_sequence: 0,
            temporary_focus: None,
            observed_damage_sequence: 0,
            combat_redirect_cooldown_seconds: 0.0,
            home,
        }
    }

    fn set_home(&mut self, home: Transform) {
        self.move_target = home.translation;
        self.zoom_target_height = home.translation.y;
        self.seconds_since_acknowledgement = 0.0;
        self.auto_shot = AutoCameraShot::Inactive;
        self.auto_shot_elapsed_seconds = 0.0;
        self.auto_sequence = 0;
        self.temporary_focus = None;
        self.home = home;
    }

    fn return_home(&mut self) {
        self.move_target = self.home.translation;
        self.zoom_target_height = self.home.translation.y;
    }

    fn cancel_auto_camera(&mut self) {
        self.auto_shot = AutoCameraShot::Inactive;
        self.auto_shot_elapsed_seconds = 0.0;
        self.auto_sequence = 0;
    }
}

#[derive(Component)]
struct ActivePetVisual {
    owner: StableId,
    pet: StableId,
    movement_speed: f32,
}

#[derive(Component)]
struct WeatherParticle {
    kind: Weather,
    seed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EnvironmentPalette {
    terrain_tint: [f32; 3],
    terrain_winter_strength: f32,
    water_color: [f32; 4],
    clear_color: [f32; 3],
    sun_color: [f32; 3],
    sun_illuminance: f32,
    ambient_color: [f32; 3],
    ambient_brightness: f32,
    fog_color: [f32; 4],
    fog_start: f32,
    fog_end: f32,
    particle_count: u16,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum MovementAnimationState {
    #[default]
    Idle,
    Moving,
}

#[derive(Component, Default)]
struct AgentAnimation {
    state: MovementAnimationState,
    phase: f32,
    base_scale: Vec3,
    native: bool,
}

#[derive(Component, Default)]
struct AgentEquipmentPresentation;

#[derive(Component, Default)]
struct EquipmentNodeProcessed;

#[derive(Component, Default)]
struct TransientCarryVisibility(bool);

#[derive(Component)]
struct EquipmentNode {
    actor_root: Entity,
    name: String,
}

#[derive(Component, Default)]
struct EnemyModelNodeProcessed;

#[derive(Component)]
struct BuildingModelNode {
    building_root: Entity,
    name: String,
}

#[derive(Component, Default)]
struct BuildingModelNodeProcessed;

#[derive(Component)]
struct AuthoredRotatingNode {
    building_root: Entity,
    age: u8,
    axis: Vec3,
    radians_per_second: f32,
}

#[derive(Component, Default)]
struct AuthoredRotatingNodeProcessed;

#[derive(Component)]
struct MainMenuRotatingDefinitions(Vec<stream_town_domain::RotatingNodeDef>);

#[derive(Component)]
struct MainMenuHiddenModelNodes(BTreeSet<String>);

#[derive(Component)]
struct MainMenuModelNodeProcessed;

/// Marks authored menu buildings whose imported mesh descendants must retain
/// ordinary opaque shadow casting.
#[derive(Component)]
struct MainMenuBuildingShadowRoot;

#[derive(Component)]
struct MainMenuBuildingShadowVerified;

#[derive(Component)]
struct MainMenuRotatingNode {
    axis: Vec3,
    radians_per_second: f32,
}

#[derive(Component, Default)]
struct MainMenuRotatingNodeProcessed;

#[derive(Component)]
struct TownSun;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CosmeticNodeKind {
    Eyes,
    Hair,
    FacialHair,
    Skin,
}

#[derive(Component)]
struct CosmeticNode {
    actor_root: Entity,
    kind: CosmeticNodeKind,
    index: u8,
}

#[derive(Component, Default)]
struct CosmeticNodeProcessed;

#[derive(Component)]
struct CosmeticRenderer {
    actor_root: Entity,
    kind: CosmeticNodeKind,
    base_material: Handle<CharacterMaterial>,
    applied_color: Option<u8>,
}

struct CosmeticMaterialVariant {
    base_material: Handle<CharacterMaterial>,
    kind: CosmeticNodeKind,
    color: u8,
    material: Handle<CharacterMaterial>,
}

#[derive(Resource, Default)]
struct CosmeticMaterialCache(Vec<CosmeticMaterialVariant>);

struct CharacterBaseMaterialVariant {
    source: Handle<StandardMaterial>,
    material: Handle<CharacterMaterial>,
}

#[derive(Resource, Default)]
struct CharacterBaseMaterialCache(Vec<CharacterBaseMaterialVariant>);

#[derive(Resource, Default)]
struct RoleActionAudioCache(BTreeMap<String, CachedRoleActionAudio>);

struct CachedRoleActionAudio {
    source: Handle<AudioSource>,
    wav: Arc<[u8]>,
}

#[derive(Component)]
struct AmbienceAudio;

#[derive(Resource, Default)]
struct WorldAudioRuntime {
    ambience: Option<Handle<AudioSource>>,
    ambience_wav: Option<Arc<[u8]>>,
    seagull_calls: Vec<Handle<AudioSource>>,
    seagull_call_wavs: Vec<Arc<[u8]>>,
}

#[derive(Component, Clone)]
struct NativeAnimationSpec {
    graph: Handle<AnimationGraph>,
    idle: AnimationNodeIndex,
    moving: AnimationNodeIndex,
}

#[derive(Component, Clone)]
struct NativeAnimationRequest {
    asset_path: String,
    animation_index: u32,
}

#[derive(Resource, Default)]
struct NativeAnimationCache(BTreeMap<(String, u32), NativeAnimationSpec>);

#[derive(Component, Clone)]
struct ConvertedAnimationSpec {
    controller: StableId,
    state: StableId,
    rig_scene: String,
}

#[derive(Component)]
struct ConvertedAnimationApplied;

/// Marks a converted rig whose imported scene hierarchy has finished spawning.
///
/// Animation targets attached before `WorldInstanceReady` are descendants of
/// an instance that Bevy may still replace while it finishes loading. Waiting
/// for this marker prevents the animation player from being despawned between
/// `Update` and `PostUpdate` by the scene spawner.
#[derive(Component)]
struct ConvertedAnimationInstanceReady;

#[derive(Component)]
struct ActorAnimationDriver {
    actor_root: Entity,
    idle: AnimationNodeIndex,
    moving: AnimationNodeIndex,
    current: MovementAnimationState,
}

#[derive(Component)]
struct ConvertedAnimationDriver {
    actor_root: Entity,
    controller: StableId,
    layers: Vec<ConvertedAnimationLayerDriver>,
    last_alive: Option<bool>,
    active_action: Option<String>,
    transient_carry_visible: bool,
}

struct ConvertedAnimationLayerDriver {
    fallback_state: StableId,
    runtime: AnimationControllerRuntime,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
    active: Vec<(AnimationNodeIndex, f32)>,
    applied: Vec<ConvertedAnimationPlayback>,
    crossfade: Option<ConvertedAnimationCrossfade>,
    state_offset: f32,
    event_elapsed: BTreeMap<StableId, f32>,
}

#[derive(Clone)]
struct ConvertedAnimationLayerTemplate {
    fallback_state: StableId,
    nodes: BTreeMap<StableId, AnimationNodeIndex>,
}

struct CachedConvertedAnimation {
    graph: Handle<AnimationGraph>,
    layers: Vec<ConvertedAnimationLayerTemplate>,
}

#[derive(Resource, Default)]
struct ConvertedAnimationCache(BTreeMap<(StableId, StableId, String), CachedConvertedAnimation>);

#[derive(Clone)]
struct GateAnimationClipRequest {
    asset_path: String,
    animation_index: u32,
    speed: f32,
    transition_seconds: f32,
}

struct GateAnimationContract {
    controller: StableId,
    model_root_name: &'static str,
    animation_root_name: &'static str,
    open: GateAnimationClipRequest,
    close: GateAnimationClipRequest,
}

#[derive(Clone)]
struct CachedGateAnimation {
    graph: Handle<AnimationGraph>,
    open: AnimationNodeIndex,
    close: AnimationNodeIndex,
}

#[derive(Resource, Default)]
struct GateAnimationCache(BTreeMap<(String, u32, String, u32), CachedGateAnimation>);

#[derive(Component)]
struct GateAnimationBinding {
    age: u8,
    animation_root: Entity,
}

#[derive(Component)]
struct GateAnimationDriver {
    building_root: Entity,
    open: AnimationNodeIndex,
    close: AnimationNodeIndex,
    open_speed: f32,
    close_speed: f32,
    open_transition_seconds: f32,
    close_transition_seconds: f32,
    is_open: bool,
}

#[derive(Clone)]
struct ConvertedAnimationCrossfade {
    source: Vec<ConvertedAnimationPlayback>,
    elapsed: f32,
    duration: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ConvertedAnimationPlayback {
    node: AnimationNodeIndex,
    weight: f32,
    speed: f32,
    looping: bool,
}

#[derive(Clone, Debug)]
struct PendingRoleActionAudio {
    actor: StableId,
    clip: StableId,
    display_name: String,
    position: Vec3,
}

#[derive(Component, Clone)]
struct MaterialOverrideSpec {
    fallback: Option<ResolvedMaterialHandle>,
    model_materials: BTreeMap<String, ResolvedMaterialHandle>,
    renderer_materials: Vec<ResolvedRendererMaterialBinding>,
    suppress_self_shadows: bool,
}

#[derive(Clone)]
struct ResolvedRendererMaterialBinding {
    target_path: String,
    materials: BTreeMap<String, ResolvedMaterialHandle>,
}

#[derive(Component)]
struct MaterialOverrideApplied;

#[derive(Component)]
struct BuildingMaterialInstanced;
