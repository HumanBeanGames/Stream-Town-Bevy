use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    time::Instant,
};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use image::{DynamicImage, RgbImage, imageops::FilterType};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use stream_town_domain::{
    ActorKind, ContentCatalog, DirtyRegion, GameConfig, GridPos, NATIVE_SAVE_BACKUP_GENERATIONS,
    NativeSaveStore, PlayerSettings, PresentationCatalog, SHIPPING_SECONDS_PER_DAY,
    TechnologyGraphLayout, generate_world_with_content,
};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(about = "Stream Town repository automation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate authored configuration and generated manifests.
    Validate,
    /// Exercise deterministic path planning at the configured agent scale.
    Stress {
        #[arg(long, default_value_t = 300)]
        agents: u32,
        /// Simulation ticks to soak; 3,600 represents one minute at 60 Hz.
        #[arg(long, default_value_t = 3_600)]
        ticks: u32,
    },
    /// Build and validate a portable Windows release archive.
    PackageWindows {
        #[arg(long, default_value = "dist")]
        output: std::path::PathBuf,
        #[arg(long)]
        skip_build: bool,
    },
    /// Compare deterministic GPU captures with the curated visual references.
    VisualAcceptance {
        #[arg(long)]
        capture_dir: std::path::PathBuf,
        #[arg(long)]
        scenario: Vec<String>,
        /// Replace the curated references with the supplied, manually reviewed captures.
        #[arg(long)]
        update_baseline: bool,
    },
    /// Remove every currently saved enemy while preserving citizens and world progress.
    PurgeSaveEnemies {
        #[arg(long)]
        save: std::path::PathBuf,
    },
}

#[derive(Debug, Deserialize)]
struct VisualAcceptanceConfig {
    schema_version: u32,
    reference_width: u32,
    reference_height: u32,
    scenarios: Vec<VisualAcceptanceScenario>,
}

#[derive(Debug, Deserialize)]
struct VisualAcceptanceScenario {
    name: String,
    max_mean_absolute_error: f64,
}

#[derive(Debug, Deserialize)]
struct VisualBaselineManifest {
    schema_version: u32,
    references: BTreeMap<String, VisualBaselineReference>,
}

#[derive(Debug, Deserialize)]
struct VisualBaselineReference {
    sha256: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
struct AudioBaselineManifest {
    schema_version: u32,
    sample_rate: u32,
    clips: BTreeMap<String, String>,
}

struct StressAgent {
    position: GridPos,
    goal: GridPos,
    path: Vec<GridPos>,
    cursor: usize,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Validate => validate(),
        Command::Stress { agents, ticks } => stress(agents, ticks),
        Command::PackageWindows { output, skip_build } => {
            let report = xtask::package_windows(Path::new("."), &output, skip_build)?;
            println!(
                "Packaged {} files ({} bytes) at {}",
                report.files,
                report.bytes,
                report.archive.display()
            );
            Ok(())
        }
        Command::VisualAcceptance {
            capture_dir,
            scenario,
            update_baseline,
        } => visual_acceptance(&capture_dir, &scenario, update_baseline),
        Command::PurgeSaveEnemies { save } => purge_save_enemies(&save),
    }
}

fn purge_save_enemies(path: &Path) -> Result<()> {
    if !path.is_file() {
        bail!("save does not exist: {}", path.display());
    }
    let store = NativeSaveStore::new(path);
    let mut snapshot = store
        .load()
        .with_context(|| format!("failed to validate save {}", path.display()))?;
    let enemy_ids = snapshot
        .simulation
        .actors
        .iter()
        .filter(|(_, actor)| actor.role.as_str() == "role:enemy")
        .map(|(id, _)| id.clone())
        .chain(
            snapshot
                .actors
                .iter()
                .filter(|actor| actor.kind == ActorKind::Enemy)
                .map(|actor| actor.id.clone()),
        )
        .collect::<BTreeSet<_>>();
    let recovery = unique_purge_backup_path(path);
    fs::copy(path, &recovery).with_context(|| {
        format!(
            "failed to create pre-purge recovery copy {}",
            recovery.display()
        )
    })?;
    let archived_backups = archive_purge_backup_history(&store, &recovery)?;

    snapshot
        .actors
        .retain(|actor| !enemy_ids.contains(&actor.id));
    snapshot
        .simulation
        .actors
        .retain(|id, _| !enemy_ids.contains(id));
    for camp in snapshot.simulation.enemy_camps.values_mut() {
        camp.spawned_enemies.retain(|id| !enemy_ids.contains(id));
    }
    if snapshot.simulation.active_raid.is_some() {
        snapshot.simulation.finish_raid();
    }
    store
        .write(&snapshot)
        .with_context(|| format!("failed to write purged save {}", path.display()))?;
    let verified = store
        .load()
        .with_context(|| format!("purged save did not reload: {}", path.display()))?;
    if verified
        .simulation
        .actors
        .values()
        .any(|actor| actor.role.as_str() == "role:enemy")
        || verified
            .actors
            .iter()
            .any(|actor| actor.kind == ActorKind::Enemy)
    {
        bail!("purged save still contains enemy actors");
    }
    seed_clean_purge_backup(&store)?;
    println!(
        "Purged {} enemies from {}; recovery copy: {}; archived backup generations: {}",
        enemy_ids.len(),
        path.display(),
        recovery.display(),
        archived_backups,
    );
    Ok(())
}

