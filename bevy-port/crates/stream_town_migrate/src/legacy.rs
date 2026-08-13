use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Cursor, Read},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result, anyhow, ensure};
use flate2::read::GzDecoder;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use stream_town_domain::{
    ActorCustomization, ActorKind, ActorState, BuildingState, ContentCatalog, EnemyCampState,
    GameConfig, GridPos, LegacyMigrationMetadata, MAX_ROLE_LEVEL, NativeSaveStore, ObjectiveDef,
    ObjectiveKind, ObjectiveProgress, RULER_VOTE_INTERVAL_SECONDS, RoleProgress, SavedActor,
    SavedTerrainMesh, StableId, StreamUserType, TownGoalState, WorldSimulation, WorldSnapshot,
    generate_world,
};

const MAGIC: &[u8; 4] = b"STSV";
const PAYLOAD_TRAILER: i32 = 0x5354_454e;
const MAX_STRING_BYTES: usize = 1024 * 1024;
const MAX_MESH_VERTICES: usize = 10_000_000;
const MAX_TRIANGLE_INDICES: usize = 60_000_000;
const MAX_WORLD_INSTANCES: usize = 5_000_000;
const MAX_ENTITIES: usize = 1_000_000;
const MAX_SMALL_COLLECTION: usize = 100_000;
const MAX_DECOMPRESSED_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Debug, Serialize)]
pub(crate) struct ImportReport {
    source: String,
    destination: String,
    schema_version: u32,
    container_version: Option<u32>,
    recovered_from_backup: bool,
    entities: usize,
    relocated_entities: u32,
    preserved_terrain_mesh: bool,
    native_world_hash: String,
}

#[derive(Clone, Debug)]
struct LegacyEntity {
    key: String,
    display_name: Option<String>,
    login_name: Option<String>,
    user_type: StreamUserType,
    kind: ActorKind,
    archetype: String,
    position: [f32; 3],
    rotation_quarter_turns: i32,
    health: i32,
    role: Option<String>,
    level: i32,
    inventory: BTreeMap<String, u32>,
    role_progression: BTreeMap<String, RoleProgress>,
    station_reference: Option<LegacyReference>,
    target_reference: Option<LegacyReference>,
    active_pet: Option<String>,
    unlocked_pets: BTreeSet<String>,
    customization: ActorCustomization,
}

#[derive(Clone, Debug)]
struct LegacyReference {
    guid: u32,
    pool: String,
}

#[derive(Debug)]
struct LegacyDecodedSave {
    schema_version: u32,
    container_version: Option<u32>,
    terrain_seed: Option<i32>,
    terrain_generator_version: i32,
    terrain_mesh: Option<SavedTerrainMesh>,
    entities: Vec<LegacyEntity>,
    world_age_seconds: f64,
    town_resources: BTreeMap<String, u32>,
    unlocked_technology: BTreeSet<String>,
    active_goal: Option<LegacyGoal>,
    ruler_name: Option<String>,
    ruler_vote_cooldown_seconds: f32,
}

#[derive(Clone, Debug)]
struct LegacyGoal {
    technology: String,
    objectives: Vec<LegacyObjective>,
}

#[derive(Clone, Debug)]
struct LegacyObjective {
    objective_type: String,
    resource_type: String,
    building_type: String,
    enemy_type: String,
    required_amount: u32,
    amount: u32,
}

type LegacyWorldState = (
    f64,
    BTreeMap<String, u32>,
    BTreeSet<String>,
    Option<LegacyGoal>,
    Option<String>,
    f32,
);

pub(crate) fn import_save(
    source: &Path,
    destination: &Path,
    config: &GameConfig,
) -> Result<ImportReport> {
    ensure!(
        source.is_file(),
        "legacy save {} does not exist",
        source.display()
    );
    let source_absolute = absolute_path(source)?;
    let destination_absolute = absolute_path(destination)?;
    ensure!(
        source_absolute != destination_absolute,
        "native destination must not overwrite the legacy source"
    );

    let primary_bytes = fs::read(source)
        .with_context(|| format!("failed to read legacy save {}", source.display()))?;
    let (decoded, used_path, used_bytes, recovered_from_backup) =
        match decode_legacy(&primary_bytes) {
            Ok(decoded) => (decoded, source.to_path_buf(), primary_bytes, false),
            Err(primary_error) => {
                let backup = backup_candidate(source);
                if !backup.is_file() {
                    return Err(primary_error).with_context(|| {
                        format!(
                            "legacy save {} is invalid and no backup exists",
                            source.display()
                        )
                    });
                }
                let backup_bytes = fs::read(&backup)
                    .with_context(|| format!("failed to read backup {}", backup.display()))?;
                let decoded = decode_legacy(&backup_bytes).with_context(|| {
                    format!(
                        "legacy save {} and backup {} are both invalid",
                        source.display(),
                        backup.display()
                    )
                })?;
                (decoded, backup, backup_bytes, true)
            }
        };

    let source_sha256 = hex::encode(Sha256::digest(&used_bytes));
    let (mut snapshot, relocated_entities) = convert(decoded, config)?;
    let decoded_metadata = snapshot
        .legacy_migration
        .take()
        .expect("conversion always records migration metadata");
    snapshot.legacy_migration = Some(LegacyMigrationMetadata {
        source_schema_version: decoded_metadata.source_schema_version,
        source_container_version: decoded_metadata.source_container_version,
        source_terrain_generator_version: decoded_metadata.source_terrain_generator_version,
        source_sha256,
        recovered_from_backup,
        relocated_entities,
    });

    let store = NativeSaveStore::new(destination);
    store.write(&snapshot)?;
    let reloaded = store.load()?;
    ensure!(
        reloaded == snapshot,
        "native save failed post-write reload validation"
    );

    Ok(ImportReport {
        source: used_path.display().to_string(),
        destination: destination.display().to_string(),
        schema_version: snapshot
            .legacy_migration
            .as_ref()
            .map_or(snapshot.schema_version, |metadata| {
                metadata.source_schema_version
            }),
        container_version: snapshot
            .legacy_migration
            .as_ref()
            .and_then(|metadata| metadata.source_container_version),
        recovered_from_backup,
        entities: snapshot.actors.len(),
        relocated_entities,
        preserved_terrain_mesh: snapshot.legacy_terrain_mesh.is_some(),
        native_world_hash: snapshot.world_hash,
    })
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn backup_candidate(source: &Path) -> PathBuf {
    let file_name = source
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let backup_name = if file_name.eq_ignore_ascii_case("StreamTownSave.stsave") {
        "StreamTownSave.backup.stsave".to_owned()
    } else if file_name.eq_ignore_ascii_case("StreamTownSave.json") {
        "StreamTownSave.backup.json".to_owned()
    } else {
        format!("{file_name}.backup")
    };
    source.with_file_name(backup_name)
}

fn decode_legacy(bytes: &[u8]) -> Result<LegacyDecodedSave> {
    if bytes.starts_with(MAGIC) {
        decode_binary(bytes)
    } else {
        decode_json(bytes)
    }
}

fn decode_binary(bytes: &[u8]) -> Result<LegacyDecodedSave> {
    ensure!(bytes.len() >= 8, "legacy binary header is incomplete");
    let container_version = i32::from_le_bytes(bytes[4..8].try_into().expect("four bytes"));
    ensure!(
        container_version == 1,
        "unsupported legacy container version {container_version}"
    );
    let gzip_decoder = GzDecoder::new(&bytes[8..]);
    let mut limited = gzip_decoder.take(MAX_DECOMPRESSED_BYTES + 1);
    let mut payload = Vec::new();
    limited
        .read_to_end(&mut payload)
        .context("legacy compressed payload is invalid")?;
    ensure!(
        u64::try_from(payload.len()).unwrap_or(u64::MAX) <= MAX_DECOMPRESSED_BYTES,
        "legacy decompressed payload exceeds {MAX_DECOMPRESSED_BYTES} bytes"
    );
    let mut parser = BinaryParser::new(&payload);
    let mut decoded = parser.read_save()?;
    decoded.container_version = Some(u32::try_from(container_version).expect("positive version"));
    Ok(decoded)
}

struct BinaryParser<'a> {
    reader: Cursor<&'a [u8]>,
    entities: Vec<LegacyEntity>,
    next_foliage_id: u64,
}

