use crate::{
    EMLError, EMLErrorKind, NS_EML, NS_XAL,
    common::{CountryNameCode, LocalityName},
    io::{EMLElement, EMLElementReader, EMLElementWriter, QualifiedName, collect_struct},
};

/// The minimal details for a qualifying address
#[derive(Debug, Clone)]
pub enum MinimalQualifyingAddress {
    /// This qualifying address is a locality
    Locality(MinimalQualifyingAddressLocality),

    /// This qualifying address is a country
    Country(MinimalQualifyingAddressCountry),
}

impl EMLElement for MinimalQualifyingAddress {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("QualifyingAddress", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        let parent_name = elem.name()?.as_owned();
        let mut found_child = None;
        while let Some(mut child) = elem.next_child()? {
            match child.name()? {
                n if n == MinimalQualifyingAddressLocality::EML_NAME && found_child.is_none() => {
                    found_child = Some(MinimalQualifyingAddress::Locality(
                        MinimalQualifyingAddressLocality::read_eml(&mut child)?,
                    ));
                }
                n if n == MinimalQualifyingAddressCountry::EML_NAME && found_child.is_none() => {
                    found_child = Some(MinimalQualifyingAddress::Country(
                        MinimalQualifyingAddressCountry::read_eml(&mut child)?,
                    ));
                }
                n => {
                    let span = child.span();
                    let name = n.as_owned();

                    let err =
                        EMLErrorKind::UnexpectedElement(name, parent_name.clone()).add_span(span);
                    if child.parsing_mode().is_strict() {
                        return Err(err);
                    } else {
                        child.push_err(err);
                    }
                }
            }
        }
        let Some(result) = found_child else {
            return Err(EMLErrorKind::MissingChoiceElements(vec![
                MinimalQualifyingAddressCountry::EML_NAME.as_owned(),
                MinimalQualifyingAddressLocality::EML_NAME.as_owned(),
            ])
            .add_span(elem.full_span()));
        };
        Ok(result)
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer.empty()
    }
}

/// The minimal details for locality in a qualifying address
#[derive(Debug, Clone)]
pub struct MinimalQualifyingAddressLocality {
    /// Name of the locality
    pub locality_name: LocalityName,
}

impl EMLElement for MinimalQualifyingAddressLocality {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Locality", Some(NS_XAL));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(
            elem,
            MinimalQualifyingAddressLocality {
                locality_name: LocalityName::EML_NAME => |elem| LocalityName::read_eml(elem)?,
            }
        ))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(LocalityName::EML_NAME, &self.locality_name)?
            .finish()
    }
}

/// The minimal details for country in a qualifying address
#[derive(Debug, Clone)]
pub struct MinimalQualifyingAddressCountry {
    /// The country name code, if present.
    pub country_name_code: CountryNameCode,
    /// The locality within the country.
    pub locality: MinimalQualifyingAddressLocality,
}

impl EMLElement for MinimalQualifyingAddressCountry {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Country", Some(NS_XAL));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, MinimalQualifyingAddressCountry {
            country_name_code: CountryNameCode::EML_NAME => |elem| CountryNameCode::read_eml(elem)?,
            locality: MinimalQualifyingAddressLocality::EML_NAME => |elem| MinimalQualifyingAddressLocality::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(CountryNameCode::EML_NAME, &self.country_name_code)?
            .child_elem(MinimalQualifyingAddressLocality::EML_NAME, &self.locality)?
            .finish()
    }
}