fn archive_purge_backup_history(store: &NativeSaveStore, recovery: &Path) -> Result<usize> {
    let backups = (1..=NATIVE_SAVE_BACKUP_GENERATIONS)
        .map(|generation| store.backup_path_at(generation))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    if backups.is_empty() {
        return Ok(0);
    }
    let archive = std::path::PathBuf::from(format!("{}.history", recovery.display()));
    fs::create_dir_all(&archive).with_context(|| {
        format!(
            "failed to create pre-purge backup archive {}",
            archive.display()
        )
    })?;
    for backup in &backups {
        let file_name = backup
            .file_name()
            .context("rolling backup has no file name")?;
        fs::copy(backup, archive.join(file_name)).with_context(|| {
            format!(
                "failed to archive rolling backup {} before purge",
                backup.display()
            )
        })?;
    }
    Ok(backups.len())
}

fn seed_clean_purge_backup(store: &NativeSaveStore) -> Result<()> {
    let backup = store.backup_path();
    let temporary = std::path::PathBuf::from(format!("{}.clean.tmp", backup.display()));
    fs::copy(store.path(), &temporary).with_context(|| {
        format!(
            "failed to stage clean post-purge backup {}",
            temporary.display()
        )
    })?;
    NativeSaveStore::new(&temporary)
        .load()
        .with_context(|| format!("staged clean backup is invalid: {}", temporary.display()))?;
    for generation in 1..=NATIVE_SAVE_BACKUP_GENERATIONS {
        let path = store.backup_path_at(generation);
        if path.exists() {
            fs::remove_file(&path).with_context(|| {
                format!("failed to clear old rolling backup {}", path.display())
            })?;
        }
    }
    fs::rename(&temporary, &backup).with_context(|| {
        format!(
            "failed to install clean post-purge backup {}",
            backup.display()
        )
    })?;
    NativeSaveStore::new(&backup)
        .load()
        .with_context(|| format!("clean post-purge backup is invalid: {}", backup.display()))?;
    Ok(())
}

