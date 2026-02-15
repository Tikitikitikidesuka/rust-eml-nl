//! Document variant for the EML_NL Result (`520`) document.

use crate::{
    EML_SCHEMA_VERSION, EMLError, EMLErrorKind, EMLResultExt as _, NS_EML, NS_KR,
    common::{
        CandidateIdentifier, CanonicalizationMethod, ContestIdentifier, CreationDateTime,
        ElectionDomain, ManagingAuthority, MinimalQualifyingAddress, PersonNameStructure,
        TransactionId,
    },
    documents::accepted_root,
    io::{
        EMLElement, EMLElementReader, EMLElementWriter, EMLReadElement as _, EMLWriteElement as _,
        QualifiedName, collect_struct,
    },
    utils::{
        AffiliationIdType, ElectionCategory, ElectionIdType, ElectionSubcategory, GenderType,
        StringValue, StringValueData, XsDate,
    },
};

pub(crate) const EML_ELECTION_RESULT_ID: &str = "520";

/// Representing a `110a` document, containing an election definition.
#[derive(Debug, Clone)]
pub struct ElectionResult {
    /// Transaction id of the document.
    pub transaction_id: TransactionId,

    /// Managing authority of the election, if present.
    pub managing_authority: ManagingAuthority,

    /// Time this document was created.
    pub creation_date_time: CreationDateTime,

    /// Canonicalization method used in this document, if present.
    pub canonicalization_method: Option<CanonicalizationMethod>,

    /// The result data.
    pub result: ElectionResultResult,
}

impl EMLElement for ElectionResult {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("EML", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        // TODO: parse the rest of the document
        accepted_root(elem)?;

        let document_id = elem.attribute_value_req(("Id", None))?;
        if document_id != EML_ELECTION_RESULT_ID {
            return Err(EMLErrorKind::InvalidDocumentType(
                EML_ELECTION_RESULT_ID,
                document_id.to_string(),
            ))
            .with_span(elem.span());
        }

        Ok(collect_struct!(elem, ElectionResult {
            transaction_id: TransactionId::EML_NAME => |elem| TransactionId::read_eml(elem)?,
            managing_authority: ManagingAuthority::EML_NAME => |elem| ManagingAuthority::read_eml(elem)?,
            creation_date_time: CreationDateTime::EML_NAME => |elem| CreationDateTime::read_eml(elem)?,
            canonicalization_method as Option: CanonicalizationMethod::EML_NAME => |elem| CanonicalizationMethod::read_eml(elem)?,
            result: ElectionResultResult::EML_NAME => |elem| ElectionResultResult::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr(("Id", None), EML_ELECTION_RESULT_ID)?
            .attr(("SchemaVersion", None), EML_SCHEMA_VERSION)?
            .child_elem(TransactionId::EML_NAME, &self.transaction_id)?
            .child_elem(ManagingAuthority::EML_NAME, &self.managing_authority)?
            .child_elem(CreationDateTime::EML_NAME, &self.creation_date_time)?
            // Note: we don't output the CanonicalizationMethod because we aren't canonicalizing our output
            // .child_elem_option(
            //     CanonicalizationMethod::EML_NAME,
            //     self.canonicalization_method.as_ref(),
            // )?
            .child_elem(ElectionResultResult::EML_NAME, &self.result)?
            .finish()
    }
}

/// The result data of an election result document.
#[derive(Debug, Clone)]
pub struct ElectionResultResult {
    /// The election for which the result applies.
    pub election: ElectionResultElection,
}

impl EMLElement for ElectionResultResult {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Result", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionResultResult {
            election: ElectionResultElection::EML_NAME => |elem| ElectionResultElection::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(ElectionResultElection::EML_NAME, &self.election)?
            .finish()
    }
}

/// The election for which the result applies.
#[derive(Debug, Clone)]
pub struct ElectionResultElection {
    /// Identifier for the election.
    pub identifier: ElectionResultElectionIdentifier,

    /// Contests within the election.
    pub contests: Vec<ElectionResultContest>,
}

impl EMLElement for ElectionResultElection {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Election", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionResultElection {
            identifier: ElectionResultElectionIdentifier::EML_NAME => |elem| ElectionResultElectionIdentifier::read_eml(elem)?,
            contests as Vec: ElectionResultContest::EML_NAME => |elem| ElectionResultContest::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(ElectionResultElectionIdentifier::EML_NAME, &self.identifier)?
            .child_elems(ElectionResultContest::EML_NAME, &self.contests)?
            .finish()
    }
}

/// Identifier for the election for which the result applies.
#[derive(Debug, Clone)]
pub struct ElectionResultElectionIdentifier {
    /// Id of the election
    pub id: StringValue<ElectionIdType>,

    /// Name of the election
    pub name: Option<String>,

