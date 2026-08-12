use binrw::{BinRead, BinWrite};

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysResetOnLoopCtlr {
    pub base: NiTimeController,
}
