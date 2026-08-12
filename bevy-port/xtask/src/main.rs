use std::{collections::BTreeMap, fs, path::Path, time::Instant};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use stream_town_domain::{
    ContentCatalog, GameConfig, GridPos, PresentationCatalog, generate_world_with_content,
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
    },
    /// Build and validate a portable Windows release archive.
    PackageWindows {
        #[arg(long, default_value = "dist")]
        output: std::path::PathBuf,
        #[arg(long)]
        skip_build: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Validate => validate(),
        Command::Stress { agents } => stress(agents),
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
    }
}

fn validate() -> Result<()> {
    let config_path = Path::new("assets/config/game.ron");
    let config: GameConfig = ron::from_str(
        &fs::read_to_string(config_path)
            .with_context(|| format!("failed to read {}", config_path.display()))?,
    )?;
    config.validate()?;

    let content_path = Path::new("assets/content/catalog.ron");
    let content: ContentCatalog = ron::from_str(
        &fs::read_to_string(content_path)
            .with_context(|| format!("failed to read {}", content_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", content_path.display()))?;
    content.validate()?;
    let presentation_path = Path::new("assets/content/presentation.ron");
    let presentation: PresentationCatalog = ron::from_str(
        &fs::read_to_string(presentation_path)
            .with_context(|| format!("failed to read {}", presentation_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", presentation_path.display()))?;
    presentation.validate()?;
    let model_baseline_path = Path::new("assets/content/model-conversion-baseline.json");
    let model_baseline: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(model_baseline_path)
            .with_context(|| format!("failed to read {}", model_baseline_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", model_baseline_path.display()))?;
    for (field, expected) in [
        ("/schema_version", 2_u64),
        ("/models", 253),
        ("/bytes", 95_464_596),
        ("/meshes", 820),
        ("/skins", 43),
        ("/animations", 33),
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
    if content.foliage.len() != 4
        || foliage_variants != 21
        || passive_resource_generators != 1
        || building_model_handlers != 42
        || storage_model_handlers != 6
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
        ) != (20, 215, 288, 26, 15, 422, 363, 20, 362, 1, 404)
    {
        bail!("authored content counts differ from the verified Unity baseline");
    }
    let health_definitions = content
        .archetypes
        .values()
        .filter(|archetype| archetype.health.is_some())
        .count();
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
    let enemy_spawners = content
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy_spawner.is_some())
        .count();
    if (
        health_definitions,
        projectile_shooters,
        enemy_definitions,
        enemy_spawners,
        enemy_resource_rewards,
    ) != (42, 1, 9, 1, 9)
    {
        bail!("authored combat component counts differ from the verified Unity baseline");
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
    ) != (11, 133, 33, 75, 31, 94, 166, 22, 18, 141, 181)
        || (converted_transform_clips, transform_tracks) != (57, 1196)
        || (blend_states, inferred_parameters) != (11, 2)
        || (fixed_transitions, offset_transitions) != (166, 2)
        || weighted_property_keys != 0
        || material_vector_properties != 141
        || material_texture_transforms != 32
        || (presentation_state_machines, presentation_layers) != (45, 33)
        || presentation.avatar_masks.len() != 3
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
            != 10
        || presentation
            .clips
            .values()
            .map(|clip| clip.events.len())
            .sum::<usize>()
            != 10
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
        "Configuration, 215 prefab archetypes, 4 foliage layers with 21 variants, 42 building model handlers, 6 storage model handlers, 1 passive resource generator, 42 health definitions, 9 enemy definitions with 9 kill rewards, 1 enemy camp, 1 projectile shooter, 422 objectives, 404 source records, 133 textures, 33 materials, 31 animation controllers, and all 253 converted models are valid; checked {checked_json} generated JSON files"
    );
    Ok(())
}

fn stress(agents: u32) -> Result<()> {
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
    let mut planned_steps = 0_usize;
    for index in 0..agents {
        let index = usize::try_from(index).unwrap_or_default();
        let start = reachable[index % reachable.len()];
        let goal = reachable[(reachable.len() - 1 - index) % reachable.len()];
        planned_steps += world.navigation.find_path(start, goal)?.len();
    }
    let elapsed = started.elapsed();
    println!(
        "Planned {agents} routes ({planned_steps} steps) in {:.2?}; {} foliage instances; world {}",
        elapsed,
        world.foliage.len(),
        &world.deterministic_hash[..16]
    );
    Ok(())
}