impl<'a> BinaryParser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            reader: Cursor::new(bytes),
            entities: Vec::new(),
            next_foliage_id: 0,
        }
    }

    fn read_save(&mut self) -> Result<LegacyDecodedSave> {
        let schema_version = self.u32()?;
        ensure!(
            (1..=3).contains(&schema_version),
            "unsupported legacy payload schema {schema_version}"
        );
        let _saved_at_utc = self.string()?;
        let (terrain_seed, terrain_generator_version, terrain_mesh) =
            self.world_generation(schema_version)?;
        self.buildings()?;
        self.enemies()?;
        let (
            world_age_seconds,
            town_resources,
            unlocked_technology,
            active_goal,
            ruler_name,
            ruler_vote_cooldown_seconds,
        ) = self.world_state(schema_version)?;
        self.players(schema_version)?;
        ensure!(
            self.i32()? == PAYLOAD_TRAILER,
            "legacy payload trailer is missing or corrupt"
        );
        ensure!(
            self.reader.position() == self.reader.get_ref().len() as u64,
            "legacy save contains unexpected trailing payload data"
        );
        Ok(LegacyDecodedSave {
            schema_version,
            container_version: None,
            terrain_seed,
            terrain_generator_version,
            terrain_mesh,
            entities: std::mem::take(&mut self.entities),
            world_age_seconds,
            town_resources,
            unlocked_technology,
            active_goal,
            ruler_name,
            ruler_vote_cooldown_seconds,
        })
    }

    fn world_generation(
        &mut self,
        schema: u32,
    ) -> Result<(Option<i32>, i32, Option<SavedTerrainMesh>)> {
        let (terrain_seed, generator_version, mesh) = if schema >= 2 {
            let has_seed = self.boolean()?;
            let seed = self.i32()?;
            let generator = self.i32()?;
            let mesh = if has_seed { None } else { Some(self.mesh()?) };
            (has_seed.then_some(seed), generator, mesh)
        } else {
            (None, 0, Some(self.mesh()?))
        };
        self.resources(schema)?;
        self.foliage(schema)?;
        self.enemy_camps()?;
        Ok((terrain_seed, generator_version, mesh))
    }

    fn mesh(&mut self) -> Result<SavedTerrainMesh> {
        let vertices = self.list(MAX_MESH_VERTICES, Self::vec3)?;
        let triangle_indices = self.list(MAX_TRIANGLE_INDICES, Self::i32)?;
        let uvs = self.list(MAX_MESH_VERTICES, Self::vec2)?;
        Ok(SavedTerrainMesh {
            vertices: vertices.unwrap_or_default(),
            triangle_indices: triangle_indices.unwrap_or_default(),
            uvs: uvs.unwrap_or_default(),
            uses_32_bit_indices: self.boolean()?,
        })
    }

    fn resources(&mut self, schema: u32) -> Result<()> {
        let Some(group_count) = self.count(MAX_SMALL_COLLECTION)? else {
            return Ok(());
        };
        for group_index in 0..group_count {
            let resource_type = self.string()?.unwrap_or_else(|| "unknown".to_owned());
            let Some(instance_count) = self.count(MAX_WORLD_INSTANCES)? else {
                continue;
            };
            for instance_index in 0..instance_count {
                let position = self.vec3()?;
                let (amount, guid) = if schema >= 2 {
                    let amount = self.i32()?;
                    let _unlimited = self.boolean()?;
                    (amount, self.u32()?)
                } else {
                    for _ in 0..16 {
                        let _ = self.f32()?;
                    }
                    let legacy_type = self.string()?;
                    let amount = self.i32()?;
                    let _unlimited = self.boolean()?;
                    let guid = self.u32()?;
                    let _mesh_index = self.i32()?;
                    let _material_index = self.i32()?;
                    let _ = legacy_type;
                    (amount, guid)
                };
                self.entities.push(LegacyEntity {
                    key: format!("resource:{guid}:{group_index}:{instance_index}"),
                    display_name: None,
                    login_name: None,
                    user_type: StreamUserType::Normal,
                    kind: ActorKind::Resource,
                    archetype: resource_type.clone(),
                    position,
                    rotation_quarter_turns: 0,
                    health: amount,
                    role: None,
                    level: 0,
                    inventory: BTreeMap::new(),
                    role_progression: BTreeMap::new(),
                    station_reference: None,
                    target_reference: None,
                    active_pet: None,
                    unlocked_pets: BTreeSet::new(),
                    customization: ActorCustomization::default(),
                });
            }
        }
        Ok(())
    }

    fn foliage(&mut self, schema: u32) -> Result<()> {
        if schema >= 2 {
            self.foliage_groups("land")?;
            self.foliage_groups("water")?;
        } else {
            self.foliage_instances("land")?;
            self.foliage_instances("water")?;
        }
        Ok(())
    }

    fn foliage_groups(&mut self, layer: &str) -> Result<()> {
        let Some(group_count) = self.count(MAX_SMALL_COLLECTION)? else {
            return Ok(());
        };
        for group_index in 0..group_count {
            let settings = self.string()?.unwrap_or_else(|| "unknown".to_owned());
            let positions = self
                .list(MAX_WORLD_INSTANCES, Self::vec3)?
                .unwrap_or_default();
            for (position_index, position) in positions.into_iter().enumerate() {
                self.push_foliage(
                    format!("{settings}:{layer}:{group_index}:{position_index}"),
                    settings.clone(),
                    position,
                );
            }
        }
        Ok(())
    }

    fn foliage_instances(&mut self, layer: &str) -> Result<()> {
        let Some(count) = self.count(MAX_WORLD_INSTANCES)? else {
            return Ok(());
        };
        for index in 0..count {
            let position = self.transform()?;
            let settings = self.string()?.unwrap_or_else(|| "unknown".to_owned());
            let mesh_index = self.i32()?;
            self.push_foliage(
                format!("{settings}:{layer}:{mesh_index}:{index}"),
                settings,
                position,
            );
        }
        Ok(())
    }

    fn push_foliage(&mut self, key: String, archetype: String, position: [f32; 3]) {
        self.next_foliage_id += 1;
        self.entities.push(LegacyEntity {
            key: format!("foliage:{}:{key}", self.next_foliage_id),
            display_name: None,
            login_name: None,
            user_type: StreamUserType::Normal,
            kind: ActorKind::Foliage,
            archetype,
            position,
            rotation_quarter_turns: 0,
            health: 1,
            role: None,
            level: 0,
            inventory: BTreeMap::new(),
            role_progression: BTreeMap::new(),
            station_reference: None,
            target_reference: None,
            active_pet: None,
            unlocked_pets: BTreeSet::new(),
            customization: ActorCustomization::default(),
        });
    }

    fn enemy_camps(&mut self) -> Result<()> {
        let Some(count) = self.count(MAX_ENTITIES)? else {
            return Ok(());
        };
        for index in 0..count {
            let position = self.transform()?;
            let health = self.i32()?;
            let guid = self.u32()?;
            self.entities.push(LegacyEntity {
                key: format!("enemy_camp:{guid}:{index}"),
                display_name: None,
                login_name: None,
                user_type: StreamUserType::Normal,
                kind: ActorKind::EnemyCamp,
                archetype: "enemy_camp".to_owned(),
                position,
                rotation_quarter_turns: 0,
                health,
                role: None,
                level: 0,
                inventory: BTreeMap::new(),
                role_progression: BTreeMap::new(),
                station_reference: None,
                target_reference: None,
                active_pet: None,
                unlocked_pets: BTreeSet::new(),
                customization: ActorCustomization::default(),
            });
        }
        Ok(())
    }

    fn buildings(&mut self) -> Result<()> {
        let Some(count) = self.count(MAX_ENTITIES)? else {
            return Ok(());
        };
        for index in 0..count {
            let (position, rotation_quarter_turns) = self.transform_with_rotation()?;
            let archetype = self.string()?.unwrap_or_else(|| "unknown".to_owned());
            let health = self.i32()?;
            let guid = self.u32()?;
            let _state = self.i32()?;
            let level = self.i32()?;
            if let Some(destroyed_count) = self.count(MAX_WORLD_INSTANCES)? {
                for _ in 0..destroyed_count {
                    let _ = self.transform()?;
                    let _ = self.string()?;
                }
            }
            self.entities.push(LegacyEntity {
                key: format!("building:{guid}:{index}"),
                display_name: None,
                login_name: None,
                user_type: StreamUserType::Normal,
                kind: ActorKind::Building,
                archetype,
                position,
                rotation_quarter_turns,
                health,
                role: None,
                level,
                inventory: BTreeMap::new(),
                role_progression: BTreeMap::new(),
                station_reference: None,
                target_reference: None,
                active_pet: None,
                unlocked_pets: BTreeSet::new(),
                customization: ActorCustomization::default(),
            });
        }
        Ok(())
    }

    fn enemies(&mut self) -> Result<()> {
        let Some(count) = self.count(MAX_ENTITIES)? else {
            return Ok(());
        };
        for index in 0..count {
            let position = self.transform()?;
            let archetype = self.string()?.unwrap_or_else(|| "unknown".to_owned());
            let health = self.i32()?;
            let guid = self.u32()?;
            let target_guid = self.u32()?;
            let target_pool = self.string()?.unwrap_or_default();
            let _camp_guid = self.u32()?;
            let _camp_pool = self.string()?;
            self.entities.push(LegacyEntity {
                key: format!("enemy:{guid}:{index}"),
                display_name: None,
                login_name: None,
                user_type: StreamUserType::Normal,
                kind: ActorKind::Enemy,
                archetype,
                position,
                rotation_quarter_turns: 0,
                health,
                role: Some("enemy".to_owned()),
                level: 0,
                inventory: BTreeMap::new(),
                role_progression: BTreeMap::new(),
                station_reference: None,
                target_reference: (target_guid != 0).then_some(LegacyReference {
                    guid: target_guid,
                    pool: target_pool,
                }),
                active_pet: None,
                unlocked_pets: BTreeSet::new(),
                customization: ActorCustomization::default(),
            });
        }
        Ok(())
    }

    fn world_state(&mut self, schema: u32) -> Result<LegacyWorldState> {
        let world_age = f64::from(self.f32()?);
        let _last_event = self.i32()?;
        let _time_since_last_event = self.i32()?;
        let (_tech_available, unlocked, active_goal) = self.tech_tree(schema)?;
        let mut resources = BTreeMap::new();
        if let Some(count) = self.count(MAX_SMALL_COLLECTION)? {
            for _ in 0..count {
                let kind = self.string()?.unwrap_or_else(|| "unknown".to_owned());
                let amount = nonnegative_u32(self.i32()?);
                resources.insert(kind, amount);
            }
        }
        for kind in ["wood", "ore", "food", "gold"] {
            let amount = nonnegative_u32(self.i32()?);
            resources.entry(kind.to_owned()).or_insert(amount);
        }
        let is_current_ruler = self.boolean()?;
        let until_vote = self.f32()?.max(0.0);
        let ruler_name = self.string()?.filter(|name| !name.trim().is_empty());
        Ok((
            world_age.max(0.0),
            resources,
            unlocked,
            active_goal,
            is_current_ruler.then_some(ruler_name).flatten(),
            until_vote,
        ))
    }

    fn tech_tree(&mut self, schema: u32) -> Result<(bool, BTreeSet<String>, Option<LegacyGoal>)> {
        let available = self.boolean()?;
        let ids = self
            .list(MAX_SMALL_COLLECTION, Self::string)?
            .unwrap_or_default();
        let positional = self
            .list(MAX_SMALL_COLLECTION, Self::boolean)?
            .unwrap_or_default();
        let current = self.string()?.unwrap_or_default();
        let mut objectives = Vec::new();
        if let Some(objective_count) = self.count(MAX_SMALL_COLLECTION)? {
            for _ in 0..objective_count {
                objectives.push(LegacyObjective {
                    objective_type: self.string()?.unwrap_or_default(),
                    resource_type: self.string()?.unwrap_or_default(),
                    building_type: self.string()?.unwrap_or_default(),
                    enemy_type: self.string()?.unwrap_or_default(),
                    required_amount: nonnegative_u32(self.i32()?),
                    amount: nonnegative_u32(self.i32()?),
                });
            }
        }
        if schema >= 3 && self.boolean()? {
            let _until_start = self.f32()?;
            let _duration = self.f32()?;
            let _names = self.list(MAX_SMALL_COLLECTION, Self::string)?;
            if let Some(vote_count) = self.count(MAX_ENTITIES)? {
                for _ in 0..vote_count {
                    let _ = self.string()?;
                    let _ = self.string()?;
                }
            }
        }
        let mut unlocked: BTreeSet<String> = ids.into_iter().flatten().collect();
        for (index, is_unlocked) in positional.into_iter().enumerate() {
            if is_unlocked && unlocked.is_empty() {
                unlocked.insert(format!("legacy_index_{index}"));
            }
        }
        let active_goal = (!current.is_empty() && !objectives.is_empty()).then_some(LegacyGoal {
            technology: current,
            objectives,
        });
        Ok((available, unlocked, active_goal))
    }

    fn players(&mut self, schema: u32) -> Result<()> {
        let Some(count) = self.count(MAX_ENTITIES)? else {
            return Ok(());
        };
        for index in 0..count {
            let twitch_id = self.string()?.unwrap_or_default();
            let twitch_name = self.string()?.unwrap_or_default();
            let _twitch_user_type = self.i32()?;
            let game_user_type = legacy_user_type(self.i32()?);
            let _is_broadcaster = self.boolean()?;
            if schema >= 3 {
                let _is_user_player = self.boolean()?;
            }
            let guid = self.u32()?;
            let target_guid = self.u32()?;
            let target_pool = self.string()?.unwrap_or_default();
            let station_guid = self.u32()?;
            let station_pool = self.string()?.unwrap_or_default();
            let pet_active = self.boolean()?;
            let current_pet = legacy_pet_name(self.i32()?);
            let unlocked_pets = self
                .list(MAX_SMALL_COLLECTION, Self::i32)?
                .unwrap_or_default()
                .into_iter()
                .map(legacy_pet_name)
                .filter(|pet| pet != "none")
                .collect();
            let position = self.transform()?;
            let current_role = self.i32()?;
            let _previous_role = self.i32()?;
            let mut role_progression = BTreeMap::new();
            if let Some(role_count) = self.count(MAX_SMALL_COLLECTION)? {
                for _ in 0..role_count {
                    let role = legacy_role_name(self.i32()?);
                    let level = self.i32()?.clamp(1, i32::from(MAX_ROLE_LEVEL));
                    let experience = nonnegative_u32(self.i32()?);
                    role_progression.insert(
                        role,
                        RoleProgress {
                            level: u16::try_from(level).unwrap_or(u16::MAX),
                            experience,
                        },
                    );
                }
            }
            let mut inventory = BTreeMap::new();
            if let Some(entry_count) = self.count(MAX_SMALL_COLLECTION)? {
                for _ in 0..entry_count {
                    let resource = self.string()?.unwrap_or_else(|| "unknown".to_owned());
                    let amount = nonnegative_u32(self.i32()?);
                    let _maximum = self.i32()?;
                    let _unlimited = self.boolean()?;
                    inventory.insert(resource, amount);
                }
            }
            let customization = ActorCustomization {
                eyes: legacy_customization_index(self.i32()?),
                hair: legacy_customization_index(self.i32()?),
                facial_hair: legacy_customization_index(self.i32()?),
                ..ActorCustomization::default()
            };
            let _skin = self.i32()?;
            let customization = ActorCustomization {
                hair_color: legacy_customization_index(self.i32()?),
                eye_color: legacy_customization_index(self.i32()?),
                body_type: legacy_customization_index(self.i32()?),
                ..customization
            };
            let health = self.i32()?;
            let _regen_requires_food = self.boolean()?;
            let identity = if twitch_id.is_empty() {
                format!("{guid}:{index}")
            } else {
                twitch_id
            };
            self.entities.push(LegacyEntity {
                key: format!("player:{identity}"),
                display_name: (!twitch_name.is_empty()).then_some(twitch_name.clone()),
                login_name: (!twitch_name.is_empty()).then_some(twitch_name.to_ascii_lowercase()),
                user_type: game_user_type,
                kind: ActorKind::Player,
                archetype: if twitch_name.is_empty() {
                    "viewer".to_owned()
                } else {
                    twitch_name
                },
                position,
                rotation_quarter_turns: 0,
                health,
                role: Some(legacy_role_name(current_role)),
                level: role_progression
                    .get(&legacy_role_name(current_role))
                    .map_or(1, |progress| i32::from(progress.level)),
                inventory,
                role_progression,
                station_reference: (station_guid != 0).then_some(LegacyReference {
                    guid: station_guid,
                    pool: station_pool,
                }),
                target_reference: (target_guid != 0).then_some(LegacyReference {
                    guid: target_guid,
                    pool: target_pool,
                }),
                active_pet: pet_active
                    .then_some(current_pet)
                    .filter(|pet| pet != "none"),
                unlocked_pets,
                customization,
            });
        }
        Ok(())
    }

    fn transform(&mut self) -> Result<[f32; 3]> {
        self.transform_with_rotation().map(|(position, _)| position)
    }

    fn transform_with_rotation(&mut self) -> Result<([f32; 3], i32)> {
        let position = self.vec3()?;
        let rotation = self.vec3()?;
        let _scale = self.vec3()?;
        #[allow(clippy::cast_possible_truncation)]
        let quarter_turns = (rotation[1] / 90.0).round() as i32;
        Ok((position, quarter_turns))
    }

    fn vec3(&mut self) -> Result<[f32; 3]> {
        Ok([self.f32()?, self.f32()?, self.f32()?])
    }

    fn vec2(&mut self) -> Result<[f32; 2]> {
        Ok([self.f32()?, self.f32()?])
    }

    fn list<T>(
        &mut self,
        maximum: usize,
        mut read_item: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Option<Vec<T>>> {
        let Some(count) = self.count(maximum)? else {
            return Ok(None);
        };
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(read_item(self)?);
        }
        Ok(Some(values))
    }

    fn count(&mut self, maximum: usize) -> Result<Option<usize>> {
        let count = self.i32()?;
        if count == -1 {
            return Ok(None);
        }
        ensure!(count >= 0, "invalid negative collection length {count}");
        let count = usize::try_from(count).expect("nonnegative i32");
        ensure!(
            count <= maximum,
            "collection length {count} exceeds maximum {maximum}"
        );
        Ok(Some(count))
    }

    fn string(&mut self) -> Result<Option<String>> {
        let Some(length) = self.count(MAX_STRING_BYTES)? else {
            return Ok(None);
        };
        let mut bytes = vec![0_u8; length];
        self.reader
            .read_exact(&mut bytes)
            .context("legacy string ended unexpectedly")?;
        Ok(Some(
            String::from_utf8(bytes).context("legacy string is not valid UTF-8")?,
        ))
    }

    fn boolean(&mut self) -> Result<bool> {
        let mut bytes = [0_u8; 1];
        self.reader.read_exact(&mut bytes)?;
        Ok(bytes[0] != 0)
    }

    fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    fn f32(&mut self) -> Result<f32> {
        let value = f32::from_le_bytes(self.read_array()?);
        ensure!(value.is_finite(), "legacy save contains a non-finite float");
        Ok(value)
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut bytes = [0_u8; N];
        self.reader
            .read_exact(&mut bytes)
            .context("legacy payload ended unexpectedly")?;
        Ok(bytes)
    }
}

