use super::super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn update_environment_presentation(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    catalog: Res<RuntimePresentation>,
    render: Res<RenderAssets>,
    mut presentation: ResMut<EnvironmentPresentation>,
    mut clear_color: Option<ResMut<ClearColor>>,
    mut terrain_materials: Option<ResMut<Assets<TerrainMaterial>>>,
    mut water_materials: Option<ResMut<Assets<WaterMaterial>>>,
    mut building_materials: Option<ResMut<Assets<BuildingMaterial>>>,
    mut tree_materials: Option<ResMut<Assets<TreeMaterial>>>,
    mut grass_materials: Option<ResMut<Assets<GrassMaterial>>>,
    mut cameras: Query<(&mut DistanceFog, &mut AmbientLight), With<TownCamera>>,
    mut sun: TownSunMutQuery,
    particles: Query<Entity, With<WeatherParticle>>,
) {
    let environment = (simulation.0.season, simulation.0.weather);
    let (season_from, season_to, season_blend) = season_visual_blend(
        simulation.0.elapsed_seconds,
        config.0.time.seconds_per_day,
        simulation.0.season,
    );
    let time_cycle = config.0.time.sample(simulation.0.elapsed_seconds);
    let daylight = time_cycle.daylight.clamp(0.0, 1.0);
    let color_filter = authored_rgb_filter(&authored_post_process_stack(
        &catalog.0,
        WORLD_SCENE_PATH,
        daylight,
    ));
    let daylight_bits = daylight_signature(daylight);
    let season_blend_bits = daylight_signature(season_blend);
    let environment_changed = presentation.environment != Some(environment);
    let daylight_changed = presentation.daylight_bits != Some(daylight_bits);
    let season_visual_changed = presentation.season_blend_bits != Some(season_blend_bits);
    if !environment_changed && !daylight_changed && !season_visual_changed {
        return;
    }
    let palette = blend_environment_palette(
        environment_palette(season_from, environment.1, &config.0.terrain),
        environment_palette(season_to, environment.1, &config.0.terrain),
        season_blend,
    );
    let authored_daylight = f32::from(config.0.time.day_light_intensity_milli) / 1_000.0;
    let authored_nightlight = f32::from(config.0.time.night_light_intensity_milli) / 1_000.0;
    let light_ratio = (authored_nightlight + (authored_daylight - authored_nightlight) * daylight)
        / authored_daylight.max(f32::EPSILON);
    if let Some(clear_color) = clear_color.as_deref_mut() {
        let sky_ratio = 0.35 + 0.65 * light_ratio;
        clear_color.0 = Color::srgb(
            palette.clear_color[0] * sky_ratio * color_filter[0],
            palette.clear_color[1] * sky_ratio * color_filter[1],
            palette.clear_color[2] * sky_ratio * color_filter[2],
        );
    }
    if (environment_changed || season_visual_changed)
        && let Some(terrain_materials) = terrain_materials.as_deref_mut()
        && let Some(mut ground) = terrain_materials.get_mut(&render.ground)
    {
        ground.extension.parameters.season_tint = Vec4::new(
            palette.terrain_tint[0],
            palette.terrain_tint[1],
            palette.terrain_tint[2],
            palette.terrain_winter_strength,
        );
        let from_path = seasonal_terrain_palette(&config.0.terrain, season_from).traversal_tint;
        let to_path = seasonal_terrain_palette(&config.0.terrain, season_to).traversal_tint;
        ground.extension.parameters.traversal_dirt_color =
            Vec4::from_array(std::array::from_fn(|index| {
                from_path[index] + (to_path[index] - from_path[index]) * season_blend
            }));
        let from_surface = seasonal_terrain_palette(&config.0.terrain, season_from).path_tint;
        let to_surface = seasonal_terrain_palette(&config.0.terrain, season_to).path_tint;
        ground.extension.parameters.constructed_path_color =
            Vec4::from_array(std::array::from_fn(|index| {
                from_surface[index] + (to_surface[index] - from_surface[index]) * season_blend
            }));
    }
    if (environment_changed || season_visual_changed)
        && let Some(water_materials) = water_materials.as_deref_mut()
        && let Some(mut water) = water_materials.get_mut(&render.water)
    {
        let surface = water.extension.parameters.surface_color;
        water.extension.parameters.season_tint = water_color_tint(surface, palette.water_color);
        water.extension.parameters.scale_foam_ice.w = water_ice_strength(season_from)
            + (water_ice_strength(season_to) - water_ice_strength(season_from)) * season_blend;
    }
    if (environment_changed || season_visual_changed)
        && let Some(building_materials) = building_materials.as_deref_mut()
        && let Some(mut building) = building_materials.get_mut(&render.authored_building)
    {
        let snow = building_snow_strength(season_from)
            + (building_snow_strength(season_to) - building_snow_strength(season_from))
                * season_blend;
        building.extension.parameters.snow_damage.x = snow;
        building.extension.parameters.snow_damage.y = snow;
    }
    if (environment_changed || season_visual_changed)
        && let Some(tree_materials) = tree_materials.as_deref_mut()
        && let Some(mut tree) = tree_materials.get_mut(&render.tree)
    {
        tree.extension.parameters.season_controls =
            tree_season_controls(season_from).lerp(tree_season_controls(season_to), season_blend);
    }
    if (environment_changed || season_visual_changed)
        && let Some(grass_materials) = grass_materials.as_deref_mut()
        && let Some(mut grass) = grass_materials.get_mut(&render.grass)
    {
        let from = grass_season_controls(season_from);
        let to = grass_season_controls(season_to);
        let grid_1 = from.0.lerp(to.0, season_blend);
        let grid_2 = from.1.lerp(to.1, season_blend);
        let wind = from.2.lerp(to.2, season_blend);
        let spring = from.3 + (to.3 - from.3) * season_blend;
        let tint = from.4 + (to.4 - from.4) * season_blend;
        grass.extension.parameters.grid_color_1 = grid_1;
        grass.extension.parameters.grid_color_2 = grid_2;
        grass.extension.parameters.wind_color = wind;
        grass.extension.parameters.surface_controls.y = spring;
        grass.extension.parameters.surface_controls.z = tint;
    }
    for (mut fog, mut ambient) in &mut cameras {
        fog.color = Color::srgba(
            palette.fog_color[0] * color_filter[0],
            palette.fog_color[1] * color_filter[1],
            palette.fog_color[2] * color_filter[2],
            palette.fog_color[3],
        );
        if environment_changed || season_visual_changed {
            fog.falloff = FogFalloff::Linear {
                start: palette.fog_start,
                end: palette.fog_end,
            };
        }
        ambient.color = Color::srgb(
            palette.ambient_color[0] * color_filter[0],
            palette.ambient_color[1] * color_filter[1],
            palette.ambient_color[2] * color_filter[2],
        );
        let ambient_ratio = 0.55 + 0.45 * light_ratio;
        ambient.brightness = in_game_ambient_brightness(palette.ambient_brightness) * ambient_ratio;
    }
    for (mut light, mut transform) in &mut sun {
        light.color = Color::srgb(
            palette.sun_color[0] * color_filter[0],
            palette.sun_color[1] * color_filter[1],
            palette.sun_color[2] * color_filter[2],
        );
        light.illuminance = palette.sun_illuminance * light_ratio;
        *transform = in_game_sun_transform_for_daylight(daylight);
    }
    if environment_changed {
        for entity in &particles {
            commands.entity(entity).try_despawn();
        }
        spawn_weather_particles(
            &mut commands,
            &config.0,
            &render,
            simulation.0.world_seed,
            environment.1,
            palette.particle_count,
        );
    }
    if environment_changed {
        info!(
            season = ?environment.0,
            weather = ?environment.1,
            daylight,
            particles = palette.particle_count,
            "environment presentation updated"
        );
    }
    presentation.environment = Some(environment);
    presentation.daylight_bits = Some(daylight_bits);
    presentation.season_blend_bits = Some(season_blend_bits);
}

