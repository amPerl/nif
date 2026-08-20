use binrw::{BinRead, BinWrite};

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysResetOnLoopCtlr {
    pub base: NiTimeController,
}