fn decode_json(bytes: &[u8]) -> Result<LegacyDecodedSave> {
    let root: Value = serde_json::from_slice(bytes).context("legacy JSON is invalid")?;
    let schema_version = json_u32(&root, "SchemaVersion")?;
    ensure!(
        (1..=3).contains(&schema_version),
        "unsupported legacy JSON schema {schema_version}"
    );
    let game = root
        .get("Game")
        .ok_or_else(|| anyhow!("legacy JSON is missing Game"))?;
    let world_gen = game
        .get("WorldGenData")
        .ok_or_else(|| anyhow!("legacy JSON is missing WorldGenData"))?;
    let has_seed = world_gen
        .get("HasTerrainSeed")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let terrain_seed = has_seed.then(|| json_i32_default(world_gen, "TerrainSeed"));
    let terrain_generator_version = json_i32_default(world_gen, "TerrainGeneratorVersion");
    let terrain_mesh = if has_seed {
        None
    } else {
        world_gen.get("MapMesh").map(json_mesh).transpose()?
    };
    let mut entities = Vec::new();
    json_resources(world_gen, &mut entities)?;
    json_foliage(world_gen, schema_version, &mut entities)?;
    json_enemy_camps(world_gen, &mut entities)?;
    json_buildings(game, &mut entities)?;
    json_enemies(game, &mut entities)?;
    json_players(&root, &mut entities)?;
    let world = game.get("WorldSaveData").unwrap_or(&Value::Null);
    let world_age_seconds = world
        .get("WorldAgeInSeconds")
        .and_then(Value::as_f64)
        .unwrap_or(0.0)
        .max(0.0);
    let town_resources = json_town_resources(world);
    let unlocked_technology = world
        .pointer("/TechTree/UnlockedTechIds")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let active_goal = json_active_goal(world);
    let ruler_name = world
        .get("IsCurrentRuler")
        .and_then(Value::as_bool)
        .unwrap_or(false)
        .then(|| json_string(world, "RulerName", ""))
        .filter(|name| !name.trim().is_empty());
    let ruler_vote_cooldown_seconds = json_f32_default(world, "TimeUntillNextRulerVote").max(0.0);
    Ok(LegacyDecodedSave {
        schema_version,
        container_version: None,
        terrain_seed,
        terrain_generator_version,
        terrain_mesh,
        entities,
        world_age_seconds,
        town_resources,
        unlocked_technology,
        active_goal,
        ruler_name,
        ruler_vote_cooldown_seconds,
    })
}

