use binrw::BinRead;

use crate::blocks::{NiSingleInterpController, NiString};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifierCtlr {
    pub base: NiSingleInterpController,
    pub modifier_name: NiString,
}
