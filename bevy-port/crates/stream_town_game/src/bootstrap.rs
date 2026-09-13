use super::*;

pub fn run(config: GameConfig, mut player_settings: PlayerSettings) {
    if std::env::var_os("STREAM_TOWN_SMOKE_OVERLAYS").is_some() {
        player_settings.interface.display_names = NameDisplayMode::AllPlayers;
        player_settings.interface.display_building_health = BuildingHealthDisplayMode::Always;
    }
    let content = embedded_content();
    let asset_root = locate_asset_root();
    let resolution = WindowResolution::new(config.window.width, config.window.height);
    let title = config.window.title.clone();
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.025, 0.04, 0.055)))
        .insert_resource(RuntimeConfig(config))
        .insert_resource(RuntimePlayerSettings(player_settings.clone()))
        .insert_resource(DirectionalLightShadowMap {
            size: usize::from(player_settings.video.shadow_map_resolution),
        })
        .insert_resource(RuntimeContent(content))
        .insert_resource(RuntimePresentation(embedded_presentation()))
        .insert_resource(RuntimeAssetRoot(asset_root.clone()))
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: runtime_wgpu_settings().into(),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: asset_root.to_string_lossy().into_owned(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title,
                        resolution,
                        present_mode: if std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some()
                        {
                            PresentMode::Immediate
                        } else if player_settings.video.vsync {
                            PresentMode::AutoVsync
                        } else {
                            PresentMode::AutoNoVsync
                        },
                        // Bevy 0.19 panics when exclusive fullscreen asks for the current
                        // monitor during window creation, while changing into exclusive mode
                        // after renderer setup can invalidate the DX12 swapchain on some
                        // drivers. Preserve the setting but use borderless compatibility.
                        mode: startup_window_mode(
                            player_settings.video.display_mode,
                            std::env::var_os("STREAM_TOWN_REPORT_FRAME_TIME").is_some(),
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(SurfaceErrorRuntime::default())
        .insert_resource(RenderErrorHandler(stream_town_render_error_handler))
        .add_plugins(tidal_music::tidal_plugin(&asset_root))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::new(600))
        .add_plugins(MaterialPlugin::<TerrainMaterial>::default())
        .add_plugins(MaterialPlugin::<WaterMaterial>::default())
        .add_plugins(MaterialPlugin::<BuildingMaterial>::default())
        .add_plugins(MaterialPlugin::<CloudMaterial>::default())
        .add_plugins(MaterialPlugin::<MenuSkyMaterial>::default())
        .add_plugins(MaterialPlugin::<GodrayMaterial>::default())
        .add_plugins(MaterialPlugin::<GiraffeMaterial>::default())
        .add_plugins(MaterialPlugin::<BoundsMaterial>::default())
        .add_plugins(MaterialPlugin::<TreeMaterial>::default())
        .add_plugins(MaterialPlugin::<GrassMaterial>::default())
        .add_plugins(MaterialPlugin::<CritterMaterial>::default())
        .add_plugins(MaterialPlugin::<FlagMaterial>::default())
        .add_plugins(MaterialPlugin::<CharacterMaterial>::default())
        .add_plugins(StreamTownGamePlugin);
    #[cfg(all(feature = "stream-profiling", target_os = "windows"))]
    app.add_plugins(profiling::StreamTownProfilingPlugin);
    app.run();
}

fn runtime_wgpu_settings() -> WgpuSettings {
    let mut settings = WgpuSettings::default();
    #[cfg(target_os = "windows")]
    {
        // Vulkan swapchain validation is unreliable on the current Windows
        // driver. DX12 is the supported shipping backend for the first release.
        settings.backends = Some(Backends::DX12);
    }
    settings
}

pub(crate) fn is_transient_surface_configuration_error(error: &RenderError) -> bool {
    matches!(error.ty, ErrorType::Validation)
        && error.description.contains("Surface::configure")
        && error.description.contains("Invalid surface")
}

fn stream_town_render_error_handler(
    error: &RenderError,
    main_world: &mut World,
    _render_world: &mut World,
) -> RenderErrorPolicy {
    if is_transient_surface_configuration_error(error) {
        let now = Instant::now();
        let mut recovery = main_world.resource_mut::<SurfaceErrorRuntime>();
        if recovery
            .last_error
            .is_none_or(|last| now.saturating_duration_since(last) > Duration::from_secs(2))
        {
            recovery.consecutive_errors = 0;
        }
        recovery.last_error = Some(now);
        recovery.consecutive_errors = recovery.consecutive_errors.saturating_add(1);
        if recovery.consecutive_errors <= 3 {
            warn!(
                attempt = recovery.consecutive_errors,
                "ignored a transient DX12 surface reconfiguration failure"
            );
            return RenderErrorPolicy::Ignore;
        }
    }
    error!(kind = ?error.ty, "rendering cannot safely continue");
    main_world.write_message(AppExit::error());
    RenderErrorPolicy::StopRendering
}