fn unique_purge_backup_path(path: &Path) -> std::path::PathBuf {
    let direct = std::path::PathBuf::from(format!("{}.pre-enemy-purge", path.display()));
    if !direct.exists() {
        return direct;
    }
    for generation in 2_u32.. {
        let candidate =
            std::path::PathBuf::from(format!("{}.pre-enemy-purge.{generation}", path.display()));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!("the pre-purge backup suffix range is unbounded")
}

fn validate() -> Result<()> {
    let forbidden_reference = Path::new("assets/content/unity_generation_reference.ron");
    if forbidden_reference.exists() {
        bail!(
            "{} must not exist: Unity generation output is validation-only and cannot be a Bevy input",
            forbidden_reference.display()
        );
    }
    let world_source_path = Path::new("crates/stream_town_domain/src/world.rs");
    let world_source = fs::read_to_string(world_source_path)
        .with_context(|| format!("failed to read {}", world_source_path.display()))?;
    for forbidden_symbol in [
        "UnityGenerationReference",
        "converted_unity_generation_reference",
        "candidate_half_units",
        "height_half_metres",
    ] {
        if world_source.contains(forbidden_symbol) {
            bail!(
                "world generator contains forbidden Unity-output replay symbol {forbidden_symbol}"
            );
        }
    }

    let config_path = Path::new("assets/config/game.ron");
    let config: GameConfig = ron::from_str(
        &fs::read_to_string(config_path)
            .with_context(|| format!("failed to read {}", config_path.display()))?,
    )?;
    config.validate()?;
    let player_settings_path = Path::new("assets/config/player-settings.ron");
    let player_settings: PlayerSettings = ron::from_str(
        &fs::read_to_string(player_settings_path)
            .with_context(|| format!("failed to read {}", player_settings_path.display()))?,
    )?;
    player_settings.validate()?;
    if player_settings != PlayerSettings::default() {
        bail!("checked-in player settings no longer match Unity defaults");
    }
    if config.time.seconds_per_day != SHIPPING_SECONDS_PER_DAY
        || config.time.daylight_per_thousand != 666
        || config.time.transition_seconds != 100
        || config.time.day_light_intensity_milli != 10_000
        || config.time.night_light_intensity_milli != 5_000
        || config.time.max_building_emission_milli != 5_000
    {
        bail!("shipping time-cycle settings no longer match the converted Unity assets");
    }

    let content_path = Path::new("assets/content/catalog.ron");
    let content: ContentCatalog = ron::from_str(
        &fs::read_to_string(content_path)
            .with_context(|| format!("failed to read {}", content_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", content_path.display()))?;
    content.validate()?;
    let technology_layout_path = Path::new("assets/content/technology_layout.ron");
    let technology_layout: TechnologyGraphLayout = ron::from_str(
        &fs::read_to_string(technology_layout_path)
            .with_context(|| format!("failed to read {}", technology_layout_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", technology_layout_path.display()))?;
    technology_layout.validate(&content.technology)?;
    let converted_unity_nodes = technology_layout
        .nodes
        .keys()
        .filter(|id| !id.as_str().starts_with("tech:native_"))
        .count();
    let converted_unity_groups = content
        .technology
        .groups
        .values()
        .filter(|group| {
            group
                .nodes
                .iter()
                .any(|id| !id.as_str().starts_with("tech:native_"))
        })
        .count();
    if converted_unity_nodes != 363 || converted_unity_groups != 20 {
        bail!("technology graph layout no longer covers the verified Unity graph");
    }
    let presentation_path = Path::new("assets/content/presentation.ron");
    let presentation: PresentationCatalog = ron::from_str(
        &fs::read_to_string(presentation_path)
            .with_context(|| format!("failed to read {}", presentation_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", presentation_path.display()))?;
    presentation.validate()?;
    validate_visual_baseline_assets()?;
    validate_audio_baseline_assets()?;
    let model_baseline_path = Path::new("assets/content/model-conversion-baseline.json");
    let model_baseline: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(model_baseline_path)
            .with_context(|| format!("failed to read {}", model_baseline_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", model_baseline_path.display()))?;
    for (field, expected) in [
        ("/schema_version", 3_u64),
        ("/models", 253),
        ("/bytes", 96_885_184),
        ("/meshes", 820),
        ("/skins", 43),
        ("/animations", 165),
        ("/materials", 253),
        ("/images", 1),
    ] {
        if model_baseline
            .pointer(field)
            .and_then(serde_json::Value::as_u64)
            != Some(expected)
        {
            bail!("model conversion baseline field {field} changed");
        }
    }
    let technology_edges: usize = content
        .technology
        .nodes
        .values()
        .map(|node| node.prerequisites.len())
        .sum();
    let technology_roots = content
        .technology
        .nodes
        .values()
        .filter(|node| node.prerequisites.is_empty())
        .count();
    let archetype_scenes: usize = content
        .archetypes
        .values()
        .map(|archetype| archetype.scenes.len())
        .sum();
    let disable_after_time_prefabs = content
        .archetypes
        .values()
        .filter(|archetype| archetype.disable_after_milliseconds.is_some())
        .count();
    let unit_health_bar_prefabs = content
        .archetypes
        .values()
        .filter(|archetype| archetype.health_bar_hide_milliseconds.is_some())
        .count();
    let pet_archetypes = content
        .archetypes
        .values()
        .filter(|archetype| archetype.pet.is_some())
        .collect::<Vec<_>>();
    let pet = pet_archetypes
        .first()
        .and_then(|archetype| archetype.pet.as_ref());
    let pet_model_count = pet.map_or(0, |pet| pet.models.len());
    let exact = |actual: f32, expected: f32| (actual - expected).abs() < f32::EPSILON;
    let pet_contract_matches = pet.is_some_and(|pet| {
        exact(pet.closest_distance, 1.0)
            && exact(pet.max_distance, 5.0)
            && pet.min_move_speed.abs() < f32::EPSILON
            && exact(pet.max_move_speed, 10.0)
            && exact(pet.rotation_radians_per_second, 5.0)
            && pet
                .models
                .values()
                .all(|model| model.local_scale.iter().all(|value| exact(*value, 1.0)))
            && pet
                .models
                .values()
                .filter(|model| exact(model.local_position[1], 1.403))
                .count()
                == 1
    });
    let foliage_variants: usize = content
        .foliage
        .iter()
        .map(|layer| layer.variants.len())
        .sum();
    let passive_resource_generators: usize = content
        .buildings
        .values()
        .map(|building| building.passive_resources.len())
        .sum();
    let building_model_handlers: usize = content
        .buildings
        .values()
        .map(|building| building.model_handlers.len())
        .sum();
    let storage_model_handlers: usize = content
        .buildings
        .values()
        .map(|building| building.storage_models.len())
        .sum();
    let rotating_nodes: usize = content
        .archetypes
        .values()
        .map(|archetype| archetype.rotating_nodes.len())
        .sum();
    let targeting_scores = content
        .buildings
        .values()
        .filter(|building| building.targeting.is_some())
        .count();
    let target_sizes = content
        .archetypes
        .values()
        .filter(|archetype| archetype.target_size_milli_cells > 0)
        .count();
    let enemy_model_handlers = content
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy_models.is_some())
        .count();
    let enemy_base_models: usize = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy_models.as_ref())
        .map(|models| models.base_models.len())
        .sum();
    let enemy_permanent_models: usize = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy_models.as_ref())
        .map(|models| models.permanent_models.len())
        .sum();
    let enemy_optional_models: usize = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy_models.as_ref())
        .map(|models| models.optional_models.len())
        .sum();
    let enemy_weapon_models: usize = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy_models.as_ref())
        .map(|models| models.weapons.len())
        .sum();
    if content.foliage.len() != 4
        || foliage_variants != 21
        || passive_resource_generators != 1
        || building_model_handlers != 48
        || storage_model_handlers != 6
        || rotating_nodes != 3
        || disable_after_time_prefabs != 1
        || unit_health_bar_prefabs != 1
        || pet_archetypes.len() != 1
        || pet_model_count != 5
        || !pet_contract_matches
        || targeting_scores != 29
        || target_sizes != 47
        || (
            enemy_model_handlers,
            enemy_base_models,
            enemy_permanent_models,
            enemy_optional_models,
            enemy_weapon_models,
        ) != (16, 21, 9, 66, 16)
        || (
            content.schema_version,
            content.archetypes.len(),
            archetype_scenes,
            content.buildings.len(),
            content.roles.len(),
            content.objectives.len(),
            content.technology.nodes.len(),
            content.technology.groups.len(),
            technology_edges,
            technology_roots,
            content.source_records.len(),
        ) != (
            stream_town_domain::CURRENT_CONTENT_SCHEMA,
            218,
            294,
            29,
            18,
            425,
            366,
            21,
            362,
            4,
            404,
        )
        || content.loading_screen.progress_milli_per_second != 500
        || content.loading_screen.completion_hold_milliseconds != 500
        || content.loading_screen.tooltips.len() != 1
    {
        bail!("authored content counts differ from the verified catalog baseline");
    }
    let health_definitions = content
        .archetypes
        .values()
        .filter(|archetype| archetype.health.is_some())
        .count();
    let building_health = content
        .buildings
        .values()
        .filter_map(|building| content.archetypes[&building.archetype].health.as_ref())
        .collect::<Vec<_>>();
    let building_base_health: u32 = building_health.iter().map(|health| health.max_health).sum();
    let building_level_health: u32 = building_health
        .iter()
        .map(|health| health.health_gain_per_level)
        .sum();
    let projectile_shooters = content
        .buildings
        .values()
        .filter(|building| building.projectile_shooter.is_some())
        .count();
    let enemy_definitions = content
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy.is_some())
        .count();
    let enemy_resource_rewards = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy.as_ref())
        .filter(|enemy| enemy.kill_reward.amount > 0)
        .count();
    let enemy_retaliation = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy.as_ref())
        .filter(|enemy| enemy.attack_attacker)
        .count();
    let goblin_sensor_ranges = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy.as_ref())
        .filter(|enemy| enemy.target_search_range_milli_cells == 4_000)
        .count();
    let standard_sensor_ranges = content
        .archetypes
        .values()
        .filter_map(|archetype| archetype.enemy.as_ref())
        .filter(|enemy| enemy.target_search_range_milli_cells == 12_500)
        .count();
    let enemy_spawners = content
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy_spawner.is_some())
        .count();
    if building_health.len() != 29
        || building_base_health != 4_425
        || building_level_health != 1_055
        || (
            health_definitions,
            projectile_shooters,
            enemy_definitions,
            enemy_spawners,
            enemy_resource_rewards,
            enemy_retaliation,
            goblin_sensor_ranges,
            standard_sensor_ranges,
        ) != (45, 1, 9, 1, 9, 9, 1, 8)
    {
        bail!(
            "authored combat component counts differ from the verified Unity baseline: building health {} definitions, {} base total, {} per-level total; {health_definitions} total health, {projectile_shooters} projectile shooters, {enemy_definitions} enemies, {enemy_spawners} spawners, {enemy_resource_rewards} rewards",
            building_health.len(),
            building_base_health,
            building_level_health
        );
    }
    for (archetype_id, archetype) in &content.archetypes {
        for scene in &archetype.scenes {
            let path = Path::new("assets").join(&scene.asset_path);
            if !path.is_file() {
                bail!(
                    "archetype {archetype_id} references missing converted model {}",
                    path.display()
                );
            }
        }
        if let Some(models) = &archetype.enemy_models {
            let scene = archetype
                .scenes
                .iter()
                .find(|scene| scene.is_default)
                .context("enemy model handler archetype has no default scene")?;
            let packaged_nodes = glb_node_names(&Path::new("assets").join(&scene.asset_path))?;
            for expected in models
                .base_models
                .iter()
                .chain(&models.permanent_models)
                .chain(&models.optional_models)
                .chain(models.weapons.iter().flat_map(|weapon| {
                    std::iter::once(&weapon.main_model).chain(&weapon.off_hand_models)
                }))
            {
                if !packaged_nodes.contains(expected) {
                    bail!(
                        "enemy model handler {archetype_id} references node {expected:?} absent from its packaged GLB scenes"
                    );
                }
            }
        }
        for rotating in &archetype.rotating_nodes {
            let Some(age) = rotating.age else {
                continue;
            };
            let scene = archetype
                .scenes
                .iter()
                .find(|scene| scene.age == Some(age))
                .with_context(|| {
                    format!(
                        "rotating node {} has no packaged age-{} scene",
                        rotating.node, age
                    )
                })?;
            let packaged_nodes = glb_node_names(&Path::new("assets").join(&scene.asset_path))?;
            if !packaged_nodes.contains(&rotating.node) {
                bail!(
                    "rotating node {:?} is absent from its packaged GLB scene {}",
                    rotating.node,
                    scene.asset_path
                );
            }
        }
    }
    for layer in &content.foliage {
        if !presentation
            .materials
            .values()
            .any(|material| material.source_path == layer.material_source_path)
        {
            bail!(
                "foliage layer {} references missing material {}",
                layer.id,
                layer.material_source_path
            );
        }
        for variant in &layer.variants {
            let path = Path::new("assets").join(&variant.asset_path);
            if !path.is_file() {
                bail!(
                    "foliage layer {} references missing converted model {}",
                    layer.id,
                    path.display()
                );
            }
        }
    }
    let presentation_states: usize = presentation
        .controllers
        .values()
        .map(|controller| controller.states.len())
        .sum();
    let presentation_transitions: usize = presentation
        .controllers
        .values()
        .map(|controller| controller.transitions.len())
        .sum();
    let presentation_state_machines: usize = presentation
        .controllers
        .values()
        .map(|controller| controller.state_machines.len())
        .sum();
    let presentation_layers: usize = presentation
        .controllers
        .values()
        .map(|controller| controller.layers.len())
        .sum();
    let native_animation_bindings = presentation
        .prefab_bindings
        .values()
        .filter(|binding| binding.gltf_animation_index.is_some())
        .count();
    let converted_transform_clips = presentation
        .clips
        .values()
        .filter(|clip| !clip.transform_tracks.is_empty())
        .count();
    let embedded_animation_clips = presentation
        .clips
        .values()
        .filter(|clip| clip.converted_asset_path.is_some())
        .count();
    let mut glb_animation_counts = BTreeMap::new();
    for (clip_id, clip) in &presentation.clips {
        let Some(path) = clip.converted_asset_path.as_ref() else {
            if clip.gltf_animation_index.is_some() {
                bail!("animation clip {clip_id} has an index without a converted GLB path");
            }
            continue;
        };
        let Some(index) = clip.gltf_animation_index else {
            bail!("animation clip {clip_id} has a converted GLB path without an index");
        };
        let count = if let Some(count) = glb_animation_counts.get(path) {
            *count
        } else {
            let count = glb_animation_count(&Path::new("assets").join(path))?;
            glb_animation_counts.insert(path.clone(), count);
            count
        };
        if usize::try_from(index)
            .ok()
            .is_none_or(|index| index >= count)
        {
            bail!(
                "animation clip {clip_id} references animation {index} outside {path}'s {count} animations"
            );
        }
    }
    for (controller_id, controller) in &presentation.controllers {
        for motion in controller.states.values().flat_map(|state| &state.motions) {
            let clip = &presentation.clips[&motion.clip];
            if Path::new(&clip.source_path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("fbx"))
                && (clip.converted_asset_path.is_none() || clip.gltf_animation_index.is_none())
            {
                bail!(
                    "animation controller {controller_id} references unconverted embedded FBX clip {} ({})",
                    motion.clip,
                    clip.source_path
                );
            }
        }
    }
    let transform_tracks: usize = presentation
        .clips
        .values()
        .map(|clip| clip.transform_tracks.len())
        .sum();
    let blend_states = presentation
        .controllers
        .values()
        .flat_map(|controller| controller.states.values())
        .filter(|state| state.blend_parameter.is_some())
        .count();
    let inferred_parameters = presentation
        .controllers
        .values()
        .flat_map(|controller| &controller.parameters)
        .filter(|parameter| parameter.inferred)
        .count();
    let fixed_transitions = presentation
        .controllers
        .values()
        .flat_map(|controller| &controller.transitions)
        .filter(|transition| transition.fixed_duration)
        .count();
    let offset_transitions = presentation
        .controllers
        .values()
        .flat_map(|controller| &controller.transitions)
        .filter(|transition| transition.offset > f32::EPSILON)
        .count();
    let weighted_property_keys = presentation
        .clips
        .values()
        .flat_map(|clip| &clip.property_curves)
        .flat_map(|curve| &curve.keys)
        .filter(|key| key.weighted_mode != 0)
        .count();
    let material_vector_properties = presentation
        .materials
        .values()
        .map(|material| material.custom_vectors.len())
        .sum::<usize>();
    let material_texture_transforms = presentation
        .materials
        .values()
        .map(|material| material.texture_transforms.len())
        .sum::<usize>();
    if (
        presentation.schema_version,
        presentation.textures.len(),
        presentation.materials.len(),
        presentation.clips.len(),
        presentation.controllers.len(),
        presentation_states,
        presentation_transitions,
        presentation.prefab_bindings.len(),
        native_animation_bindings,
        presentation.prefab_materials.len(),
        presentation
            .prefab_materials
            .values()
            .map(Vec::len)
            .sum::<usize>(),
    ) != (21, 133, 33, 184, 31, 94, 166, 22, 18, 141, 181)
        || (converted_transform_clips, transform_tracks) != (57, 1196)
        || embedded_animation_clips != 122
        || (blend_states, inferred_parameters) != (11, 2)
        || (fixed_transitions, offset_transitions) != (166, 2)
        || weighted_property_keys != 0
        || material_vector_properties != 141
        || material_texture_transforms != 32
        || (presentation_state_machines, presentation_layers) != (45, 33)
        || presentation.avatar_masks.len() != 3
        || presentation.post_process_profiles.len() != 2
        || presentation
            .scene_post_process
            .values()
            .map(Vec::len)
            .sum::<usize>()
            != 3
        || presentation.fireworks_effects.len() != 1
        || presentation
            .scene_fireworks
            .values()
            .map(Vec::len)
            .sum::<usize>()
            != 2
        || presentation.chimney_smoke_effects.len() != 1
        || presentation
            .prefab_chimney_emitters
            .values()
            .map(Vec::len)
            .sum::<usize>()
            != 7
        || presentation.raining_fish_effects.len() != 1
        || presentation.fish_school_effects.len() != 1
        || presentation
            .scene_fish_schools
            .values()
            .map(Vec::len)
            .sum::<usize>()
            != 2
        || presentation.role_action_audio.len() != 14
        || presentation
            .role_action_audio
            .values()
            .map(|audio| audio.clip_guids.len())
            .sum::<usize>()
            != 35
        || presentation.healing_channel_effects.len() != 1
        || presentation.healing_burst_effects.len() != 1
        || presentation
            .avatar_masks
            .values()
            .map(|mask| mask.transform_weights.len())
            .sum::<usize>()
            != 477
        || presentation
            .avatar_masks
            .values()
            .flat_map(|mask| mask.transform_weights.values())
            .filter(|weight| weight.abs() < f32::EPSILON)
            .count()
            != 118
        || presentation
            .model_materials
            .values()
            .map(BTreeMap::len)
            .sum::<usize>()
            != 241
        || presentation
            .prefab_renderer_materials
            .values()
            .map(Vec::len)
            .sum::<usize>()
            != 903
        || presentation
            .prefab_renderer_materials
            .values()
            .flat_map(|renderers| renderers.iter())
            .map(|renderer| renderer.materials.len())
            .sum::<usize>()
            != 912
        || presentation
            .clips
            .values()
            .filter(|clip| !clip.property_curves.is_empty())
            .count()
            != 18
        || presentation
            .clips
            .values()
            .map(|clip| clip.property_curves.len())
            .sum::<usize>()
            != 110
        || presentation
            .clips
            .values()
            .flat_map(|clip| &clip.property_curves)
            .map(|curve| curve.keys.len())
            .sum::<usize>()
            != 261
        || presentation
            .clips
            .values()
            .filter(|clip| !clip.events.is_empty())
            .count()
            != 11
        || presentation
            .clips
            .values()
            .map(|clip| clip.events.len())
            .sum::<usize>()
            != 12
    {
        bail!("presentation counts differ from the verified Unity baseline");
    }
    let missing_clip_sources: Vec<_> = presentation
        .clips
        .values()
        .filter(|clip| clip.source_path.is_empty())
        .map(|clip| clip.source_guid.as_str())
        .collect();
    if missing_clip_sources != ["3efab8b2dfb3f994f82d137fd8cf2c18"] {
        bail!("Unity missing-clip baseline changed: {missing_clip_sources:?}");
    }
    for (texture_id, texture) in &presentation.textures {
        let path = Path::new("assets").join(&texture.asset_path);
        if !path.is_file() {
            bail!(
                "texture {texture_id} references missing packaged asset {}",
                path.display()
            );
        }
    }
    for (prefab_guid, binding) in &presentation.prefab_bindings {
        if let Some(scene) = &binding.rig_scene {
            let path = Path::new("assets").join(scene);
            if !path.is_file() {
                bail!(
                    "prefab {prefab_guid} references missing rig scene {}",
                    path.display()
                );
            }
        }
        if let Some(scene) = &binding.animated_scene {
            let path = Path::new("assets").join(scene);
            if !path.is_file() {
                bail!(
                    "prefab {prefab_guid} references missing animated scene {}",
                    path.display()
                );
            }
        }
    }

    let mut checked_json = 0_usize;
    for entry in WalkDir::new("generated").into_iter().filter_map(Result::ok) {
        if entry
            .path()
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            let contents = fs::read_to_string(entry.path())?;
            serde_json::from_str::<serde_json::Value>(&contents)
                .with_context(|| format!("invalid JSON in {}", entry.path().display()))?;
            checked_json += 1;
        }
    }
    if WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .any(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "meta")
        })
    {
        bail!("Unity .meta files must not be created inside bevy-port");
    }
    println!(
        "Configuration, 218 prefab archetypes with 47 target sizes, 1 disable-after-time lifetime, 1 unit health-bar contract, and 1 exact pet follower with 5 authored models, 16 enemy model handlers (21 base / 9 permanent / 66 optional / 16 weapons), 4 foliage layers with 21 variants, 48 building model handlers, 6 storage model handlers, 3 authored rotating nodes, 1 passive resource generator, 29 target scoring definitions, 29 building health definitions, 45 total health definitions, 9 enemy definitions with 9 kill rewards, 1 enemy camp, 1 projectile shooter, 425 objectives, 404 source records, 133 textures, 33 materials, 31 animation controllers, 122 embedded FBX clips, 2 active fish-school bindings, 14 role-audio contracts with 35 variants, and all 253 converted models are valid; checked {checked_json} generated JSON files"
    );
    Ok(())
}