fn json_active_goal(world: &Value) -> Option<LegacyGoal> {
    let tree = world.get("TechTree")?;
    let technology = tree.get("CurrentTechName")?.as_str()?.to_owned();
    let objectives = tree
        .get("CurrentTechData")?
        .as_array()?
        .iter()
        .map(|objective| LegacyObjective {
            objective_type: json_string(objective, "ObjectiveType", ""),
            resource_type: json_string(objective, "ResourceType", ""),
            building_type: json_string(objective, "BuildingType", ""),
            enemy_type: json_string(objective, "EnemyType", ""),
            required_amount: nonnegative_u32(json_i32_default(objective, "RequiredAmount")),
            amount: nonnegative_u32(json_i32_default(objective, "Amount")),
        })
        .collect::<Vec<_>>();
    (!technology.is_empty() && !objectives.is_empty()).then_some(LegacyGoal {
        technology,
        objectives,
    })
}

fn json_mesh(value: &Value) -> Result<SavedTerrainMesh> {
    let vertices = value
        .get("Verticies")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(json_vec3)
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        vertices.len() <= MAX_MESH_VERTICES,
        "legacy JSON mesh is too large"
    );
    let triangle_indices = value
        .get("Triangles")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|item| {
            item.as_i64()
                .and_then(|number| i32::try_from(number).ok())
                .ok_or_else(|| anyhow!("invalid legacy triangle index"))
        })
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        triangle_indices.len() <= MAX_TRIANGLE_INDICES,
        "legacy JSON triangle array is too large"
    );
    let uvs = value
        .get("UVs")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(json_vec2)
        .collect::<Result<Vec<_>>>()?;
    Ok(SavedTerrainMesh {
        vertices,
        triangle_indices,
        uvs,
        uses_32_bit_indices: value
            .get("Uses32BitIndices")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })
}

#[allow(clippy::unnecessary_wraps)]
fn json_resources(world_gen: &Value, entities: &mut Vec<LegacyEntity>) -> Result<()> {
    let groups = world_gen
        .pointer("/Resources/Groups")
        .and_then(Value::as_array)
        .into_iter()
        .flatten();
    for (group_index, group) in groups.enumerate() {
        let archetype = json_string(group, "ResourceType", "unknown");
        for (index, instance) in group
            .get("Instances")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            entities.push(LegacyEntity {
                key: format!(
                    "resource:{}:{group_index}:{index}",
                    json_u32_default(instance, "GUID")
                ),
                display_name: None,
                login_name: None,
                user_type: StreamUserType::Normal,
                kind: ActorKind::Resource,
                archetype: archetype.clone(),
                position: [
                    json_f32_default(instance, "PositionX"),
                    json_f32_default(instance, "PositionY"),
                    json_f32_default(instance, "PositionZ"),
                ],
                rotation_quarter_turns: 0,
                health: json_i32_default(instance, "CurrentAmount"),
                role: None,
                level: 0,
                inventory: BTreeMap::new(),
                role_progression: BTreeMap::new(),
                station_reference: None,
                target_reference: None,
                active_pet: None,
                unlocked_pets: BTreeSet::new(),
                customization: ActorCustomization::default(),
            });
        }
    }
    Ok(())
}

fn json_foliage(world_gen: &Value, schema: u32, entities: &mut Vec<LegacyEntity>) -> Result<()> {
    if schema >= 2 {
        for layer in ["OnLandGroups", "UnderWaterGroups"] {
            for (group_index, group) in world_gen
                .pointer(&format!("/Foliage/{layer}"))
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .enumerate()
            {
                let settings = json_string(group, "SettingsId", "unknown");
                for (index, position) in group
                    .get("Positions")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .enumerate()
                {
                    entities.push(LegacyEntity {
                        key: format!("foliage:{layer}:{group_index}:{index}"),
                        display_name: None,
                        login_name: None,
                        user_type: StreamUserType::Normal,
                        kind: ActorKind::Foliage,
                        archetype: settings.clone(),
                        position: json_vec3(position)?,
                        rotation_quarter_turns: 0,
                        health: 1,
                        role: None,
                        level: 0,
                        inventory: BTreeMap::new(),
                        role_progression: BTreeMap::new(),
                        station_reference: None,
                        target_reference: None,
                        active_pet: None,
                        unlocked_pets: BTreeSet::new(),
                        customization: ActorCustomization::default(),
                    });
                }
            }
        }
    } else {
        for layer in ["OnLand", "UnderWater"] {
            for (index, instance) in world_gen
                .pointer(&format!("/Foliage/{layer}"))
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .enumerate()
            {
                entities.push(LegacyEntity {
                    key: format!("foliage:{layer}:{index}"),
                    display_name: None,
                    login_name: None,
                    user_type: StreamUserType::Normal,
                    kind: ActorKind::Foliage,
                    archetype: json_string(instance, "SettingsId", "unknown"),
                    position: json_transform(instance.get("Transform").unwrap_or(&Value::Null))?,
                    rotation_quarter_turns: 0,
                    health: 1,
                    role: None,
                    level: 0,
                    inventory: BTreeMap::new(),
                    role_progression: BTreeMap::new(),
                    station_reference: None,
                    target_reference: None,
                    active_pet: None,
                    unlocked_pets: BTreeSet::new(),
                    customization: ActorCustomization::default(),
                });
            }
        }
    }
    Ok(())
}

fn json_enemy_camps(world_gen: &Value, entities: &mut Vec<LegacyEntity>) -> Result<()> {
    for (index, camp) in world_gen
        .get("EnemyCamps")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        entities.push(LegacyEntity {
            key: format!("enemy_camp:{}:{index}", json_u32_default(camp, "GUID")),
            display_name: None,
            login_name: None,
            user_type: StreamUserType::Normal,
            kind: ActorKind::EnemyCamp,
            archetype: "enemy_camp".to_owned(),
            position: json_transform(camp.get("Transform").unwrap_or(&Value::Null))?,
            rotation_quarter_turns: 0,
            health: json_i32_default(camp, "Health"),
            role: None,
            level: 0,
            inventory: BTreeMap::new(),
            role_progression: BTreeMap::new(),
            station_reference: None,
            target_reference: None,
            active_pet: None,
            unlocked_pets: BTreeSet::new(),
            customization: ActorCustomization::default(),
        });
    }
    Ok(())
}

fn json_buildings(game: &Value, entities: &mut Vec<LegacyEntity>) -> Result<()> {
    for (index, building) in game
        .get("BuildingSaveData")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        entities.push(LegacyEntity {
            key: format!("building:{}:{index}", json_u32_default(building, "GUID")),
            display_name: None,
            login_name: None,
            user_type: StreamUserType::Normal,
            kind: ActorKind::Building,
            archetype: json_string(building, "BuildingType", "unknown"),
            position: json_transform(building.get("BuildingTranform").unwrap_or(&Value::Null))?,
            rotation_quarter_turns: json_rotation_quarter_turns(
                building.get("BuildingTranform").unwrap_or(&Value::Null),
            ),
            health: json_i32_default(building, "BuildingHealth"),
            role: None,
            level: json_i32_default(building, "Level"),
            inventory: BTreeMap::new(),
            role_progression: BTreeMap::new(),
            station_reference: None,
            target_reference: None,
            active_pet: None,
            unlocked_pets: BTreeSet::new(),
            customization: ActorCustomization::default(),
        });
    }
    Ok(())
}

