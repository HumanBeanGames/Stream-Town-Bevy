//! Feature-gated, read-only profiling telemetry for long-running Stream Town sessions.
//!
//! The profiler samples on a low cadence, writes through a bounded background
//! queue, and serves only on loopback. It never serializes configuration secrets,
//! OAuth material, chat content, save paths, or the RTMP URL.

use std::{
    cmp::Reverse,
    collections::{BTreeMap, VecDeque},
    fs::{self, OpenOptions},
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::{
        Arc, RwLock,
        atomic::{AtomicU64, Ordering},
        mpsc::{self, SyncSender, TrySendError},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    gltf::Gltf,
    prelude::*,
    shader::Shader,
    window::{PrimaryWindow, Window},
};
use serde::Serialize;

use super::{
    AgentCommandQueue, BoundsMaterial, BuildingCommandQueue, BuildingMaterial,
    BuildingMaterialInstances, BuildingMaterialUpdateRuntime, CharacterMaterial, CloudMaterial,
    ConvertedAnimationCache, CritterMaterial, CrowdSeparationRuntime, EnemyNavigationRuntime,
    FineNavigationRuntime, FlagMaterial, GateAnimationCache, GiraffeMaterial, GodrayMaterial,
    GpuReadinessProbe, GrassMaterial, InjectedCommands, MenuSkyMaterial, NightEnemyWaveRuntime,
    OperatorChatRuntime, PathFailureRuntime, PathSurfaceRuntime, PresentedRenderFrames,
    RuntimeConfig, RuntimePlayerSettings, SessionStats, SimulationRuntime,
    StationResourceTargetIndex, StationTargetRuntime, TerrainMaterial, TraversalWearRuntime,
    TreeMaterial, WaterMaterial, WorldRenderStats, WorldRuntime,
};

const PROFILE_SCHEMA: u16 = 1;
const PROFILE_SAMPLE_INTERVAL: Duration = Duration::from_secs(5);
const PROFILE_INVENTORY_INTERVAL: Duration = Duration::from_mins(1);
const PROFILE_HISTORY_SAMPLES: usize = 4_320;
const PROFILE_LOG_MAX_BYTES: u64 = 128 * 1_024 * 1_024;
const PROFILE_LOG_QUEUE_CAPACITY: usize = 64;
const DEFAULT_PROFILE_BIND: &str = "127.0.0.1:17831";
const DEFAULT_PROFILE_LOG: &str = ".stream-town/diagnostics/stream-profile.jsonl";

/// Installs the profiling collector and loopback API.
///
/// This type is compiled only when the crate's `stream-profiling` feature is
/// enabled. The ordinary game binary has no profiler code or runtime overhead.
pub struct StreamTownProfilingPlugin;

impl Plugin for StreamTownProfilingPlugin {
    fn build(&self, app: &mut App) {
        let shared = Arc::new(RwLock::new(ProfileStore::default()));
        let api_address = start_profile_api(Arc::clone(&shared));
        let log_drops = Arc::new(AtomicU64::new(0));
        let log_sender = start_profile_log_writer(profile_log_path(), Arc::clone(&log_drops));
        app.add_plugins((
            EntityCountDiagnosticsPlugin::new(600),
            SystemInformationDiagnosticsPlugin,
        ))
        .insert_resource(ProfileRuntime::new(
            shared,
            log_sender,
            log_drops,
            api_address,
        ))
        .add_systems(First, begin_profiled_frame)
        .add_systems(
            Last,
            (finish_profiled_frame, collect_profile_sample).chain(),
        );
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ProfileSample {
    pub schema: u16,
    pub timestamp_ms: u128,
    pub session_id: String,
    pub process_id: u32,
    pub process_age_seconds: f64,
    pub game_state: String,
    pub perf: PerformanceProfile,
    pub diagnostics: BTreeMap<String, DiagnosticProfile>,
    pub stream: Option<crate::direct_broadcast::DirectBroadcastProfile>,
    pub simulation: Option<SimulationProfile>,
    pub quality: QualityProfile,
    pub windows: WindowProfile,
    pub gpu: Option<GpuProfile>,
    pub assets: BTreeMap<&'static str, u64>,
    pub workloads: BTreeMap<&'static str, u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<WorldInventory>,
    pub profiler: ProfilerProfile,
}

#[derive(Clone, Debug, Serialize)]
pub struct DiagnosticProfile {
    pub value: Option<f64>,
    pub average: Option<f64>,
    pub smoothed: Option<f64>,
    pub suffix: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PerformanceProfile {
    pub render_fps: Option<f64>,
    pub frame_time_ms: Option<f64>,
    pub presented_fps: Option<f64>,
    pub main_schedule_ms: f64,
    pub wall_frame_ms: f64,
    pub process_cpu_percent: Option<f64>,
    pub process_memory_gib: Option<f64>,
    pub system_cpu_percent: Option<f64>,
    pub system_memory_percent: Option<f64>,
    pub entities: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SimulationProfile {
    pub elapsed_seconds: f64,
    pub day: u32,
    pub season: String,
    pub weather: String,
    pub phase: &'static str,
    pub cycle_phase_seconds: f64,
    pub transition_progress: f64,
    pub daylight: f32,
    pub actors: usize,
    pub buildings: usize,
    pub enemy_camps: usize,
    pub queued_events: usize,
    pub active_event: bool,
    pub active_raid: bool,
}

#[derive(Clone, Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct QualityProfile {
    pub output_width: u16,
    pub output_height: u16,
    pub output_fps: u8,
    pub video_bitrate_kbps: u32,
    pub shadow_map_resolution: u16,
    pub ambient_occlusion: bool,
    pub shadows_enabled: bool,
    pub msaa_samples: u8,
    pub post_process_aa: String,
    pub vsync: bool,
    pub fixed_quality_contract_met: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WindowProfile {
    pub count: usize,
    pub primary_visible: Option<bool>,
    pub primary_present_mode: Option<String>,
    pub primary_width: Option<f32>,
    pub primary_height: Option<f32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct GpuProfile {
    pub presented_frames: u64,
    pub expected_images: usize,
    pub ready_images: usize,
    pub expected_meshes: usize,
    pub ready_meshes: usize,
    pub expected_materials: usize,
    pub ready_materials: usize,
    pub pending_pipelines: usize,
    pub failed_pipelines: usize,
    pub selection_expected: bool,
    pub selection_draw_ready: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorldInventory {
    pub registered_components: usize,
    pub archetypes: usize,
    pub component_counts: BTreeMap<String, u64>,
    pub largest_archetypes: Vec<ArchetypeProfile>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArchetypeProfile {
    pub entities: u32,
    pub components: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProfilerProfile {
    pub sample_interval_seconds: u64,
    pub inventory_interval_seconds: u64,
    pub api_address: Option<String>,
    pub dropped_log_samples: u64,
}

#[derive(Default)]
struct ProfileStore {
    samples: VecDeque<ProfileSample>,
}

impl ProfileStore {
    fn push(&mut self, sample: ProfileSample) {
        self.samples.push_back(sample);
        while self.samples.len() > PROFILE_HISTORY_SAMPLES {
            self.samples.pop_front();
        }
    }
}

#[derive(Resource)]
struct ProfileRuntime {
    started_at: Instant,
    frame_started_at: Instant,
    previous_frame_started_at: Instant,
    main_schedule_ms: f64,
    wall_frame_ms: f64,
    next_sample_at: Instant,
    next_inventory_at: Instant,
    previous_sample_at: Instant,
    previous_presented_frames: u64,
    session_id: String,
    api_address: Option<String>,
    shared: Arc<RwLock<ProfileStore>>,
    log_sender: SyncSender<ProfileSample>,
    log_drops: Arc<AtomicU64>,
}

impl ProfileRuntime {
    fn new(
        shared: Arc<RwLock<ProfileStore>>,
        log_sender: SyncSender<ProfileSample>,
        log_drops: Arc<AtomicU64>,
        api_address: Option<SocketAddr>,
    ) -> Self {
        let now = Instant::now();
        let started_ms = unix_timestamp_ms();
        Self {
            started_at: now,
            frame_started_at: now,
            previous_frame_started_at: now,
            main_schedule_ms: 0.0,
            wall_frame_ms: 0.0,
            next_sample_at: now,
            next_inventory_at: now,
            previous_sample_at: now,
            previous_presented_frames: 0,
            session_id: format!("{}-{started_ms}", std::process::id()),
            api_address: api_address.map(|address| address.to_string()),
            shared,
            log_sender,
            log_drops,
        }
    }
}

fn begin_profiled_frame(mut runtime: ResMut<ProfileRuntime>) {
    let now = Instant::now();
    runtime.wall_frame_ms = now
        .saturating_duration_since(runtime.previous_frame_started_at)
        .as_secs_f64()
        * 1_000.0;
    runtime.previous_frame_started_at = now;
    runtime.frame_started_at = now;
}

fn finish_profiled_frame(mut runtime: ResMut<ProfileRuntime>) {
    runtime.main_schedule_ms = runtime.frame_started_at.elapsed().as_secs_f64() * 1_000.0;
}

struct SampleTiming {
    process_age_seconds: f64,
    main_schedule_ms: f64,
    wall_frame_ms: f64,
    presented_fps: Option<f64>,
    session_id: String,
    api_address: Option<String>,
    inventory_due: bool,
    log_drops: u64,
}

fn collect_profile_sample(world: &mut World) {
    let now = Instant::now();
    if world
        .resource::<ProfileRuntime>()
        .next_sample_at
        .saturating_duration_since(now)
        > Duration::ZERO
    {
        return;
    }
    let presented_frames = world
        .get_resource::<PresentedRenderFrames>()
        .map_or(0, PresentedRenderFrames::current);
    let timing = {
        let mut runtime = world.resource_mut::<ProfileRuntime>();
        let sample_elapsed = now
            .saturating_duration_since(runtime.previous_sample_at)
            .as_secs_f64();
        let presented_delta = presented_frames.saturating_sub(runtime.previous_presented_frames);
        let inventory_due = now >= runtime.next_inventory_at;
        if inventory_due {
            runtime.next_inventory_at = now + PROFILE_INVENTORY_INTERVAL;
        }
        runtime.next_sample_at = now + PROFILE_SAMPLE_INTERVAL;
        runtime.previous_sample_at = now;
        runtime.previous_presented_frames = presented_frames;
        SampleTiming {
            process_age_seconds: runtime.started_at.elapsed().as_secs_f64(),
            main_schedule_ms: runtime.main_schedule_ms,
            wall_frame_ms: runtime.wall_frame_ms,
            presented_fps: (sample_elapsed > 0.0).then_some(
                f64::from(u32::try_from(presented_delta).unwrap_or(u32::MAX)) / sample_elapsed,
            ),
            session_id: runtime.session_id.clone(),
            api_address: runtime.api_address.clone(),
            inventory_due,
            log_drops: runtime.log_drops.load(Ordering::Relaxed),
        }
    };
    let sample = build_profile_sample(world, presented_frames, timing);
    let (shared, sender, log_drops) = {
        let runtime = world.resource::<ProfileRuntime>();
        (
            Arc::clone(&runtime.shared),
            runtime.log_sender.clone(),
            Arc::clone(&runtime.log_drops),
        )
    };
    if let Ok(mut store) = shared.write() {
        store.push(sample.clone());
    }
    if let Err(error) = sender.try_send(sample)
        && matches!(error, TrySendError::Full(_) | TrySendError::Disconnected(_))
    {
        log_drops.fetch_add(1, Ordering::Relaxed);
    }
}

fn build_profile_sample(
    world: &World,
    presented_frames: u64,
    timing: SampleTiming,
) -> ProfileSample {
    let diagnostics = world.get_resource::<DiagnosticsStore>();
    let game_state = world.get_resource::<State<super::GameState>>().map_or_else(
        || "unavailable".to_owned(),
        |state| format!("{:?}", state.get()),
    );
    let config = world.resource::<RuntimeConfig>();
    let settings = world.resource::<RuntimePlayerSettings>();
    let stream = crate::direct_broadcast::profiling_snapshot(world);
    let simulation = world
        .get_resource::<SimulationRuntime>()
        .map(|simulation| simulation_profile(&config.0.time, &simulation.0));
    let gpu = world.get_resource::<GpuReadinessProbe>().map(|probe| {
        let snapshot = probe.snapshot();
        GpuProfile {
            presented_frames,
            expected_images: snapshot.expected_images,
            ready_images: snapshot.ready_images,
            expected_meshes: snapshot.expected_meshes,
            ready_meshes: snapshot.ready_meshes,
            expected_materials: snapshot.expected_materials,
            ready_materials: snapshot.ready_materials,
            pending_pipelines: snapshot.pending_pipelines,
            failed_pipelines: snapshot.failed_pipelines,
            selection_expected: snapshot.selection_expected,
            selection_draw_ready: snapshot.selection_draw_ready,
        }
    });
    ProfileSample {
        schema: PROFILE_SCHEMA,
        timestamp_ms: unix_timestamp_ms(),
        session_id: timing.session_id,
        process_id: std::process::id(),
        process_age_seconds: timing.process_age_seconds,
        game_state,
        perf: PerformanceProfile {
            render_fps: diagnostic_value(diagnostics, &FrameTimeDiagnosticsPlugin::FPS),
            frame_time_ms: diagnostic_value(diagnostics, &FrameTimeDiagnosticsPlugin::FRAME_TIME),
            presented_fps: timing.presented_fps,
            main_schedule_ms: timing.main_schedule_ms,
            wall_frame_ms: timing.wall_frame_ms,
            process_cpu_percent: diagnostic_value(
                diagnostics,
                &SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
            ),
            process_memory_gib: diagnostic_value(
                diagnostics,
                &SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
            ),
            system_cpu_percent: diagnostic_value(
                diagnostics,
                &SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
            ),
            system_memory_percent: diagnostic_value(
                diagnostics,
                &SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
            ),
            entities: diagnostic_value(diagnostics, &EntityCountDiagnosticsPlugin::ENTITY_COUNT),
        },
        diagnostics: diagnostic_profiles(diagnostics),
        quality: quality_profile(&config.0, &settings.0),
        windows: window_profile(world),
        gpu,
        assets: asset_counts(world),
        workloads: workload_counts(world),
        inventory: timing.inventory_due.then(|| world_inventory(world)),
        profiler: ProfilerProfile {
            sample_interval_seconds: PROFILE_SAMPLE_INTERVAL.as_secs(),
            inventory_interval_seconds: PROFILE_INVENTORY_INTERVAL.as_secs(),
            api_address: timing.api_address,
            dropped_log_samples: timing.log_drops,
        },
        stream,
        simulation,
    }
}

fn diagnostic_profiles(
    diagnostics: Option<&DiagnosticsStore>,
) -> BTreeMap<String, DiagnosticProfile> {
    diagnostics.map_or_else(BTreeMap::new, |diagnostics| {
        diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.is_enabled)
            .map(|diagnostic| {
                (
                    diagnostic.path().as_str().to_owned(),
                    DiagnosticProfile {
                        value: diagnostic.value().filter(|value| value.is_finite()),
                        average: diagnostic.average().filter(|value| value.is_finite()),
                        smoothed: diagnostic.smoothed().filter(|value| value.is_finite()),
                        suffix: diagnostic.suffix.to_string(),
                    },
                )
            })
            .collect()
    })
}

fn diagnostic_value(
    diagnostics: Option<&DiagnosticsStore>,
    path: &bevy::diagnostic::DiagnosticPath,
) -> Option<f64> {
    diagnostics
        .and_then(|diagnostics| diagnostics.get(path))
        .and_then(|diagnostic| diagnostic.smoothed().or_else(|| diagnostic.value()))
        .filter(|value| value.is_finite())
}

fn simulation_profile(
    time: &stream_town_domain::TimeCycleConfig,
    simulation: &stream_town_domain::WorldSimulation,
) -> SimulationProfile {
    let cycle = f64::from(time.seconds_per_day.max(1));
    let transition = f64::from(time.transition_seconds).min(cycle);
    let daylight_end = cycle * f64::from(time.daylight_per_thousand) / 1_000.0;
    let dusk_start = (daylight_end - transition).max(0.0);
    let dawn_start = (cycle - transition).max(daylight_end);
    let phase_seconds = simulation.elapsed_seconds.max(0.0).rem_euclid(cycle);
    let (phase, progress) = if phase_seconds < dusk_start {
        ("stable_day", 0.0)
    } else if phase_seconds < daylight_end {
        (
            "dusk_transition",
            (phase_seconds - dusk_start) / transition.max(f64::EPSILON),
        )
    } else if phase_seconds < dawn_start {
        ("stable_night", 0.0)
    } else {
        (
            "dawn_transition",
            (phase_seconds - dawn_start) / transition.max(f64::EPSILON),
        )
    };
    SimulationProfile {
        elapsed_seconds: simulation.elapsed_seconds,
        day: simulation.day,
        season: format!("{:?}", simulation.season),
        weather: format!("{:?}", simulation.weather),
        phase,
        cycle_phase_seconds: phase_seconds,
        transition_progress: progress.clamp(0.0, 1.0),
        daylight: time.sample(simulation.elapsed_seconds).daylight,
        actors: simulation.actors.len(),
        buildings: simulation.buildings.len(),
        enemy_camps: simulation.enemy_camps.len(),
        queued_events: simulation.queued_events.len(),
        active_event: simulation.active_event.is_some(),
        active_raid: simulation.active_raid.is_some(),
    }
}

fn quality_profile(
    config: &stream_town_domain::GameConfig,
    settings: &stream_town_domain::PlayerSettings,
) -> QualityProfile {
    let broadcast = &config.twitch.broadcast;
    let video = &settings.video;
    let post_process_aa = format!("{:?}", video.post_process_aa);
    QualityProfile {
        output_width: broadcast.width,
        output_height: broadcast.height,
        output_fps: broadcast.frames_per_second,
        video_bitrate_kbps: broadcast.video_bitrate_kbps,
        shadow_map_resolution: video.shadow_map_resolution,
        ambient_occlusion: video.ambient_occlusion,
        shadows_enabled: video.shadows_enabled,
        msaa_samples: video.msaa_samples,
        post_process_aa: post_process_aa.clone(),
        vsync: video.vsync,
        fixed_quality_contract_met: broadcast.width == 1_920
            && broadcast.height == 1_080
            && broadcast.frames_per_second == 30
            && broadcast.video_bitrate_kbps == 6_000
            && video.shadow_map_resolution == 4_096
            && video.ambient_occlusion
            && video.msaa_samples == 8
            && post_process_aa.eq_ignore_ascii_case("smaa"),
    }
}

fn window_profile(world: &World) -> WindowProfile {
    let mut profile = WindowProfile::default();
    for entity in world.iter_entities() {
        let Some(window) = entity.get::<Window>() else {
            continue;
        };
        profile.count += 1;
        if entity.contains::<PrimaryWindow>() {
            profile.primary_visible = Some(window.visible);
            profile.primary_present_mode = Some(format!("{:?}", window.present_mode));
            profile.primary_width = Some(window.resolution.width());
            profile.primary_height = Some(window.resolution.height());
        }
    }
    profile
}

fn asset_count<A: Asset>(world: &World) -> u64 {
    world
        .get_resource::<Assets<A>>()
        .map_or(0, |assets| assets.len() as u64)
}

fn asset_counts(world: &World) -> BTreeMap<&'static str, u64> {
    BTreeMap::from([
        ("animation_clips", asset_count::<AnimationClip>(world)),
        ("animation_graphs", asset_count::<AnimationGraph>(world)),
        ("audio_sources", asset_count::<AudioSource>(world)),
        ("bounds_materials", asset_count::<BoundsMaterial>(world)),
        ("building_materials", asset_count::<BuildingMaterial>(world)),
        (
            "character_materials",
            asset_count::<CharacterMaterial>(world),
        ),
        ("cloud_materials", asset_count::<CloudMaterial>(world)),
        ("critter_materials", asset_count::<CritterMaterial>(world)),
        ("flag_materials", asset_count::<FlagMaterial>(world)),
        ("giraffe_materials", asset_count::<GiraffeMaterial>(world)),
        ("gltfs", asset_count::<Gltf>(world)),
        ("godray_materials", asset_count::<GodrayMaterial>(world)),
        ("grass_materials", asset_count::<GrassMaterial>(world)),
        ("images", asset_count::<Image>(world)),
        ("menu_sky_materials", asset_count::<MenuSkyMaterial>(world)),
        ("meshes", asset_count::<Mesh>(world)),
        ("shaders", asset_count::<Shader>(world)),
        ("standard_materials", asset_count::<StandardMaterial>(world)),
        ("terrain_materials", asset_count::<TerrainMaterial>(world)),
        ("tree_materials", asset_count::<TreeMaterial>(world)),
        ("water_materials", asset_count::<WaterMaterial>(world)),
    ])
}

fn insert_count(counts: &mut BTreeMap<&'static str, u64>, name: &'static str, value: usize) {
    counts.insert(name, value as u64);
}

fn workload_counts(world: &World) -> BTreeMap<&'static str, u64> {
    let mut counts = BTreeMap::new();
    if let Some(simulation) = world.get_resource::<SimulationRuntime>() {
        insert_count(&mut counts, "simulation_actors", simulation.0.actors.len());
        insert_count(
            &mut counts,
            "simulation_buildings",
            simulation.0.buildings.len(),
        );
        insert_count(
            &mut counts,
            "simulation_enemy_camps",
            simulation.0.enemy_camps.len(),
        );
        insert_count(
            &mut counts,
            "simulation_event_queue",
            simulation.0.queued_events.len(),
        );
    }
    if let Some(runtime) = world.get_resource::<WorldRuntime>() {
        insert_count(
            &mut counts,
            "generated_resources",
            runtime.generated.resources.len(),
        );
        insert_count(
            &mut counts,
            "generated_foliage",
            runtime.generated.foliage.len(),
        );
        counts.insert(
            "navigation_cells",
            u64::from(runtime.generated.navigation.width())
                * u64::from(runtime.generated.navigation.height()),
        );
    }
    if let Some(runtime) = world.get_resource::<TraversalWearRuntime>() {
        insert_count(&mut counts, "traversal_wear_cells", runtime.cells.len());
    }
    if let Some(runtime) = world.get_resource::<PathSurfaceRuntime>() {
        insert_count(&mut counts, "path_surface_cells", runtime.levels.len());
    }
    if let Some(runtime) = world.get_resource::<FineNavigationRuntime>() {
        counts.insert(
            "fine_navigation_cells",
            runtime
                .grid
                .as_ref()
                .map_or(0, |grid| u64::from(grid.width()) * u64::from(grid.height())),
        );
    }
    if let Some(runtime) = world.get_resource::<EnemyNavigationRuntime>() {
        insert_count(
            &mut counts,
            "enemy_navigation_cells",
            runtime
                .field
                .as_ref()
                .map_or(0, |field| field.component_by_cell.len()),
        );
        counts.insert("enemy_navigation_task", u64::from(runtime.task.is_some()));
    }
    if let Some(runtime) = world.get_resource::<NightEnemyWaveRuntime>() {
        insert_count(
            &mut counts,
            "pending_enemy_spawns",
            runtime.pending_spawns.len(),
        );
    }
    if let Some(runtime) = world.get_resource::<ConvertedAnimationCache>() {
        insert_count(&mut counts, "converted_animation_cache", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<GateAnimationCache>() {
        insert_count(&mut counts, "gate_animation_cache", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<BuildingMaterialInstances>() {
        insert_count(&mut counts, "building_material_instances", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<BuildingMaterialUpdateRuntime>() {
        insert_count(
            &mut counts,
            "building_material_update_order",
            runtime.order.len(),
        );
    }
    if let Some(runtime) = world.get_resource::<CrowdSeparationRuntime>() {
        insert_count(&mut counts, "crowd_offsets", runtime.applied_offsets.len());
    }
    if let Some(runtime) = world.get_resource::<StationTargetRuntime>() {
        insert_count(&mut counts, "station_target_caches", runtime.stations.len());
        insert_count(
            &mut counts,
            "station_refresh_queue",
            runtime.refresh_queue.len(),
        );
        insert_count(
            &mut counts,
            "station_reachability_queue",
            runtime
                .stations
                .values()
                .map(|station| station.reachability_queue.len())
                .sum(),
        );
    }
    if let Some(runtime) = world.get_resource::<StationResourceTargetIndex>() {
        insert_count(&mut counts, "station_resource_index", runtime.by_id.len());
    }
    if let Some(runtime) = world.get_resource::<PathFailureRuntime>() {
        insert_count(&mut counts, "path_failure_states", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<InjectedCommands>() {
        insert_count(&mut counts, "injected_commands", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<AgentCommandQueue>() {
        insert_count(&mut counts, "agent_commands", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<BuildingCommandQueue>() {
        insert_count(&mut counts, "building_commands", runtime.0.len());
    }
    if let Some(runtime) = world.get_resource::<OperatorChatRuntime>() {
        insert_count(&mut counts, "operator_chat_lines", runtime.lines.len());
        insert_count(
            &mut counts,
            "operator_seen_message_ids",
            runtime.seen_message_ids.len(),
        );
    }
    if let Some(runtime) = world.get_resource::<SessionStats>() {
        counts.insert("paths_completed", runtime.paths_completed);
        counts.insert("commands_processed", runtime.commands_processed);
    }
    if let Some(stats) = world.get_resource::<WorldRenderStats>() {
        insert_count(
            &mut counts,
            "terrain_high_chunks",
            stats.terrain_high_chunks,
        );
        insert_count(
            &mut counts,
            "terrain_medium_chunks",
            stats.terrain_medium_chunks,
        );
        insert_count(&mut counts, "terrain_low_chunks", stats.terrain_low_chunks);
        insert_count(&mut counts, "foliage_instances", stats.foliage_instances);
        insert_count(
            &mut counts,
            "foliage_visible_instances",
            stats.foliage_visible_instances,
        );
        insert_count(&mut counts, "foliage_batches", stats.foliage_batches);
        insert_count(
            &mut counts,
            "foliage_spatial_groups",
            stats.foliage_spatial_groups,
        );
        insert_count(
            &mut counts,
            "foliage_unbatched_instances",
            stats.foliage_unbatched_instances,
        );
        insert_count(
            &mut counts,
            "crowd_adjusted_agents",
            stats.crowd_adjusted_agents,
        );
        insert_count(
            &mut counts,
            "crowd_yielding_agents",
            stats.crowd_yielding_agents,
        );
    }
    counts
}

fn world_inventory(world: &World) -> WorldInventory {
    let components = world.components();
    let mut component_counts = BTreeMap::<String, u64>::new();
    let mut archetypes = Vec::new();
    for archetype in world
        .archetypes()
        .iter()
        .filter(|archetype| !archetype.is_empty())
    {
        let names = archetype
            .components()
            .iter()
            .filter_map(|id| components.get_info(*id))
            .map(|info| info.name().to_string())
            .collect::<Vec<_>>();
        for name in &names {
            *component_counts.entry(name.clone()).or_default() += u64::from(archetype.len());
        }
        archetypes.push(ArchetypeProfile {
            entities: archetype.len(),
            components: names,
        });
    }
    archetypes.sort_by_key(|profile| Reverse(profile.entities));
    archetypes.truncate(24);
    WorldInventory {
        registered_components: components.len(),
        archetypes: world.archetypes().len(),
        component_counts,
        largest_archetypes: archetypes,
    }
}

fn profile_log_path() -> PathBuf {
    std::env::var_os("STREAM_TOWN_PROFILE_LOG")
        .map_or_else(|| PathBuf::from(DEFAULT_PROFILE_LOG), PathBuf::from)
}

fn start_profile_log_writer(path: PathBuf, failures: Arc<AtomicU64>) -> SyncSender<ProfileSample> {
    let (sender, receiver) = mpsc::sync_channel::<ProfileSample>(PROFILE_LOG_QUEUE_CAPACITY);
    let spawn = thread::Builder::new()
        .name("stream-town-profile-log".to_owned())
        .spawn(move || {
            while let Ok(sample) = receiver.recv() {
                if append_profile_sample(&path, &sample).is_err() {
                    failures.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
    if let Err(error) = spawn {
        warn!(%error, "could not start Stream Town profile log writer");
    }
    sender
}

fn append_profile_sample(path: &Path, sample: &ProfileSample) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if path
        .metadata()
        .is_ok_and(|metadata| metadata.len() >= PROFILE_LOG_MAX_BYTES)
    {
        let previous = path.with_file_name("stream-profile.previous.jsonl");
        if previous.exists() {
            fs::remove_file(&previous)?;
        }
        fs::rename(path, previous)?;
    }
    let encoded = serde_json::to_vec(sample).map_err(std::io::Error::other)?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(&encoded)?;
    file.write_all(b"\n")
}

fn profile_bind_address() -> Result<SocketAddr, String> {
    let raw = std::env::var("STREAM_TOWN_PROFILE_BIND")
        .unwrap_or_else(|_| DEFAULT_PROFILE_BIND.to_owned());
    let address = raw
        .parse::<SocketAddr>()
        .map_err(|error| format!("invalid STREAM_TOWN_PROFILE_BIND {raw:?}: {error}"))?;
    if !matches!(address.ip(), IpAddr::V4(ip) if ip.is_loopback())
        && !matches!(address.ip(), IpAddr::V6(ip) if ip.is_loopback())
    {
        return Err("STREAM_TOWN_PROFILE_BIND must be a loopback address".to_owned());
    }
    Ok(address)
}

fn start_profile_api(shared: Arc<RwLock<ProfileStore>>) -> Option<SocketAddr> {
    let address = match profile_bind_address() {
        Ok(address) => address,
        Err(error) => {
            warn!(%error, "Stream Town profile API is disabled");
            return None;
        }
    };
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => {
            warn!(%address, %error, "could not bind Stream Town profile API");
            return None;
        }
    };
    let actual_address = listener.local_addr().ok().unwrap_or(address);
    let spawn = thread::Builder::new()
        .name("stream-town-profile-api".to_owned())
        .spawn(move || {
            for connection in listener.incoming() {
                match connection {
                    Ok(stream) => serve_profile_request(stream, &shared),
                    Err(error) => {
                        warn!(%error, "Stream Town profile API accept failed");
                    }
                }
            }
        });
    if let Err(error) = spawn {
        warn!(%error, "could not start Stream Town profile API thread");
        return None;
    }
    info!(%actual_address, "Stream Town profile API listening on loopback");
    Some(actual_address)
}

fn serve_profile_request(mut stream: TcpStream, shared: &Arc<RwLock<ProfileStore>>) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
    let mut request = [0_u8; 8_192];
    let Ok(length) = stream.read(&mut request) else {
        return;
    };
    let request = String::from_utf8_lossy(&request[..length]);
    let Some((method, target)) = request.lines().next().and_then(|line| {
        let mut parts = line.split_whitespace();
        Some((parts.next()?, parts.next()?))
    }) else {
        write_http_response(&mut stream, 400, b"{\"error\":\"invalid request\"}");
        return;
    };
    if method != "GET" {
        write_http_response(&mut stream, 405, b"{\"error\":\"read-only API\"}");
        return;
    }
    let (path, query) = target.split_once('?').map_or((target, ""), |parts| parts);
    let response = match path {
        "/healthz" => Ok(serde_json::json!({"ok": true, "schema": PROFILE_SCHEMA})),
        "/v1/profile/schema" => Ok(profile_schema()),
        "/v1/profile/latest" => shared
            .read()
            .map_err(|_| "profile store unavailable".to_owned())
            .and_then(|store| {
                store
                    .samples
                    .back()
                    .cloned()
                    .map(|sample| serde_json::json!({"sample": sample}))
                    .ok_or_else(|| "profile has no samples yet".to_owned())
            }),
        "/v1/profile/samples" => profile_samples_response(shared, query),
        _ => Err("not found".to_owned()),
    };
    match response {
        Ok(document) => match serde_json::to_vec(&document) {
            Ok(body) => write_http_response(&mut stream, 200, &body),
            Err(_) => {
                write_http_response(&mut stream, 500, b"{\"error\":\"serialization failed\"}");
            }
        },
        Err(error) => {
            let status = if error == "not found" { 404 } else { 503 };
            let body = serde_json::to_vec(&serde_json::json!({"error": error}))
                .unwrap_or_else(|_| b"{\"error\":\"request failed\"}".to_vec());
            write_http_response(&mut stream, status, &body);
        }
    }
}

fn profile_samples_response(
    shared: &Arc<RwLock<ProfileStore>>,
    query: &str,
) -> Result<serde_json::Value, String> {
    let minutes = query_value(query, "minutes")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(30)
        .clamp(1, 360);
    let limit = query_value(query, "limit")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(PROFILE_HISTORY_SAMPLES)
        .clamp(1, PROFILE_HISTORY_SAMPLES);
    let cutoff = unix_timestamp_ms().saturating_sub(u128::from(minutes) * 60_000);
    let store = shared
        .read()
        .map_err(|_| "profile store unavailable".to_owned())?;
    let mut samples = store
        .samples
        .iter()
        .filter(|sample| sample.timestamp_ms >= cutoff)
        .cloned()
        .collect::<Vec<_>>();
    if samples.len() > limit {
        samples.drain(..samples.len() - limit);
    }
    Ok(serde_json::json!({
        "schema": PROFILE_SCHEMA,
        "minutes": minutes,
        "sample_count": samples.len(),
        "samples": samples,
    }))
}

fn query_value<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|pair| {
        let (candidate, value) = pair.split_once('=')?;
        (candidate == key).then_some(value)
    })
}

fn profile_schema() -> serde_json::Value {
    serde_json::json!({
        "schema": PROFILE_SCHEMA,
        "sample_interval_seconds": PROFILE_SAMPLE_INTERVAL.as_secs(),
        "inventory_interval_seconds": PROFILE_INVENTORY_INTERVAL.as_secs(),
        "history_hours": PROFILE_HISTORY_SAMPLES.saturating_mul(
            usize::try_from(PROFILE_SAMPLE_INTERVAL.as_secs()).unwrap_or(usize::MAX)
        ) / 3_600,
        "endpoints": [
            "/healthz",
            "/v1/profile/schema",
            "/v1/profile/latest",
            "/v1/profile/samples?minutes=30&limit=4320"
        ],
        "privacy": "No credentials, tokens, RTMP URL, chat content, save paths, or user identities are collected.",
        "coverage": [
            "captured and encoded FPS plus cumulative pipeline counters",
            "readback, encode, queue, packet, mux, reconnect, and drop pressure",
            "main schedule, render, process CPU/RAM, and system CPU/RAM",
            "every registered Bevy diagnostic, including future diagnostics",
            "simulation clock and stable-day/dusk/stable-night/dawn phase",
            "fixed output-quality contract",
            "world, cache, navigation, command, effect, asset, component, and archetype growth"
        ]
    })
}

fn write_http_response(stream: &mut TcpStream, status: u16, body: &[u8]) {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Service Unavailable",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
}

fn unix_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_bind_rejects_non_loopback_addresses() {
        let address = "0.0.0.0:17831".parse::<SocketAddr>().unwrap();
        assert!(!address.ip().is_loopback());
    }

    #[test]
    fn query_value_extracts_exact_keys() {
        assert_eq!(query_value("minutes=30&limit=12", "minutes"), Some("30"));
        assert_eq!(query_value("minutes=30&limit=12", "min"), None);
    }

    #[test]
    fn phase_classifier_matches_authored_boundaries() {
        let time = stream_town_domain::TimeCycleConfig {
            seconds_per_day: 1_000,
            daylight_per_thousand: 600,
            transition_seconds: 100,
            ..Default::default()
        };
        let mut simulation = stream_town_domain::WorldSimulation::new(1);
        simulation.elapsed_seconds = 550.0;
        assert_eq!(
            simulation_profile(&time, &simulation).phase,
            "dusk_transition"
        );
        simulation.elapsed_seconds = 950.0;
        assert_eq!(
            simulation_profile(&time, &simulation).phase,
            "dawn_transition"
        );
    }
}
