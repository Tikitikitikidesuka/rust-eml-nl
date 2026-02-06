use std::borrow::Cow;

use crate::{
    EMLError, NS_XAL,
    io::{EMLElement, EMLElementReader, EMLElementWriter, QualifiedName},
};

/// Country name code information.
#[derive(Debug, Clone)]
pub struct CountryNameCode {
    /// The country name code value.
    pub value: String,
    /// The Scheme attribute, if present.
    pub scheme: Option<String>,
    /// The Code attribute, if present.
    pub code: Option<String>,
}

impl CountryNameCode {
    /// Create a new CountryNameCode.
    pub fn new(value: impl Into<String>) -> Self {
        CountryNameCode {
            value: value.into(),
            scheme: None,
            code: None,
        }
    }
}

impl EMLElement for CountryNameCode {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("CountryNameCode", Some(NS_XAL));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(CountryNameCode {
            value: elem.text_without_children()?,
            scheme: elem.attribute_value("Scheme")?.map(Cow::into_owned),
            code: elem.attribute_value("Code")?.map(Cow::into_owned),
        })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr_opt("Scheme", self.scheme.as_ref())?
            .attr_opt("Code", self.code.as_ref())?
            .text(self.value.as_ref())?
            .finish()
    }
}