fn json_enemies(game: &Value, entities: &mut Vec<LegacyEntity>) -> Result<()> {
    for (index, enemy) in game
        .get("EnemySaveData")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        entities.push(LegacyEntity {
            key: format!("enemy:{}:{index}", json_u32_default(enemy, "GUID")),
            display_name: None,
            login_name: None,
            user_type: StreamUserType::Normal,
            kind: ActorKind::Enemy,
            archetype: json_string(enemy, "EnemyType", "unknown"),
            position: json_transform(enemy.get("Transform").unwrap_or(&Value::Null))?,
            rotation_quarter_turns: 0,
            health: json_i32_default(enemy, "Health"),
            role: Some("enemy".to_owned()),
            level: 0,
            inventory: BTreeMap::new(),
            role_progression: BTreeMap::new(),
            station_reference: None,
            target_reference: {
                let guid = json_u32_default(enemy, "TargetGUID");
                (guid != 0).then(|| LegacyReference {
                    guid,
                    pool: json_string(enemy, "TargetPoolType", "Player"),
                })
            },
            active_pet: None,
            unlocked_pets: BTreeSet::new(),
            customization: ActorCustomization::default(),
        });
    }
    Ok(())
}

fn json_players(root: &Value, entities: &mut Vec<LegacyEntity>) -> Result<()> {
    for (index, player) in root
        .pointer("/Players/PlayerSaveDatas")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let twitch_id = json_string(player, "TwitchID", "");
        let key = if twitch_id.is_empty() {
            format!("{}:{index}", json_u32_default(player, "GUID"))
        } else {
            twitch_id
        };
        let inventory = player
            .pointer("/Inventory/Entries")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|entry| {
                (
                    json_string(entry, "ResourceType", "unknown"),
                    nonnegative_u32(json_i32_default(entry, "Amount")),
                )
            })
            .collect();
        let role = player.get("CurrentRole").map(json_role_name);
        let role_progression: BTreeMap<_, _> = player
            .get("Roles")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|saved_role| {
                let role = saved_role
                    .get("Role")
                    .map_or_else(|| "legacy_0".to_owned(), json_role_name);
                let level =
                    json_i32_default(saved_role, "Level").clamp(1, i32::from(MAX_ROLE_LEVEL));
                let experience = nonnegative_u32(json_i32_default(saved_role, "Experience"));
                (
                    role,
                    RoleProgress {
                        level: u16::try_from(level).unwrap_or(u16::MAX),
                        experience,
                    },
                )
            })
            .collect();
        let level = role
            .as_ref()
            .and_then(|role| role_progression.get(role))
            .map_or(1, |progress| i32::from(progress.level));
        entities.push(LegacyEntity {
            key: format!("player:{key}"),
            display_name: Some(json_string(player, "TwitchName", "viewer")),
            login_name: Some(json_string(player, "TwitchName", "viewer").to_ascii_lowercase()),
            user_type: json_user_type(player),
            kind: ActorKind::Player,
            archetype: json_string(player, "TwitchName", "viewer"),
            position: json_transform(player.get("Transform").unwrap_or(&Value::Null))?,
            rotation_quarter_turns: 0,
            health: json_i32_default(player, "Health"),
            role,
            level,
            inventory,
            role_progression,
            station_reference: {
                let guid = json_u32_default(player, "StationGUID");
                (guid != 0).then(|| LegacyReference {
                    guid,
                    pool: json_string(player, "StationPoolType", "Building"),
                })
            },
            target_reference: {
                let guid = json_u32_default(player, "TargetGUID");
                (guid != 0).then(|| LegacyReference {
                    guid,
                    pool: json_string(player, "TargetPoolType", "Target"),
                })
            },
            active_pet: player
                .get("PetActive")
                .and_then(Value::as_bool)
                .unwrap_or(false)
                .then(|| {
                    player
                        .get("CurrentPet")
                        .map_or_else(|| "none".to_owned(), json_pet_name)
                })
                .filter(|pet| pet != "none"),
            unlocked_pets: player
                .get("UnlockedPets")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(json_pet_name)
                .filter(|pet| pet != "none")
                .collect(),
            customization: player
                .get("Customization")
                .map_or_else(ActorCustomization::default, json_customization),
        });
    }
    Ok(())
}

fn json_town_resources(world: &Value) -> BTreeMap<String, u32> {
    let mut resources: BTreeMap<String, u32> = world
        .get("TownResources")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|entry| {
            (
                json_string(entry, "ResourceType", "unknown"),
                nonnegative_u32(json_i32_default(entry, "Amount")),
            )
        })
        .collect();
    for (name, field) in [
        ("wood", "WoodResourceAmount"),
        ("ore", "OreResourceAmount"),
        ("food", "FoodResourceAmount"),
        ("gold", "GoldResourceAmount"),
    ] {
        resources
            .entry(name.to_owned())
            .or_insert_with(|| nonnegative_u32(json_i32_default(world, field)));
    }
    resources
}

fn json_transform(value: &Value) -> Result<[f32; 3]> {
    json_vec3(value.get("Position").unwrap_or(&Value::Null))
}

#[allow(clippy::cast_possible_truncation)]
fn json_rotation_quarter_turns(value: &Value) -> i32 {
    (json_f32_default(value.get("Rotation").unwrap_or(&Value::Null), "Y") / 90.0).round() as i32
}

#[allow(clippy::unnecessary_wraps)]
fn json_vec3(value: &Value) -> Result<[f32; 3]> {
    Ok([
        json_f32_default(value, "X"),
        json_f32_default(value, "Y"),
        json_f32_default(value, "Z"),
    ])
}

#[allow(clippy::unnecessary_wraps)]
fn json_vec2(value: &Value) -> Result<[f32; 2]> {
    Ok([json_f32_default(value, "X"), json_f32_default(value, "Y")])
}

fn json_u32(value: &Value, field: &str) -> Result<u32> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|number| u32::try_from(number).ok())
        .ok_or_else(|| anyhow!("legacy JSON field {field} is missing or invalid"))
}

fn json_u32_default(value: &Value, field: &str) -> u32 {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|number| u32::try_from(number).ok())
        .unwrap_or_default()
}

fn json_i32_default(value: &Value, field: &str) -> i32 {
    value
        .get(field)
        .and_then(Value::as_i64)
        .and_then(|number| i32::try_from(number).ok())
        .unwrap_or_default()
}

fn legacy_user_type(value: i32) -> StreamUserType {
    match value {
        0 => StreamUserType::GameMaster,
        1 => StreamUserType::Broadcaster,
        2 => StreamUserType::Moderator,
        3 => StreamUserType::Subscriber,
        _ => StreamUserType::Normal,
    }
}

fn json_user_type(player: &Value) -> StreamUserType {
    player
        .get("GameUserType")
        .and_then(|value| {
            value
                .as_i64()
                .and_then(|number| i32::try_from(number).ok())
                .map(legacy_user_type)
                .or_else(|| {
                    value.as_str().map(|name| match name {
                        "GameMaster" => StreamUserType::GameMaster,
                        "Broadcaster" => StreamUserType::Broadcaster,
                        "Moderator" => StreamUserType::Moderator,
                        "Subscriber" => StreamUserType::Subscriber,
                        _ => StreamUserType::Normal,
                    })
                })
        })
        .unwrap_or(StreamUserType::Normal)
}

#[allow(clippy::cast_possible_truncation)]
fn json_f32_default(value: &Value, field: &str) -> f32 {
    value
        .get(field)
        .and_then(Value::as_f64)
        .filter(|number| {
            number.is_finite() && *number >= f64::from(f32::MIN) && *number <= f64::from(f32::MAX)
        })
        .map(|number| number as f32)
        .unwrap_or_default()
}

fn json_string(value: &Value, field: &str, fallback: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_owned()
}

fn json_role_name(value: &Value) -> String {
    value.as_str().map_or_else(
        || legacy_role_name(i32::try_from(value.as_i64().unwrap_or_default()).unwrap_or_default()),
        str::to_ascii_lowercase,
    )
}

fn nonnegative_u32(value: i32) -> u32 {
    u32::try_from(value.max(0)).expect("nonnegative i32")
}

fn legacy_role_name(value: i32) -> String {
    const ROLES: [&str; 15] = [
        "logger",
        "builder",
        "defender",
        "fisher",
        "ruler",
        "miner",
        "ranger",
        "farmer",
        "soldier",
        "gatherer",
        "priest",
        "wizard",
        "necromancer",
        "paladin",
        "blacksmith",
    ];
    usize::try_from(value)
        .ok()
        .and_then(|index| ROLES.get(index))
        .map_or_else(|| format!("legacy_{value}"), |role| (*role).to_owned())
}

fn legacy_pet_name(value: i32) -> String {
    const PETS: [&str; 6] = [
        "none",
        "red_panda",
        "fish_god",
        "giraffe",
        "duck",
        "butterfly",
    ];
    usize::try_from(value)
        .ok()
        .and_then(|index| PETS.get(index))
        .map_or_else(|| format!("legacy_{value}"), |pet| (*pet).to_owned())
}

fn json_pet_name(value: &Value) -> String {
    value.as_str().map_or_else(
        || legacy_pet_name(i32::try_from(value.as_i64().unwrap_or_default()).unwrap_or_default()),
        slug,
    )
}

fn legacy_customization_index(value: i32) -> u8 {
    u8::try_from(value.max(0)).unwrap_or(u8::MAX)
}

fn json_customization(value: &Value) -> ActorCustomization {
    ActorCustomization {
        eyes: legacy_customization_index(json_i32_default(value, "ChosenEyeIndex")),
        hair: legacy_customization_index(json_i32_default(value, "ChosenHairIndex")),
        facial_hair: legacy_customization_index(json_i32_default(value, "ChosenFacialHairIndex")),
        hair_color: legacy_customization_index(json_i32_default(value, "ChosenHairColourIndex")),
        eye_color: legacy_customization_index(json_i32_default(value, "ChosenEyeColourIndex")),
        body_type: legacy_customization_index(json_i32_default(value, "ChosenBodyTypeIndex")),
    }
}

