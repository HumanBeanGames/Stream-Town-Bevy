use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    time::Instant,
};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use stream_town_domain::{
    ContentCatalog, GameConfig, GridPos, PlayerSettings, PresentationCatalog,
    SHIPPING_SECONDS_PER_DAY, generate_world_with_content,
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
        ("/schema_version", 3_u64),
        ("/models", 253),
        ("/bytes", 96_889_124),
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
        .buildings
        .values()
        .map(|building| building.rotating_nodes.len())
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
        || building_model_handlers != 42
        || storage_model_handlers != 6
        || rotating_nodes != 2
        || disable_after_time_prefabs != 1
        || targeting_scores != 26
        || target_sizes != 44
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
        ) != (27, 215, 288, 26, 15, 422, 363, 20, 362, 1, 404)
    {
        bail!("authored content counts differ from the verified Unity baseline");
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
    let enemy_spawners = content
        .archetypes
        .values()
        .filter(|archetype| archetype.enemy_spawner.is_some())
        .count();
    if building_health.len() != 26
        || building_base_health != 4_275
        || building_level_health != 905
        || (
            health_definitions,
            projectile_shooters,
            enemy_definitions,
            enemy_spawners,
            enemy_resource_rewards,
        ) != (42, 1, 9, 1, 9)
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
        for rotating in content
            .buildings
            .values()
            .filter(|building| building.archetype == *archetype_id)
            .flat_map(|building| &building.rotating_nodes)
        {
            let scene = archetype
                .scenes
                .iter()
                .find(|scene| scene.age == Some(rotating.age))
                .with_context(|| {
                    format!(
                        "rotating node {} has no packaged age-{} scene",
                        rotating.node, rotating.age
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
    ) != (13, 133, 33, 184, 31, 94, 166, 22, 18, 141, 181)
        || (converted_transform_clips, transform_tracks) != (57, 1196)
        || embedded_animation_clips != 122
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
        "Configuration, 215 prefab archetypes with 44 target sizes and 1 disable-after-time lifetime, 16 enemy model handlers (21 base / 9 permanent / 66 optional / 16 weapons), 4 foliage layers with 21 variants, 42 building model handlers, 6 storage model handlers, 2 authored rotating nodes, 1 passive resource generator, 26 target scoring definitions, 26 building health definitions, 42 total health definitions, 9 enemy definitions with 9 kill rewards, 1 enemy camp, 1 projectile shooter, 422 objectives, 404 source records, 133 textures, 33 materials, 31 animation controllers, 122 embedded FBX clips, and all 253 converted models are valid; checked {checked_json} generated JSON files"
    );
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