fn visual_acceptance_config() -> Result<VisualAcceptanceConfig> {
    let path = Path::new("assets/acceptance/visual-scenarios.json");
    let config: VisualAcceptanceConfig = serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", path.display()))?;
    if config.schema_version != 1
        || config.reference_width < 64
        || config.reference_height < 36
        || config.scenarios.is_empty()
    {
        bail!("visual acceptance configuration is invalid");
    }
    let mut names = BTreeSet::new();
    for scenario in &config.scenarios {
        if !names.insert(scenario.name.clone())
            || scenario.name.is_empty()
            || !scenario
                .name
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
            || !(0.0..=0.25).contains(&scenario.max_mean_absolute_error)
        {
            bail!("visual acceptance scenario {:?} is invalid", scenario.name);
        }
    }
    Ok(config)
}

fn visual_baseline_manifest() -> Result<VisualBaselineManifest> {
    let path = Path::new("assets/acceptance/visual-baseline.json");
    serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", path.display()))
}

fn sha256_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn reference_image(image: DynamicImage, width: u32, height: u32) -> RgbImage {
    image::imageops::resize(&image.to_rgb8(), width, height, FilterType::Lanczos3)
}

#[allow(clippy::cast_precision_loss)]
fn image_mean_absolute_error(left: &RgbImage, right: &RgbImage) -> Result<f64> {
    if left.dimensions() != right.dimensions() {
        bail!("visual acceptance images have different dimensions");
    }
    let total_difference = left
        .pixels()
        .zip(right.pixels())
        .flat_map(|(left, right)| {
            left.0
                .into_iter()
                .zip(right.0)
                .map(|(left, right)| u64::from(left.abs_diff(right)))
        })
        .sum::<u64>();
    let channels = u64::from(left.width()) * u64::from(left.height()) * 3;
    Ok(total_difference as f64 / channels as f64 / 255.0)
}