pub(crate) fn previous_season(season: Season) -> Season {
    match season {
        Season::Spring => Season::Winter,
        Season::Summer => Season::Spring,
        Season::Autumn => Season::Summer,
        Season::Winter => Season::Autumn,
    }
}

pub(crate) fn season_visual_blend(
    elapsed_seconds: f64,
    seconds_per_day: u32,
    current: Season,
) -> (Season, Season, f32) {
    let season_seconds = f64::from(seconds_per_day.max(1)) * f64::from(DAYS_PER_SEASON);
    let elapsed = elapsed_seconds.max(0.0);
    let season_index = Duration::from_secs_f64(elapsed / season_seconds).as_secs();
    let phase = elapsed.rem_euclid(season_seconds);
    let transition_seconds = SEASON_TRANSITION_SECONDS.min(season_seconds);
    if season_index == 0 || phase >= transition_seconds || transition_seconds <= f64::EPSILON {
        return (current, current, 1.0);
    }
    let blend = Duration::from_secs_f64(phase / transition_seconds).as_secs_f32();
    (previous_season(current), current, blend.clamp(0.0, 1.0))
}

pub(crate) fn blend_environment_palette(
    from: EnvironmentPalette,
    to: EnvironmentPalette,
    blend: f32,
) -> EnvironmentPalette {
    let blend_array = |from: &[f32], to: &[f32]| {
        from.iter()
            .zip(to)
            .map(|(from, to)| from + (to - from) * blend)
            .collect::<Vec<_>>()
    };
    let terrain_tint = blend_array(&from.terrain_tint, &to.terrain_tint);
    let water_color = blend_array(&from.water_color, &to.water_color);
    let clear_color = blend_array(&from.clear_color, &to.clear_color);
    let sun_color = blend_array(&from.sun_color, &to.sun_color);
    let ambient_color = blend_array(&from.ambient_color, &to.ambient_color);
    let fog_color = blend_array(&from.fog_color, &to.fog_color);
    EnvironmentPalette {
        terrain_tint: [terrain_tint[0], terrain_tint[1], terrain_tint[2]],
        terrain_winter_strength: from.terrain_winter_strength
            + (to.terrain_winter_strength - from.terrain_winter_strength) * blend,
        water_color: [
            water_color[0],
            water_color[1],
            water_color[2],
            water_color[3],
        ],
        clear_color: [clear_color[0], clear_color[1], clear_color[2]],
        sun_color: [sun_color[0], sun_color[1], sun_color[2]],
        sun_illuminance: from.sun_illuminance + (to.sun_illuminance - from.sun_illuminance) * blend,
        ambient_color: [ambient_color[0], ambient_color[1], ambient_color[2]],
        ambient_brightness: from.ambient_brightness
            + (to.ambient_brightness - from.ambient_brightness) * blend,
        fog_color: [fog_color[0], fog_color[1], fog_color[2], fog_color[3]],
        fog_start: from.fog_start + (to.fog_start - from.fog_start) * blend,
        fog_end: from.fog_end + (to.fog_end - from.fog_end) * blend,
        particle_count: to.particle_count,
    }
}

