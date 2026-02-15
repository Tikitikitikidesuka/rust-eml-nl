//! Document variant for the EML_NL Count (`510a`, `510b`, `510c` or `510d`) document.

use std::{num::NonZeroU64, sync::LazyLock};

use regex::Regex;

use crate::{
    EML_SCHEMA_VERSION, EMLError, EMLErrorKind, EMLResultExt as _, NS_EML, NS_KR,
    common::{
        CandidateIdentifier, CanonicalizationMethod, ContestIdentifier, CreationDateTime,
        ElectionDomain, ManagingAuthority, MinimalQualifyingAddress, PersonNameStructure,
        TransactionId,
    },
    documents::accepted_root,
    io::{
        EMLElement, EMLElementReader, EMLElementWriter, EMLReadElement as _, EMLWriteElement,
        QualifiedName, collect_struct,
    },
    utils::{
        AffiliationIdType, ElectionCategory, ElectionIdType, ElectionSubcategory, GenderType,
        StringValue, StringValueData, XsDate,
    },
};

/// Representing a `510a`, `510b`, `510c` or `510d` document, containing a count.
#[derive(Debug, Clone)]
pub struct ElectionCount {
    /// Type of count document.
    pub count_type: CountType,

    /// Transaction ID of the document.
    pub transaction_id: TransactionId,

    /// Managing authority of the document.
    pub managing_authority: ManagingAuthority,

    /// Creation date and time of the document.
    pub creation_date_time: CreationDateTime,

    /// Canonicalization method used in this document, if present.
    pub canonicalization_method: Option<CanonicalizationMethod>,

    /// The actual count data.
    pub count: ElectionCountCount,
}

impl EMLElement for ElectionCount {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("EML", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        accepted_root(elem)?;

        let document_id = elem.attribute_value_req(("Id", None))?;
        let Some(count_type) = CountType::from_eml_id(document_id.as_ref()) else {
            return Err(EMLErrorKind::InvalidDocumentType(
                "510a/510b/510c/510d",
                document_id.to_string(),
            ))
            .with_span(elem.span());
        };

        Ok(collect_struct!(elem, ElectionCount {
            count_type: count_type,
            transaction_id: TransactionId::EML_NAME => |elem| TransactionId::read_eml(elem)?,
            managing_authority: ManagingAuthority::EML_NAME => |elem| ManagingAuthority::read_eml(elem)?,
            creation_date_time: CreationDateTime::EML_NAME => |elem| CreationDateTime::read_eml(elem)?,
            canonicalization_method as Option: CanonicalizationMethod::EML_NAME => |elem| CanonicalizationMethod::read_eml(elem)?,
            count: ElectionCountCount::EML_NAME => |elem| ElectionCountCount::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr(("Id", None), self.count_type.to_eml_id())?
            .attr(("SchemaVersion", None), EML_SCHEMA_VERSION)?
            .child_elem(TransactionId::EML_NAME, &self.transaction_id)?
            .child_elem(ManagingAuthority::EML_NAME, &self.managing_authority)?
            .child_elem(CreationDateTime::EML_NAME, &self.creation_date_time)?
            // Note: we don't output the CanonicalizationMethod because we aren't canonicalizing our output
            // .child_elem_option(
            //     CanonicalizationMethod::EML_NAME,
            //     self.canonicalization_method.as_ref(),
            // )?
            .child_elem(ElectionCountCount::EML_NAME, &self.count)?
            .finish()
    }
}

/// EML document ID for Count of polling stationdocuments.
pub(crate) const EML_COUNT_POLLING_STATION_ID: &str = "510a";

/// EML document ID for Count of municipality documents.
pub(crate) const EML_COUNT_MUNICIPAL_ID: &str = "510b";

/// EML document ID for Count of district documents.
pub(crate) const EML_COUNT_DISTRICT_ID: &str = "510c";

/// EML document ID for central Count documents.
pub(crate) const EML_COUNT_CENTRAL_ID: &str = "510d";

/// Type of Count document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountType {
    /// Representing a `510a` document, containing the count for a polling station.
    PollingStation,
    /// Representing a `510b` document, containing the count for a local region (= municipality).
    Municipal,
    /// Representing a `510c` document, containing the count for a district (= HSB).
    District,
    /// Representing a `510d` document, containing the count for the entire election.
    Central,
}

impl CountType {
    /// Create a CountType from an EML document ID string.
    pub fn from_eml_id(s: &str) -> Option<Self> {
        match s {
            EML_COUNT_POLLING_STATION_ID => Some(CountType::PollingStation),
            EML_COUNT_MUNICIPAL_ID => Some(CountType::Municipal),
            EML_COUNT_DISTRICT_ID => Some(CountType::District),
            EML_COUNT_CENTRAL_ID => Some(CountType::Central),
            _ => None,
        }
    }

    /// Get the EML document ID string for this CountType.
    pub fn to_eml_id(&self) -> &'static str {
        match self {
            CountType::PollingStation => EML_COUNT_POLLING_STATION_ID,
            CountType::Municipal => EML_COUNT_MUNICIPAL_ID,
            CountType::District => EML_COUNT_DISTRICT_ID,
            CountType::Central => EML_COUNT_CENTRAL_ID,
        }
    }

    /// Get a friendly name for this CountType.
    pub fn to_friendly_name(&self) -> &'static str {
        match self {
            CountType::PollingStation => "Polling Station Count",
            CountType::Municipal => "Municipal Count",
            CountType::District => "District Count",
            CountType::Central => "Central Count",
        }
    }

