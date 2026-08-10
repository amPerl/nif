use binrw::BinRead;

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysUpdateCtlr {
    pub base: NiTimeController,
}
