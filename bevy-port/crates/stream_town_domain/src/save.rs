use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{CURRENT_SIMULATION_SCHEMA, GridPos, StableId, WorldSimulation};

pub const NATIVE_SAVE_VERSION: u32 = 1;
pub const CURRENT_WORLD_SNAPSHOT_SCHEMA: u32 = 3;
/// Defensive persistence ceiling for the decaying traversal integral. Visible
/// thresholds are authored as sustained crossings per minute and converted at runtime.
pub const MAX_TRAVERSAL_WEAR_SCORE: f32 = 1_000_000_000_000.0;
pub const NATIVE_SAVE_BACKUP_GENERATIONS: usize = 5;

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
#[doc(hidden)]
/// Retired payload shape kept only because schema-3 checksums include these
/// fields. The shipping runtime neither creates nor renders this mesh.
pub struct RetiredTerrainMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangle_indices: Vec<i32>,
    pub uvs: Vec<[f32; 2]>,
    pub uses_32_bit_indices: bool,
}

impl RetiredTerrainMesh {
    pub fn validate(&self) -> Result<(), RetiredTerrainMeshError> {
        if self.vertices.len() < 3 {
            return Err(RetiredTerrainMeshError::VertexCount(self.vertices.len()));
        }
        if self.triangle_indices.is_empty() || !self.triangle_indices.len().is_multiple_of(3) {
            return Err(RetiredTerrainMeshError::TriangleIndexCount(
                self.triangle_indices.len(),
            ));
        }
        if !self.uvs.is_empty() && self.uvs.len() != self.vertices.len() {
            return Err(RetiredTerrainMeshError::UvCount {
                vertices: self.vertices.len(),
                uvs: self.uvs.len(),
            });
        }
        if self
            .vertices
            .iter()
            .any(|vertex| vertex.iter().any(|coordinate| !coordinate.is_finite()))
        {
            return Err(RetiredTerrainMeshError::NonFiniteVertex);
        }
        if self
            .uvs
            .iter()
            .any(|uv| uv.iter().any(|coordinate| !coordinate.is_finite()))
        {
            return Err(RetiredTerrainMeshError::NonFiniteUv);
        }
        for (position, index) in self.triangle_indices.iter().copied().enumerate() {
            let Ok(index) = usize::try_from(index) else {
                return Err(RetiredTerrainMeshError::IndexOutOfBounds {
                    position,
                    index,
                    vertices: self.vertices.len(),
                });
            };
            if index >= self.vertices.len() {
                return Err(RetiredTerrainMeshError::IndexOutOfBounds {
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
#[doc(hidden)]
pub enum RetiredTerrainMeshError {
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
#[doc(hidden)]
/// Retired payload shape kept only for schema-3 checksum compatibility.
pub struct RetiredMigrationMetadata {
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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub traversal_wear: BTreeMap<GridPos, f32>,
    #[serde(default)]
    #[doc(hidden)]
    pub legacy_terrain_mesh: Option<RetiredTerrainMesh>,
    #[serde(default)]
    #[doc(hidden)]
    pub legacy_migration: Option<RetiredMigrationMetadata>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct NativeSaveEnvelope {
    format_version: u32,
    payload_checksum: String,
    payload: WorldSnapshot,
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
    TerrainMesh(#[from] RetiredTerrainMeshError),
    #[error("unsupported world snapshot schema {0}")]
    SnapshotSchema(u32),
    #[error("unsupported world simulation schema {0}")]
    SimulationSchema(u32),
    #[error(
        "world seed mismatch between snapshot ({snapshot_seed}) and simulation ({simulation_seed})"
    )]
    WorldSeedMismatch {
        snapshot_seed: u64,
        simulation_seed: u64,
    },
    #[error("duplicate saved actor ID {0}")]
    DuplicateActor(StableId),
    #[error("actor map key {key} does not match embedded ID {embedded}")]
    ActorKeyMismatch { key: StableId, embedded: StableId },
    #[error("building map key {key} does not match embedded ID {embedded}")]
    BuildingKeyMismatch { key: StableId, embedded: StableId },
    #[error("enemy camp map key {key} does not match embedded ID {embedded}")]
    EnemyCampKeyMismatch { key: StableId, embedded: StableId },
    #[error("persistent ID {0} is reused across simulation entity kinds")]
    DuplicatePersistentId(StableId),
    #[error("path navigation position references missing building {0}")]
    MissingPathBuilding(StableId),
    #[error("multiple paths occupy fine navigation cell {0:?}")]
    DuplicatePathNavigationPosition(GridPos),
    #[error("saved {kind} {id} has no authoritative simulation state")]
    MissingSimulationState { kind: &'static str, id: StableId },
    #[error("simulation actor {0} has no saved world entity record")]
    MissingSavedActor(StableId),
    #[error("terrain traversal wear at {position:?} has invalid score {score}")]
    InvalidTraversalWear { position: GridPos, score: f32 },
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
        self.backup_path_at(1)
    }

    #[must_use]
    pub fn backup_path_at(&self, generation: usize) -> PathBuf {
        assert!(
            (1..=NATIVE_SAVE_BACKUP_GENERATIONS).contains(&generation),
            "native save backup generation must be between 1 and {NATIVE_SAVE_BACKUP_GENERATIONS}"
        );
        let suffix = if generation == 1 {
            ".bak".to_owned()
        } else {
            format!(".bak.{generation}")
        };
        PathBuf::from(format!("{}{}", self.path.display(), suffix))
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
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        drop(file);

        if self.path.exists()
            && let Err(error) = self.rotate_backups()
        {
            let _ = fs::remove_file(&temporary);
            return Err(error);
        }
        let backup = self.backup_path();
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
            Err(primary_error) => {
                for generation in 1..=NATIVE_SAVE_BACKUP_GENERATIONS {
                    let backup = self.backup_path_at(generation);
                    if backup.exists()
                        && let Ok(snapshot) = load_native(&backup)
                    {
                        return Ok(snapshot);
                    }
                }
                Err(primary_error)
            }
        }
    }

    fn rotate_backups(&self) -> Result<(), NativeSaveError> {
        let oldest = self.backup_path_at(NATIVE_SAVE_BACKUP_GENERATIONS);
        if oldest.exists() {
            fs::remove_file(oldest)?;
        }
        for generation in (2..=NATIVE_SAVE_BACKUP_GENERATIONS).rev() {
            let source = self.backup_path_at(generation - 1);
            if source.exists() {
                fs::rename(source, self.backup_path_at(generation))?;
            }
        }
        fs::rename(&self.path, self.backup_path())?;
        Ok(())
    }
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
    if !(1..=CURRENT_WORLD_SNAPSHOT_SCHEMA).contains(&snapshot.schema_version) {
        return Err(NativeSaveError::SnapshotSchema(snapshot.schema_version));
    }
    if !(1..=CURRENT_SIMULATION_SCHEMA).contains(&snapshot.simulation.schema_version) {
        return Err(NativeSaveError::SimulationSchema(
            snapshot.simulation.schema_version,
        ));
    }
    if snapshot.world_seed != snapshot.simulation.world_seed {
        return Err(NativeSaveError::WorldSeedMismatch {
            snapshot_seed: snapshot.world_seed,
            simulation_seed: snapshot.simulation.world_seed,
        });
    }
    for (position, score) in &snapshot.traversal_wear {
        if !score.is_finite() || *score <= 0.0 || *score > MAX_TRAVERSAL_WEAR_SCORE {
            return Err(NativeSaveError::InvalidTraversalWear {
                position: *position,
                score: *score,
            });
        }
    }

    let mut persistent_ids = BTreeSet::new();
    for (key, actor) in &snapshot.simulation.actors {
        if key != &actor.id {
            return Err(NativeSaveError::ActorKeyMismatch {
                key: key.clone(),
                embedded: actor.id.clone(),
            });
        }
        persistent_ids.insert(key.clone());
    }
    for (key, building) in &snapshot.simulation.buildings {
        if key != &building.id {
            return Err(NativeSaveError::BuildingKeyMismatch {
                key: key.clone(),
                embedded: building.id.clone(),
            });
        }
        if !persistent_ids.insert(key.clone()) {
            return Err(NativeSaveError::DuplicatePersistentId(key.clone()));
        }
    }
    for (key, camp) in &snapshot.simulation.enemy_camps {
        if key != &camp.id {
            return Err(NativeSaveError::EnemyCampKeyMismatch {
                key: key.clone(),
                embedded: camp.id.clone(),
            });
        }
        if !persistent_ids.insert(key.clone()) {
            return Err(NativeSaveError::DuplicatePersistentId(key.clone()));
        }
    }
    let mut path_positions = BTreeSet::new();
    for (building, position) in &snapshot.simulation.path_navigation_positions {
        if !snapshot.simulation.buildings.contains_key(building) {
            return Err(NativeSaveError::MissingPathBuilding(building.clone()));
        }
        if !path_positions.insert(*position) {
            return Err(NativeSaveError::DuplicatePathNavigationPosition(*position));
        }
    }

    let mut actor_ids = BTreeSet::new();
    let mut saved_simulation_actor_ids = BTreeSet::new();
    for actor in &snapshot.actors {
        if !actor_ids.insert(actor.id.clone()) {
            return Err(NativeSaveError::DuplicateActor(actor.id.clone()));
        }
        let state_exists = match actor.kind {
            ActorKind::Player | ActorKind::Enemy => {
                saved_simulation_actor_ids.insert(actor.id.clone());
                snapshot.simulation.actors.contains_key(&actor.id)
            }
            ActorKind::Building => snapshot.simulation.buildings.contains_key(&actor.id),
            ActorKind::EnemyCamp => snapshot.simulation.enemy_camps.contains_key(&actor.id),
            ActorKind::Resource | ActorKind::Foliage => true,
        };
        if !state_exists {
            let kind = match actor.kind {
                ActorKind::Player | ActorKind::Enemy => "actor",
                ActorKind::Building => "building",
                ActorKind::EnemyCamp => "enemy camp",
                ActorKind::Resource | ActorKind::Foliage => unreachable!(),
            };
            return Err(NativeSaveError::MissingSimulationState {
                kind,
                id: actor.id.clone(),
            });
        }
    }
    for actor_id in snapshot.simulation.actors.keys() {
        if !saved_simulation_actor_ids.contains(actor_id) {
            return Err(NativeSaveError::MissingSavedActor(actor_id.clone()));
        }
    }
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
    use tempfile::tempdir;

    use super::*;
    use crate::{ActorCustomization, ActorState, BuildingState, EnemyCampState, StreamUserType};

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
            traversal_wear: BTreeMap::new(),
            legacy_terrain_mesh: None,
            legacy_migration: None,
        }
    }

    fn saved_player(id: &StableId) -> SavedActor {
        SavedActor {
            id: id.clone(),
            kind: ActorKind::Player,
            archetype: StableId::new("archetype:player").unwrap(),
            grid_position: GridPos { x: 1, z: 2 },
            height_centimetres: 0,
            health: 100,
        }
    }

    fn actor_state(id: StableId) -> ActorState {
        ActorState {
            id,
            display_name: None,
            login_name: None,
            user_type: StreamUserType::Normal,
            role: StableId::new("role:villager").unwrap(),
            archetype: Some(StableId::new("archetype:player").unwrap()),
            position: GridPos { x: 1, z: 2 },
            last_building_position: None,
            building_rotation_quarter_turns: 0,
            health: 100,
            max_health: 100,
            alive: true,
            respawn_remaining_seconds: None,
            inventory: BTreeMap::new(),
            station: None,
            role_progression: BTreeMap::new(),
            unlocked_pets: BTreeSet::new(),
            active_pet: None,
            preferred_target: None,
            customization: ActorCustomization::default(),
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
    fn native_save_keeps_only_the_five_latest_backup_generations() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        for seed in 1..=7 {
            store.write(&snapshot(seed)).unwrap();
        }
        assert_eq!(store.load().unwrap().world_seed, 7);
        for (generation, expected_seed) in (1..=5).zip((2..=6).rev()) {
            assert_eq!(
                load_native(&store.backup_path_at(generation))
                    .unwrap()
                    .world_seed,
                expected_seed
            );
        }
        assert!(!PathBuf::from(format!("{}.bak.6", store.path().display())).exists());
    }

    #[test]
    fn snapshot_validation_rejects_schema_and_seed_mismatches() {
        let mut invalid = snapshot(7);
        invalid.schema_version = CURRENT_WORLD_SNAPSHOT_SCHEMA + 1;
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::SnapshotSchema(_))
        ));

        invalid = snapshot(7);
        invalid.simulation.schema_version = CURRENT_SIMULATION_SCHEMA + 1;
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::SimulationSchema(_))
        ));

