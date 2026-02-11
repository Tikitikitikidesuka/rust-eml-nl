use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;

use crate::utils::StringValueData;

/// Regular expression for validating ElectionId values.
static ELECTION_ID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(EP|EK|TK|GR|BC|GC|ER|PS|AB|NR|PR|LR|IR)2\d\d\d(\d\d\d\d)?(_[\w_-]*)?$")
        .expect("Failed to compile Election ID regex")
});

/// A string of type ElectionId as defined in the EML_NL specification
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ElectionIdType(String);

impl ElectionIdType {
    /// Create a new ElectionIdType from a string, validating its format
    pub fn new(s: impl AsRef<str>) -> Result<Self, InvalidElectionIdError> {
        StringValueData::parse_from_str(s.as_ref())
    }

    /// Get the raw string value of the ElectionIdType.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for ElectionIdType {
    type Error = InvalidElectionIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Error returned when a string could not be parsed as a ElectionId
#[derive(Debug, Clone, Error)]
#[error("Invalid ElectionId: {0}")]
pub struct InvalidElectionIdError(String);

impl StringValueData for ElectionIdType {
    type Error = InvalidElectionIdError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if ELECTION_ID_RE.is_match(s) {
            Ok(ElectionIdType(s.to_string()))
        } else {
            Err(InvalidElectionIdError(s.to_string()))
        }
    }

    fn to_raw_value(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_election_id_regex_compiles() {
        LazyLock::force(&ELECTION_ID_RE);
    }

    #[test]
    fn test_valid_election_ids() {
        let valid_ids = [
            "EP2021",
            "EK2021",
            "TK2021",
            "GR2021",
            "GR2021_Amsterdam",
            "TK20210101",
        ];
        for id in valid_ids {
            assert!(
                ElectionIdType::new(id).is_ok(),
                "ElectionIdType should accept valid id: {}",
                id
            );
        }
    }

    #[test]
    fn test_invalid_election_ids() {
        let invalid_ids = ["", "2021", "EP21", "EP202A", "EP2021-01-01"];
        for id in invalid_ids {
            assert!(
                ElectionIdType::new(id).is_err(),
                "ElectionIdType should reject invalid id: {}",
                id
            );
        }
    }
}
