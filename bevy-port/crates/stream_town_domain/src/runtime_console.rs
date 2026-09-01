use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::StableId;

pub const CURRENT_RUNTIME_CONSOLE_SCHEMA: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeConsoleRequest {
    pub schema_version: u32,
    pub sequence: u64,
    pub action: RuntimeConsoleAction,
}

impl RuntimeConsoleRequest {
    #[must_use]
    pub const fn new(sequence: u64, action: RuntimeConsoleAction) -> Self {
        Self {
            schema_version: CURRENT_RUNTIME_CONSOLE_SCHEMA,
            sequence,
            action,
        }
    }

    pub fn validate(&self) -> Result<(), RuntimeConsoleValidationError> {
        if self.schema_version != CURRENT_RUNTIME_CONSOLE_SCHEMA {
            return Err(RuntimeConsoleValidationError::Schema(self.schema_version));
        }
        if self.sequence == 0 {
            return Err(RuntimeConsoleValidationError::Sequence);
        }
        if let RuntimeConsoleAction::InjectChat {
            actor_id,
            login_name,
            display_name,
            command,
            ..
        } = &self.action
            && (actor_id.as_str().is_empty()
                || login_name.trim().is_empty()
                || display_name.trim().is_empty()
                || command.trim().is_empty())
        {
            return Err(RuntimeConsoleValidationError::ChatIdentity);
        }
        Ok(())
    }
}

impl RuntimeConsoleStatus {
    pub fn validate(&self) -> Result<(), RuntimeConsoleValidationError> {
        if self.schema_version != CURRENT_RUNTIME_CONSOLE_SCHEMA {
            return Err(RuntimeConsoleValidationError::Schema(self.schema_version));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuntimeConsoleAction {
    InjectChat {
        actor_id: StableId,
        login_name: String,
        display_name: String,
        command: String,
        is_broadcaster: bool,
        is_moderator: bool,
        is_subscriber: bool,
    },
    Save,
    SaveJumpStart,
    Load,
    CaptureFrame,
    ReturnToMainMenu,
    Exit,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct RuntimeConsoleStatus {
    pub schema_version: u32,
    pub process_id: u32,
    pub updated_unix_millis: u64,
    pub state: String,
    pub world_seed: Option<u64>,
    pub world_hash: Option<String>,
    pub elapsed_seconds: f64,
    pub actor_count: usize,
    pub building_count: usize,
    pub town_resources: BTreeMap<String, u64>,
    pub paths_completed: u64,
    pub commands_processed: u64,
    pub average_frame_ms: Option<f64>,
    pub p95_frame_ms: Option<f64>,
    pub terrain_high_chunks: usize,
    pub terrain_medium_chunks: usize,
    pub terrain_low_chunks: usize,
    pub foliage_instances: usize,
    pub foliage_visible_instances: usize,
    pub foliage_batches: usize,
    pub foliage_spatial_groups: usize,
    pub foliage_unbatched_instances: usize,
    pub crowd_adjusted_agents: usize,
    pub crowd_yielding_agents: usize,
    pub save_exists: bool,
    pub save_path: String,
    pub twitch_status: String,
    pub direct_broadcast_status: String,
    pub last_processed_sequence: u64,
    pub last_result: String,
}

#[derive(Clone, Debug)]
pub struct RuntimeConsoleStore {
    root: PathBuf,
}

impl RuntimeConsoleStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn from_environment() -> Self {
        Self::new(
            std::env::var_os("STREAM_TOWN_RUNTIME_CONSOLE_DIR").map_or_else(
                || PathBuf::from(".stream-town").join("runtime-console"),
                PathBuf::from,
            ),
        )
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn request_path(&self) -> PathBuf {
        self.root.join("request.json")
    }

    #[must_use]
    pub fn status_path(&self) -> PathBuf {
        self.root.join("status.json")
    }

    pub fn read_request(&self) -> Result<Option<RuntimeConsoleRequest>, RuntimeConsoleStoreError> {
        let Some(request) = read_optional_json::<RuntimeConsoleRequest>(&self.request_path())?
        else {
            return Ok(None);
        };
        request.validate()?;
        Ok(Some(request))
    }

    pub fn write_request(
        &self,
        request: &RuntimeConsoleRequest,
    ) -> Result<(), RuntimeConsoleStoreError> {
        request.validate()?;
        write_json_atomic(&self.request_path(), request)
    }

    pub fn read_status(&self) -> Result<Option<RuntimeConsoleStatus>, RuntimeConsoleStoreError> {
        let status = read_optional_json::<RuntimeConsoleStatus>(&self.status_path())?;
        if let Some(status) = &status {
            status.validate()?;
        }
        Ok(status)
    }

    pub fn write_status(
        &self,
        status: &RuntimeConsoleStatus,
    ) -> Result<(), RuntimeConsoleStoreError> {
        status.validate()?;
        write_json_atomic(&self.status_path(), status)
    }
}

#[derive(Debug, Error)]
pub enum RuntimeConsoleValidationError {
    #[error("unsupported runtime-console schema {0}")]
    Schema(u32),
    #[error("runtime-console sequence must be greater than zero")]
    Sequence,
    #[error("chat injection requires stable non-empty identity and command fields")]
    ChatIdentity,
}

#[derive(Debug, Error)]
pub enum RuntimeConsoleStoreError {
    #[error("runtime-console I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("runtime-console JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("runtime-console request is invalid: {0}")]
    Validation(#[from] RuntimeConsoleValidationError),
}

fn read_optional_json<T: DeserializeOwned>(
    path: &Path,
) -> Result<Option<T>, RuntimeConsoleStoreError> {
    match fs::read(path) {
        Ok(encoded) => Ok(Some(serde_json::from_slice(&encoded)?)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), RuntimeConsoleStoreError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    let encoded = serde_json::to_vec_pretty(value)?;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(&encoded)?;
    file.sync_all()?;
    if path.is_file() {
        fs::remove_file(path)?;
    }
    if let Err(error) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_and_status_round_trip_through_atomic_store() {
        let directory = tempfile::tempdir().unwrap();
        let store = RuntimeConsoleStore::new(directory.path());
        let request = RuntimeConsoleRequest::new(
            7,
            RuntimeConsoleAction::InjectChat {
                actor_id: StableId::new("tool:operator").unwrap(),
                login_name: "operator".to_owned(),
                display_name: "Operator".to_owned(),
                command: "!join".to_owned(),
                is_broadcaster: true,
                is_moderator: true,
                is_subscriber: true,
            },
        );
        store.write_request(&request).unwrap();
        assert_eq!(store.read_request().unwrap(), Some(request));

        let status = RuntimeConsoleStatus {
            schema_version: CURRENT_RUNTIME_CONSOLE_SCHEMA,
            process_id: 42,
            updated_unix_millis: 1_234,
            state: "InGame".to_owned(),
            actor_count: 300,
            average_frame_ms: Some(9.5),
            last_processed_sequence: 7,
            ..RuntimeConsoleStatus::default()
        };
        store.write_status(&status).unwrap();
        assert_eq!(store.read_status().unwrap(), Some(status));
        assert!(!store.request_path().with_extension("json.tmp").exists());
        assert!(!store.status_path().with_extension("json.tmp").exists());
    }

    #[test]
    fn invalid_requests_are_rejected_before_writing() {
        let directory = tempfile::tempdir().unwrap();
        let store = RuntimeConsoleStore::new(directory.path());
        let request = RuntimeConsoleRequest::new(0, RuntimeConsoleAction::Save);
        assert!(store.write_request(&request).is_err());
        assert!(!store.request_path().exists());
    }
}