#[must_use]
pub fn player_settings_path() -> PathBuf {
    std::env::var_os("STREAM_TOWN_PLAYER_SETTINGS_PATH").map_or_else(
        || PathBuf::from(".stream-town").join("settings.ron"),
        PathBuf::from,
    )
}

pub fn load_player_settings() -> AnyResult<PlayerSettings> {
    let store = PlayerSettingsStore::new(player_settings_path());
    if store.path().is_file() || store.backup_path().is_file() {
        return store.load().context("player settings failed to load");
    }
    let settings: PlayerSettings =
        ron::from_str(include_str!("../../../assets/config/player-settings.ron"))
            .context("checked-in player settings are invalid RON")?;
    settings
        .validate()
        .context("checked-in player settings failed validation")?;
    store
        .write(&settings)
        .context("default player settings could not be persisted")?;
    Ok(settings)
}

pub(crate) fn player_window_mode(
    mode: stream_town_domain::DisplayMode,
    monitor: Option<Entity>,
) -> WindowMode {
    match mode {
        stream_town_domain::DisplayMode::Windowed => WindowMode::Windowed,
        stream_town_domain::DisplayMode::Borderless
        | stream_town_domain::DisplayMode::Fullscreen => WindowMode::BorderlessFullscreen(
            monitor.map_or(MonitorSelection::Primary, MonitorSelection::Entity),
        ),
    }
}

pub(crate) fn startup_window_mode(
    mode: stream_town_domain::DisplayMode,
    benchmarking: bool,
) -> WindowMode {
    if benchmarking {
        return WindowMode::Windowed;
    }
    match mode {
        stream_town_domain::DisplayMode::Windowed => WindowMode::Windowed,
        stream_town_domain::DisplayMode::Borderless
        | stream_town_domain::DisplayMode::Fullscreen => {
            WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
        }
    }
}

pub fn load_runtime_config() -> AnyResult<GameConfig> {
    let configured = std::env::var_os("STREAM_TOWN_CONFIG").is_some();
    let path = runtime_config_path();
    let encoded = if configured {
        std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read runtime config {}", path.display()))?
    } else if path.is_file() {
        std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read runtime config {}", path.display()))?
    } else {
        include_str!("../../../assets/config/game.ron").to_owned()
    };
    ron::from_str::<GameConfig>(&encoded)
        .context("runtime config is invalid RON")?
        .upgrade()
        .context("runtime config failed validation")
}

#[must_use]
pub fn runtime_config_path() -> PathBuf {
    std::env::var_os("STREAM_TOWN_CONFIG").map_or_else(
        || PathBuf::from(".stream-town").join("config.ron"),
        PathBuf::from,
    )
}

/// Renders the exact adaptive score expression and tempo used by the live
/// world, allowing the authoring tool to audition edits without duplicating
/// composition logic.
pub fn adaptive_music_preview_program(
    config: &AdaptiveMusicConfig,
    intensity: f64,
    season: f64,
    time_of_day: f64,
    population: usize,
    building_count: usize,
) -> Result<(String, f64), String> {
    tidal_music::preview_adaptive_music_program(
        config,
        intensity,
        season,
        time_of_day,
        population,
        building_count,
    )
}

pub fn save_runtime_config(config: &GameConfig) -> AnyResult<PathBuf> {
    config
        .validate()
        .context("runtime config failed validation")?;
    let path = runtime_config_path();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent).with_context(|| {
        format!(
            "failed to create runtime config directory {}",
            parent.display()
        )
    })?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let backup = PathBuf::from(format!("{}.bak", path.display()));
    let encoded =
        ron::ser::to_string_pretty(config, ron::ser::PrettyConfig::new().struct_names(true))
            .context("failed to encode runtime config")?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)
        .with_context(|| format!("failed to create {}", temporary.display()))?;
    file.write_all(encoded.as_bytes())
        .with_context(|| format!("failed to write {}", temporary.display()))?;
    file.sync_all()
        .with_context(|| format!("failed to flush {}", temporary.display()))?;
    if path.is_file() {
        std::fs::copy(&path, &backup)
            .with_context(|| format!("failed to back up runtime config to {}", backup.display()))?;
        std::fs::remove_file(&path)
            .with_context(|| format!("failed to replace runtime config {}", path.display()))?;
    }
    if let Err(error) = std::fs::rename(&temporary, &path) {
        if backup.is_file() && !path.exists() {
            let _ = std::fs::copy(&backup, &path);
        }
        let _ = std::fs::remove_file(&temporary);
        return Err(error)
            .with_context(|| format!("failed to install runtime config {}", path.display()));
    }
    let reloaded = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to verify runtime config {}", path.display()))?;
    let reloaded = ron::from_str::<GameConfig>(&reloaded)
        .context("saved runtime config is invalid RON")?
        .upgrade()
        .context("saved runtime config failed validation")?;
    if reloaded != *config {
        anyhow::bail!("reloaded runtime config does not match the saved configuration");
    }
    Ok(path)
}
