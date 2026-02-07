use std::num::NonZeroU64;

use crate::{
    NS_EML,
    io::{EMLElement, collect_struct},
    utils::{CandidateIdType, NameShortCodeType, StringValue},
};

/// Candidate identifier, but not for 510 document types.\
#[derive(Debug, Clone)]
pub struct CandidateIdentifier {
    /// The candidate id.
    pub id: StringValue<CandidateIdType>,
    /// The display order of the candidate.
    pub display_order: Option<StringValue<NonZeroU64>>,
    /// The short code of the candidate.
    pub short_code: Option<StringValue<NameShortCodeType>>,
    /// The expected confirmation reference of the candidate.
    pub expected_confirmation_reference: Option<String>,
}

impl CandidateIdentifier {
    /// Create a new CandidateIdentifier.
    pub fn new(id: StringValue<CandidateIdType>) -> Self {
        CandidateIdentifier {
            id,
            display_order: None,
            short_code: None,
            expected_confirmation_reference: None,
        }
    }
}

impl EMLElement for CandidateIdentifier {
    const EML_NAME: crate::io::QualifiedName<'_, '_> =
        crate::io::QualifiedName::from_static("CandidateIdentifier", Some(NS_EML));

    fn read_eml(elem: &mut crate::io::EMLElementReader<'_, '_>) -> Result<Self, crate::EMLError> {
        let id = elem.string_value_attr("Id", None)?;
        let display_order = elem.string_value_attr_opt("DisplayOrder")?;
        let short_code = elem.string_value_attr_opt("ShortCode")?;
        let expected_confirmation_reference = elem
            .attribute_value("ExpectedConfirmationReference")?
            .map(|s| s.into_owned());

        struct CandidateIdentifierTmp {
            short_code: Option<StringValue<NameShortCodeType>>,
        }

        let elem = collect_struct!(
            elem,
            CandidateIdentifierTmp {
                short_code as Option: ("ShortCode", NS_EML) => |elem| elem.string_value()?,
            }
        );

        Ok(CandidateIdentifier {
            id,
            display_order,
            short_code: short_code.or(elem.short_code), // attribute takes precedence
            expected_confirmation_reference,
        })
    }

    fn write_eml(&self, writer: crate::io::EMLElementWriter) -> Result<(), crate::EMLError> {
        writer
            .attr("Id", &self.id.raw())?
            .attr_opt("DisplayOrder", self.display_order.as_ref().map(|v| v.raw()))?
            .attr_opt("ShortCode", self.short_code.as_ref().map(|v| v.raw()))?
            .attr_opt(
                "ExpectedConfirmationReference",
                self.expected_confirmation_reference.as_ref(),
            )?
            .empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::io::{EMLParsingMode, EMLRead as _, test_write_eml_element, test_xml_fragment};

    use super::*;

    #[test]
    fn test_simple_candidate_identifier() {
        let xml = test_xml_fragment(
            r#"
            <CandidateIdentifier xmlns="urn:oasis:names:tc:evs:schema:eml" Id="1"/>
            "#,
        );
        let can_id = CandidateIdentifier::parse_eml(&xml, EMLParsingMode::Strict).unwrap();
        assert_eq!(
            can_id.id,
            StringValue::Parsed(CandidateIdType::new("1").unwrap())
        );
        assert_eq!(can_id.display_order, None);
        assert_eq!(can_id.short_code, None);
        assert_eq!(can_id.expected_confirmation_reference, None);

        let xml_output = test_write_eml_element(&can_id, &[NS_EML]).unwrap();
        assert_eq!(xml_output, xml);
    }

    #[test]
    fn test_all_attributes_candidate_identifier() {
        let xml = test_xml_fragment(
            r#"
            <CandidateIdentifier xmlns="urn:oasis:names:tc:evs:schema:eml" Id="2254" DisplayOrder="2" ShortCode="1234" ExpectedConfirmationReference="Ref123"/>
            "#,
        );
        let can_id = CandidateIdentifier::parse_eml(&xml, EMLParsingMode::Strict).unwrap();
        assert_eq!(
            can_id.id,
            StringValue::Parsed(CandidateIdType::new("2254").unwrap())
        );
        assert_eq!(
            can_id.display_order,
            Some(StringValue::Parsed(NonZeroU64::new(2).unwrap()))
        );
        assert_eq!(
            can_id.short_code,
            Some(StringValue::Parsed(NameShortCodeType::new("1234").unwrap()))
        );
        assert_eq!(
            can_id.expected_confirmation_reference,
            Some("Ref123".to_string())
        );

        let xml_output = test_write_eml_element(&can_id, &[NS_EML]).unwrap();
        assert_eq!(xml_output, xml);
    }
}
