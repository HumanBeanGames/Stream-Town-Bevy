use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use stream_town_domain::inspect_legacy_save;
use walkdir::WalkDir;

mod content;
mod legacy;
mod menu_scene;
mod models;
mod presentation;
mod technology_layout;

#[derive(Debug, Parser)]
#[command(about = "Migration and deterministic content-baking tools for Stream Town")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Inventory Unity metadata and YAML references without changing the source tree.
    Inventory {
        unity_root: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Validate an existing migration manifest.
    ValidateManifest { manifest: PathBuf },
    /// Validate the neutral JSON emitted by the Unity editor exporter.
    ValidateUnityExport { export: PathBuf },
    /// Convert active Unity buildings, roles, and `TechTreeV2` into versioned RON.
    ConvertContent {
        export: PathBuf,
        #[arg(long, default_value = "..")]
        unity_root: PathBuf,
        #[arg(long)]
        out_dir: PathBuf,
    },
    /// Convert Unity's authored technology graph positions into a stable-ID RON sidecar.
    ConvertTechnologyLayout {
        graph: PathBuf,
        catalog: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Validate hashes and glTF metadata emitted by the Blender model converter.
    ValidateModels {
        report: PathBuf,
        #[arg(long, default_value = "..")]
        repository_root: PathBuf,
        #[arg(long)]
        expected_count: Option<usize>,
    },
    /// Inspect a legacy JSON or STSV binary save without modifying it.
    InspectSave { save: PathBuf },
    /// Export generated-world positions from a legacy save as an offline parity oracle.
    ExportWorldOracle {
        save: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Convert the authored Unity main-menu scene reference into portable RON.
    ConvertMainMenuReference {
        reference: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Apply the deterministic once-off terrain, foundation, resource, and foliage menu bake.
    BakeMainMenuScene {
        scene: PathBuf,
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        content: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Reflatten building foundations in an existing baked main-menu reference.
    RepairMainMenuFoundations {
        scene: PathBuf,
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        content: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Convert a legacy JSON or schema 1-3 binary save into a validated native save.
    ImportSave {
        save: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        config: Option<PathBuf>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MigrationManifest {
    schema_version: u32,
    source_root: String,
    entries: Vec<ManifestEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ManifestEntry {
    source: String,
    source_guid: Option<String>,
    kind: AssetKind,
    bytes: u64,
    destination_id: String,
    referenced_guids: Vec<String>,
    status: MigrationStatus,
    warnings: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum MigrationStatus {
    Discovered,
    Converted,
    ManualReview,
    Packaged,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum AssetKind {
    Scene,
    Prefab,
    ScriptableAsset,
    Model,
    Animation,
    AnimatorController,
    Material,
    Shader,
    Texture,
    Vfx,
    Other,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Inventory { unity_root, out } => {
            let manifest = inventory(&unity_root)?;
            write_manifest(&out, &manifest)?;
            println!(
                "Inventoried {} assets into {}",
                manifest.entries.len(),
                out.display()
            );
        }
        Command::ValidateManifest { manifest } => {
            let encoded = fs::read_to_string(&manifest)
                .with_context(|| format!("failed to read {}", manifest.display()))?;
            let manifest: MigrationManifest = serde_json::from_str(&encoded)
                .with_context(|| format!("failed to parse {}", manifest.display()))?;
            let warnings = validate_manifest(&manifest)?;
            println!("Manifest valid: {} entries", manifest.entries.len());
            for warning in warnings {
                println!("warning: {warning}");
            }
        }
        Command::ValidateUnityExport { export } => {
            let summary = validate_unity_export(&export)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Command::ConvertContent {
            export,
            unity_root,
            out_dir,
        } => {
            let report = content::convert(&export, &out_dir)?;
            let presentation = presentation::convert(&export, &unity_root, &out_dir)?;
            let graph_path =
                unity_root.join("Assets/Scripts/TechTree/Editor/Graphs/TechTreeV2Graph.asset");
            let technology_layout = graph_path
                .exists()
                .then(|| {
                    technology_layout::convert(
                        &graph_path,
                        &out_dir.join("catalog.ron"),
                        &out_dir.join("technology_layout.ron"),
                    )
                })
                .transpose()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "content": report,
                    "presentation": presentation,
                    "technology_layout": technology_layout,
                }))?
            );
        }
        Command::ConvertTechnologyLayout {
            graph,
            catalog,
            out,
        } => {
            let report = technology_layout::convert(&graph, &catalog, &out)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::ValidateModels {
            report,
            repository_root,
            expected_count,
        } => {
            let summary = models::validate(&report, &repository_root, expected_count)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Command::InspectSave { save } => {
            let info = inspect_legacy_save(&save)
                .with_context(|| format!("failed to inspect {}", save.display()))?;
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        Command::ExportWorldOracle { save, out } => {
            let report = legacy::export_world_oracle(&save, &out)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::ConvertMainMenuReference { reference, out } => {
            let report = menu_scene::convert(&reference, &out)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::BakeMainMenuScene {
            scene,
            config,
            content,
            out,
        } => {
            let report = menu_scene::bake(&scene, &config, &content, &out)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::RepairMainMenuFoundations {
            scene,
            config,
            content,
            out,
        } => {
            let report = menu_scene::repair_foundations(&scene, &config, &content, &out)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::ImportSave { save, out, config } => {
            let config = if let Some(path) = config {
                let encoded = fs::read_to_string(&path)
                    .with_context(|| format!("failed to read {}", path.display()))?;
                ron::from_str(&encoded)
                    .with_context(|| format!("failed to parse {}", path.display()))?
            } else {
                stream_town_domain::GameConfig::default()
            };
            config.validate()?;
            let report = legacy::import_save(&save, &out, &config)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct UnityExportSummary {
    schema_version: u64,
    unity_version: String,
    assets: usize,
    warnings: usize,
    kinds: BTreeMap<String, usize>,
    statuses: BTreeMap<String, usize>,
    shipping_scenes: Vec<String>,
}

fn validate_unity_export(path: &Path) -> Result<UnityExportSummary> {
    let file = fs::File::open(path)
        .with_context(|| format!("failed to open Unity export {}", path.display()))?;
    let root: serde_json::Value = serde_json::from_reader(BufReader::new(file))
        .with_context(|| format!("failed to parse Unity export {}", path.display()))?;
    let schema_version = root
        .get("SchemaVersion")
        .and_then(serde_json::Value::as_u64)
        .context("Unity export is missing SchemaVersion")?;
    if schema_version != 1 {
        bail!("unsupported Unity export schema {schema_version}");
    }
    let unity_version = root
        .get("UnityVersion")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .context("Unity export is missing UnityVersion")?
        .to_owned();
    let assets = root
        .get("Assets")
        .and_then(serde_json::Value::as_array)
        .context("Unity export is missing Assets")?;
    let warnings = root
        .get("Warnings")
        .and_then(serde_json::Value::as_array)
        .context("Unity export is missing Warnings")?;

    let expected_scenes = BTreeSet::from([
        "Assets/Scenes/LOADER_INITIAL.unity",
        "Assets/Scenes/Menu/Credits.unity",
        "Assets/Scenes/Menu/Main_Menu_02.unity",
        "Assets/Scenes/Worlds/World_Town.unity",
    ]);
    let mut guids = BTreeMap::<&str, &str>::new();
    let mut paths = BTreeSet::<&str>::new();
    let mut kinds = BTreeMap::<String, usize>::new();
    let mut statuses = BTreeMap::<String, usize>::new();
    let mut shipping_scenes = BTreeSet::new();
    for asset in assets {
        let guid = asset
            .get("Guid")
            .and_then(serde_json::Value::as_str)
            .context("Unity export asset is missing Guid")?;
        let asset_path = asset
            .get("Path")
            .and_then(serde_json::Value::as_str)
            .context("Unity export asset is missing Path")?;
        let kind = asset
            .get("Kind")
            .and_then(serde_json::Value::as_str)
            .context("Unity export asset is missing Kind")?;
        let status = asset
            .get("Status")
            .and_then(serde_json::Value::as_str)
            .context("Unity export asset is missing Status")?;
        if guid.len() != 32 || !guid.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            bail!("{asset_path} has invalid Unity GUID {guid}");
        }
        if let Some(previous) = guids.insert(guid, asset_path) {
            bail!("duplicate Unity GUID {guid} in {previous} and {asset_path}");
        }
        if !paths.insert(asset_path) {
            bail!("duplicate Unity asset path {asset_path}");
        }
        if !asset_path.starts_with("Assets/") || asset_path.contains('\\') {
            bail!("Unity asset path is not normalized: {asset_path}");
        }
        if !matches!(
            status,
            "exported" | "missing_main_object" | "reference_only"
        ) {
            bail!("{asset_path} has unsuccessful or unknown status {status}");
        }
        *kinds.entry(kind.to_owned()).or_default() += 1;
        *statuses.entry(status.to_owned()).or_default() += 1;
        if asset.get("Scene").is_some_and(|scene| !scene.is_null()) {
            shipping_scenes.insert(asset_path);
        }
    }
    if shipping_scenes != expected_scenes {
        bail!("shipping scene set differs from the migration contract: {shipping_scenes:?}");
    }

    Ok(UnityExportSummary {
        schema_version,
        unity_version,
        assets: assets.len(),
        warnings: warnings.len(),
        kinds,
        statuses,
        shipping_scenes: shipping_scenes.into_iter().map(str::to_owned).collect(),
    })
}

fn inventory(unity_root: &Path) -> Result<MigrationManifest> {
    let root = unity_root
        .canonicalize()
        .with_context(|| format!("Unity root {} does not exist", unity_root.display()))?;
    let assets = root.join("Assets");
    if !assets.is_dir() || !root.join("ProjectSettings").is_dir() {
        bail!("{} is not a Unity project root", root.display());
    }

    let mut entries = Vec::new();
    for entry in WalkDir::new(&assets)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| !is_excluded(entry.path()))
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file()
            || path
                .extension()
                .is_some_and(|extension| extension == "meta")
        {
            continue;
        }
        let Some(kind) = classify(path) else {
            continue;
        };
        let relative = path.strip_prefix(&root).expect("asset is below root");
        let metadata_path = PathBuf::from(format!("{}.meta", path.display()));
        let source_guid = read_guid(&metadata_path)?;
        let referenced_guids = if is_yaml_kind(kind) {
            read_yaml_references(path)?
        } else {
            Vec::new()
        };
        let mut warnings = Vec::new();
        if source_guid.is_none() {
            warnings.push("missing Unity .meta GUID".to_owned());
        }
        if matches!(
            kind,
            AssetKind::Shader | AssetKind::Vfx | AssetKind::AnimatorController
        ) {
            warnings.push("requires engine-specific manual conversion".to_owned());
        }
        entries.push(ManifestEntry {
            source: normalize_path(relative),
            source_guid,
            kind,
            bytes: entry.metadata()?.len(),
            destination_id: destination_id(relative),
            referenced_guids,
            status: MigrationStatus::Discovered,
            warnings,
        });
    }
    entries.sort_by(|left, right| left.source.cmp(&right.source));
    Ok(MigrationManifest {
        schema_version: 1,
        source_root: normalize_path(&root),
        entries,
    })
}

fn validate_manifest(manifest: &MigrationManifest) -> Result<Vec<String>> {
    if manifest.schema_version != 1 {
        bail!(
            "unsupported migration manifest schema {}",
            manifest.schema_version
        );
    }
    let mut by_guid = BTreeMap::<&str, &str>::new();
    let mut destination_ids = BTreeSet::<&str>::new();
    for entry in &manifest.entries {
        if !destination_ids.insert(&entry.destination_id) {
            bail!("duplicate destination ID {}", entry.destination_id);
        }
        if let Some(guid) = entry.source_guid.as_deref()
            && let Some(previous) = by_guid.insert(guid, &entry.source)
        {
            bail!(
                "duplicate Unity GUID {guid} in {previous} and {}",
                entry.source
            );
        }
    }
    let known_guids: BTreeSet<_> = by_guid.keys().copied().collect();
    let mut warnings = Vec::new();
    for entry in &manifest.entries {
        for reference in &entry.referenced_guids {
            if !known_guids.contains(reference.as_str()) {
                warnings.push(format!(
                    "{} references external or excluded GUID {reference}",
                    entry.source
                ));
            }
        }
    }
    Ok(warnings)
}

fn write_manifest(path: &Path, manifest: &MigrationManifest) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let encoded = serde_json::to_string_pretty(manifest)?;
    fs::write(path, encoded).with_context(|| format!("failed to write {}", path.display()))
}

fn read_guid(metadata_path: &Path) -> Result<Option<String>> {
    if !metadata_path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(metadata_path)?;
    Ok(contents
        .lines()
        .find_map(|line| line.trim().strip_prefix("guid: "))
        .map(str::to_owned))
}

fn read_yaml_references(path: &Path) -> Result<Vec<String>> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::InvalidData => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let mut references = BTreeSet::new();
    for segment in contents.split("guid: ").skip(1) {
        let guid: String = segment
            .chars()
            .take_while(char::is_ascii_hexdigit)
            .collect();
        if guid.len() == 32 && guid != "00000000000000000000000000000000" {
            references.insert(guid);
        }
    }
    Ok(references.into_iter().collect())
}

fn classify(path: &Path) -> Option<AssetKind> {
    let extension = path.extension()?.to_string_lossy().to_ascii_lowercase();
    match extension.as_str() {
        "unity" => Some(AssetKind::Scene),
        "prefab" => Some(AssetKind::Prefab),
        "asset" => Some(AssetKind::ScriptableAsset),
        "fbx" | "obj" | "blend" => Some(AssetKind::Model),
        "anim" => Some(AssetKind::Animation),
        "controller" | "overridecontroller" => Some(AssetKind::AnimatorController),
        "mat" => Some(AssetKind::Material),
        "shader" | "shadergraph" => Some(AssetKind::Shader),
        "png" | "tga" | "jpg" | "jpeg" | "psd" => Some(AssetKind::Texture),
        "vfx" => Some(AssetKind::Vfx),
        "wav" | "ogg" | "mp3" | "json" | "txt" | "bytes" => Some(AssetKind::Other),
        _ => None,
    }
}

fn is_yaml_kind(kind: AssetKind) -> bool {
    matches!(
        kind,
        AssetKind::Scene
            | AssetKind::Prefab
            | AssetKind::ScriptableAsset
            | AssetKind::Animation
            | AssetKind::AnimatorController
            | AssetKind::Material
            | AssetKind::Shader
            | AssetKind::Vfx
    )
}

fn is_excluded(path: &Path) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    [
        "/assets/plugins/",
        "/assets/textmesh pro/",
        "/assets/astarpathfindingproject/",
        "/assets/reflexoverride/",
    ]
    .iter()
    .any(|excluded| normalized.contains(excluded))
}

fn destination_id(path: &Path) -> String {
    let without_extension = path.with_extension("");
    let mut value = normalize_path(&without_extension).to_ascii_lowercase();
    value = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else if character == '/' {
                ':'
            } else {
                '_'
            }
        })
        .collect();
    format!("asset:{value}")
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destination_ids_are_platform_independent() {
        assert_eq!(
            destination_id(Path::new("Assets/Models/Town Hall.fbx")),
            "asset:assets:models:town_hall"
        );
    }
}
