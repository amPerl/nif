use binrw::{BinRead, BinWrite};

use crate::blocks::{NiSingleInterpController, NiString};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysModifierCtlr {
    pub base: NiSingleInterpController,
    pub modifier_name: NiString,
}
