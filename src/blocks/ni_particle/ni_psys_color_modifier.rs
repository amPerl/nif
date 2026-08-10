use binrw::BinRead;

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysColorModifier {
    pub base: NiPSysModifier,
    pub data_ref: BlockRef,
}