fn slug(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && !output.is_empty() && !output.ends_with('_') {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else if !output.is_empty() && !output.ends_with('_') {
            output.push('_');
        }
    }
    output.trim_matches('_').to_owned()
}

fn convert(decoded: LegacyDecodedSave, config: &GameConfig) -> Result<(WorldSnapshot, u32)> {
    let content: ContentCatalog = ron::from_str(include_str!(
        "../../../assets/content/catalog.ron"
    ))
    .context("checked-in content catalog could not be parsed for legacy reference mapping")?;
    content
        .validate()
        .context("checked-in content catalog is invalid")?;
    if let Some(mesh) = &decoded.terrain_mesh {
        mesh.validate().context("legacy terrain mesh is invalid")?;
    }
    let mut world_config = config.world.clone();
    if let Some(seed) = decoded.terrain_seed {
        world_config.seed = u64::from(u32::from_ne_bytes(seed.to_ne_bytes()));
    }
    let generated = generate_world(&world_config);
    let mut simulation = WorldSimulation::new(generated.seed);
    simulation.elapsed_seconds = decoded.world_age_seconds;
    simulation.day = duration_days(decoded.world_age_seconds);
    for (kind, amount) in &decoded.town_resources {
        let id = content_id("resource", kind)?;
        simulation.town_resources.insert(id, *amount);
    }
    for technology in &decoded.unlocked_technology {
        simulation
            .unlocked_technology
            .insert(resolve_legacy_technology(&content, technology)?);
    }
    if let Some(goal) = &decoded.active_goal
        && let Some(restored) = restore_legacy_goal(&content, goal)
    {
        simulation.active_goals.push(restored);
    }

    let mut actors = Vec::with_capacity(decoded.entities.len());
    let building_ids: BTreeMap<u32, StableId> = decoded
        .entities
        .iter()
        .filter(|entity| matches!(entity.kind, ActorKind::Building))
        .filter_map(|entity| {
            let guid = entity
                .key
                .strip_prefix("building:")?
                .split(':')
                .next()?
                .parse()
                .ok()?;
            Some((
                guid,
                StableId::new(entity_id(&entity.kind, &entity.key)).ok()?,
            ))
        })
        .collect();
    let mut ids = BTreeMap::<String, u32>::new();
    let mut relocated_entities = 0_u32;
    for entity in decoded.entities {
        let base = entity_id(&entity.kind, &entity.key);
        let duplicate = ids.entry(base.clone()).or_default();
        let id_text = if *duplicate == 0 {
            base
        } else {
            format!("{base}:{}", *duplicate)
        };
        *duplicate += 1;
        let id = StableId::new(id_text)?;
        let (position, relocated) = snap_position(entity.position, config, &generated);
        relocated_entities = relocated_entities.saturating_add(u32::from(relocated));
        let archetype = resolve_legacy_archetype(&content, &entity.kind, &entity.archetype)?;
        let height = generated.navigation.height_at(position).unwrap_or_default();
        actors.push(SavedActor {
            id: id.clone(),
            kind: entity.kind.clone(),
            archetype: archetype.clone(),
            grid_position: position,
            height_centimetres: height,
            health: entity.health,
        });
        match entity.kind {
            ActorKind::Player | ActorKind::Enemy => {
                let role = entity.role.as_deref().map_or_else(
                    || content_id("role", "villager"),
                    |role| content_id("role", role),
                )?;
                let inventory = entity
                    .inventory
                    .into_iter()
                    .map(|(kind, amount)| Ok((content_id("resource", &kind)?, amount)))
                    .collect::<Result<BTreeMap<_, _>>>()?;
                let mut role_progression = entity
                    .role_progression
                    .into_iter()
                    .map(|(role, progress)| Ok((content_id("role", &role)?, progress)))
                    .collect::<Result<BTreeMap<_, _>>>()?;
                role_progression
                    .entry(role.clone())
                    .or_insert(RoleProgress {
                        level: u16::try_from(entity.level.clamp(1, i32::from(MAX_ROLE_LEVEL)))
                            .unwrap_or(1),
                        experience: 0,
                    });
                simulation.actors.insert(
                    id.clone(),
                    ActorState {
                        id,
                        display_name: entity.display_name,
                        login_name: entity.login_name,
                        user_type: entity.user_type,
                        role,
                        archetype: Some(archetype),
                        position,
                        last_building_position: None,
                        building_rotation_quarter_turns: 0,
                        health: entity.health,
                        max_health: entity.health.max(1),
                        alive: entity.health > 0,
                        respawn_remaining_seconds: None,
                        inventory,
                        station: entity.station_reference.as_ref().map(|reference| {
                            building_ids
                                .get(&reference.guid)
                                .cloned()
                                .unwrap_or_else(|| {
                                    content_id(
                                        "legacy_station",
                                        &format!("{}_{}", reference.pool, reference.guid),
                                    )
                                    .expect("sanitized legacy station reference is valid")
                                })
                        }),
                        preferred_target: entity.target_reference.as_ref().map(|reference| {
                            building_ids
                                .get(&reference.guid)
                                .cloned()
                                .unwrap_or_else(|| {
                                    content_id(
                                        "legacy_target",
                                        &format!("{}_{}", reference.pool, reference.guid),
                                    )
                                    .expect("sanitized legacy target reference is valid")
                                })
                        }),
                        role_progression,
                        unlocked_pets: entity
                            .unlocked_pets
                            .into_iter()
                            .map(|pet| content_id("pet", &pet))
                            .collect::<Result<BTreeSet<_>>>()?,
                        active_pet: entity
                            .active_pet
                            .as_deref()
                            .map(|pet| content_id("pet", pet))
                            .transpose()?,
                        customization: entity.customization,
                    },
                );
            }
            ActorKind::Building => {
                simulation.buildings.insert(
                    id.clone(),
                    BuildingState {
                        id,
                        archetype,
                        position,
                        rotation_quarter_turns: entity.rotation_quarter_turns,
                        level: u16::try_from(entity.level.max(0)).unwrap_or(u16::MAX),
                        health: entity.health,
                        complete: true,
                    },
                );
            }
            ActorKind::EnemyCamp => {
                simulation.enemy_camps.insert(
                    id.clone(),
                    EnemyCampState {
                        id,
                        archetype,
                        position,
                        health: entity.health,
                        spawn_remaining_seconds: 0.0,
                        spawned_enemies: BTreeSet::new(),
                    },
                );
            }
            _ => {}
        }
    }

    simulation.ruler_vote_cooldown_seconds = RULER_VOTE_INTERVAL_SECONDS;
    simulation.ruler_vote_scheduled = true;
    if let Some(ruler_name) = decoded.ruler_name.as_deref()
        && let Some(ruler) = simulation
            .actors
            .values()
            .find(|actor| {
                actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(ruler_name))
            })
            .map(|actor| actor.id.clone())
    {
        simulation.set_ruler(ruler)?;
        simulation.ruler_vote_cooldown_seconds = decoded.ruler_vote_cooldown_seconds;
    }

    let source_schema_version = decoded.schema_version;
    let source_container_version = decoded.container_version;
    let elapsed_seconds = Duration::from_secs_f64(decoded.world_age_seconds.max(0.0)).as_secs();
    Ok((
        WorldSnapshot {
            schema_version: 1,
            world_seed: generated.seed,
            generator_version: generated.generator_version,
            world_hash: generated.deterministic_hash,
            elapsed_seconds,
            actors,
            simulation,
            resource_nodes: BTreeMap::new(),
            legacy_terrain_mesh: decoded.terrain_mesh,
            legacy_migration: Some(LegacyMigrationMetadata {
                source_schema_version,
                source_container_version,
                source_terrain_generator_version: decoded.terrain_generator_version,
                source_sha256: String::new(),
                recovered_from_backup: false,
                relocated_entities,
            }),
        },
        relocated_entities,
    ))
}

fn duration_days(seconds: f64) -> u32 {
    let days = Duration::from_secs_f64(seconds.max(0.0)).as_secs() / 86_400;
    u32::try_from(days).unwrap_or(u32::MAX)
}

fn actor_prefix(kind: &ActorKind) -> &'static str {
    match kind {
        ActorKind::Player => "archetype",
        ActorKind::Enemy => "enemy",
        ActorKind::Building => "building",
        ActorKind::Resource => "resource",
        ActorKind::Foliage => "foliage",
        ActorKind::EnemyCamp => "enemy_camp",
    }
}

fn resolve_legacy_archetype(
    content: &ContentCatalog,
    kind: &ActorKind,
    value: &str,
) -> Result<StableId> {
    let requested = sanitize_component(value, 96);
    match kind {
        ActorKind::Enemy => content
            .archetypes
            .iter()
            .find(|(_, archetype)| {
                archetype.enemy.as_ref().is_some_and(|enemy| {
                    enemy.pool.as_str().trim_start_matches("pool:") == requested
                        || enemy.enemy_type.as_str().trim_start_matches("enemy:") == requested
                })
            })
            .map(|(id, _)| id.clone())
            .map_or_else(|| content_id(actor_prefix(kind), value), Ok),
        ActorKind::EnemyCamp => content
            .archetypes
            .iter()
            .find(|(_, archetype)| archetype.enemy_spawner.is_some())
            .map(|(id, _)| id.clone())
            .map_or_else(|| content_id(actor_prefix(kind), value), Ok),
        _ => content_id(actor_prefix(kind), value),
    }
}

