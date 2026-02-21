use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;

use crate::utils::StringValueData;

/// Regular expression for validating ContestId values.
static CONTEST_ID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([1-9]\d*|geen|alle|M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3}))$")
        .expect("Failed to compile Contest ID regex")
});

/// A string of type ContestId as defined in the EML_NL specification
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ContestId(String);

impl ContestId {
    /// Create a new `ContestId` from a string, validating its format
    pub fn new(s: impl AsRef<str>) -> Result<Self, InvalidContestIdError> {
        StringValueData::parse_from_str(s.as_ref())
    }

    /// Get the raw string value of the `ContestId`
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Check if the `ContestId` is "geen"
    pub fn is_geen(&self) -> bool {
        self.0 == "geen"
    }

    /// Check if the `ContestId` is "alle"
    pub fn is_alle(&self) -> bool {
        self.0 == "alle"
    }

    /// Create a `ContestId` representing "geen"
    pub fn geen() -> Self {
        ContestId("geen".to_string())
    }

    /// Create a `ContestId` representing "alle"
    pub fn alle() -> Self {
        ContestId("alle".to_string())
    }
}

/// Error returned when a string could not be parsed as a [`ContestId`]
#[derive(Debug, Clone, Error)]
#[error("Invalid contest id: {0}")]
pub struct InvalidContestIdError(String);

impl StringValueData for ContestId {
    type Error = InvalidContestIdError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if !s.is_empty() && CONTEST_ID_RE.is_match(s) {
            Ok(ContestId(s.to_string()))
        } else {
            Err(InvalidContestIdError(s.to_string()))
        }
    }

    fn to_raw_value(&self) -> String {
        self.0.clone()
    }
}

/// A ContestId representing a fixed "geen" value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContestIdGeen;

impl ContestIdGeen {
    /// The fixed string value for 'geen'
    pub const GEEN: &str = "geen";

    /// Create a new `ContestIdGeen`
    pub fn new() -> Self {
        ContestIdGeen
    }

    /// Convert to a regular [`ContestId`]
    pub fn to_contest_id(&self) -> ContestId {
        ContestId::geen()
    }
}

impl Default for ContestIdGeen {
    fn default() -> Self {
        ContestIdGeen::new()
    }
}

/// Error returned when a string could not be parsed as a [`ContestIdGeen`]
#[derive(Debug, Clone, Error)]
#[error("Invalid contest id, expected 'geen': {0}")]
pub struct InvalidContestIdGeenError(String);

impl StringValueData for ContestIdGeen {
    type Error = InvalidContestIdGeenError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if s == Self::GEEN {
            Ok(ContestIdGeen)
        } else {
            Err(InvalidContestIdGeenError(s.to_string()))
        }
    }

    fn to_raw_value(&self) -> String {
        Self::GEEN.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contest_id_regex_compiles() {
        LazyLock::force(&CONTEST_ID_RE);
    }

    #[test]
    fn test_valid_contest_ids() {
        let valid_ids = [
            "1", "12345", "geen", "alle", "III", "IV", "V", "C", "D", "MMMM", "CM",
        ];
        for id in valid_ids {
            assert!(
                ContestId::new(id).is_ok(),
                "ContestId should accept valid id: {}",
                id
            );
        }
    }

    #[test]
    fn test_invalid_contest_ids() {
        let invalid_ids = ["", "0", "0123", "abc", "123abc", "-1", "MMMMM", "IC"];
        for id in invalid_ids {
            assert!(
                ContestId::new(id).is_err(),
                "ContestId should reject invalid id: {}",
                id
            );
        }
    }

    #[test]
    fn test_contest_id_types() {
        let geen = ContestId::geen();
        assert_eq!(geen.value(), "geen");
        assert!(geen.is_geen());
        assert!(!geen.is_alle());

        let alle = ContestId::alle();
        assert_eq!(alle.value(), "alle");
        assert!(!alle.is_geen());
        assert!(alle.is_alle());
    }

    #[test]
    fn test_contest_id_geen() {
        let valid_geen = "geen";
        let invalid_geen = "alle";
        assert!(ContestIdGeen::parse_from_str(valid_geen).is_ok());
        assert!(ContestIdGeen::parse_from_str(invalid_geen).is_err());
    }

    #[test]
    fn test_contest_id_geen_to_contest_id() {
        let geen = ContestIdGeen::new();
        let contest_id = geen.to_contest_id();
        assert_eq!(contest_id.value(), "geen");
    }
}
