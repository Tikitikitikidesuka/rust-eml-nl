use crate::{
    NS_KR,
    io::{EMLElement, QualifiedName, collect_struct},
};

/// Election tree as defined in EML_NL.
#[derive(Debug, Clone)]
pub struct ElectionTree {
    /// Regions defined for this part of the election tree
    pub regions: Vec<ElectionTreeRegion>,
}

impl ElectionTree {
    /// Create a new election tree with the given regions.
    pub fn new(regions: impl Into<Vec<ElectionTreeRegion>>) -> Self {
        ElectionTree {
            regions: regions.into(),
        }
    }
}

impl From<Vec<ElectionTreeRegion>> for ElectionTree {
    fn from(regions: Vec<ElectionTreeRegion>) -> Self {
        ElectionTree::new(regions)
    }
}

impl EMLElement for ElectionTree {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("ElectionTree", Some(NS_KR));

    fn read_eml(elem: &mut crate::io::EMLElementReader<'_, '_>) -> Result<Self, crate::EMLError>
    where
        Self: Sized,
    {
        Ok(collect_struct!(elem, ElectionTree {
            regions as Vec: ElectionTreeRegion::EML_NAME => |elem| ElectionTreeRegion::read_eml(elem)?,
        }))
    }

    fn write_eml(&self, writer: crate::io::EMLElementWriter) -> Result<(), crate::EMLError> {
        writer
            .child_elems(ElectionTreeRegion::EML_NAME, &self.regions)?
            .finish()
    }
}

/// A region in the election tree.
#[derive(Debug, Clone)]
pub struct ElectionTreeRegion {}

impl EMLElement for ElectionTreeRegion {
    const EML_NAME: QualifiedName<'_, '_> = QualifiedName::from_static("Region", Some(NS_KR));

    fn read_eml(elem: &mut crate::io::EMLElementReader<'_, '_>) -> Result<Self, crate::EMLError>
    where
        Self: Sized,
    {
        // TODO: handle election tree region
        elem.skip()?;
        Ok(ElectionTreeRegion {})
    }

    fn write_eml(&self, writer: crate::io::EMLElementWriter) -> Result<(), crate::EMLError> {
        // TODO: write election tree region
        writer.empty()
    }
}
