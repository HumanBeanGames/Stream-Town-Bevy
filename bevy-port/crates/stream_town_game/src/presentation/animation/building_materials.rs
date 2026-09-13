pub(crate) fn instantiate_building_materials(
    mut commands: Commands,
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    time: Res<Time>,
    content: Res<RuntimeContent>,
    parents: Query<&ChildOf>,
    buildings: Query<&RuntimeBuilding>,
    mut instances: ResMut<BuildingMaterialInstances>,
    mut materials: Option<ResMut<Assets<BuildingMaterial>>>,
    renderers: Query<
        (Entity, &MeshMaterial3d<BuildingMaterial>),
        Without<BuildingMaterialInstanced>,
    >,
) {
    let Some(materials) = materials.as_deref_mut() else {
        return;
    };
    for (entity, source) in &renderers {
        let mut ancestor = entity;
        let mut building = None;
        for _ in 0..64 {
            if let Ok(runtime) = buildings.get(ancestor) {
                building = Some(runtime.id.clone());
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        let Some(building) = building else {
            commands.entity(entity).insert(BuildingMaterialInstanced);
            continue;
        };
        let handle =
            if let Some(instance) = instances.0.get(&building) {
                instance.handle.clone()
            } else {
                let Some(mut material) = materials.get(&source.0).cloned() else {
                    continue;
                };
                let health = simulation
                    .0
                    .buildings
                    .get(&building)
                    .map_or(BUILDING_MAX_HEALTH, |state| state.health);
                let season = simulation.0.season;
                let (season_from, season_to, season_blend) = season_visual_blend(
                    simulation.0.elapsed_seconds,
                    config.0.time.seconds_per_day,
                    season,
                );
                let snow = building_snow_strength(season_from)
                    + (building_snow_strength(season_to) - building_snow_strength(season_from))
                        * season_blend;
                material.extension.parameters.snow_damage.x = snow;
                material.extension.parameters.snow_damage.y = snow;
                let max_health = simulation
                    .0
                    .buildings
                    .get(&building)
                    .map_or(BUILDING_MAX_HEALTH, |state| {
                        building_max_health(&content.0, state)
                    });
                material.extension.parameters.snow_damage.z =
                    building_damage_value(health, max_health);
                if simulation.0.buildings.get(&building).is_some_and(|state| {
                    state.archetype.as_str() == "archetype:building:guardhouse"
                }) {
                    material.extension.parameters.tint_color_strength =
                        Vec4::new(0.22, 0.62, 1.0, 0.72);
                }
                material.extension.parameters.surface_controls.z =
                    f32::from(config.0.time.max_building_emission_milli) / 1_000.0;
                material.extension.parameters.time_cycle = building_material_time_cycle(
                    &config.0.time,
                    simulation.0.elapsed_seconds,
                    time.elapsed_secs_wrapped_f64(),
                );
                let handle = materials.add(material);
                instances.0.insert(
                    building,
                    BuildingMaterialInstance {
                        handle: handle.clone(),
                        applied_health: health,
                        applied_season: season,
                        applied_season_blend_bits: daylight_signature(season_blend),
                        applied_time_cycle: BuildingTimeCycleSignature::from(&config.0.time),
                    },
                );
                handle
            };
        commands
            .entity(entity)
            .insert((MeshMaterial3d(handle), BuildingMaterialInstanced));
    }
}

pub(crate) fn sync_building_material_instances(
    simulation: Res<SimulationRuntime>,
    config: Res<RuntimeConfig>,
    time: Res<Time>,
    content: Res<RuntimeContent>,
    mut instances: ResMut<BuildingMaterialInstances>,
    mut update_runtime: ResMut<BuildingMaterialUpdateRuntime>,
    mut materials: Option<ResMut<Assets<BuildingMaterial>>>,
) {
    let Some(materials) = materials.as_deref_mut() else {
        return;
    };
    let removed: Vec<_> = instances
        .0
        .iter()
        .filter(|(id, _)| !simulation.0.buildings.contains_key(*id))
        .map(|(id, instance)| (id.clone(), instance.handle.id()))
        .collect();
    for (id, handle) in removed {
        materials.remove(handle);
        instances.0.remove(&id);
    }
    if update_runtime.order.len() != instances.0.len()
        || update_runtime
            .order
            .iter()
            .any(|id| !instances.0.contains_key(id))
    {
        update_runtime.order.clear();
        update_runtime.order.extend(instances.0.keys().cloned());
        update_runtime.next_index = update_runtime
            .next_index
            .min(update_runtime.order.len().saturating_sub(1));
    }
    if update_runtime.order.is_empty() {
        update_runtime.next_index = 0;
        return;
    }

    let mut visited = 0_usize;
    let mut updated = 0_usize;
    for index in round_robin_indices(update_runtime.order.len(), update_runtime.next_index) {
        if updated >= MAX_BUILDING_MATERIAL_UPDATES_PER_FRAME {
            break;
        }
        let id = &update_runtime.order[index];
        visited += 1;
        let Some(instance) = instances.0.get_mut(id) else {
            continue;
        };
        let Some(building) = simulation.0.buildings.get(id) else {
            continue;
        };
        let (season_from, season_to, season_blend) = season_visual_blend(
            simulation.0.elapsed_seconds,
            config.0.time.seconds_per_day,
            simulation.0.season,
        );
        let season_blend_bits = daylight_signature(season_blend);
        let time_cycle_signature = BuildingTimeCycleSignature::from(&config.0.time);
        if instance.applied_health == building.health
            && instance.applied_season == simulation.0.season
            && instance.applied_season_blend_bits == season_blend_bits
            && instance.applied_time_cycle == time_cycle_signature
        {
            continue;
        }
        let Some(mut material) = materials.get_mut(&instance.handle) else {
            continue;
        };
        let snow = building_snow_strength(season_from)
            + (building_snow_strength(season_to) - building_snow_strength(season_from))
                * season_blend;
        material.extension.parameters.snow_damage.x = snow;
        material.extension.parameters.snow_damage.y = snow;
        if instance.applied_time_cycle != time_cycle_signature {
            material.extension.parameters.surface_controls.z =
                f32::from(config.0.time.max_building_emission_milli) / 1_000.0;
            material.extension.parameters.time_cycle = building_material_time_cycle(
                &config.0.time,
                simulation.0.elapsed_seconds,
                time.elapsed_secs_wrapped_f64(),
            );
        }
        material.extension.parameters.snow_damage.z =
            building_damage_value(building.health, building_max_health(&content.0, building));
        instance.applied_health = building.health;
        instance.applied_season = simulation.0.season;
        instance.applied_season_blend_bits = season_blend_bits;
        instance.applied_time_cycle = time_cycle_signature;
        updated += 1;
    }
    update_runtime.next_index = (update_runtime.next_index + visited) % update_runtime.order.len();
}

pub(crate) fn building_damage_value(health: i32, max_health: i32) -> f32 {
    if max_health <= 0 {
        return 0.0;
    }
    let bounded_max = u16::try_from(max_health).unwrap_or(u16::MAX).max(1);
    let bounded_health = u16::try_from(health.clamp(0, i32::from(bounded_max))).unwrap_or_default();
    f32::from(bounded_health) / f32::from(bounded_max)
}
