use crate::error::EMLValueResultExt;
use crate::utils::StringValueData;
use crate::EMLError;
use thiserror::Error;

/// Region category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionCategory {
    /// Todo: Unknown meaning
    DEELGEMEENTE,
    /// Todo: Unknown meaning
    GEMEENTE,
    /// Todo: Unknown meaning
    KIESKRING,
    /// Todo: Unknown meaning
    PROVINCIE,
    /// Todo: Unknown meaning
    PROVINCIAAL_KIESKRING,
    /// Todo: Unknown meaning
    PROVINCIAAL_STEMBUREAU,
    /// Todo: Unknown meaning
    STAAT,
    /// Todo: Unknown meaning
    STEMBUREAU,
    /// Todo: Unknown meaning
    WATERSCHAP,
    /// Todo: Unknown meaning
    WATERSCHAP_KIESKRING,
    /// Todo: Unknown meaning
    WATERSCHAP_GEMEENTE,
}

impl RegionCategory {
    /// Create a new ElectionCategory from a string, validating its format
    pub fn new(s: impl AsRef<str>) -> Result<Self, EMLError> {
        Self::from_eml_value(s).wrap_value_error()
    }

    /// Create a [`RegionCategory`] from a `&str`, if possible.
    pub fn from_eml_value(s: impl AsRef<str>) -> Result<Self, UnknownRegionCategoryError> {
        let data = s.as_ref();
        match data {
            "DEELGEMEENTE" => Ok(Self::DEELGEMEENTE),
            "GEMEENTE" => Ok(Self::GEMEENTE),
            "KIESKRING" => Ok(Self::KIESKRING),
            "PROVINCIE" => Ok(Self::PROVINCIE),
            "PROVINCIAAL_KIESKRING" => Ok(Self::PROVINCIAAL_KIESKRING),
            "PROVINCIAAL_STEMBUREAU" => Ok(Self::PROVINCIAAL_STEMBUREAU),
            "STAAT" => Ok(Self::STAAT),
            "STEMBUREAU" => Ok(Self::STEMBUREAU),
            "WATERSCHAP" => Ok(Self::WATERSCHAP),
            "WATERSCHAP_KIESKRING" => Ok(Self::WATERSCHAP_KIESKRING),
            "WATERSCHAP_GEMEENTE" => Ok(Self::WATERSCHAP_GEMEENTE),
            _ => Err(UnknownRegionCategoryError(data.to_string())),
        }
    }

    /// Get the `&str` representation of this [`RegionCategory`].
    pub fn to_eml_value(&self) -> &'static str {
        match self {
            RegionCategory::DEELGEMEENTE => "DEELGEMEENTE",
            RegionCategory::GEMEENTE => "GEMEENTE",
            RegionCategory::KIESKRING => "KIESKRING",
            RegionCategory::PROVINCIE => "PROVINCIE",
            RegionCategory::PROVINCIAAL_KIESKRING => "PROVINCIAAL_KIESKRING",
            RegionCategory::PROVINCIAAL_STEMBUREAU => "PROVINCIAAL_STEMBUREAU",
            RegionCategory::STAAT => "STAAT",
            RegionCategory::STEMBUREAU => "STEMBUREAU",
            RegionCategory::WATERSCHAP => "WATERSCHAP",
            RegionCategory::WATERSCHAP_KIESKRING => "WATERSCHAP_KIESKRING",
            RegionCategory::WATERSCHAP_GEMEENTE => "WATERSCHAP_GEMEENTE",
        }
    }
}

/// Error returned when an unknown election category string is encountered.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("Unknown region category: {0}")]
pub struct UnknownRegionCategoryError(String);

impl StringValueData for RegionCategory {
    type Error = UnknownRegionCategoryError;

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