    /// Returns if the given EML document ID string is a valid CountType ID.
    pub fn is_valid_eml_id(s: &str) -> bool {
        matches!(
            s,
            EML_COUNT_POLLING_STATION_ID
                | EML_COUNT_MUNICIPAL_ID
                | EML_COUNT_DISTRICT_ID
                | EML_COUNT_CENTRAL_ID
        )
    }
}

/// The actual count data.
#[derive(Debug, Clone)]
pub struct ElectionCountCount {
    /// The election for this count.
    pub election: ElectionCountElection,
}

impl EMLElement for ElectionCountCount {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Count", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionCountCount {
            id as None: ("EventIdentifier", NS_EML) => |elem| elem.skip().map(|_| ())?,
            election: ElectionCountElection::EML_NAME => |elem| ElectionCountElection::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child(("EventIdentifier", NS_EML), |w| w.empty())?
            .child_elem(ElectionCountElection::EML_NAME, &self.election)?
            .finish()
    }
}

/// The election for this count.
#[derive(Debug, Clone)]
pub struct ElectionCountElection {
    /// Identifier
    pub identifier: ElectionCountElectionIdentifier,

    /// Contests within this election.
    pub contests: Vec<ElectionCountContest>,
}

impl EMLElement for ElectionCountElection {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Election", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionCountElection {
            identifier: ElectionCountElectionIdentifier::EML_NAME => |elem| ElectionCountElectionIdentifier::read_eml(elem)?,
            contests: ("Contests", NS_EML) => |elem| {
                struct VecCollector {
                    contests: Vec<ElectionCountContest>,
                }

                let data = collect_struct!(elem, VecCollector {
                    contests as Vec: ElectionCountContest::EML_NAME => |elem| ElectionCountContest::read_eml(elem)?,
                });

                data.contests
            },
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(ElectionCountElectionIdentifier::EML_NAME, &self.identifier)?
            .child(("Contests", NS_EML), |writer| {
                writer
                    .child_elems(ElectionCountContest::EML_NAME, &self.contests)?
                    .finish()
            })?
            .finish()
    }
}

/// Identifier for the election in this count.
#[derive(Debug, Clone)]
pub struct ElectionCountElectionIdentifier {
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

impl EMLElement for ElectionCountElectionIdentifier {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ElectionIdentifier", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionCountElectionIdentifier {
            id: elem.string_value_attr("Id", None)?,
            name as Option: ("ElectionName", NS_EML) => |elem| elem.text_without_children()?,
            category: ("ElectionCategory", NS_EML) => |elem| elem.string_value()?,
            subcategory as Option: ("ElectionSubcategory", NS_KR) => |elem| elem.string_value()?,
            domain as Option: ElectionDomain::EML_NAME => |elem| ElectionDomain::read_eml(elem)?,
            election_date: ("ElectionDate", NS_KR) => |elem| elem.string_value()?,
        }))
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

/// A contest within the election count.
#[derive(Debug, Clone)]
pub struct ElectionCountContest {
    /// Identifier for the contest.
    pub identifier: ContestIdentifier,

    /// Total votes in this contest, if present.
    pub total_votes: Option<TotalVotes>,

    /// Votes per reporting unit in this contest.
    pub reporting_unit_votes: Vec<ReportingUnitVotes>,
}

