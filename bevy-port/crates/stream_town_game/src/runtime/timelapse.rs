use super::super::*;
use std::fmt::Write as _;

pub(crate) struct CityTimelapsePlugin;

impl Plugin for CityTimelapsePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CityTimelapseBuildConfirmed>()
            .init_resource::<CityTimelapseRuntime>()
            .add_systems(
                Update,
                apply_confirmed_builds.before(capture_city_timelapse),
            )
            .add_systems(
                Update,
                capture_city_timelapse
                    .after(capture_foliage_acceptance)
                    .after(sync_world_render_lod)
                    .after(update_hud)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

/// Emitted after a build command has been accepted and committed to the town.
///
/// Keeping this as a message prevents command dispatch from depending on the
/// timelapse timer implementation; other build producers can publish the same
/// fact without reaching into capture state.
#[derive(Message)]
pub(crate) struct CityTimelapseBuildConfirmed;

#[derive(Resource)]
pub(crate) struct CityTimelapseRuntime {
    configured_interval: TimelapseInterval,
    configured_dynamic: bool,
    pub(crate) remaining_seconds: Option<f32>,
    restore_next_frame: bool,
    camera_backup: Option<(Transform, TownCameraControllerRuntime)>,
    visibility_backups: Vec<(Entity, Visibility)>,
    active_town: Option<String>,
}

impl Default for CityTimelapseRuntime {
    fn default() -> Self {
        Self {
            configured_interval: TimelapseInterval::Off,
            configured_dynamic: false,
            remaining_seconds: None,
            restore_next_frame: false,
            camera_backup: None,
            visibility_backups: Vec::new(),
            active_town: None,
        }
    }
}

impl CityTimelapseRuntime {
    pub(crate) fn sync_settings(&mut self, interval: TimelapseInterval, dynamic: bool) {
        if self.configured_interval != interval {
            self.remaining_seconds = interval.seconds().map(|seconds| {
                let seconds = Duration::from_secs(u64::from(seconds)).as_secs_f32();
                self.remaining_seconds.unwrap_or(seconds).min(seconds)
            });
            self.configured_interval = interval;
        }
        self.configured_dynamic = dynamic;
    }

    pub(crate) fn confirmed_build(&mut self, interval: TimelapseInterval, dynamic: bool) {
        self.sync_settings(interval, dynamic);
        if dynamic && let Some(remaining) = &mut self.remaining_seconds {
            *remaining = (*remaining - 10.0 * 60.0).max(0.0);
        }
    }

    pub(crate) fn activate_town(&mut self, town_name: &str) {
        if self.active_town.as_deref() == Some(town_name) {
            return;
        }
        self.active_town = Some(town_name.to_owned());
        // A configured town timeline starts with a baseline frame. Subsequent
        // frames continue to use the selected fixed/dynamic interval.
        self.remaining_seconds = self.configured_interval.seconds().map(|_| 0.0);
    }
}

fn apply_confirmed_builds(
    mut builds: MessageReader<CityTimelapseBuildConfirmed>,
    settings: Res<RuntimePlayerSettings>,
    mut runtime: ResMut<CityTimelapseRuntime>,
) {
    if builds.read().next().is_none() {
        return;
    }
    runtime.confirmed_build(settings.0.timelapse.interval, settings.0.timelapse.dynamic);
}

#[derive(Component)]
struct CityTimelapseScreenshot {
    screenshot_path: PathBuf,
    video_frame_path: PathBuf,
    video_path: PathBuf,
    frames_directory: PathBuf,
    label: String,
}

const TIMELAPSE_BLANK_CAPTURE_RETRY_SECONDS: f32 = 5.0;

fn timelapse_interval_slug(interval: TimelapseInterval) -> &'static str {
    match interval {
        TimelapseInterval::Off => "off",
        TimelapseInterval::TenMinutes => "10-min",
        TimelapseInterval::OneHour => "1-hour",
        TimelapseInterval::ThreeHours => "3-hours",
        TimelapseInterval::SixHours => "6-hours",
        TimelapseInterval::TwelveHours => "12-hours",
        TimelapseInterval::TwentyFourHours => "24-hours",
    }
}

