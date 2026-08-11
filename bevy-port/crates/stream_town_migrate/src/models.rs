use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
struct ModelConversionReport {
    schema_version: u32,
    blender_version: String,
    source_model_count: usize,
    entries: Vec<ModelEntry>,
    failures: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ModelEntry {
    source: String,
    output: String,
    source_sha256: String,
    output_sha256: String,
    output_bytes: u64,
    meshes: usize,
    skins: usize,
    animations: usize,
    materials: usize,
    images: usize,
}

#[derive(Debug, Serialize)]
pub struct ModelValidationSummary {
    schema_version: u32,
    blender_version: String,
    models: usize,
    bytes: u64,
    meshes: usize,
    skins: usize,
    animations: usize,
    materials: usize,
    images: usize,
}

pub fn validate(
    report_path: &Path,
    repository_root: &Path,
    expected_count: Option<usize>,
) -> Result<ModelValidationSummary> {
    let encoded = fs::read_to_string(report_path)
        .with_context(|| format!("failed to read model report {}", report_path.display()))?;
    let report: ModelConversionReport = serde_json::from_str(&encoded)
        .with_context(|| format!("failed to parse model report {}", report_path.display()))?;
    if report.schema_version != 1 {
        bail!(
            "unsupported model conversion schema {}",
            report.schema_version
        );
    }
    if report.blender_version.is_empty() {
        bail!("model conversion report is missing the Blender version");
    }
    if !report.failures.is_empty() {
        bail!("model conversion has failures: {:?}", report.failures);
    }
    if report.source_model_count != report.entries.len() {
        bail!(
            "model conversion contains {} entries for {} sources",
            report.entries.len(),
            report.source_model_count
        );
    }
    if let Some(expected_count) = expected_count
        && report.source_model_count != expected_count
    {
        bail!(
            "model conversion contains {} sources; expected {expected_count}",
            report.source_model_count
        );
    }

    let root = repository_root.canonicalize().with_context(|| {
        format!(
            "repository root {} does not exist",
            repository_root.display()
        )
    })?;
    let mut sources = BTreeSet::new();
    let mut outputs = BTreeSet::new();
    let mut summary = ModelValidationSummary {
        schema_version: report.schema_version,
        blender_version: report.blender_version,
        models: report.entries.len(),
        bytes: 0,
        meshes: 0,
        skins: 0,
        animations: 0,
        materials: 0,
        images: 0,
    };
    for entry in report.entries {
        if !sources.insert(entry.source.clone()) {
            bail!("duplicate model source {}", entry.source);
        }
        if !outputs.insert(entry.output.clone()) {
            bail!("duplicate model output {}", entry.output);
        }
        if !entry.source.starts_with("Assets/")
            || !Path::new(&entry.source)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("fbx"))
        {
            bail!("invalid model source path {}", entry.source);
        }
        if !entry
            .output
            .starts_with("bevy-port/assets/migrated/models/")
            || !Path::new(&entry.output)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
        {
            bail!("invalid model output path {}", entry.output);
        }
        let source = safe_join(&root, &entry.source)?;
        let output = safe_join(&root, &entry.output)?;
        validate_hash(&source, &entry.source_sha256)?;
        validate_hash(&output, &entry.output_sha256)?;
        let metadata = fs::metadata(&output)
            .with_context(|| format!("failed to inspect {}", output.display()))?;
        if metadata.len() != entry.output_bytes {
            bail!("GLB byte count changed for {}", entry.output);
        }
        validate_glb_header(&output, entry.output_bytes)?;
        summary.bytes = summary.bytes.saturating_add(entry.output_bytes);
        summary.meshes += entry.meshes;
        summary.skins += entry.skins;
        summary.animations += entry.animations;
        summary.materials += entry.materials;
        summary.images += entry.images;
    }
    Ok(summary)
}

fn safe_join(root: &Path, relative: &str) -> Result<PathBuf> {
    let relative = Path::new(relative);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        bail!(
            "unsafe path in model conversion report: {}",
            relative.display()
        );
    }
    Ok(root.join(relative))
}

fn validate_hash(path: &Path, expected: &str) -> Result<()> {
    if expected.len() != 64 || !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("invalid SHA-256 for {}", path.display());
    }
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let actual = hex::encode(Sha256::digest(bytes));
    if actual != expected {
        bail!("SHA-256 mismatch for {}", path.display());
    }
    Ok(())
}

fn validate_glb_header(path: &Path, expected_length: u64) -> Result<()> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    if bytes.len() < 20 || &bytes[0..4] != b"glTF" {
        bail!("{} is not a GLB file", path.display());
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().expect("four bytes"));
    let declared_length = u32::from_le_bytes(bytes[8..12].try_into().expect("four bytes"));
    let json_chunk_type = u32::from_le_bytes(bytes[16..20].try_into().expect("four bytes"));
    if version != 2
        || u64::from(declared_length) != expected_length
        || json_chunk_type != 0x4E4F_534A
    {
        bail!("{} has an invalid glTF 2.0 header", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn rejects_parent_path_components() {
        assert!(safe_join(Path::new("C:/repo"), "../escape.glb").is_err());
        assert!(safe_join(Path::new("C:/repo"), "Assets/model.fbx").is_ok());
    }

    #[test]
    fn validates_glb_two_header_and_length() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("fixture.glb");
        let mut header = Vec::new();
        header.extend_from_slice(b"glTF");
        header.extend_from_slice(&2_u32.to_le_bytes());
        header.extend_from_slice(&20_u32.to_le_bytes());
        header.extend_from_slice(&0_u32.to_le_bytes());
        header.extend_from_slice(&0x4E4F_534A_u32.to_le_bytes());
        fs::write(&path, header).unwrap();
        validate_glb_header(&path, 20).unwrap();
        assert!(validate_glb_header(&path, 21).is_err());
    }
}