impl EMLElement for ElectionCountContest {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Contest", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ElectionCountContest {
            identifier: ContestIdentifier::EML_NAME => |elem| ContestIdentifier::read_eml(elem)?,
            total_votes as Option: TotalVotes::EML_NAME => |elem| TotalVotes::read_eml(elem)?,
            reporting_unit_votes as Vec: ReportingUnitVotes::EML_NAME => |elem| ReportingUnitVotes::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(ContestIdentifier::EML_NAME, &self.identifier)?
            .child_elem_option(TotalVotes::EML_NAME, self.total_votes.as_ref())?
            .child_elems(ReportingUnitVotes::EML_NAME, &self.reporting_unit_votes)?
            .finish()
    }
}

/// Total votes in a contest.
#[derive(Debug, Clone)]
pub struct TotalVotes {
    /// Selections within the total votes.
    pub selections: Vec<ElectionCountSelection>,

    /// Total number of eligible voters within the reporting unit votes.
    ///
    /// This element is called `Cast` within EML_NL, but is renamed here to
    /// `eligible_voter_count` for clarity. This element was repurposed in
    /// EML_NL to represent the total number of eligible voters instead of the
    /// actual number of cast votes.
    pub eligible_voter_count: StringValue<u64>,

    /// Total number of votes on candidates.
    ///
    /// This element is called `TotalCounted` within EML_NL, but is renamed here
    /// to `candidate_votes_count` for clarity. This element was repurposed in
    /// EML_NL to represent the total number of votes on candidates instead of
    /// the actual total number of counted votes.
    pub candidate_votes_count: StringValue<u64>,

    /// Rejected blank votes within the reporting unit votes.
    ///
    /// In EML_NL this element is called `RejectedVotes`, with a `ReasonCode`
    /// attribute set to the value `blanco`.
    pub rejected_votes_blank: StringValue<u64>,

    /// Rejected invalid votes within the reporting unit votes.
    ///
    /// In EML_NL this element is called `RejectedVotes`, with a `ReasonCode`
    /// attribute set to the value `ongeldig`.
    pub rejected_votes_invalid: StringValue<u64>,

    /// Uncounted votes reasons within the reporting unit votes.
    pub uncounted_votes: Vec<UncountedVotes>,
}

impl EMLElement for TotalVotes {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("TotalVotes", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, TotalVotes {
            selections as Vec: ElectionCountSelection::EML_NAME => |elem| ElectionCountSelection::read_eml(elem)?,
            eligible_voter_count: ("Cast", NS_EML) => |elem| elem.string_value()?,
            candidate_votes_count: ("TotalCounted", NS_EML) => |elem| elem.string_value()?,
            rejected_votes_blank as Mapped: ("RejectedVotes", NS_EML) => |elem| {
                if elem.attribute_value("ReasonCode")?.as_deref() == Some("blanco") {
                    Some(elem.string_value()?)
                } else {
                    None
                }
            } else EMLErrorKind::MissingRejectedVotesBlank,
            rejected_votes_invalid as Mapped: ("RejectedVotes", NS_EML) => |elem| {
                if elem.attribute_value("ReasonCode")?.as_deref() == Some("ongeldig") {
                    Some(elem.string_value()?)
                } else {
                    None
                }
            } else EMLErrorKind::MissingRejectedVotesInvalid,
            uncounted_votes as Vec: UncountedVotes::EML_NAME => |elem| UncountedVotes::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elems(ElectionCountSelection::EML_NAME, &self.selections)?
            .child(("Cast", NS_EML), |elem| {
                elem.text(self.eligible_voter_count.raw().as_ref())?
                    .finish()
            })?
            .child(("TotalCounted", NS_EML), |elem| {
                elem.text(self.candidate_votes_count.raw().as_ref())?
                    .finish()
            })?
            .child(("RejectedVotes", NS_EML), |elem| {
                elem.attr("ReasonCode", "blanco")?
                    .text(self.rejected_votes_blank.raw().as_ref())?
                    .finish()
            })?
            .child(("RejectedVotes", NS_EML), |elem| {
                elem.attr("ReasonCode", "ongeldig")?
                    .text(self.rejected_votes_invalid.raw().as_ref())?
                    .finish()
            })?
            .child_elems(UncountedVotes::EML_NAME, &self.uncounted_votes)?
            .finish()
    }
}

/// Votes per reporting unit.
#[derive(Debug, Clone)]
pub struct ReportingUnitVotes {
    /// Identifier for the reporting unit votes.
    pub identifier: ReportingUnitIdentifier,