fn entity_id(kind: &ActorKind, key: &str) -> String {
    let prefix = match kind {
        ActorKind::Player => "actor",
        ActorKind::Enemy => "enemy",
        ActorKind::Building => "building",
        ActorKind::Resource => "resource_instance",
        ActorKind::Foliage => "foliage_instance",
        ActorKind::EnemyCamp => "enemy_camp",
    };
    let component = sanitize_component(key, 96_usize.saturating_sub(prefix.len()));
    format!("legacy:{prefix}:{component}")
}

fn content_id(prefix: &str, value: &str) -> Result<StableId> {
    let component = sanitize_component(value, 120_usize.saturating_sub(prefix.len()));
    StableId::new(format!("{prefix}:{component}")).map_err(Into::into)
}

fn resolve_legacy_technology(content: &ContentCatalog, name: &str) -> Result<StableId> {
    content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| technology.display_name.eq_ignore_ascii_case(name))
        .map_or_else(|| content_id("technology", name), |(id, _)| Ok(id.clone()))
}

fn restore_legacy_goal(content: &ContentCatalog, legacy: &LegacyGoal) -> Option<TownGoalState> {
    let (technology, node) = content
        .technology
        .nodes
        .iter()
        .find(|(_, node)| node.display_name.eq_ignore_ascii_case(&legacy.technology))?;
    let mut unmatched = node.objectives.clone();
    let mut objectives = Vec::new();
    for saved in &legacy.objectives {
        let index = unmatched.iter().position(|objective| {
            content
                .objectives
                .get(objective)
                .is_some_and(|definition| legacy_objective_matches(definition, saved))
        })?;
        let objective = unmatched.remove(index);
        objectives.push(ObjectiveProgress {
            objective,
            amount: saved.amount.min(saved.required_amount),
            required_amount: saved.required_amount.max(1),
        });
    }
    (!objectives.is_empty()).then_some(TownGoalState {
        technology: technology.clone(),
        objectives,
    })
}

fn legacy_objective_matches(definition: &ObjectiveDef, legacy: &LegacyObjective) -> bool {
    let kind = match legacy
        .objective_type
        .to_ascii_lowercase()
        .replace(' ', "_")
        .as_str()
    {
        "build" => ObjectiveKind::Build,
        "buildany" | "build_any" => ObjectiveKind::BuildAny,
        "collect" => ObjectiveKind::Collect,
        "kill" => ObjectiveKind::Kill,
        "killany" | "kill_any" => ObjectiveKind::KillAny,
        "earnperhour" | "earn_per_hour" => ObjectiveKind::EarnPerHour,
        "sell" => ObjectiveKind::Sell,
        "sellany" | "sell_any" => ObjectiveKind::SellAny,
        "buy" => ObjectiveKind::Buy,
        "buyany" | "buy_any" => ObjectiveKind::BuyAny,
        _ => return false,
    };
    definition.kind == kind
        && objective_target_matches(
            definition.resource.as_ref(),
            "resource",
            &legacy.resource_type,
        )
        && objective_target_matches(
            definition.building.as_ref(),
            "building",
            &legacy.building_type,
        )
        && objective_target_matches(definition.enemy.as_ref(), "enemy", &legacy.enemy_type)
}

fn objective_target_matches(target: Option<&StableId>, prefix: &str, legacy: &str) -> bool {
    target.is_none_or(|target| {
        content_id(prefix, legacy).is_ok_and(|legacy_target| legacy_target == *target)
    })
}

fn sanitize_component(value: &str, maximum: usize) -> String {
    let mut output = String::with_capacity(value.len().min(maximum));
    let mut previous_separator = false;
    for character in value.chars() {
        if output.len() >= maximum {
            break;
        }
        let normalized = character.to_ascii_lowercase();
        if normalized.is_ascii_alphanumeric() || ".-".contains(normalized) {
            output.push(normalized);
            previous_separator = false;
        } else if !previous_separator && !output.is_empty() {
            output.push('_');
            previous_separator = true;
        }
    }
    while output.ends_with('_') {
        output.pop();
    }
    if output.is_empty() {
        "unknown".to_owned()
    } else {
        output
    }
}

fn snap_position(
    unity: [f32; 3],
    config: &GameConfig,
    world: &stream_town_domain::GeneratedWorld,
) -> (GridPos, bool) {
    let raw_x = unity[0] / config.world.cell_size + f32::from(config.world.width) * 0.5;
    let raw_z = unity[2] / config.world.cell_size + f32::from(config.world.height) * 0.5;
    let desired = GridPos {
        x: clamped_cell(raw_x, config.world.width),
        z: clamped_cell(raw_z, config.world.height),
    };
    let was_clamped = !raw_x.is_finite()
        || !raw_z.is_finite()
        || raw_x < 0.0
        || raw_z < 0.0
        || raw_x >= f32::from(config.world.width)
        || raw_z >= f32::from(config.world.height);
    if world.navigation.is_walkable(desired) {
        return (desired, was_clamped);
    }
    (
        nearest_walkable(world, desired).unwrap_or(GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        }),
        true,
    )
}