fn validate_visual_baseline_assets() -> Result<()> {
    let config = visual_acceptance_config()?;
    let manifest = visual_baseline_manifest()?;
    if manifest.schema_version != config.schema_version
        || manifest.references.len() != config.scenarios.len()
    {
        bail!("visual baseline manifest does not cover the configured scenarios");
    }
    for scenario in &config.scenarios {
        let path = Path::new("assets/acceptance/visual").join(format!("{}.png", scenario.name));
        let reference = manifest
            .references
            .get(&scenario.name)
            .with_context(|| format!("visual baseline is missing {}", scenario.name))?;
        let image =
            image::open(&path).with_context(|| format!("failed to decode {}", path.display()))?;
        if image.width() != config.reference_width
            || image.height() != config.reference_height
            || reference.width != image.width()
            || reference.height != image.height()
            || reference.sha256 != sha256_file(&path)?
        {
            bail!(
                "visual baseline {} failed its integrity check",
                scenario.name
            );
        }
    }
    Ok(())
}

fn validate_audio_baseline_assets() -> Result<()> {
    let path = Path::new("assets/acceptance/audio-baseline.json");
    let manifest: AudioBaselineManifest = serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", path.display()))?;
    if manifest.schema_version != 1
        || manifest.sample_rate != 16_000
        || manifest.clips.len() != 39
        || !manifest.clips.contains_key("ambience")
        || !(0..3).all(|variant| manifest.clips.contains_key(&format!("seagull-{variant}")))
    {
        bail!("audio acceptance manifest does not cover the shipping replacement clips");
    }
    for (name, hash) in &manifest.clips {
        if name.is_empty()
            || hash.len() != 64
            || !hash
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            bail!("audio acceptance fingerprint for {name:?} is invalid");
        }
    }
    Ok(())
}