    /// Selections within the reporting unit votes.
    pub selections: Vec<ElectionCountSelection>,

    /// Total number of eligible voters within the reporting unit votes.
    ///
    /// This element is called `Cast` within EML_NL, but is renamed here to
    /// `eligible_voter_count` for clarity. This element was repurposed in
    /// EML_NL to represent the total number of eligible voters instead of the
    /// actual number of cast votes.
    pub eligible_voter_count: StringValue<u64>,

    /// Total number of votes on candidates.
    ///
    /// This element is called `TotalCounted` within EML_NL, but is renamed here
    /// to `candidate_votes_count` for clarity. This element was repurposed in
    /// EML_NL to represent the total number of votes on candidates instead of
    /// the actual total number of counted votes.
    pub candidate_votes_count: StringValue<u64>,

    /// Rejected blank votes within the reporting unit votes.
    ///
    /// In EML_NL this element is called `RejectedVotes`, with a `ReasonCode`
    /// attribute set to the value `blanco`.
    pub rejected_votes_blank: StringValue<u64>,

    /// Rejected invalid votes within the reporting unit votes.
    ///
    /// In EML_NL this element is called `RejectedVotes`, with a `ReasonCode`
    /// attribute set to the value `ongeldig`.
    pub rejected_votes_invalid: StringValue<u64>,

    /// Uncounted votes reasons within the reporting unit votes.
    pub uncounted_votes: Vec<UncountedVotes>,

    /// Investigations within the reporting unit votes.
    pub investigations: Option<ReportingUnitInvestigations>,
}

impl EMLElement for ReportingUnitVotes {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ReportingUnitVotes", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ReportingUnitVotes {
            identifier: ReportingUnitIdentifier::EML_NAME => |elem| ReportingUnitIdentifier::read_eml(elem)?,
            selections as Vec: ElectionCountSelection::EML_NAME => |elem| ElectionCountSelection::read_eml(elem)?,
            eligible_voter_count: ("Cast", NS_EML) => |elem| elem.string_value()?,
            candidate_votes_count: ("TotalCounted", NS_EML) => |elem| elem.string_value()?,
            rejected_votes_blank as Mapped: ("RejectedVotes", NS_EML) => |elem| {
                if elem.attribute_value("ReasonCode")?.as_deref() == Some("blanco") {
                    Some(elem.string_value()?)
                } else {
                    None
                }
            } else EMLErrorKind::MissingRejectedVotesBlank,
            rejected_votes_invalid as Mapped: ("RejectedVotes", NS_EML) => |elem| {
                if elem.attribute_value("ReasonCode")?.as_deref() == Some("ongeldig") {
                    Some(elem.string_value()?)
                } else {
                    None
                }
            } else EMLErrorKind::MissingRejectedVotesInvalid,
            uncounted_votes as Vec: UncountedVotes::EML_NAME => |elem| UncountedVotes::read_eml(elem)?,
            investigations as Option: ReportingUnitInvestigations::EML_NAME => |elem| ReportingUnitInvestigations::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(ReportingUnitIdentifier::EML_NAME, &self.identifier)?
            .child_elems(ElectionCountSelection::EML_NAME, &self.selections)?
            .child(("Cast", NS_EML), |elem| {
                elem.text(self.eligible_voter_count.raw().as_ref())?
                    .finish()
            })?
            .child(("TotalCounted", NS_EML), |elem| {
                elem.text(self.candidate_votes_count.raw().as_ref())?
                    .finish()
            })?
            .child(("RejectedVotes", NS_EML), |elem| {
                elem.attr("ReasonCode", "blanco")?
                    .text(self.rejected_votes_blank.raw().as_ref())?
                    .finish()
            })?
            .child(("RejectedVotes", NS_EML), |elem| {
                elem.attr("ReasonCode", "ongeldig")?
                    .text(self.rejected_votes_invalid.raw().as_ref())?
                    .finish()
            })?
            .child_elems(UncountedVotes::EML_NAME, &self.uncounted_votes)?
            .child_elem_option(
                ReportingUnitInvestigations::EML_NAME,
                self.investigations.as_ref(),
            )?
            .finish()
    }
}

/// Identifier for the reporting unit votes.
#[derive(Debug, Clone)]
pub struct ReportingUnitIdentifier {
    /// Id of the reporting unit
    pub id: StringValue<ReportingUnitIdentifierIdType>,