    /// Category of the election
    pub category: StringValue<ElectionCategory>,

    /// Subcategory of the election
    pub subcategory: Option<StringValue<ElectionSubcategory>>,

    /// The (top level) region where the election takes place.
    pub domain: Option<ElectionDomain>,

    /// Date of the election
    pub election_date: StringValue<XsDate>,
}

impl EMLElement for ElectionResultElectionIdentifier {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ElectionIdentifier", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(
            elem,
            ElectionResultElectionIdentifier {
                id: elem.string_value_attr("Id", None)?,
                name as Option: ("ElectionName", NS_EML) => |elem| elem.text_without_children()?,
                category: ("ElectionCategory", NS_EML) => |elem| elem.string_value()?,
                subcategory as Option: ("ElectionSubcategory", NS_KR) => |elem| elem.string_value()?,
                domain as Option: ElectionDomain::EML_NAME => |elem| ElectionDomain::read_eml(elem)?,
                election_date: ("ElectionDate", NS_KR) => |elem| elem.string_value()?,
            }
        ))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr("Id", self.id.raw().as_ref())?
            .child_option(
                ("ElectionName", NS_EML),
                self.name.as_ref(),
                |elem, value| elem.text(value.as_ref())?.finish(),
            )?
            .child(("ElectionCategory", NS_EML), |elem| {
                elem.text(self.category.raw().as_ref())?.finish()
            })?
            .child_option(
                ("ElectionSubcategory", NS_KR),
                self.subcategory.as_ref(),
                |elem, value| elem.text(value.raw().as_ref())?.finish(),
            )?
            .child_elem_option(ElectionDomain::EML_NAME, self.domain.as_ref())?
            .child(("ElectionDate", NS_KR), |elem| {
                elem.text(self.election_date.raw().as_ref())?.finish()
            })?
            .finish()
    }
}

/// A contest within an election result.
#[derive(Debug, Clone)]
pub struct ElectionResultContest {
    /// Identifier of the contest.
    pub identifier: ContestIdentifier,

    /// Selections within the contest.
    pub selections: Vec<ElectionResultSelection>,
}

impl EMLElement for ElectionResultContest {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Contest", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionResultContest {
            identifier: ContestIdentifier::EML_NAME => |elem| ContestIdentifier::read_eml(elem)?,
            selections as Vec: ElectionResultSelection::EML_NAME => |elem| ElectionResultSelection::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(ContestIdentifier::EML_NAME, &self.identifier)?
            .child_elems(ElectionResultSelection::EML_NAME, &self.selections)?
            .finish()
    }
}

/// The ranking of a selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankingType {
    /// First ranked selection.
    First,
    /// Second ranked selection.
    Second,
}

impl RankingType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(RankingType::First),
            "2" => Some(RankingType::Second),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            RankingType::First => "1",
            RankingType::Second => "2",
        }
    }
}

/// Error type for invalid ranking type values.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid ranking type value: {0}")]
pub struct InvalidRankingTypeError(String);

impl StringValueData for RankingType {
    type Error = InvalidRankingTypeError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error> {
        RankingType::from_str(s).ok_or_else(|| InvalidRankingTypeError(s.to_string()))
    }

    fn to_raw_value(&self) -> String {
        self.as_str().to_string()
    }
}

/// Yes/no type, represented as a boolean.
/// In the EML, this is represented as "yes" or "no".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YesNoType(bool);

/// Error type for invalid yes/no type values.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid yes/no type value: {0}")]
pub struct InvalidYesNoTypeError(String);

impl StringValueData for YesNoType {
    type Error = InvalidYesNoTypeError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error> {
        match s {
            "yes" => Ok(YesNoType(true)),
            "no" => Ok(YesNoType(false)),
            _ => Err(InvalidYesNoTypeError(s.to_string())),
        }
    }

    fn to_raw_value(&self) -> String {
        if self.0 {
            "yes".to_string()
        } else {
            "no".to_string()
        }
    }
}

/// A selection within an election contest.
#[derive(Debug, Clone)]
pub struct ElectionResultSelection {
    /// The type of selection.
    pub selection_type: ElectionResultSelectionType,

    /// Number of votes received.
    pub votes: Option<StringValue<u64>>,

    /// Ranking of the selection, if applicable.
    pub ranking: Option<StringValue<RankingType>>,

    /// Whether the selection was elected.
    pub elected: StringValue<YesNoType>,
}

const VOTES_EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Votes", Some(NS_EML));
const RANKING_EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Ranking", Some(NS_EML));
const ELECTED_EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Elected", Some(NS_EML));

impl EMLElement for ElectionResultSelection {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Selection", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        let mut selection_type = None;
        let mut votes = None;
        let mut ranking = None;
        let mut elected = None;