fn visual_acceptance(
    capture_dir: &Path,
    selected_scenarios: &[String],
    update_baseline: bool,
) -> Result<()> {
    let config = visual_acceptance_config()?;
    if update_baseline && !selected_scenarios.is_empty() {
        bail!("updating the curated baseline requires the complete scenario matrix");
    }
    let scenarios = config
        .scenarios
        .iter()
        .filter(|scenario| {
            selected_scenarios.is_empty() || selected_scenarios.contains(&scenario.name)
        })
        .collect::<Vec<_>>();
    if !selected_scenarios.is_empty() && scenarios.len() != selected_scenarios.len() {
        bail!("one or more requested visual-acceptance scenarios are unknown");
    }
    let baseline_dir = Path::new("assets/acceptance/visual");
    if update_baseline {
        fs::create_dir_all(baseline_dir)?;
    } else {
        validate_visual_baseline_assets()?;
    }
    let mut references = serde_json::Map::new();
    for scenario in scenarios {
        let capture_path = capture_dir.join(format!("{}.png", scenario.name));
        let capture = image::open(&capture_path)
            .with_context(|| format!("failed to decode capture {}", capture_path.display()))?;
        if capture.width() < config.reference_width
            || capture.height() < config.reference_height
            || u64::from(capture.width()) * u64::from(config.reference_height)
                != u64::from(capture.height()) * u64::from(config.reference_width)
        {
            bail!(
                "capture {} must preserve the {}x{} reference aspect ratio",
                capture_path.display(),
                config.reference_width,
                config.reference_height
            );
        }
        let sampled = reference_image(capture, config.reference_width, config.reference_height);
        let baseline_path = baseline_dir.join(format!("{}.png", scenario.name));
        if update_baseline {
            sampled
                .save(&baseline_path)
                .with_context(|| format!("failed to save {}", baseline_path.display()))?;
        }
        let baseline = image::open(&baseline_path)
            .with_context(|| format!("failed to decode {}", baseline_path.display()))?
            .to_rgb8();
        let error = image_mean_absolute_error(&sampled, &baseline)?;
        if error > scenario.max_mean_absolute_error {
            bail!(
                "visual acceptance {} differs by {:.4}, above its {:.4} limit",
                scenario.name,
                error,
                scenario.max_mean_absolute_error
            );
        }
        println!(
            "Visual acceptance {:<20} mean error {:.4} / {:.4}",
            scenario.name, error, scenario.max_mean_absolute_error
        );
        references.insert(
            scenario.name.clone(),
            serde_json::json!({
                "sha256": sha256_file(&baseline_path)?,
                "width": baseline.width(),
                "height": baseline.height(),
            }),
        );
    }
    if update_baseline {
        let manifest = serde_json::json!({
            "schema_version": config.schema_version,
            "references": references,
        });
        fs::write(
            "assets/acceptance/visual-baseline.json",
            format!("{}\n", serde_json::to_string_pretty(&manifest)?),
        )?;
        validate_visual_baseline_assets()?;
        println!(
            "Updated {} curated visual references",
            config.scenarios.len()
        );
    }
    Ok(())
}

