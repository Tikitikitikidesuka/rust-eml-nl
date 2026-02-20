use thiserror::Error;

use crate::utils::StringValueData;

/// Voting method used in the election.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenderType {
    /// Male gender
    Male,
    /// Female gender
    Female,
    /// Gender unknown
    Unknown,
}

impl GenderType {
    /// Create a GenderType from a `&str`, if possible.
    pub fn from_eml_value(s: &str) -> Result<Self, UnknownGenderTypeError> {
        match s {
            "male" => Ok(GenderType::Male),
            "female" => Ok(GenderType::Female),
            "unknown" => Ok(GenderType::Unknown),
            _ => Err(UnknownGenderTypeError(s.to_string())),
        }
    }

    /// Get the `&str` representation of this GenderType.
    pub fn to_eml_value(&self) -> &'static str {
        match self {
            GenderType::Male => "male",
            GenderType::Female => "female",
            GenderType::Unknown => "unknown",
        }
    }
}

/// Error returned when an unknown gender type string is encountered.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("Unknown gender type: {0}")]
pub struct UnknownGenderTypeError(String);

impl StringValueData for GenderType {
    type Error = UnknownGenderTypeError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Self::from_eml_value(s)
    }

    fn to_raw_value(&self) -> String {
        self.to_eml_value().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gender_types() {
        let valid_genders = ["male", "female", "unknown"];
        for gender in valid_genders {
            assert!(
                GenderType::from_eml_value(gender).is_ok(),
                "GenderType should accept valid gender: {}",
                gender
            );
        }
    }

    #[test]
    fn test_invalid_gender_types() {
        let invalid_genders = ["", "test", "abc"];
        for gender in invalid_genders {
            assert!(
                GenderType::from_eml_value(gender).is_err(),
                "GenderType should reject invalid gender: {}",
                gender
            );
        }
    }
}
