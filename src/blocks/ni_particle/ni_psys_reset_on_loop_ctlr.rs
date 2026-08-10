use binrw::BinRead;

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysResetOnLoopCtlr {
    pub base: NiTimeController,
}
