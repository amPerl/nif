use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysBoundUpdateModifier {
    pub base: NiPSysModifier,
    pub update_skip: u16,
}