#[derive(Clone, Copy)]
pub(crate) struct NightLightSpec {
    pub(crate) position: Vec3,
    pub(crate) color: Color,
    pub(crate) intensity: f32,
    pub(crate) range: f32,
    pub(crate) transition_delay_seconds: f32,
}

pub(crate) fn night_light_transition_delay(hash: u64) -> f32 {
    let fraction = u16::try_from(hash % 10_001).expect("night-light hash is bounded");
    f32::from(fraction) / 10_000.0 * NIGHT_LIGHT_TRANSITION_SECONDS
}

pub(crate) fn night_light_is_active(
    config: &stream_town_domain::TimeCycleConfig,
    elapsed_seconds: f64,
    delay_seconds: f32,
) -> bool {
    let cycle_seconds = f64::from(config.seconds_per_day.max(1));
    let phase_seconds =
        Duration::from_secs_f64(elapsed_seconds.max(0.0).rem_euclid(cycle_seconds)).as_secs_f32();
    let day_seconds = Duration::from_secs(u64::from(config.seconds_per_day.max(1))).as_secs_f32();
    let night_start_seconds = day_seconds * f32::from(config.daylight_per_thousand) / 1_000.0;
    if config.sample(elapsed_seconds).is_daytime {
        phase_seconds < delay_seconds
    } else {
        phase_seconds - night_start_seconds >= delay_seconds
    }
}

