use thiserror::Error;

use crate::utils::StringValueData;

/// The publication language of something in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PublicationLanguageType {
    /// Dutch language.
    #[default]
    Dutch,

    /// Frisian language.
    Frisian,
}

impl PublicationLanguageType {
    /// Parse a publication language type from its string representation.
    pub fn from_eml_value(s: &str) -> Result<Self, UnknownPublicationLanguageType> {
        match s {
            "nl" => Ok(PublicationLanguageType::Dutch),
            "fy" => Ok(PublicationLanguageType::Frisian),
            _ => Err(UnknownPublicationLanguageType(s.to_string())),
        }
    }

    /// Get the `&str` representation of this [`PublicationLanguageType`].
    pub fn to_eml_value(&self) -> &'static str {
        match self {
            PublicationLanguageType::Dutch => "nl",
            PublicationLanguageType::Frisian => "fy",
        }
    }
}

/// Error returned when an unknown publication language type string is encountered.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("Unknown publication language type: {0}")]
pub struct UnknownPublicationLanguageType(String);

impl StringValueData for PublicationLanguageType {
    type Error = UnknownPublicationLanguageType;

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
    fn test_valid_publication_languages() {
        let valid_languages = ["nl", "fy"];
        for lang in valid_languages {
            assert!(
                PublicationLanguageType::from_eml_value(lang).is_ok(),
                "PublicationLanguageType should accept valid language code: {}",
                lang
            );
        }
    }

    #[test]
    fn test_invalid_publication_languages() {
        let invalid_languages = ["", "de", "dutch", "nederlands"];
        for lang in invalid_languages {
            assert!(
                PublicationLanguageType::from_eml_value(lang).is_err(),
                "PublicationLanguageType should reject invalid language code: {}",
                lang
            );
        }
    }
}
