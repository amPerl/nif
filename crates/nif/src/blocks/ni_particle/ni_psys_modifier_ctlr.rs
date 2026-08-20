use binrw::{BinRead, BinWrite};

use crate::blocks::{NiSingleInterpController, NiString};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysModifierCtlr {
    pub base: NiSingleInterpController,
    pub modifier_name: NiString,
}