pub(crate) fn night_light_sources_are_inactive(
    config: &stream_town_domain::TimeCycleConfig,
    elapsed_seconds: f64,
) -> bool {
    let cycle_seconds = f64::from(config.seconds_per_day.max(1));
    let phase_seconds =
        Duration::from_secs_f64(elapsed_seconds.max(0.0).rem_euclid(cycle_seconds)).as_secs_f32();
    config.sample(elapsed_seconds).is_daytime && phase_seconds >= NIGHT_LIGHT_TRANSITION_SECONDS
}

pub(crate) fn append_nearest_night_lights(
    sources: &mut Vec<NightLightSpec>,
    candidates: &mut [(f32, NightLightSpec)],
    maximum: usize,
) {
    candidates.sort_by(|left, right| left.0.total_cmp(&right.0));
    sources.extend(candidates.iter().take(maximum).map(|(_, spec)| *spec));
}

pub(crate) fn building_night_light_profile(
    archetype: &StableId,
    cell_size: f32,
) -> Option<(f32, f32, f32)> {
    match archetype.as_str() {
        "archetype:building:torch" => Some((cell_size * 1.5, 220_000.0, cell_size * 5.0)),
        "archetype:building:streetlight" => Some((cell_size * 1.8, 180_000.0, cell_size * 5.0)),
        _ => None,
    }
}

