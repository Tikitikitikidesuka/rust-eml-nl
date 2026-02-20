use thiserror::Error;

use crate::utils::StringValueData;

/// Affiliation type used in the election.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AffiliationType {
    /// lijstengroep
    GroupOfLists,
    /// stel gelijkluidende lijsten
    SetOfEqualLists,
    /// op zichzelf staande lijst
    StandAloneList,
}

impl AffiliationType {
    /// Create an [`AffiliationType`] from a `&str`, if possible.
    pub fn from_eml_value(s: &str) -> Result<Self, UnknownAffiliationTypeError> {
        match s {
            "lijstengroep" => Ok(AffiliationType::GroupOfLists),
            "stel gelijkluidende lijsten" => Ok(AffiliationType::SetOfEqualLists),
            "op zichzelf staande lijst" => Ok(AffiliationType::StandAloneList),
            _ => Err(UnknownAffiliationTypeError(s.to_string())),
        }
    }

    /// Get the `&str` representation of this [`AffiliationType`].
    pub fn to_eml_value(&self) -> &'static str {
        match self {
            AffiliationType::GroupOfLists => "lijstengroep",
            AffiliationType::SetOfEqualLists => "stel gelijkluidende lijsten",
            AffiliationType::StandAloneList => "op zichzelf staande lijst",
        }
    }
}

/// Error returned when an unknown election category string is encountered.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("Unknown affiliation type: {0}")]
pub struct UnknownAffiliationTypeError(String);

impl StringValueData for AffiliationType {
    type Error = UnknownAffiliationTypeError;
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
    fn test_election_category_from_str() {
        assert_eq!(
            AffiliationType::from_eml_value("lijstengroep"),
            Ok(AffiliationType::GroupOfLists)
        );
        assert_eq!(
            AffiliationType::from_eml_value("stel gelijkluidende lijsten"),
            Ok(AffiliationType::SetOfEqualLists)
        );
        assert_eq!(
            AffiliationType::from_eml_value("op zichzelf staande lijst"),
            Ok(AffiliationType::StandAloneList)
        );
        assert_eq!(
            AffiliationType::from_eml_value("UNKNOWN"),
            Err(UnknownAffiliationTypeError("UNKNOWN".to_string()))
        );
    }

    #[test]
    fn test_election_category_to_str() {
        assert_eq!(AffiliationType::GroupOfLists.to_eml_value(), "lijstengroep");
        assert_eq!(
            AffiliationType::SetOfEqualLists.to_eml_value(),
            "stel gelijkluidende lijsten"
        );
        assert_eq!(
            AffiliationType::StandAloneList.to_eml_value(),
            "op zichzelf staande lijst"
        );
    }
}
