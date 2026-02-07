use crate::{
    EMLError, NS_KR,
    io::{EMLElement, EMLElementReader, EMLElementWriter, QualifiedName},
    utils::{ElectionDomainIdType, StringValue},
};

/// Election domain of an election.
///
/// The (top level) region where the election takes place. Only needed if the
/// ElectionDomain is part of the election name, e.g. election of the council of
/// a municipality or province. Not needed e.g. for Tweede Kamer or European
/// Parliament.
#[derive(Debug, Clone)]
pub struct ElectionDomain {
    /// Identifier of the election domain
    pub id: StringValue<ElectionDomainIdType>,
    /// Name of the election domain
    pub name: String,
}

impl ElectionDomain {
    /// Create a new ElectionDomain
    pub fn new(id: ElectionDomainIdType, name: impl Into<String>) -> Self {
        ElectionDomain {
            id: StringValue::from_value(id),
            name: name.into(),
        }
    }
}

impl EMLElement for ElectionDomain {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ElectionDomain", Some(NS_KR));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        let id = elem.string_value_attr("Id", None)?;
        let name = elem.text_without_children()?;

        Ok(ElectionDomain { id, name })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr("Id", self.id.raw().as_ref())?
            .text(self.name.as_ref())?
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::io::{EMLParsingMode, EMLRead, test_write_eml_element, test_xml_fragment};

    use super::*;

    #[test]
    fn test_election_domain_construction() {
        let ed = ElectionDomain::new(ElectionDomainIdType::new("1234").unwrap(), "Test Domain");
        assert_eq!(ed.id.raw(), "1234");
        assert_eq!(ed.name, "Test Domain");
    }

    #[test]
    fn test_election_domain_parsing() {
        let xml = test_xml_fragment(
            r#"<kr:ElectionDomain xmlns:kr="http://www.kiesraad.nl/extensions" Id="1234">Test Domain</kr:ElectionDomain>"#,
        );
        let ed = ElectionDomain::parse_eml(&xml, EMLParsingMode::Strict).unwrap();
        assert_eq!(ed.id.raw(), "1234");
        assert_eq!(ed.name, "Test Domain");

        let xml_output = test_write_eml_element(&ed, &[NS_KR]).unwrap();
        assert_eq!(xml_output, xml);
    }
}
