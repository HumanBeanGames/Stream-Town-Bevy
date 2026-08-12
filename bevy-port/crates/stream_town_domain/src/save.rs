use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{GridPos, StableId, WorldSimulation};

pub const NATIVE_SAVE_VERSION: u32 = 1;
const LEGACY_MAGIC: &[u8; 4] = b"STSV";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ActorKind {
    Player,
    Enemy,
    Building,
    Resource,
    Foliage,
    EnemyCamp,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SavedActor {
    pub id: StableId,
    pub kind: ActorKind,
    pub archetype: StableId,
    pub grid_position: GridPos,
    pub height_centimetres: i16,
    pub health: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SavedTerrainMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangle_indices: Vec<i32>,
    pub uvs: Vec<[f32; 2]>,
    pub uses_32_bit_indices: bool,
}

impl SavedTerrainMesh {
    pub fn validate(&self) -> Result<(), SavedTerrainMeshError> {
        if self.vertices.len() < 3 {
            return Err(SavedTerrainMeshError::VertexCount(self.vertices.len()));
        }
        if self.triangle_indices.is_empty() || !self.triangle_indices.len().is_multiple_of(3) {
            return Err(SavedTerrainMeshError::TriangleIndexCount(
                self.triangle_indices.len(),
            ));
        }
        if !self.uvs.is_empty() && self.uvs.len() != self.vertices.len() {
            return Err(SavedTerrainMeshError::UvCount {
                vertices: self.vertices.len(),
                uvs: self.uvs.len(),
            });
        }
        if self
            .vertices
            .iter()
            .any(|vertex| vertex.iter().any(|coordinate| !coordinate.is_finite()))
        {
            return Err(SavedTerrainMeshError::NonFiniteVertex);
        }
        if self
            .uvs
            .iter()
            .any(|uv| uv.iter().any(|coordinate| !coordinate.is_finite()))
        {
            return Err(SavedTerrainMeshError::NonFiniteUv);
        }
        for (position, index) in self.triangle_indices.iter().copied().enumerate() {
            let Ok(index) = usize::try_from(index) else {
                return Err(SavedTerrainMeshError::IndexOutOfBounds {
                    position,
                    index,
                    vertices: self.vertices.len(),
                });
            };
            if index >= self.vertices.len() {
                return Err(SavedTerrainMeshError::IndexOutOfBounds {
                    position,
                    index: i32::try_from(index).unwrap_or(i32::MAX),
                    vertices: self.vertices.len(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SavedTerrainMeshError {
    #[error("retained terrain mesh must contain at least three vertices, found {0}")]
    VertexCount(usize),
    #[error("retained terrain mesh index count must be a non-zero multiple of three, found {0}")]
    TriangleIndexCount(usize),
    #[error("retained terrain mesh has {vertices} vertices but {uvs} UV coordinates")]
    UvCount { vertices: usize, uvs: usize },
    #[error("retained terrain mesh contains a non-finite vertex coordinate")]
    NonFiniteVertex,
    #[error("retained terrain mesh contains a non-finite UV coordinate")]
    NonFiniteUv,
    #[error(
        "retained terrain mesh index {index} at position {position} is outside {vertices} vertices"
    )]
    IndexOutOfBounds {
        position: usize,
        index: i32,
        vertices: usize,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LegacyMigrationMetadata {
    pub source_schema_version: u32,
    pub source_container_version: Option<u32>,
    pub source_terrain_generator_version: i32,
    pub source_sha256: String,
    pub recovered_from_backup: bool,
    pub relocated_entities: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WorldSnapshot {
    pub schema_version: u32,
    pub world_seed: u64,
    pub generator_version: u32,
    pub world_hash: String,
    pub elapsed_seconds: u64,
    pub actors: Vec<SavedActor>,
    pub simulation: WorldSimulation,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub resource_nodes: BTreeMap<StableId, u32>,
    #[serde(default)]
    pub legacy_terrain_mesh: Option<SavedTerrainMesh>,
    #[serde(default)]
    pub legacy_migration: Option<LegacyMigrationMetadata>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct NativeSaveEnvelope {
    format_version: u32,
    payload_checksum: String,
    payload: WorldSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LegacySaveInfo {
    pub kind: LegacySaveKind,
    pub source_size_bytes: u64,
    pub payload_schema_version: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum LegacySaveKind {
    Binary { container_version: u32 },
    Json,
}

#[derive(Debug, Error)]
pub enum NativeSaveError {
    #[error("save I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("save serialization failed: {0}")]
    Serialize(#[from] ron::Error),
    #[error("save deserialization failed: {0}")]
    Deserialize(#[from] ron::error::SpannedError),
    #[error("unsupported native save version {0}")]
    Version(u32),
    #[error("native save checksum mismatch")]
    Checksum,
    #[error("native save retained terrain mesh is invalid: {0}")]
    TerrainMesh(#[from] SavedTerrainMeshError),
    #[error("legacy save header is incomplete")]
    LegacyHeader,
    #[error("file is not a recognized Stream Town save")]
    UnknownLegacyFormat,
    #[error("unsupported legacy container version {0}")]
    LegacyContainer(u32),
    #[error("unsupported legacy payload schema {0}")]
    LegacySchema(u32),
    #[error("legacy compressed payload is invalid: {0}")]
    LegacyCompression(io::Error),
    #[error("legacy JSON is invalid: {0}")]
    LegacyJson(serde_json::Error),
}

pub struct NativeSaveStore {
    path: PathBuf,
}

impl NativeSaveStore {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn backup_path(&self) -> PathBuf {
        PathBuf::from(format!("{}.bak", self.path.display()))
    }

    pub fn write(&self, snapshot: &WorldSnapshot) -> Result<(), NativeSaveError> {
        validate_snapshot(snapshot)?;
        let envelope = NativeSaveEnvelope {
            format_version: NATIVE_SAVE_VERSION,
            payload_checksum: snapshot_checksum(snapshot)?,
            payload: snapshot.clone(),
        };
        let serialized = ron::ser::to_string_pretty(&envelope, ron::ser::PrettyConfig::default())?;
        let parent = self.path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let temporary = self.path.with_extension("stbevy.tmp");
        let backup = self.backup_path();

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        drop(file);

        if self.path.exists() {
            if backup.exists() {
                fs::remove_file(&backup)?;
            }
            fs::rename(&self.path, &backup)?;
        }
        if let Err(error) = fs::rename(&temporary, &self.path) {
            if backup.exists() && !self.path.exists() {
                let _ = fs::rename(&backup, &self.path);
            }
            let _ = fs::remove_file(&temporary);
            return Err(error.into());
        }
        Ok(())
    }

    pub fn load(&self) -> Result<WorldSnapshot, NativeSaveError> {
        match load_native(&self.path) {
            Ok(snapshot) => Ok(snapshot),
            Err(primary_error) if self.backup_path().exists() => {
                load_native(&self.backup_path()).map_err(|_| primary_error)
            }
            Err(error) => Err(error),
        }
    }
}

pub fn inspect_legacy_save(path: &Path) -> Result<LegacySaveInfo, NativeSaveError> {
    let source_size_bytes = path.metadata()?.len();
    let mut file = File::open(path)?;
    let mut prefix = [0_u8; 8];
    let read = file.read(&mut prefix)?;
    if read == 0 {
        return Err(NativeSaveError::LegacyHeader);
    }

    if read >= 4 && &prefix[..4] == LEGACY_MAGIC {
        if read < 8 {
            return Err(NativeSaveError::LegacyHeader);
        }
        let container_version = u32::from_le_bytes(prefix[4..8].try_into().expect("four bytes"));
        if container_version != 1 {
            return Err(NativeSaveError::LegacyContainer(container_version));
        }
        let mut compressed_payload = Vec::new();
        file.read_to_end(&mut compressed_payload)?;
        let mut decoder = GzDecoder::new(compressed_payload.as_slice());
        let mut schema_bytes = [0_u8; 4];
        decoder
            .read_exact(&mut schema_bytes)
            .map_err(NativeSaveError::LegacyCompression)?;
        let schema = u32::from_le_bytes(schema_bytes);
        if !(1..=3).contains(&schema) {
            return Err(NativeSaveError::LegacySchema(schema));
        }
        return Ok(LegacySaveInfo {
            kind: LegacySaveKind::Binary { container_version },
            source_size_bytes,
            payload_schema_version: Some(schema),
        });
    }

    let mut bytes = prefix[..read].to_vec();
    file.read_to_end(&mut bytes)?;
    if bytes
        .iter()
        .copied()
        .find(|byte| !byte.is_ascii_whitespace())
        == Some(b'{')
    {
        let value: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(NativeSaveError::LegacyJson)?;
        let schema = value
            .get("SchemaVersion")
            .or_else(|| value.get("schemaVersion"))
            .and_then(serde_json::Value::as_u64)
            .and_then(|value| u32::try_from(value).ok());
        return Ok(LegacySaveInfo {
            kind: LegacySaveKind::Json,
            source_size_bytes,
            payload_schema_version: schema,
        });
    }
    Err(NativeSaveError::UnknownLegacyFormat)
}

fn load_native(path: &Path) -> Result<WorldSnapshot, NativeSaveError> {
    let encoded = fs::read_to_string(path)?;
    let envelope: NativeSaveEnvelope = ron::from_str(&encoded)?;
    if envelope.format_version != NATIVE_SAVE_VERSION {
        return Err(NativeSaveError::Version(envelope.format_version));
    }
    if snapshot_checksum(&envelope.payload)? != envelope.payload_checksum {
        return Err(NativeSaveError::Checksum);
    }
    validate_snapshot(&envelope.payload)?;
    Ok(envelope.payload)
}

fn validate_snapshot(snapshot: &WorldSnapshot) -> Result<(), NativeSaveError> {
    if let Some(mesh) = &snapshot.legacy_terrain_mesh {
        mesh.validate()?;
    }
    Ok(())
}

fn snapshot_checksum(snapshot: &WorldSnapshot) -> Result<String, ron::Error> {
    let payload = ron::to_string(snapshot)?;
    Ok(hex::encode(Sha256::digest(payload.as_bytes())))
}

#[cfg(test)]
mod tests {
    use flate2::{Compression, write::GzEncoder};
    use tempfile::tempdir;

    use super::*;

    fn snapshot(seed: u64) -> WorldSnapshot {
        WorldSnapshot {
            schema_version: 1,
            world_seed: seed,
            generator_version: 1,
            world_hash: "abc".into(),
            elapsed_seconds: 42,
            actors: vec![],
            simulation: WorldSimulation::new(seed),
            resource_nodes: BTreeMap::new(),
            legacy_terrain_mesh: None,
            legacy_migration: None,
        }
    }

    #[test]
    fn native_save_is_atomic_and_keeps_backup() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        store.write(&snapshot(1)).unwrap();
        let mut latest = snapshot(2);
        latest
            .resource_nodes
            .insert(StableId::new("resource:1:2").unwrap(), 17);
        store.write(&latest).unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.world_seed, 2);
        assert_eq!(loaded.resource_nodes, latest.resource_nodes);
        assert!(store.backup_path().exists());
        assert!(
            !ron::to_string(&snapshot(3))
                .unwrap()
                .contains("resource_nodes")
        );
    }

    #[test]
    fn detects_corruption_and_recovers_backup() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        store.write(&snapshot(1)).unwrap();
        store.write(&snapshot(2)).unwrap();
        fs::write(store.path(), "corrupt").unwrap();
        assert_eq!(store.load().unwrap().world_seed, 1);
    }

    #[test]
    fn retained_terrain_mesh_validates_and_round_trips() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        let mut terrain_save = snapshot(4);
        terrain_save.legacy_terrain_mesh = Some(SavedTerrainMesh {
            vertices: vec![[-1.0, 0.0, -1.0], [1.0, 0.0, -1.0], [0.0, 2.0, 1.0]],
            triangle_indices: vec![0, 1, 2],
            uvs: Vec::new(),
            uses_32_bit_indices: false,
        });
        store.write(&terrain_save).unwrap();
        assert_eq!(store.load().unwrap(), terrain_save);
    }

    #[test]
    fn retained_terrain_mesh_rejects_malformed_geometry() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        let mut terrain_save = snapshot(4);
        terrain_save.legacy_terrain_mesh = Some(SavedTerrainMesh {
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            triangle_indices: vec![0, 1, 3],
            uvs: vec![[0.0, 0.0]; 3],
            uses_32_bit_indices: false,
        });
        assert!(matches!(
            store.write(&terrain_save),
            Err(NativeSaveError::TerrainMesh(
                SavedTerrainMeshError::IndexOutOfBounds { .. }
            ))
        ));
        assert!(!store.path().exists());
    }

    #[test]
    fn inspects_legacy_binary_header_without_modifying_source() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("StreamTownSave.stsave");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&3_u32.to_le_bytes()).unwrap();
        encoder.write_all(b"payload").unwrap();
        let mut bytes = b"STSV".to_vec();
        bytes.extend_from_slice(&1_u32.to_le_bytes());
        bytes.extend_from_slice(&encoder.finish().unwrap());
        fs::write(&path, bytes).unwrap();
        let before = fs::read(&path).unwrap();
        let info = inspect_legacy_save(&path).unwrap();
        assert_eq!(info.payload_schema_version, Some(3));
        assert_eq!(fs::read(&path).unwrap(), before);
    }
}