pub(crate) fn next_timelapse_frame_index(directory: &Path) -> u64 {
    fs::read_dir(directory)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.strip_prefix("frame-"))
                .and_then(|value| value.parse::<u64>().ok())
        })
        .max()
        .map_or(1, |value| value.saturating_add(1))
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn capture_city_timelapse(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<RuntimePlayerSettings>,
    save: Res<SaveRuntime>,
    town_catalog: Res<TownSaveCatalogRuntime>,
    gameplay_ready: Option<Res<GameplayReady>>,
    mut runtime: ResMut<CityTimelapseRuntime>,
    mut camera: Query<
        (
            &mut Transform,
            &mut TownCameraControllerRuntime,
            &RenderTarget,
        ),
        With<TownCamera>,
    >,
    mut hidden: Query<
        (Entity, &mut Visibility),
        Or<(
            With<Node>,
            With<WorldDiagnosticOverlay>,
            With<BuildingPlacementVisual>,
            With<BuildingPlacementGhost>,
            With<BuildingPlacementGhostMesh>,
            With<BuildingPlacementOwnerOverlay>,
        )>,
    >,
) {
    if runtime.restore_next_frame {
        if let Some((transform, controller)) = runtime.camera_backup.take()
            && let Ok((mut current_transform, mut current_controller, _)) = camera.single_mut()
        {
            *current_transform = transform;
            *current_controller = controller;
        }
        for (entity, visibility) in std::mem::take(&mut runtime.visibility_backups) {
            if let Ok((_, mut current)) = hidden.get_mut(entity) {
                *current = visibility;
            }
        }
        runtime.restore_next_frame = false;
        return;
    }
    if gameplay_ready.is_none() || std::env::var_os("STREAM_TOWN_FOLIAGE_CAPTURE_DIR").is_some() {
        return;
    }

    runtime.sync_settings(settings.0.timelapse.interval, settings.0.timelapse.dynamic);
    let town_name = town_catalog
        .active_town
        .clone()
        .or_else(|| {
            save.store
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "Stream Town".to_owned());
    runtime.activate_town(&town_name);
    let Some(remaining) = &mut runtime.remaining_seconds else {
        return;
    };
    *remaining = (*remaining - time.delta_secs()).max(0.0);
    if *remaining > f32::EPSILON {
        return;
    }

    let directory = PathBuf::from(".stream-town")
        .join("timelapses")
        .join(safe_town_filename(&town_name));
    let frames_directory = directory.join("video-frames");
    if let Err(error) = fs::create_dir_all(&frames_directory) {
        error!(%error, path = %frames_directory.display(), "failed to create timelapse directory");
        return;
    }
    let Ok((mut transform, mut controller, render_target)) = camera.single_mut() else {
        return;
    };
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis());
    let mode = if settings.0.timelapse.dynamic {
        "dynamic"
    } else {
        "fixed"
    };
    let rate = timelapse_interval_slug(settings.0.timelapse.interval);
    let screenshot_path = directory.join(format!("city-{timestamp}-{rate}-{mode}.png"));
    let frame_index = next_timelapse_frame_index(&frames_directory);
    let video_frame_path = frames_directory.join(format!("frame-{frame_index:08}.png"));
    let video_path = directory.join("city-timelapse.mp4");
    let label = format!(
        "{} - {}",
        timelapse_interval_label(settings.0.timelapse.interval),
        if settings.0.timelapse.dynamic {
            "Dynamic"
        } else {
            "Fixed"
        }
    );

    runtime.camera_backup = Some((*transform, controller.clone()));
    let home = controller.home;
    *transform = home;
    *controller = TownCameraControllerRuntime::new(home);
    runtime.visibility_backups.clear();
    for (entity, mut visibility) in &mut hidden {
        runtime.visibility_backups.push((entity, *visibility));
        *visibility = Visibility::Hidden;
    }
    commands
        .spawn((
            // Stream-only deployments move the town camera off the hidden
            // primary window and onto the broadcast image. Capture the camera's
            // actual target so timelapses and Twitch see the same rendered town.
            Screenshot(render_target.clone()),
            CityTimelapseScreenshot {
                screenshot_path,
                video_frame_path,
                video_path,
                frames_directory,
                label,
            },
        ))
        .observe(save_city_timelapse_frame);
    runtime.restore_next_frame = true;
    runtime.remaining_seconds = settings
        .0
        .timelapse
        .interval
        .seconds()
        .map(|seconds| Duration::from_secs(u64::from(seconds)).as_secs_f32());
}

fn save_city_timelapse_frame(
    captured: On<ScreenshotCaptured>,
    pending_captures: Query<&CityTimelapseScreenshot>,
    mut runtime: ResMut<CityTimelapseRuntime>,
) {
    let Ok(metadata) = pending_captures.get(captured.entity) else {
        return;
    };
    let Ok(dynamic) = captured.image.clone().try_into_dynamic() else {
        error!(path = %metadata.screenshot_path.display(), "could not convert timelapse screenshot");
        return;
    };
    let image = dynamic.to_rgb8();
    if timelapse_frame_is_blank(&image) {
        error!(
            path = %metadata.screenshot_path.display(),
            "discarding blank timelapse screenshot and scheduling a retry"
        );
        runtime.remaining_seconds = runtime
            .configured_interval
            .seconds()
            .map(|_| TIMELAPSE_BLANK_CAPTURE_RETRY_SECONDS);
        return;
    }
    if let Err(error) = image.save(&metadata.screenshot_path) {
        error!(%error, path = %metadata.screenshot_path.display(), "could not save timelapse screenshot");
        return;
    }
    let mut video_frame = image;
    draw_timelapse_label(&mut video_frame, &metadata.label);
    if let Err(error) = video_frame.save(&metadata.video_frame_path) {
        error!(%error, path = %metadata.video_frame_path.display(), "could not save timelapse video frame");
        return;
    }
    info!(path = %metadata.screenshot_path.display(), "city timelapse frame saved");
    let frames_directory = metadata.frames_directory.clone();
    let video_path = metadata.video_path.clone();
    thread::spawn(move || rebuild_timelapse_video(&frames_directory, &video_path));
}

