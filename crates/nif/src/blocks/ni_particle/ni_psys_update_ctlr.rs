use binrw::{BinRead, BinWrite};

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysUpdateCtlr {
    pub base: NiTimeController,
}

impl std::ops::Deref for NiPSysUpdateCtlr {
    type Target = NiTimeController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
