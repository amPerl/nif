use binrw::BinRead;

use crate::{blocks::NiString, common::BlockRef};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifier {
    pub name: NiString,
    pub order: u32, // NiPSysModifierOrder
    pub target_ref: BlockRef,
    #[br(map = |x: u8| x > 0)]
    pub active: bool,
}
