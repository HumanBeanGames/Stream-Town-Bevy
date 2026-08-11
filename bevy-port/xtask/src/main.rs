use std::{fs, path::Path, time::Instant};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use stream_town_domain::{ContentCatalog, GameConfig, GridPos, generate_world};
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
    if (
        content.buildings.len(),
        content.roles.len(),
        content.technology.nodes.len(),
        content.technology.groups.len(),
        technology_edges,
        technology_roots,
    ) != (27, 15, 363, 20, 362, 1)
    {
        bail!("authored content counts differ from the verified Unity baseline");
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
        "Configuration and 405 semantic content records valid; checked {checked_json} generated JSON files"
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
