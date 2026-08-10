use binrw::BinRead;

use crate::common::BlockRef;

use super::NiPSysModifierCtlr;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysEmitterCtlr {
    pub base: NiPSysModifierCtlr,
    pub visibility_interpolator_ref: BlockRef,
}
