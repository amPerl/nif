use binrw::{BinRead, BinWrite};

use super::NiInterpolator;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiKeyBasedInterpolator {
    pub base: NiInterpolator,
}