    /// Name of the reporting unit
    pub name: String,
}

impl EMLElement for ReportingUnitIdentifier {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ReportingUnitIdentifier", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(ReportingUnitIdentifier {
            id: elem.string_value_attr("Id", None)?,
            name: elem.text_without_children()?,
        })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr("Id", self.id.raw().as_ref())?
            .text(self.name.as_ref())?
            .finish()
    }
}

/// Type for ReportingUnitVotesIdentifier id.
#[derive(Debug, Clone)]
pub struct ReportingUnitIdentifierIdType(String);

/// Regular expression for validating ReportingUnitVotesIdentifier id values.
static REPORTING_UNIT_VOTES_IDENTIFIER_TYPE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^((HSB\d+)|((HSB\d+::)?\d{4})|(((HSB\d+::)?\d{4}::)?SB\d+)|(HSB\d+::SB\d+))$")
        .expect("Failed to compile ReportingUnitVotesIdentifier id regex")
});

impl ReportingUnitIdentifierIdType {
    /// Create a new ReportingUnitVotesIdentifierType from a string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, InvalidReportingUnitVotesIdentifierType> {
        ReportingUnitIdentifierIdType::parse_from_str(s.as_ref())
    }

    /// Get the raw string value of the ReportingUnitVotesIdentifierType.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Error indicating an invalid ReportingUnitVotesIdentifierType.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid reporting unit votes identifier type: {0}")]
pub struct InvalidReportingUnitVotesIdentifierType(String);

impl StringValueData for ReportingUnitIdentifierIdType {
    type Error = InvalidReportingUnitVotesIdentifierType;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error> {
        if REPORTING_UNIT_VOTES_IDENTIFIER_TYPE_RE.is_match(s) {
            Ok(ReportingUnitIdentifierIdType(s.to_string()))
        } else {
            Err(InvalidReportingUnitVotesIdentifierType(s.to_string()))
        }
    }

    fn to_raw_value(&self) -> String {
        self.0.clone()
    }
}

/// Investigations within the reporting unit.
#[derive(Debug, Clone)]
pub struct ReportingUnitInvestigations {
    /// Investigations within the reporting unit.
    pub investigations: Vec<Investigation>,
}

impl EMLElement for ReportingUnitInvestigations {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ReportingUnitInvestigations", Some(NS_KR));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, ReportingUnitInvestigations {
            investigations as Vec: Investigation::EML_NAME => |elem| Investigation::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elems(Investigation::EML_NAME, &self.investigations)?
            .finish()
    }
}

/// An investigation within the reporting unit.
#[derive(Debug, Clone)]
pub struct Investigation {
    /// Whether the type of investigation as specified by reason was conducted.
    pub investigated: StringValue<bool>,
    /// Reason for the investigation.
    pub reason: StringValue<InvestigationReason>,
}

impl EMLElement for Investigation {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("Investigation", Some(NS_KR));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(
            elem,
            Investigation {
                investigated: elem.string_value()?,
                reason: elem.string_value_attr("ReasonCode", None)?,
            }
        ))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr("ReasonCode", self.reason.raw().as_ref())?
            .text(self.investigated.raw().as_ref())?
            .finish()
    }
}

/// Reason code for a specific investigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvestigationReason {
    /// onderzocht vanwege onverklaard verschil
    UnexplainedDifference,
    /// onderzocht vanwege andere fout
    OtherError,
    /// uitslag gecorrigeerd
    ResultCorrected,
    /// toegelaten kiezers opnieuw vastgesteld
    AdmittedVotersReestablished,
    /// onderzocht vanwege andere reden
    OtherReason,
    /// stembiljetten deels herteld
    PartiallyRecountedBallots,
}

impl InvestigationReason {
    /// Create an InvestigationReason from an EML reason code string.
    pub fn from_eml_code(s: &str) -> Option<Self> {
        match s {
            "onderzocht vanwege onverklaard verschil" => {
                Some(InvestigationReason::UnexplainedDifference)
            }
            "onderzocht vanwege andere fout" => Some(InvestigationReason::OtherError),
            "uitslag gecorrigeerd" => Some(InvestigationReason::ResultCorrected),
            "toegelaten kiezers opnieuw vastgesteld" => {
                Some(InvestigationReason::AdmittedVotersReestablished)
            }
            "onderzocht vanwege andere reden" => Some(InvestigationReason::OtherReason),
            "stembiljetten deels herteld" => Some(InvestigationReason::PartiallyRecountedBallots),
            _ => None,
        }
    }