pub(crate) fn timelapse_frame_is_blank(image: &image::RgbImage) -> bool {
    image.pixels().all(|pixel| pixel.0 == [0, 0, 0])
}

pub(crate) fn draw_timelapse_label(image: &mut image::RgbImage, label: &str) {
    const SCALE: u32 = 3;
    const MARGIN: u32 = 12;
    let label = label.to_ascii_uppercase();
    let text_width = u32::try_from(label.chars().count())
        .unwrap_or(u32::MAX)
        .saturating_mul(6 * SCALE);
    let box_width = text_width.saturating_add(MARGIN * 2).min(image.width());
    let box_height = (7 * SCALE + MARGIN * 2).min(image.height());
    let top = image.height().saturating_sub(box_height);
    for y in top..image.height() {
        for x in 0..box_width {
            let pixel = image.get_pixel_mut(x, y);
            pixel.0 = [pixel.0[0] / 4, pixel.0[1] / 4, pixel.0[2] / 4];
        }
    }
    let origin_y = top.saturating_add(MARGIN);
    for (index, character) in label.chars().enumerate() {
        let origin_x = MARGIN.saturating_add(
            u32::try_from(index)
                .unwrap_or(u32::MAX)
                .saturating_mul(6 * SCALE),
        );
        for (row, bits) in timelapse_glyph(character).into_iter().enumerate() {
            for column in 0..5_u32 {
                if bits & (1 << (4 - column)) == 0 {
                    continue;
                }
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        let x = origin_x.saturating_add(column * SCALE + dx);
                        let y = origin_y
                            .saturating_add(u32::try_from(row).unwrap_or_default() * SCALE + dy);
                        if x < image.width() && y < image.height() {
                            *image.get_pixel_mut(x, y) = image::Rgb([245, 245, 245]);
                        }
                    }
                }
            }
        }
    }
}

fn timelapse_glyph(character: char) -> [u8; 7] {
    match character {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 14],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [31, 4, 4, 4, 4, 4, 31],
        'J' => [7, 2, 2, 2, 18, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 21, 19, 17, 17, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        _ => [0; 7],
    }
}

fn rebuild_timelapse_video(frames_directory: &Path, video_path: &Path) {
    static VIDEO_REBUILD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let Ok(_guard) = VIDEO_REBUILD_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(PoisonError::into_inner)
    else {
        return;
    };
    let mut frames = fs::read_dir(frames_directory)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
                && path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .is_some_and(|stem| stem.starts_with("frame-"))
        })
        .collect::<Vec<_>>();
    frames.sort();
    let Some(last) = frames.last() else {
        return;
    };
    let mut manifest = String::from("ffconcat version 1.0\n");
    for frame in &frames {
        if let Some(name) = frame.file_name().and_then(|name| name.to_str()) {
            let _ = writeln!(manifest, "file '{name}'\nduration 1");
        }
    }
    if let Some(name) = last.file_name().and_then(|name| name.to_str()) {
        let _ = writeln!(manifest, "file '{name}'");
    }
    let manifest_path = frames_directory.join("frames.ffconcat");
    if let Err(error) = fs::write(&manifest_path, manifest) {
        error!(%error, path = %manifest_path.display(), "could not write timelapse manifest");
        return;
    }
    let temporary = video_path.with_extension("tmp.mp4");
    let ffmpeg = std::env::var_os("STREAM_TOWN_FFMPEG_PATH")
        .map_or_else(|| PathBuf::from("ffmpeg"), PathBuf::from);
    let status = std::process::Command::new(ffmpeg)
        .args(["-hide_banner", "-loglevel", "error", "-y", "-safe", "0"])
        .args(["-f", "concat", "-i"])
        .arg(&manifest_path)
        .args(["-vf", "fps=2,format=yuv420p", "-c:v", "libx264"])
        .args([
            "-preset",
            "veryfast",
            "-crf",
            "20",
            "-movflags",
            "+faststart",
        ])
        .arg(&temporary)
        .status();
    match status {
        Ok(status) if status.success() => {
            if let Err(error) = fs::copy(&temporary, video_path) {
                error!(%error, path = %video_path.display(), "could not publish timelapse video");
            } else {
                let _ = fs::remove_file(&temporary);
                info!(path = %video_path.display(), frames = frames.len(), "city timelapse video updated");
            }
        }
        Ok(status) => error!(?status, "FFmpeg could not update the city timelapse video"),
        Err(error) => error!(%error, "FFmpeg is unavailable for city timelapse video generation"),
    }
}
