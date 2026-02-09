use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;

use crate::utils::StringValueData;

/// Regular expression for validating CandidateIdType values.
static CANDIDATE_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([1-9]\d*)$").expect("Failed to compile Candidate ID regex"));

/// A string of type CandidateIdType as defined in the EML_NL specification
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct CandidateIdType(String);

impl CandidateIdType {
    /// Create a new CandidateIdType from a string, validating its format
    pub fn new(s: impl AsRef<str>) -> Result<Self, InvalidCandidateIdError> {
        StringValueData::parse_from_str(s.as_ref())
    }

    /// Get the raw string value of the CandidateIdType.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Error returned when a string could not be parsed as a ElectionId
#[derive(Debug, Clone, Error)]
#[error("Invalid CandidateIdType: {0}")]
pub struct InvalidCandidateIdError(String);

impl StringValueData for CandidateIdType {
    type Error = InvalidCandidateIdError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if CANDIDATE_ID_RE.is_match(s) {
            Ok(CandidateIdType(s.to_string()))
        } else {
            Err(InvalidCandidateIdError(s.to_string()))
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
    fn test_candidate_id_regex_compiles() {
        LazyLock::force(&CANDIDATE_ID_RE);
    }

    #[test]
    fn test_valid_candidate_ids() {
        let valid_ids = ["1", "12345"];
        for id in valid_ids {
            assert!(
                CandidateIdType::new(id).is_ok(),
                "CandidateIdType should accept valid id: {}",
                id
            );
        }
    }

    #[test]
    fn test_invalid_candidate_ids() {
        let invalid_ids = ["", "0", "0123", "abc", "123abc", "-1"];
        for id in invalid_ids {
            assert!(
                CandidateIdType::new(id).is_err(),
                "CandidateIdType should reject invalid id: {}",
                id
            );
        }
    }
}