        while let Some(mut child) = elem.next_child()? {
            let name = child.name()?;

            match name {
                n if n == CandidateSelection::EML_NAME => {
                    selection_type = Some(ElectionResultSelectionType::Candidate(Box::new(
                        CandidateSelection::read_eml(&mut child)?,
                    )));
                }
                n if n == AffiliationSelection::EML_NAME => {
                    selection_type = Some(ElectionResultSelectionType::Affiliation(Box::new(
                        AffiliationSelection::read_eml(&mut child)?,
                    )));
                }
                n if n == VOTES_EML_NAME => {
                    votes = Some(child.string_value()?);
                }
                n if n == RANKING_EML_NAME => {
                    ranking = Some(child.string_value()?);
                }
                n if n == ELECTED_EML_NAME => {
                    elected = Some(child.string_value()?);
                }
                n => {
                    let err =
                        EMLErrorKind::UnexpectedElement(n.as_owned(), Self::EML_NAME.as_owned())
                            .with_span(child.inner_span());
                    if child.parsing_mode().is_strict() {
                        return Err(err);
                    } else {
                        child.push_err(err);
                        child.skip()?;
                    }
                }
            }
        }
        Ok(ElectionResultSelection {
            selection_type: selection_type
                .ok_or_else(|| EMLErrorKind::MissingSelectionType.with_span(elem.inner_span()))?,
            votes,
            ranking,
            elected: elected.ok_or_else(|| {
                EMLErrorKind::MissingElement(ELECTED_EML_NAME.as_owned())
                    .with_span(elem.inner_span())
            })?,
        })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        let writer = match &self.selection_type {
            ElectionResultSelectionType::Candidate(candidate_selection) => {
                writer.child_elem(CandidateSelection::EML_NAME, candidate_selection.as_ref())?
            }
            ElectionResultSelectionType::Affiliation(affiliation_selection) => writer.child_elem(
                AffiliationSelection::EML_NAME,
                affiliation_selection.as_ref(),
            )?,
        };
        writer
            .child_option(VOTES_EML_NAME, self.votes.as_ref(), |elem, value| {
                elem.text(value.raw().as_ref())?.finish()
            })?
            .child_option(RANKING_EML_NAME, self.ranking.as_ref(), |elem, value| {
                elem.text(value.raw().as_ref())?.finish()
            })?
            .child(ELECTED_EML_NAME, |elem| {
                elem.text(self.elected.raw().as_ref())?.finish()
            })?
            .finish()
    }
}

/// The type of selection.
#[derive(Debug, Clone)]
pub enum ElectionResultSelectionType {
    /// Selection of a candidate.
    Candidate(Box<CandidateSelection>),
    /// Selection of an affiliation.
    Affiliation(Box<AffiliationSelection>),
}

/// Selection of a candidate.
#[derive(Debug, Clone)]
pub struct CandidateSelection {
    /// Identifier of the candidate.
    pub identifier: CandidateIdentifier,

    /// Name of the candidate.
    pub name: PersonNameStructure,

    /// Gender of the candidate.
    pub gender: Option<StringValue<GenderType>>,

    /// The minimal qualifying address of the candidate, if present.
    pub qualifying_address: MinimalQualifyingAddress,
}

impl EMLElement for CandidateSelection {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Candidate", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, CandidateSelection {
            identifier: CandidateIdentifier::EML_NAME => |elem| CandidateIdentifier::read_eml(elem)?,
            name: ("CandidateFullName", NS_EML) => |elem| PersonNameStructure::read_eml_element(elem)?,
            gender as Option: ("Gender", NS_EML) => |elem| elem.string_value()?,
            qualifying_address: MinimalQualifyingAddress::EML_NAME => |elem| MinimalQualifyingAddress::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(CandidateIdentifier::EML_NAME, &self.identifier)?
            .child(("CandidateFullName", NS_EML), |elem| {
                self.name.write_eml_element(elem)
            })?
            .child_option(("Gender", NS_EML), self.gender.as_ref(), |elem, value| {
                elem.text(value.raw().as_ref())?.finish()
            })?
            .child_elem(MinimalQualifyingAddress::EML_NAME, &self.qualifying_address)?
            .finish()
    }
}

/// Selection of an affiliation.
#[derive(Debug, Clone)]
pub struct AffiliationSelection {
    /// Id of the affiliation.
    pub id: StringValue<AffiliationIdType>,

    /// Name of the affiliation.
    pub name: String,
}

impl EMLElement for AffiliationSelection {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("AffiliationIdentifier", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, AffiliationSelection {
            id: elem.string_value_attr("Id", None)?,
            name: ("RegisteredName", NS_EML) => |elem| elem.text_without_children()?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr("Id", self.id.raw().as_ref())?
            .child(("RegisteredName", NS_EML), |elem| {
                elem.text(self.name.as_ref())?.finish()
            })?
            .finish()
    }
}
