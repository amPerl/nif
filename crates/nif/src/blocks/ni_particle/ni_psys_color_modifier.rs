use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysColorModifier {
    pub base: NiPSysModifier,
    pub data_ref: BlockRef,
}