fn glb_node_names(path: &Path) -> Result<BTreeSet<String>> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    glb_node_names_from_bytes(&bytes)
        .with_context(|| format!("failed to inspect GLB nodes in {}", path.display()))
}

fn glb_animation_count(path: &Path) -> Result<usize> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let document = glb_document_from_bytes(&bytes)
        .with_context(|| format!("failed to inspect GLB animations in {}", path.display()))?;
    Ok(document
        .get("animations")
        .and_then(serde_json::Value::as_array)
        .map_or(0, Vec::len))
}

fn glb_node_names_from_bytes(bytes: &[u8]) -> Result<BTreeSet<String>> {
    let document = glb_document_from_bytes(bytes)?;
    let nodes = document
        .get("nodes")
        .and_then(serde_json::Value::as_array)
        .context("GLB JSON has no nodes array")?;
    Ok(nodes
        .iter()
        .filter_map(|node| node.get("name").and_then(serde_json::Value::as_str))
        .map(str::to_owned)
        .collect())
}

fn glb_document_from_bytes(bytes: &[u8]) -> Result<serde_json::Value> {
    if bytes.len() < 20 || &bytes[0..4] != b"glTF" {
        bail!("missing glTF 2.0 binary header");
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().expect("four-byte GLB version"));
    let declared_length = u32::from_le_bytes(
        bytes[8..12]
            .try_into()
            .expect("four-byte GLB declared length"),
    );
    let json_length =
        u32::from_le_bytes(bytes[12..16].try_into().expect("four-byte GLB JSON length"));
    let json_type = u32::from_le_bytes(bytes[16..20].try_into().expect("four-byte GLB JSON type"));
    if version != 2
        || usize::try_from(declared_length).ok() != Some(bytes.len())
        || json_type != 0x4E4F_534A
    {
        bail!("invalid glTF 2.0 binary header");
    }
    let json_end = 20_usize
        .checked_add(usize::try_from(json_length).context("GLB JSON chunk is too large")?)
        .filter(|end| *end <= bytes.len())
        .context("GLB JSON chunk exceeds file bounds")?;
    serde_json::from_slice(&bytes[20..json_end]).context("GLB JSON chunk is invalid")
}

