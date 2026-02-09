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
pub struct ContestIdType(String);

impl ContestIdType {
    /// Create a new `ContestIdType` from a string, validating its format
    pub fn new(s: impl AsRef<str>) -> Result<Self, InvalidContestIdError> {
        StringValueData::parse_from_str(s.as_ref())
    }

    /// Get the raw string value of the `ContestIdType`
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Check if the `ContestIdType` is "geen"
    pub fn is_geen(&self) -> bool {
        self.0 == "geen"
    }

    /// Check if the `ContestIdType` is "alle"
    pub fn is_alle(&self) -> bool {
        self.0 == "alle"
    }

    /// Create a `ContestIdType` representing "geen"
    pub fn geen() -> Self {
        ContestIdType("geen".to_string())
    }

    /// Create a `ContestIdType` representing "alle"
    pub fn alle() -> Self {
        ContestIdType("alle".to_string())
    }
}

/// Error returned when a string could not be parsed as a ContestId
#[derive(Debug, Clone, Error)]
#[error("Invalid ContestId: {0}")]
pub struct InvalidContestIdError(String);

impl StringValueData for ContestIdType {
    type Error = InvalidContestIdError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if !s.is_empty() && CONTEST_ID_RE.is_match(s) {
            Ok(ContestIdType(s.to_string()))
        } else {
            Err(InvalidContestIdError(s.to_string()))
        }
    }

    fn to_raw_value(&self) -> String {
        self.0.clone()
    }
}

/// A ContestIdType representing a fixed "geen" value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContestIdTypeGeen;

impl ContestIdTypeGeen {
    /// The fixed string value for 'geen'
    pub const GEEN: &str = "geen";

    /// Create a new `ContestIdTypeGeen`
    pub fn new() -> Self {
        ContestIdTypeGeen
    }

    /// Convert to a regular [`ContestIdType`]
    pub fn to_contest_id_type(&self) -> ContestIdType {
        ContestIdType::geen()
    }
}

impl Default for ContestIdTypeGeen {
    fn default() -> Self {
        ContestIdTypeGeen::new()
    }
}

/// Error returned when a string could not be parsed as a ContestId
#[derive(Debug, Clone, Error)]
#[error("Invalid ContestId, expected 'geen': {0}")]
pub struct InvalidContestIdGeenError(String);

impl StringValueData for ContestIdTypeGeen {
    type Error = InvalidContestIdGeenError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if s == Self::GEEN {
            Ok(ContestIdTypeGeen)
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
                ContestIdType::new(id).is_ok(),
                "ContestIdType should accept valid id: {}",
                id
            );
        }
    }

    #[test]
    fn test_invalid_contest_ids() {
        let invalid_ids = ["", "0", "0123", "abc", "123abc", "-1", "MMMMM", "IC"];
        for id in invalid_ids {
            assert!(
                ContestIdType::new(id).is_err(),
                "ContestIdType should reject invalid id: {}",
                id
            );
        }
    }

    #[test]
    fn test_contest_id_types() {
        let geen = ContestIdType::geen();
        assert_eq!(geen.value(), "geen");
        assert!(geen.is_geen());
        assert!(!geen.is_alle());

        let alle = ContestIdType::alle();
        assert_eq!(alle.value(), "alle");
        assert!(!alle.is_geen());
        assert!(alle.is_alle());
    }

    #[test]
    fn test_contest_id_type_geen() {
        let valid_geen = "geen";
        let invalid_geen = "alle";
        assert!(ContestIdTypeGeen::parse_from_str(valid_geen).is_ok());
        assert!(ContestIdTypeGeen::parse_from_str(invalid_geen).is_err());
    }

    #[test]
    fn test_contest_id_type_geen_to_contest_id_type() {
        let geen = ContestIdTypeGeen::new();
        let contest_id = geen.to_contest_id_type();
        assert_eq!(contest_id.value(), "geen");
    }
}