fn clamped_cell(value: f32, size: u16) -> u16 {
    if !value.is_finite() {
        return size / 2;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let rounded = value.round().clamp(0.0, f32::from(size - 1)) as u16;
    rounded
}

fn nearest_walkable(
    world: &stream_town_domain::GeneratedWorld,
    desired: GridPos,
) -> Option<GridPos> {
    let limit = world.navigation.width().max(world.navigation.height());
    for radius in 1..limit {
        let min_z = desired.z.saturating_sub(radius);
        let max_z = desired
            .z
            .saturating_add(radius)
            .min(world.navigation.height() - 1);
        let min_x = desired.x.saturating_sub(radius);
        let max_x = desired
            .x
            .saturating_add(radius)
            .min(world.navigation.width() - 1);
        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let candidate = GridPos { x, z };
                if world.navigation.is_walkable(candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::{Compression, write::GzEncoder};
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn binary_schemas_one_through_three_decode_and_validate_trailer() {
        for schema in 1..=3 {
            let fixture = binary_fixture(schema, PAYLOAD_TRAILER);
            let decoded = decode_legacy(&fixture).unwrap();
            assert_eq!(decoded.schema_version, schema);
            assert_eq!(decoded.container_version, Some(1));
            assert_eq!(decoded.terrain_seed, (schema >= 2).then_some(42));
            assert_eq!(decoded.terrain_mesh.is_some(), schema == 1);
            if schema == 1 {
                let mesh = decoded.terrain_mesh.unwrap();
                assert_eq!(mesh.vertices.len(), 3);
                assert_eq!(mesh.triangle_indices, vec![0, 1, 2]);
                mesh.validate().unwrap();
            }
        }
        assert!(decode_legacy(&binary_fixture(3, 0)).is_err());
    }

    #[test]
    fn conversion_preserves_mesh_and_relocates_invalid_positions() {
        let decoded = LegacyDecodedSave {
            schema_version: 1,
            container_version: Some(1),
            terrain_seed: None,
            terrain_generator_version: 0,
            terrain_mesh: Some(SavedTerrainMesh {
                vertices: vec![[-1.0, 0.0, -1.0], [1.0, 0.0, -1.0], [0.0, 1.0, 1.0]],
                triangle_indices: vec![0, 1, 2],
                uvs: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]],
                uses_32_bit_indices: false,
            }),
            entities: vec![LegacyEntity {
                key: "player:test".to_owned(),
                display_name: Some("test".to_owned()),
                login_name: Some("test".to_owned()),
                user_type: StreamUserType::Normal,
                kind: ActorKind::Player,
                archetype: "viewer".to_owned(),
                position: [f32::MAX, 0.0, f32::MAX],
                rotation_quarter_turns: 0,
                health: 100,
                role: Some("villager".to_owned()),
                level: 0,
                inventory: BTreeMap::new(),
                role_progression: BTreeMap::new(),
                station_reference: None,
                target_reference: None,
                active_pet: None,
                unlocked_pets: BTreeSet::new(),
                customization: ActorCustomization::default(),
            }],
            world_age_seconds: 12.0,
            town_resources: BTreeMap::new(),
            unlocked_technology: BTreeSet::new(),
            active_goal: None,
            ruler_name: None,
            ruler_vote_cooldown_seconds: 0.0,
        };
        let (snapshot, relocated) = convert(decoded, &GameConfig::default()).unwrap();
        assert_eq!(relocated, 1);
        assert!(snapshot.legacy_terrain_mesh.is_some());
        assert_eq!(snapshot.actors.len(), 1);
    }

    #[test]
    fn conversion_rejects_malformed_retained_mesh() {
        let decoded = LegacyDecodedSave {
            schema_version: 1,
            container_version: Some(1),
            terrain_seed: None,
            terrain_generator_version: 0,
            terrain_mesh: Some(SavedTerrainMesh {
                vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
                triangle_indices: vec![0, -1, 2],
                uvs: Vec::new(),
                uses_32_bit_indices: false,
            }),
            entities: Vec::new(),
            world_age_seconds: 0.0,
            town_resources: BTreeMap::new(),
            unlocked_technology: BTreeSet::new(),
            active_goal: None,
            ruler_name: None,
            ruler_vote_cooldown_seconds: 0.0,
        };
        assert!(
            convert(decoded, &GameConfig::default())
                .unwrap_err()
                .to_string()
                .contains("legacy terrain mesh is invalid")
        );
    }

    #[test]
    fn checked_in_schema_one_fixture_imports_retained_terrain() {
        let decoded = decode_legacy(include_bytes!(
            "../../../tests/fixtures/legacy-schema-1.json"
        ))
        .unwrap();
        let (snapshot, relocated) = convert(decoded, &GameConfig::default()).unwrap();
        let mesh = snapshot.legacy_terrain_mesh.unwrap();
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.triangle_indices.len(), 6);
        assert_eq!(relocated, 0);
    }

    #[test]
    fn import_preserves_source_and_recovers_named_backup() {
        let directory = tempdir().unwrap();
        let source = directory.path().join("StreamTownSave.stsave");
        let backup = directory.path().join("StreamTownSave.backup.stsave");
        let destination = directory.path().join("native.stbevy");
        fs::write(&source, b"corrupt").unwrap();
        let backup_bytes = binary_fixture(3, PAYLOAD_TRAILER);
        fs::write(&backup, &backup_bytes).unwrap();

        let report = import_save(&source, &destination, &GameConfig::default()).unwrap();
        assert!(report.recovered_from_backup);
        assert_eq!(fs::read(&source).unwrap(), b"corrupt");
        assert_eq!(fs::read(&backup).unwrap(), backup_bytes);
        let native = NativeSaveStore::new(destination).load().unwrap();
        let metadata = native.legacy_migration.unwrap();
        assert_eq!(metadata.source_schema_version, 3);
        assert!(metadata.recovered_from_backup);
        assert!(!metadata.source_sha256.is_empty());
    }

    #[test]
    fn legacy_json_is_decoded_without_type_metadata() {
        let fixture = serde_json::json!({
            "SchemaVersion": 3,
            "SavedAtUtc": "2026-08-12T00:00:00Z",
            "Game": {
                "WorldGenData": {
                    "HasTerrainSeed": true,
                    "TerrainSeed": 77,
                    "TerrainGeneratorVersion": 1,
                    "Resources": { "Groups": [] },
                    "Foliage": { "OnLandGroups": [], "UnderWaterGroups": [] },
                    "EnemyCamps": []
                },
                "BuildingSaveData": [{
                    "GUID": 42,
                    "BuildingType": "Lumbermill",
                    "BuildingTranform": {
                        "Position": { "X": 1.0, "Y": 2.0, "Z": 1.0 },
                        "Rotation": { "X": 0.0, "Y": 270.0, "Z": 0.0 }
                    },
                    "BuildingHealth": 500,
                    "Level": 1
                }],
                "EnemySaveData": [],
                "WorldSaveData": {
                    "WorldAgeInSeconds": 99.0,
                    "IsCurrentRuler": true,
                    "TimeUntillNextRulerVote": 88.0,
                    "RulerName": "Viewer",
                    "TechTree": {
                        "UnlockedTechIds": ["Forestry"],
                        "CurrentTechName": "Upgrade1Logger",
                        "CurrentTechData": [{
                            "ObjectiveType": "Collect",
                            "ResourceType": "Wood",
                            "BuildingType": "Townhall",
                            "EnemyType": "Goblin",
                            "RequiredAmount": 10,
                            "Amount": 4
                        }]
                    },
                    "TownResources": [{ "ResourceType": "Wood", "Amount": 12 }]
                }
            },
            "Players": {
                "PlayerSaveDatas": [{
                    "TwitchID": "viewer-1",
                    "TwitchName": "Viewer",
                    "GUID": 9,
                    "Transform": { "Position": { "X": 0.0, "Y": 2.0, "Z": 0.0 } },
                    "CurrentRole": "Builder",
                    "StationGUID": 42,
                    "StationPoolType": "Building",
                    "TargetGUID": 43,
                    "TargetPoolType": "Resource",
                    "PetActive": true,
                    "CurrentPet": "Duck",
                    "UnlockedPets": ["Duck", "Butterfly"],
                    "Customization": {
                        "ChosenEyeIndex": 4,
                        "ChosenHairIndex": 3,
                        "ChosenFacialHairIndex": 2,
                        "ChosenHairColourIndex": 5,
                        "ChosenEyeColourIndex": 1,
                        "ChosenBodyTypeIndex": 2
                    },
                    "Roles": [{ "Role": "Builder", "Level": 7, "Experience": 123 }],
                    "Inventory": { "Entries": [] },
                    "Health": 80
                }]
            }
        });
        let decoded = decode_legacy(fixture.to_string().as_bytes()).unwrap();
        assert_eq!(decoded.schema_version, 3);
        assert_eq!(decoded.terrain_seed, Some(77));
        assert_eq!(decoded.entities.len(), 2);
        assert!(decoded.unlocked_technology.contains("Forestry"));
        assert_eq!(
            decoded.active_goal.as_ref().unwrap().objectives[0].amount,
            4
        );
        assert_eq!(
            decoded
                .entities
                .iter()
                .find(|entity| matches!(entity.kind, ActorKind::Player))
                .unwrap()
                .role_progression["builder"],
            RoleProgress {
                level: 7,
                experience: 123,
            }
        );
        let (snapshot, _) = convert(decoded, &GameConfig::default()).unwrap();
        let actor = snapshot
            .simulation
            .actors
            .values()
            .find(|actor| actor.id.as_str().contains("player"))
            .unwrap();
        assert_eq!(actor.role.as_str(), "role:ruler");
        assert_eq!(
            actor.role_progression[&StableId::new("role:builder").unwrap()],
            RoleProgress {
                level: 7,
                experience: 123,
            }
        );
        let station = actor.station.as_ref().unwrap();
        assert!(snapshot.simulation.buildings.contains_key(station));
        assert_eq!(
            snapshot.simulation.buildings[station].rotation_quarter_turns,
            3
        );
        assert_eq!(
            actor.active_pet.as_ref().map(StableId::as_str),
            Some("pet:duck")
        );
        assert!(
            actor
                .unlocked_pets
                .contains(&StableId::new("pet:butterfly").unwrap())
        );
        assert_eq!(actor.customization.eyes, 4);
        assert_eq!(actor.customization.hair, 3);
        assert_eq!(actor.customization.facial_hair, 2);
        assert_eq!(actor.customization.hair_color, 5);
        assert_eq!(actor.customization.eye_color, 1);
        assert_eq!(actor.customization.body_type, 2);
        assert_eq!(actor.user_type, StreamUserType::Normal);
        assert!(
            actor
                .preferred_target
                .as_ref()
                .is_some_and(|id| id.as_str().starts_with("legacy_target:"))
        );
        assert_eq!(snapshot.simulation.active_goals.len(), 1);
        assert_eq!(snapshot.simulation.active_goals[0].objectives[0].amount, 4);
        assert_eq!(
            snapshot.simulation.active_goals[0].objectives[0].required_amount,
            10
        );
        assert_eq!(snapshot.simulation.current_ruler.as_ref(), Some(&actor.id));
        assert!((snapshot.simulation.ruler_vote_cooldown_seconds - 88.0).abs() <= f32::EPSILON);
        assert_eq!(
            snapshot
                .simulation
                .ruler_previous_role
                .as_ref()
                .map(StableId::as_str),
            Some("role:builder")
        );
    }

    fn binary_fixture(schema: u32, trailer: i32) -> Vec<u8> {
        let mut payload = Vec::new();
        put_u32(&mut payload, schema);
        put_string(&mut payload, Some("2026-08-12T00:00:00Z"));
        if schema >= 2 {
            payload.push(1);
            put_i32(&mut payload, 42);
            put_i32(&mut payload, 1);
        } else {
            put_i32(&mut payload, 3);
            for value in [-1.0, 0.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 1.0] {
                put_f32(&mut payload, value);
            }
            put_i32(&mut payload, 3);
            for index in [0, 1, 2] {
                put_i32(&mut payload, index);
            }
            put_i32(&mut payload, 3);
            for value in [0.0, 0.0, 1.0, 0.0, 0.5, 1.0] {
                put_f32(&mut payload, value);
            }
            payload.push(0);
        }
        put_i32(&mut payload, -1); // resources
        put_i32(&mut payload, -1); // foliage land
        put_i32(&mut payload, -1); // foliage water
        put_i32(&mut payload, -1); // camps
        put_i32(&mut payload, -1); // buildings
        put_i32(&mut payload, -1); // enemies
        put_f32(&mut payload, 120.0);
        put_i32(&mut payload, 0);
        put_i32(&mut payload, 0);
        payload.push(0); // tech available
        put_i32(&mut payload, -1);
        put_i32(&mut payload, -1);
        put_string(&mut payload, None);
        put_i32(&mut payload, -1);
        if schema >= 3 {
            payload.push(0);
        }
        put_i32(&mut payload, -1); // town resources
        for _ in 0..4 {
            put_i32(&mut payload, 0);
        }
        payload.push(0);
        put_f32(&mut payload, 0.0);
        put_string(&mut payload, None);
        put_i32(&mut payload, -1); // players
        put_i32(&mut payload, trailer);

        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&payload).unwrap();
        let compressed = encoder.finish().unwrap();
        let mut result = MAGIC.to_vec();
        put_i32(&mut result, 1);
        result.extend(compressed);
        result
    }

    fn put_i32(bytes: &mut Vec<u8>, value: i32) {
        bytes.extend(value.to_le_bytes());
    }

    fn put_u32(bytes: &mut Vec<u8>, value: u32) {
        bytes.extend(value.to_le_bytes());
    }

    fn put_f32(bytes: &mut Vec<u8>, value: f32) {
        bytes.extend(value.to_le_bytes());
    }

    fn put_string(bytes: &mut Vec<u8>, value: Option<&str>) {
        if let Some(value) = value {
            put_i32(bytes, i32::try_from(value.len()).unwrap());
            bytes.extend(value.as_bytes());
        } else {
            put_i32(bytes, -1);
        }
    }
}