    /// Get the EML reason code string for this InvestigationReason.
    pub fn to_eml_code(&self) -> &'static str {
        match self {
            InvestigationReason::UnexplainedDifference => "onderzocht vanwege onverklaard verschil",
            InvestigationReason::OtherError => "onderzocht vanwege andere fout",
            InvestigationReason::ResultCorrected => "uitslag gecorrigeerd",
            InvestigationReason::AdmittedVotersReestablished => {
                "toegelaten kiezers opnieuw vastgesteld"
            }
            InvestigationReason::OtherReason => "onderzocht vanwege andere reden",
            InvestigationReason::PartiallyRecountedBallots => "stembiljetten deels herteld",
        }
    }
}

/// Error indicating an invalid InvestigationReason.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid investigation reason: {0}")]
pub struct InvalidInvestigationReason(String);

impl StringValueData for InvestigationReason {
    type Error = InvalidInvestigationReason;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error> {
        InvestigationReason::from_eml_code(s)
            .ok_or_else(|| InvalidInvestigationReason(s.to_string()))
    }

    fn to_raw_value(&self) -> String {
        self.to_eml_code().to_string()
    }
}

/// Uncounted votes reasons within the reporting unit votes.
#[derive(Debug, Clone)]
pub struct UncountedVotes {
    /// Number of uncounted votes.
    pub value: StringValue<u64>,
    /// Reason for uncounted votes.
    pub reason: StringValue<UncountedVotesReason>,
}

impl EMLElement for UncountedVotes {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("UncountedVotes", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(UncountedVotes {
            value: elem.string_value()?,
            reason: elem.string_value_attr("ReasonCode", None)?,
        })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr("ReasonCode", self.reason.raw().as_ref())?
            .text(self.value.raw().as_ref())?
            .finish()
    }
}

/// Reason code for a specific uncounted votes entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UncountedVotesReason {
    /// geldige stempassen
    ValidPollCards,
    /// geldige volmachtbewijzen
    ValidProxyCertificates,
    /// geldige kiezerspassen
    ValidVoterCards,
    /// toegelaten kiezers
    AdmittedVoters,
    /// meer getelde stembiljetten
    MoreBallotsCounted,
    /// minder getelde stembiljetten
    FewerBallotsCounted,
    /// meegenomen stembiljetten
    BallotsTaken,
    /// te weinig uitgereikte stembiljetten
    TooFewBallotsIssued,
    /// te veel uitgereikte stembiljetten
    TooManyBallotsIssued,
    /// geen briefstembiljetten
    NoPostalBallots,
    /// te veel briefstembiljetten
    TooManyPostalBallots,
    /// kwijtgeraakte stembiljetten
    LostBallots,
    /// geen verklaring
    NoExplanation,
    /// andere verklaring
    OtherExplanation,
}

impl UncountedVotesReason {
    /// Create an UncountedVotesReason from an EML reason code string.
    pub fn from_eml_code(s: &str) -> Option<Self> {
        match s {
            "geldige stempassen" => Some(UncountedVotesReason::ValidPollCards),
            "geldige volmachtbewijzen" => Some(UncountedVotesReason::ValidProxyCertificates),
            "geldige kiezerspassen" => Some(UncountedVotesReason::ValidVoterCards),
            "toegelaten kiezers" => Some(UncountedVotesReason::AdmittedVoters),
            "meer getelde stembiljetten" => Some(UncountedVotesReason::MoreBallotsCounted),
            "minder getelde stembiljetten" => Some(UncountedVotesReason::FewerBallotsCounted),
            "meegenomen stembiljetten" => Some(UncountedVotesReason::BallotsTaken),
            "te weinig uitgereikte stembiljetten" => {
                Some(UncountedVotesReason::TooFewBallotsIssued)
            }
            "te veel uitgereikte stembiljetten" => Some(UncountedVotesReason::TooManyBallotsIssued),
            "geen briefstembiljetten" => Some(UncountedVotesReason::NoPostalBallots),
            "te veel briefstembiljetten" => Some(UncountedVotesReason::TooManyPostalBallots),
            "kwijtgeraakte stembiljetten" => Some(UncountedVotesReason::LostBallots),
            "geen verklaring" => Some(UncountedVotesReason::NoExplanation),
            "andere verklaring" => Some(UncountedVotesReason::OtherExplanation),
            _ => None,
        }
    }

