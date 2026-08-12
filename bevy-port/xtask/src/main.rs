use std::{fs, path::Path, time::Instant};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use stream_town_domain::{
    ContentCatalog, GameConfig, GridPos, PresentationCatalog, generate_world,
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
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Validate => validate(),
        Command::Stress { agents } => stress(agents),
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
    if (
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
    ) != (14, 215, 288, 26, 15, 422, 363, 20, 362, 1, 404)
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
    ) != (42, 1, 9, 1)
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
    ) != (3, 133, 33, 75, 31, 94, 165, 22, 18, 141, 181)
        || (converted_transform_clips, transform_tracks) != (57, 1196)
        || (blend_states, inferred_parameters) != (11, 2)
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
        "Configuration, 215 prefab archetypes, 42 health definitions, 9 enemy definitions, 1 enemy camp, 1 projectile shooter, 422 objectives, 404 source records, 133 textures, 33 materials, 31 animation controllers, and all 253 converted models are valid; checked {checked_json} generated JSON files"
    );
    Ok(())
}

fn stress(agents: u32) -> Result<()> {
    let config = GameConfig::default();
    let world = generate_world(&config.world);
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
        "Planned {agents} routes ({planned_steps} steps) in {:.2?}; world {}",
        elapsed,
        &world.deterministic_hash[..16]
    );
    Ok(())
}
