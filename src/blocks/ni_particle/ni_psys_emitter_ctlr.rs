use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysModifierCtlr;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysEmitterCtlr {
    pub base: NiPSysModifierCtlr,
    pub visibility_interpolator_ref: BlockRef,
}