    /// Get the EML reason code string for this UncountedVotesReason.
    pub fn to_eml_code(&self) -> &'static str {
        match self {
            UncountedVotesReason::ValidPollCards => "geldige stempassen",
            UncountedVotesReason::ValidProxyCertificates => "geldige volmachtbewijzen",
            UncountedVotesReason::ValidVoterCards => "geldige kiezerspassen",
            UncountedVotesReason::AdmittedVoters => "toegelaten kiezers",
            UncountedVotesReason::MoreBallotsCounted => "meer getelde stembiljetten",
            UncountedVotesReason::FewerBallotsCounted => "minder getelde stembiljetten",
            UncountedVotesReason::BallotsTaken => "meegenomen stembiljetten",
            UncountedVotesReason::TooFewBallotsIssued => "te weinig uitgereikte stembiljetten",
            UncountedVotesReason::TooManyBallotsIssued => "te veel uitgereikte stembiljetten",
            UncountedVotesReason::NoPostalBallots => "geen briefstembiljetten",
            UncountedVotesReason::TooManyPostalBallots => "te veel briefstembiljetten",
            UncountedVotesReason::LostBallots => "kwijtgeraakte stembiljetten",
            UncountedVotesReason::NoExplanation => "geen verklaring",
            UncountedVotesReason::OtherExplanation => "andere verklaring",
        }
    }
}

/// Error indicating an invalid uncounted votes reason.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid uncounted votes reason: {0}")]
pub struct InvalidUncountedVotesReason(String);

impl StringValueData for UncountedVotesReason {
    type Error = InvalidUncountedVotesReason;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        UncountedVotesReason::from_eml_code(s)
            .ok_or_else(|| InvalidUncountedVotesReason(s.to_string()))
    }

    fn to_raw_value(&self) -> String {
        self.to_eml_code().to_string()
    }
}

/// A selection within the reporting unit votes.
#[derive(Debug, Clone)]
pub struct ElectionCountSelection {
    /// Type of selection.
    pub selection_type: ElectionCountSelectionType,
    /// Number of valid votes for this selection.
    pub valid_votes: StringValue<u64>,
    /// Value of the `Value` attribute, if present.
    pub value: Option<String>,
    /// Value of the `Category` attribute, if present.
    pub category: Option<String>,
}

const VALID_VOTES_EML_NAME: QualifiedName<'_, '_> =
    QualifiedName::from_static("ValidVotes", Some(NS_EML));

impl EMLElement for ElectionCountSelection {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Selection", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        let value = elem.attribute_value("Value")?.map(|s| s.to_string());
        let category = elem.attribute_value("Category")?.map(|s| s.to_string());
        let mut selection_type = None;
        let mut valid_votes = None;

