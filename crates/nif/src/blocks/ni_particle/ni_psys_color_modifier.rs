use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysColorModifier {
    pub base: NiPSysModifier,
    pub data_ref: BlockRef,
}

impl std::ops::Deref for NiPSysColorModifier {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