pub(crate) fn sync_pooled_night_lights(
    mut commands: Commands,
    config: Res<RuntimeConfig>,
    simulation: Res<SimulationRuntime>,
    agents: Query<(&Agent, &GlobalTransform)>,
    buildings: Query<(&RuntimeBuilding, &GlobalTransform)>,
    projectiles: Query<(&CombatProjectile, &GlobalTransform)>,
    cameras: Query<&GlobalTransform, With<TownCamera>>,
    mut slots: Query<
        (Entity, &mut Transform, &mut PointLight, &mut Visibility),
        With<NightPointLightPoolSlot>,
    >,
) {
    if night_light_sources_are_inactive(&config.0.time, simulation.0.elapsed_seconds) {
        for (_, _, mut light, mut visibility) in &mut slots {
            if light.intensity != 0.0 {
                light.intensity = 0.0;
            }
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
        }
        return;
    }

    let cell_size = config.0.world.cell_size;
    let camera_position = cameras
        .iter()
        .next()
        .map_or(Vec3::ZERO, GlobalTransform::translation);
    let mut sources = Vec::new();
    let mut citizen_sources = Vec::new();
    for (agent, transform) in &agents {
        if agent.kind == ActorKind::Player {
            let actor = simulation.0.actors.get(&agent.id);
            if actor.is_some_and(|actor| !actor.alive) {
                continue;
            }
            let color = perceptually_normalized_light_color(
                actor
                    .and_then(|actor| actor.customization.night_light_color)
                    .unwrap_or([255, 209, 143]),
            );
            let level_multiplier = actor.map_or(1.0, player_night_light_level_multiplier);
            let spec = NightLightSpec {
                position: transform.translation() + Vec3::Y * cell_size * 0.9,
                color,
                intensity: 115_000.0 * level_multiplier,
                // Dense towns can overlap dozens of lights. Keep the authored
                // local pool while bounding per-pixel clustered-light work.
                range: cell_size * 4.0,
                transition_delay_seconds: night_light_transition_delay(stable_id_hash(&agent.id)),
            };
            citizen_sources.push((
                spec.position.xz().distance_squared(camera_position.xz()),
                spec,
            ));
        }
    }
    append_nearest_night_lights(
        &mut sources,
        &mut citizen_sources,
        MAX_ACTIVE_CITIZEN_NIGHT_LIGHTS,
    );
    let mut building_sources = Vec::new();
    for (building, transform) in &buildings {
        let Some(state) = simulation.0.buildings.get(&building.id) else {
            continue;
        };
        let Some((height, intensity, range)) = state
            .complete
            .then(|| building_night_light_profile(&state.archetype, cell_size))
            .flatten()
        else {
            continue;
        };
        let color = simulation
            .0
            .building_night_light_colors
            .get(&building.id)
            .copied()
            .map_or(Color::srgb(1.0, 0.66, 0.30), color_from_rgb8);
        let spec = NightLightSpec {
            position: transform.translation() + Vec3::Y * height,
            color,
            intensity,
            range,
            transition_delay_seconds: night_light_transition_delay(stable_id_hash(&building.id)),
        };
        building_sources.push((
            spec.position.xz().distance_squared(camera_position.xz()),
            spec,
        ));
    }
    append_nearest_night_lights(
        &mut sources,
        &mut building_sources,
        MAX_ACTIVE_BUILDING_NIGHT_LIGHTS,
    );
    let mut projectile_sources = Vec::new();
    for (projectile, transform) in &projectiles {
        let color = match projectile.visual {
            CombatVisualKind::Fireball => Color::srgb(1.0, 0.24, 0.04),
            CombatVisualKind::Necrotic => Color::srgb(0.48, 0.16, 0.92),
            CombatVisualKind::Arrow | CombatVisualKind::Physical => Color::srgb(1.0, 0.78, 0.38),
        };
        let spec = NightLightSpec {
            position: transform.translation(),
            color,
            intensity: 90_000.0,
            range: cell_size * 2.5,
            transition_delay_seconds: night_light_transition_delay(
                stable_id_hash(&projectile.target)
                    ^ match &projectile.source {
                        ProjectileSource::Actor(id) | ProjectileSource::Building(id) => {
                            stable_id_hash(id).rotate_left(17)
                        }
                    },
            ),
        };
        projectile_sources.push((
            spec.position.xz().distance_squared(camera_position.xz()),
            spec,
        ));
    }
    append_nearest_night_lights(
        &mut sources,
        &mut projectile_sources,
        MAX_ACTIVE_PROJECTILE_NIGHT_LIGHTS,
    );
    let mut slot_count = 0_usize;
    for (_, mut transform, mut light, mut visibility) in &mut slots {
        let active = sources.get(slot_count).copied().filter(|spec| {
            night_light_is_active(
                &config.0.time,
                simulation.0.elapsed_seconds,
                spec.transition_delay_seconds,
            )
        });
        if let Some(spec) = active {
            if transform.translation != spec.position {
                transform.translation = spec.position;
            }
            if light.color != spec.color {
                light.color = spec.color;
            }
            if light.intensity.to_bits() != spec.intensity.to_bits() {
                light.intensity = spec.intensity;
            }
            if light.range.to_bits() != spec.range.to_bits() {
                light.range = spec.range;
            }
            if *visibility != Visibility::Visible {
                *visibility = Visibility::Visible;
            }
        } else {
            if light.intensity != 0.0 {
                light.intensity = 0.0;
            }
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
        }
        slot_count += 1;
    }
    while slot_count < NIGHT_LIGHT_POOL_CAPACITY {
        let active = sources.get(slot_count).copied().filter(|spec| {
            night_light_is_active(
                &config.0.time,
                simulation.0.elapsed_seconds,
                spec.transition_delay_seconds,
            )
        });
        let spec = active.unwrap_or(NightLightSpec {
            position: Vec3::ZERO,
            color: Color::WHITE,
            intensity: 0.0,
            range: cell_size * 3.0,
            transition_delay_seconds: 0.0,
        });
        commands.spawn((
            WorldEntity,
            NightPointLightPoolSlot,
            PointLight {
                color: spec.color,
                intensity: spec.intensity,
                range: spec.range,
                radius: cell_size * 0.08,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(spec.position),
            if active.is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
        ));
        slot_count += 1;
    }
}

pub(crate) fn color_from_rgb8(color: [u8; 3]) -> Color {
    Color::srgb_u8(color[0], color[1], color[2])
}

pub(crate) fn preset_player_name_color(world_seed: u64, actor_id: &StableId) -> [u8; 3] {
    let hash = actor_id
        .as_str()
        .bytes()
        .fold(world_seed ^ 0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
        });
    PLAYER_NAME_COLOR_PRESETS
        [usize::try_from(hash % PLAYER_NAME_COLOR_PRESETS.len() as u64).expect("preset index fits")]
}