        while let Some(mut child) = elem.next_child()? {
            let name = child.name()?;

            match name {
                n if n == CandidateSelection::EML_NAME => {
                    selection_type = Some(ElectionCountSelectionType::Candidate(Box::new(
                        CandidateSelection::read_eml(&mut child)?,
                    )));
                }
                n if n == AffiliationSelection::EML_NAME => {
                    selection_type = Some(ElectionCountSelectionType::Affiliation(Box::new(
                        AffiliationSelection::read_eml(&mut child)?,
                    )));
                }
                n if n == ReferendumOptionSelection::EML_NAME => {
                    selection_type = Some(ElectionCountSelectionType::ReferendumOption(Box::new(
                        ReferendumOptionSelection::read_eml(&mut child)?,
                    )));
                }
                n if n == VALID_VOTES_EML_NAME => {
                    valid_votes = Some(child.string_value()?);
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
        Ok(ElectionCountSelection {
            selection_type: selection_type
                .ok_or_else(|| EMLErrorKind::MissingSelectionType.with_span(elem.inner_span()))?,
            valid_votes: valid_votes.ok_or_else(|| {
                EMLErrorKind::MissingElement(VALID_VOTES_EML_NAME.as_owned())
                    .with_span(elem.inner_span())
            })?,
            value,
            category,
        })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        let writer = writer
            .attr_opt("Value", self.value.as_deref())?
            .attr_opt("Category", self.category.as_deref())?;
        let writer = match &self.selection_type {
            ElectionCountSelectionType::Candidate(candidate_selection) => {
                writer.child_elem(CandidateSelection::EML_NAME, candidate_selection.as_ref())?
            }
            ElectionCountSelectionType::Affiliation(affiliation_selection) => writer.child_elem(
                AffiliationSelection::EML_NAME,
                affiliation_selection.as_ref(),
            )?,
            ElectionCountSelectionType::ReferendumOption(referendum_option_selection) => writer
                .child_elem(
                    ReferendumOptionSelection::EML_NAME,
                    referendum_option_selection.as_ref(),
                )?,
        };
        writer
            .child(("ValidVotes", NS_EML), |elem| {
                elem.text(self.valid_votes.raw().as_ref())?.finish()
            })?
            .finish()
    }
}

/// The type of selection.
#[derive(Debug, Clone)]
pub enum ElectionCountSelectionType {
    /// Selection of a candidate.
    Candidate(Box<CandidateSelection>),
    /// Selection of an affiliation.
    Affiliation(Box<AffiliationSelection>),
    /// Selection of a referendum option.
    ReferendumOption(Box<ReferendumOptionSelection>),
}

/// Selection of a candidate.
#[derive(Debug, Clone)]
pub struct CandidateSelection {
    /// Identifier of the candidate.
    pub identifier: CandidateIdentifier,

    /// Name of the candidate.
    pub name: Option<PersonNameStructure>,

    /// Gender of the candidate.
    pub gender: Option<StringValue<GenderType>>,

    /// Qualified address of the candidate, if present.
    pub qualified_address: Option<MinimalQualifyingAddress>,
}

impl EMLElement for CandidateSelection {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Candidate", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(collect_struct!(elem, CandidateSelection {
            identifier: CandidateIdentifier::EML_NAME => |elem| CandidateIdentifier::read_eml(elem)?,
            name as Option: ("CandidateFullName", NS_EML) => |elem| PersonNameStructure::read_eml_element(elem)?,
            gender as Option: ("Gender", NS_EML) => |elem| elem.string_value()?,
            qualified_address as Option: MinimalQualifyingAddress::EML_NAME => |elem| MinimalQualifyingAddress::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .child_elem(CandidateIdentifier::EML_NAME, &self.identifier)?
            .child_option(
                ("CandidateFullName", NS_EML),
                self.name.as_ref(),
                |elem, value| value.write_eml_element(elem),
            )?
            .child_option(("Gender", NS_EML), self.gender.as_ref(), |elem, value| {
                elem.text(value.raw().as_ref())?.finish()
            })?
            .child_elem_option(
                MinimalQualifyingAddress::EML_NAME,
                self.qualified_address.as_ref(),
            )?
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

/// Selection of a referendum option.
#[derive(Debug, Clone)]
pub struct ReferendumOptionSelection {
    /// Value of the referendum option.
    pub value: String,
    /// Id of the referendum option, if present.
    pub id: Option<String>,
    /// Display order of the referendum option, if present.
    pub display_order: Option<StringValue<NonZeroU64>>,
    /// Short code of the referendum option, if present.
    pub short_code: Option<String>,
    /// Expected confirmation reference of the referendum option, if present.
    pub expected_confirmation_reference: Option<String>,
}

impl EMLElement for ReferendumOptionSelection {
    const EML_NAME: QualifiedName<'_, '_> =
        QualifiedName::from_static("ReferendumOptionIdentifier", Some(NS_EML));

    fn read_eml(elem: &mut EMLElementReader<'_, '_>) -> Result<Self, EMLError> {
        Ok(ReferendumOptionSelection {
            value: elem.text_without_children()?,
            id: elem.attribute_value("Id")?.map(|s| s.into_owned()),
            display_order: elem.string_value_attr_opt("DisplayOrder")?,
            short_code: elem.attribute_value("ShortCode")?.map(|s| s.into_owned()),
            expected_confirmation_reference: elem
                .attribute_value("ExpectedConfirmationReference")?
                .map(|s| s.into_owned()),
        })
    }

    fn write_eml(&self, writer: EMLElementWriter) -> Result<(), EMLError> {
        writer
            .attr_opt("Id", self.id.as_deref())?
            .attr_opt("DisplayOrder", self.display_order.as_ref().map(|s| s.raw()))?
            .attr_opt("ShortCode", self.short_code.as_deref())?
            .attr_opt(
                "ExpectedConfirmationReference",
                self.expected_confirmation_reference.as_deref(),
            )?
            .text(self.value.as_ref())?
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporting_unit_votes_identifier_type_regex_compiles() {
        LazyLock::force(&REPORTING_UNIT_VOTES_IDENTIFIER_TYPE_RE);
    }
}