fn stress(agents: u32, ticks: u32) -> Result<()> {
    if agents == 0 || ticks == 0 {
        bail!("stress agents and ticks must both be non-zero");
    }
    let config = GameConfig::default();
    let content: ContentCatalog =
        ron::from_str(&fs::read_to_string("assets/content/catalog.ron")?)?;
    let world = generate_world_with_content(&config.world, &content);
    let walkable: Vec<_> = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .filter(|position| world.navigation.is_walkable(*position))
        .collect();
    if walkable.len() < 2 {
        bail!("generated world has too few walkable cells");
    }
    let anchor = walkable[walkable.len() / 2];
    let reachable: Vec<_> = walkable
        .iter()
        .copied()
        .filter(|position| world.navigation.find_path(anchor, *position).is_ok())
        .collect();
    if reachable.len() < 2 {
        bail!("generated world has too few mutually reachable cells");
    }
    let started = Instant::now();
    let mut navigation = world.navigation.clone();
    let mut planned_steps = 0_usize;
    let mut crowd = Vec::with_capacity(usize::try_from(agents).unwrap_or_default());
    for index in 0..agents {
        let index = usize::try_from(index).unwrap_or_default();
        let start = reachable[index % reachable.len()];
        let goal = reachable[(reachable.len() - 1 - index) % reachable.len()];
        let path = navigation.find_path(start, goal)?;
        planned_steps += path.len();
        crowd.push(StressAgent {
            position: start,
            goal,
            path,
            cursor: 0,
        });
    }
    let mutation_cells = reachable
        .iter()
        .copied()
        .filter(|position| *position != anchor)
        .collect::<Vec<_>>();
    let mut active_mutation = None;
    let mut dirty_regions = 0_usize;
    let mut completed_routes = 0_u64;
    let mut replans = 0_u64;
    for tick in 0..ticks {
        if tick.is_multiple_of(120) && !mutation_cells.is_empty() {
            if let Some(previous) = active_mutation.take() {
                navigation.set_blocked(
                    DirtyRegion {
                        min: previous,
                        max: previous,
                    },
                    false,
                )?;
            }
            let occupied = crowd
                .iter()
                .map(|agent| agent.position)
                .collect::<BTreeSet<_>>();
            let start = usize::try_from(tick / 120).unwrap_or_default() % mutation_cells.len();
            if let Some(candidate) = mutation_cells
                .iter()
                .cycle()
                .skip(start)
                .take(mutation_cells.len())
                .copied()
                .find(|candidate| !occupied.contains(candidate))
            {
                navigation.set_blocked(
                    DirtyRegion {
                        min: candidate,
                        max: candidate,
                    },
                    true,
                )?;
                active_mutation = Some(candidate);
            }
            dirty_regions += navigation.take_dirty_regions().len();
        }

        for (index, agent) in crowd.iter_mut().enumerate() {
            let path_invalid = agent
                .path
                .get(agent.cursor.saturating_add(1))
                .is_some_and(|next| !navigation.is_walkable(*next));
            if path_invalid || agent.cursor.saturating_add(1) >= agent.path.len() {
                if agent.position == agent.goal {
                    completed_routes = completed_routes.saturating_add(1);
                    let tick = usize::try_from(tick).unwrap_or_default();
                    agent.goal =
                        reachable[(index.wrapping_mul(31).wrapping_add(tick)) % reachable.len()];
                }
                agent.path = if let Ok(path) = navigation.find_path(agent.position, agent.goal) {
                    path
                } else {
                    agent.goal = anchor;
                    navigation.find_path(agent.position, anchor)?
                };
                agent.cursor = 0;
                planned_steps += agent.path.len();
                replans = replans.saturating_add(1);
            }
            if let Some(next) = agent.path.get(agent.cursor.saturating_add(1)).copied()
                && navigation.is_walkable(next)
            {
                agent.cursor += 1;
                agent.position = next;
            }
        }
    }
    if let Some(previous) = active_mutation {
        navigation.set_blocked(
            DirtyRegion {
                min: previous,
                max: previous,
            },
            false,
        )?;
        dirty_regions += navigation.take_dirty_regions().len();
    }
    let final_hash = crowd.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, agent| {
        [agent.position.x, agent.position.z]
            .into_iter()
            .fold(hash, |hash, value| {
                (hash ^ u64::from(value)).wrapping_mul(0x0000_0100_0000_01b3)
            })
    });
    let elapsed = started.elapsed();
    println!(
        "Soaked {agents} agents for {ticks} ticks in {:.2?}: {completed_routes} routes, {replans} replans, {planned_steps} planned steps, {dirty_regions} dirty regions, final {final_hash:016x}; {} foliage instances; world {}",
        elapsed,
        world.foliage.len(),
        &world.deterministic_hash[..16]
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glb_with_nodes(names: &[&str]) -> Vec<u8> {
        let mut json = serde_json::to_vec(&serde_json::json!({
            "asset": {"version": "2.0"},
            "nodes": names.iter().map(|name| serde_json::json!({"name": name})).collect::<Vec<_>>()
        }))
        .unwrap();
        while !json.len().is_multiple_of(4) {
            json.push(b' ');
        }
        let length = 20 + json.len();
        let mut bytes = Vec::with_capacity(length);
        bytes.extend_from_slice(b"glTF");
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        bytes.extend_from_slice(&u32::try_from(length).unwrap().to_le_bytes());
        bytes.extend_from_slice(&u32::try_from(json.len()).unwrap().to_le_bytes());
        bytes.extend_from_slice(&0x4E4F_534A_u32.to_le_bytes());
        bytes.extend_from_slice(&json);
        bytes
    }

    #[test]
    fn glb_node_validation_preserves_exact_names() {
        let names =
            glb_node_names_from_bytes(&glb_with_nodes(&["Enemy_SkeleSword", "Enemy_SkeleShield"]))
                .unwrap();
        assert_eq!(
            names,
            BTreeSet::from([
                "Enemy_SkeleShield".to_owned(),
                "Enemy_SkeleSword".to_owned(),
            ])
        );
        let mut corrupt = glb_with_nodes(&["Enemy_SkeleSword"]);
        corrupt[4] = 1;
        assert!(glb_node_names_from_bytes(&corrupt).is_err());
    }
}