pub(crate) fn srgb_channel_to_linear(channel: u8) -> f32 {
    let channel = f32::from(channel) / 255.0;
    if channel <= 0.040_45 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

pub(crate) fn perceptually_normalized_light_color(color: [u8; 3]) -> Color {
    let linear = color.map(srgb_channel_to_linear);
    let luminance = linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722;
    if luminance <= f32::EPSILON {
        return Color::BLACK;
    }
    Color::linear_rgb(
        linear[0] / luminance,
        linear[1] / luminance,
        linear[2] / luminance,
    )
}

pub(crate) fn player_night_light_level_multiplier(actor: &ActorState) -> f32 {
    let combined_level = actor
        .role_progression
        .values()
        .fold(0_u16, |total, progress| {
            total.saturating_add(progress.level)
        });
    let combined_level = f32::from(combined_level);
    1.0 + 2.0 * combined_level / (combined_level + 50.0)
}

pub(crate) fn daylight_signature(value: f32) -> u32 {
    value.clamp(0.0, 1.0).to_bits()
}

pub(crate) fn building_material_time_cycle(
    config: &stream_town_domain::TimeCycleConfig,
    simulation_elapsed_seconds: f64,
    app_elapsed_wrapped_seconds: f64,
) -> Vec4 {
    let cycle_seconds = u64::from(config.seconds_per_day.max(1));
    let cycle = Duration::from_secs(cycle_seconds).as_secs_f32();
    let phase_offset = Duration::from_secs_f64(
        (simulation_elapsed_seconds - app_elapsed_wrapped_seconds)
            .rem_euclid(f64::from(config.seconds_per_day.max(1))),
    )
    .as_secs_f32();
    Vec4::new(
        cycle,
        cycle * f32::from(config.daylight_per_thousand) / 1_000.0,
        Duration::from_secs(u64::from(config.transition_seconds)).as_secs_f32(),
        phase_offset,
    )
}

pub(crate) fn spawn_weather_particles(
    commands: &mut Commands,
    config: &GameConfig,
    render: &RenderAssets,
    world_seed: u64,
    weather: Weather,
    count: u16,
) {
    let (material, scale) = match weather {
        Weather::Rain => (
            render.rain.clone(),
            Vec3::new(0.18, config.world.cell_size * 0.48, 0.18),
        ),
        Weather::Snow => (render.snow.clone(), Vec3::splat(0.62)),
        Weather::Clear | Weather::Fog => return,
    };
    let span_x = f32::from(config.world.width) * config.world.cell_size;
    let span_z = f32::from(config.world.height) * config.world.cell_size;
    for index in 0..count {
        let seed = weather_particle_seed(world_seed, index);
        let x = (unit_from_seed(seed) - 0.5) * span_x;
        let z = (unit_from_seed(seed.rotate_left(13)) - 0.5) * span_z;
        let y = 18.0 + unit_from_seed(seed.rotate_left(23)) * 88.0;
        commands.spawn((
            WorldEntity,
            WeatherParticle {
                kind: weather,
                seed,
            },
            Mesh3d(render.cube.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(x, y, z).with_scale(scale),
            Visibility::Inherited,
            // Unity weather particles are transparent VFX. Letting these long
            // rain meshes enter Bevy's shadow map creates fast black streaks
            // racing over the terrain.
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }
}

pub(crate) fn animate_weather_particles(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    settings: Option<Res<RuntimePlayerSettings>>,
    cameras: Query<&GlobalTransform, With<TownCamera>>,
    mut particles: Query<(&WeatherParticle, &mut Transform, &mut Visibility)>,
) {
    if settings.is_some_and(|settings| settings.0.interface.reduced_motion) {
        for (_, _, mut visibility) in &mut particles {
            *visibility = Visibility::Hidden;
        }
        return;
    }
    let half_x = f32::from(config.0.world.width) * config.0.world.cell_size * 0.5;
    let half_z = f32::from(config.0.world.height) * config.0.world.cell_size * 0.5;
    let camera_position = cameras.single().ok().map(GlobalTransform::translation);
    for (particle, mut transform, mut visibility) in &mut particles {
        let speed = if particle.kind == Weather::Rain {
            52.0
        } else {
            9.0
        };
        transform.translation.y -= speed * time.delta_secs();
        if particle.kind == Weather::Snow {
            let seed_phase = f32::from(
                u16::try_from(particle.seed & 0xff).expect("masked weather seed fits u16"),
            );
            transform.translation.x +=
                (time.elapsed_secs() * 0.72 + seed_phase).sin() * time.delta_secs() * 1.8;
        }
        if transform.translation.y < -2.0 {
            transform.translation.y = 94.0 + unit_from_seed(particle.seed.rotate_left(7)) * 24.0;
        }
        if transform.translation.x < -half_x {
            transform.translation.x += half_x * 2.0;
        } else if transform.translation.x > half_x {
            transform.translation.x -= half_x * 2.0;
        }
        if transform.translation.z < -half_z {
            transform.translation.z += half_z * 2.0;
        } else if transform.translation.z > half_z {
            transform.translation.z -= half_z * 2.0;
        }
        *visibility = if camera_position.is_none_or(|camera_position| {
            weather_particle_visible(particle.kind, transform.translation, camera_position)
        }) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub(crate) fn weather_particle_visible(
    kind: Weather,
    particle_position: Vec3,
    camera_position: Vec3,
) -> bool {
    kind != Weather::Rain
        || particle_position.distance_squared(camera_position)
            >= RAIN_CAMERA_CULL_DISTANCE * RAIN_CAMERA_CULL_DISTANCE
}

pub(crate) fn seasonal_terrain_palette(
    terrain: &stream_town_domain::TerrainAppearanceConfig,
    season: Season,
) -> stream_town_domain::SeasonalTerrainPalette {
    match season {
        Season::Spring => terrain.spring,
        Season::Summer => terrain.summer,
        Season::Autumn => terrain.autumn,
        Season::Winter => terrain.winter,
    }
}

pub(crate) fn environment_palette(
    season: Season,
    weather: Weather,
    terrain: &stream_town_domain::TerrainAppearanceConfig,
) -> EnvironmentPalette {
    let terrain_tint = seasonal_terrain_palette(terrain, season).base_color;
    let (water_color, clear_color, sun_color, ambient_color) = match season {
        Season::Spring => (
            [0.05, 0.29, 0.47, 0.62],
            [0.025, 0.04, 0.055],
            [1.0, 0.95, 0.84],
            [0.70, 0.82, 0.92],
        ),
        Season::Summer => (
            [0.03, 0.34, 0.54, 0.62],
            [0.035, 0.045, 0.05],
            [1.0, 0.88, 0.68],
            [0.82, 0.78, 0.68],
        ),
        Season::Autumn => (
            [0.08, 0.25, 0.36, 0.65],
            [0.055, 0.035, 0.035],
            [1.0, 0.72, 0.50],
            [0.82, 0.62, 0.52],
        ),
        Season::Winter => (
            [0.10, 0.27, 0.40, 0.68],
            [0.032, 0.044, 0.060],
            [0.78, 0.88, 1.0],
            [0.68, 0.78, 0.92],
        ),
    };
    let (sun_illuminance, ambient_brightness, fog_color, fog_start, fog_end, particle_count) =
        match weather {
            Weather::Clear => (14_000.0, 90.0, [0.58, 0.72, 0.78, 0.08], 560.0, 940.0, 0),
            Weather::Rain => (7_500.0, 72.0, [0.32, 0.42, 0.50, 0.32], 240.0, 650.0, 180),
            Weather::Fog => (5_500.0, 80.0, [0.62, 0.68, 0.69, 0.72], 70.0, 390.0, 0),
            Weather::Snow => (9_000.0, 105.0, [0.76, 0.84, 0.90, 0.42], 170.0, 590.0, 150),
        };
    EnvironmentPalette {
        terrain_tint: [terrain_tint[0], terrain_tint[1], terrain_tint[2]],
        terrain_winter_strength: if season == Season::Winter { 1.0 } else { 0.0 },
        water_color,
        clear_color,
        sun_color,
        sun_illuminance,
        ambient_color,
        ambient_brightness,
        fog_color,
        fog_start,
        fog_end,
        particle_count,
    }
}

pub(crate) fn water_ice_strength(season: Season) -> f32 {
    if season == Season::Winter { 1.0 } else { 0.0 }
}

pub(crate) fn water_color_tint(surface: Vec4, target: [f32; 4]) -> Vec4 {
    let target = unity_shader_color(target);
    Vec4::new(
        target.x / surface.x.max(0.1),
        target.y / surface.y.max(0.1),
        target.z / surface.z.max(0.1),
        target.w,
    )
}

pub(crate) fn building_snow_strength(season: Season) -> f32 {
    if season == Season::Winter { 1.0 } else { 0.0 }
}

pub(crate) fn tree_season_controls(season: Season) -> Vec4 {
    match season {
        Season::Spring => Vec4::new(0.0, 0.0, 0.1, 0.0),
        Season::Summer => Vec4::ZERO,
        Season::Autumn => Vec4::new(0.3, 0.0, 0.0, 0.0),
        Season::Winter => Vec4::new(0.0, 0.5, 0.0, 0.0),
    }
}

pub(crate) fn grass_season_controls(season: Season) -> (Vec4, Vec4, Vec4, f32, f32) {
    let (grid_1, grid_2, wind, spring, tint) = match season {
        Season::Spring => (
            Vec4::new(0.282_352_95, 0.482_352_94, 0.149_019_61, 0.0),
            Vec4::new(0.262_745_1, 0.431_372_55, 0.129_411_77, 0.0),
            Vec4::new(0.315_821_56, 0.518, 0.187_516, 0.0),
            0.1,
            0.0,
        ),
        Season::Summer => (
            Vec4::new(0.416_476_4, 0.498_039_22, 0.152_941_2, 1.0),
            Vec4::new(0.419_094_32, 0.470_588_24, 0.141_176_5, 1.0),
            Vec4::new(0.447_058_83, 0.533_333_36, 0.164_705_89, 0.0),
            0.0,
            0.0,
        ),
        Season::Autumn => (
            Vec4::new(0.470_588_24, 0.435_294_12, 0.156_862_75, 0.0),
            Vec4::new(0.470_588_24, 0.415_686_28, 0.156_862_75, 0.0),
            Vec4::new(0.496, 0.458_985_03, 0.166_567_15, 0.0),
            0.0,
            0.0,
        ),
        Season::Winter => (
            Vec4::new(0.849_056_6, 0.849_056_6, 0.849_056_6, 0.0),
            Vec4::new(0.772_549_03, 0.772_549_03, 0.772_549_03, 0.0),
            Vec4::new(0.965, 0.965, 0.965, 0.0),
            0.0,
            0.42,
        ),
    };
    (
        unity_shader_color(grid_1.to_array()),
        unity_shader_color(grid_2.to_array()),
        unity_shader_color(wind.to_array()),
        spring,
        tint,
    )
}

pub(crate) fn weather_particle_seed(world_seed: u64, index: u16) -> u32 {
    let mut value = world_seed
        ^ u64::from(index)
            .wrapping_add(1)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    u32::try_from(value & u64::from(u32::MAX)).expect("masked weather seed fits u32")
}

pub(crate) fn unit_from_seed(seed: u32) -> f32 {
    let fraction = u16::try_from(seed & u32::from(u16::MAX)).expect("masked seed fits u16");
    f32::from(fraction) / f32::from(u16::MAX)
}