        invalid = snapshot(7);
        invalid.simulation.world_seed = 8;
        assert_eq!(
            validate_snapshot(&invalid).unwrap_err().to_string(),
            "world seed mismatch between snapshot (7) and simulation (8)"
        );

        invalid = snapshot(7);
        invalid
            .traversal_wear
            .insert(GridPos { x: 4, z: 9 }, f32::NAN);
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::InvalidTraversalWear { .. })
        ));

        invalid
            .traversal_wear
            .insert(GridPos { x: 4, z: 9 }, MAX_TRAVERSAL_WEAR_SCORE * 2.0);
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::InvalidTraversalWear { .. })
        ));
    }

    #[test]
    fn traversal_wear_round_trips_and_old_snapshots_default_to_empty() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("wear.stbevy"));
        let mut latest = snapshot(9);
        latest.schema_version = CURRENT_WORLD_SNAPSHOT_SCHEMA;
        latest.traversal_wear.insert(GridPos { x: 11, z: 12 }, 6.5);
        store.write(&latest).unwrap();
        assert_eq!(store.load().unwrap().traversal_wear, latest.traversal_wear);

        let encoded = ron::to_string(&snapshot(9)).unwrap();
        assert!(!encoded.contains("traversal_wear"));
        let decoded: WorldSnapshot = ron::from_str(&encoded).unwrap();
        assert!(decoded.traversal_wear.is_empty());
    }

    #[test]
    fn snapshot_validation_rejects_duplicate_and_mismatched_actor_ids() {
        let actor_id = StableId::new("actor:fixture").unwrap();
        let mut invalid = snapshot(7);
        invalid.actors = vec![saved_player(&actor_id), saved_player(&actor_id)];
        invalid
            .simulation
            .actors
            .insert(actor_id.clone(), actor_state(actor_id.clone()));
        assert_eq!(
            validate_snapshot(&invalid).unwrap_err().to_string(),
            "duplicate saved actor ID actor:fixture"
        );

        invalid.actors.truncate(1);
        invalid.simulation.actors.get_mut(&actor_id).unwrap().id =
            StableId::new("actor:different").unwrap();
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::ActorKeyMismatch { .. })
        ));

        invalid.simulation.actors.clear();
        assert_eq!(
            validate_snapshot(&invalid).unwrap_err().to_string(),
            "saved actor actor:fixture has no authoritative simulation state"
        );

        invalid.actors.clear();
        invalid
            .simulation
            .actors
            .insert(actor_id.clone(), actor_state(actor_id));
        assert_eq!(
            validate_snapshot(&invalid).unwrap_err().to_string(),
            "simulation actor actor:fixture has no saved world entity record"
        );
    }

    #[test]
    fn snapshot_validation_rejects_mismatched_or_reused_persistent_ids() {
        let persistent_id = StableId::new("entity:fixture").unwrap();
        let mut invalid = snapshot(7);
        invalid.simulation.buildings.insert(
            persistent_id.clone(),
            BuildingState {
                id: StableId::new("building:different").unwrap(),
                archetype: StableId::new("archetype:building:house").unwrap(),
                position: GridPos { x: 2, z: 3 },
                rotation_quarter_turns: 0,
                level: 1,
                health: 100,
                complete: true,
            },
        );
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::BuildingKeyMismatch { .. })
        ));

        invalid
            .simulation
            .buildings
            .get_mut(&persistent_id)
            .unwrap()
            .id = persistent_id.clone();
        invalid.simulation.enemy_camps.insert(
            persistent_id.clone(),
            EnemyCampState {
                id: persistent_id.clone(),
                archetype: StableId::new("archetype:camp:goblin").unwrap(),
                position: GridPos { x: 4, z: 5 },
                health: 100,
                spawn_remaining_seconds: 0.0,
                spawned_enemies: BTreeSet::new(),
            },
        );
        assert_eq!(
            validate_snapshot(&invalid).unwrap_err().to_string(),
            "persistent ID entity:fixture is reused across simulation entity kinds"
        );

        invalid.simulation.buildings.clear();
        invalid
            .simulation
            .enemy_camps
            .get_mut(&persistent_id)
            .unwrap()
            .id = StableId::new("enemy_camp:different").unwrap();
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::EnemyCampKeyMismatch { .. })
        ));
    }

    #[test]
    fn snapshot_validation_rejects_dangling_or_overlapping_fine_paths() {
        let first = StableId::new("building:path_first").unwrap();
        let second = StableId::new("building:path_second").unwrap();
        let position = GridPos { x: 13, z: 17 };
        let mut invalid = snapshot(7);
        invalid
            .simulation
            .path_navigation_positions
            .insert(first.clone(), position);
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::MissingPathBuilding(id)) if id == first
        ));

        for id in [&first, &second] {
            invalid.simulation.buildings.insert(
                id.clone(),
                BuildingState {
                    id: id.clone(),
                    archetype: StableId::new("archetype:building:path").unwrap(),
                    position: GridPos { x: 4, z: 5 },
                    rotation_quarter_turns: 0,
                    level: 1,
                    health: 50,
                    complete: true,
                },
            );
        }
        invalid
            .simulation
            .path_navigation_positions
            .insert(second, position);
        assert!(matches!(
            validate_snapshot(&invalid),
            Err(NativeSaveError::DuplicatePathNavigationPosition(cell)) if cell == position
        ));
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
    fn corruption_recovery_checks_older_backup_generations() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        store.write(&snapshot(1)).unwrap();
        store.write(&snapshot(2)).unwrap();
        store.write(&snapshot(3)).unwrap();
        fs::write(store.path(), "corrupt current").unwrap();
        fs::write(store.backup_path(), "corrupt newest backup").unwrap();
        assert_eq!(store.load().unwrap().world_seed, 1);
    }

    #[test]
    fn retained_terrain_mesh_validates_and_round_trips() {
        let directory = tempdir().unwrap();
        let store = NativeSaveStore::new(directory.path().join("save.stbevy"));
        let mut terrain_save = snapshot(4);
        terrain_save.legacy_terrain_mesh = Some(RetiredTerrainMesh {
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
        terrain_save.legacy_terrain_mesh = Some(RetiredTerrainMesh {
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            triangle_indices: vec![0, 1, 3],
            uvs: vec![[0.0, 0.0]; 3],
            uses_32_bit_indices: false,
        });
        assert!(matches!(
            store.write(&terrain_save),
            Err(NativeSaveError::TerrainMesh(
                RetiredTerrainMeshError::IndexOutOfBounds { .. }
            ))
        ));
        assert!(!store.path().exists());
    }
}
