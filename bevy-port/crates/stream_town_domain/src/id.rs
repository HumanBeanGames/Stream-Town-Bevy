use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// A persistent identifier suitable for authored assets and save files.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct StableId(String);

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum StableIdError {
    #[error("a stable ID cannot be empty")]
    Empty,
    #[error("stable ID is longer than 128 characters")]
    TooLong,
    #[error("stable ID contains unsupported character {0:?}")]
    InvalidCharacter(char),
}

impl StableId {
    pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
        let value = value.into();
        if value.is_empty() {
            return Err(StableIdError::Empty);
        }
        if value.len() > 128 {
            return Err(StableIdError::TooLong);
        }
        if let Some(character) = value.chars().find(|value| {
            !value.is_ascii_lowercase() && !value.is_ascii_digit() && !"._:-".contains(*value)
        }) {
            return Err(StableIdError::InvalidCharacter(character));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn random(prefix: &str) -> Self {
        Self(format!("{prefix}:{}", Uuid::new_v4().simple()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StableId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for StableId {
    type Err = StableIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unstable_identifiers() {
        assert_eq!(StableId::new(""), Err(StableIdError::Empty));
        assert!(matches!(
            StableId::new("Bad ID"),
            Err(StableIdError::InvalidCharacter(_))
        ));
    }

    #[test]
    fn round_trips_through_serde() {
        let id = StableId::new("building:town_hall").unwrap();
        let encoded = serde_json::to_string(&id).unwrap();
        assert_eq!(serde_json::from_str::<StableId>(&encoded).unwrap(), id);
    }
}
