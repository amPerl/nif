use binrw::BinRead;

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysAgeDeathModifier {
    pub base: NiPSysModifier,
    #[br(map = |x: u8| x > 0)]
    pub spawn_on_death: bool,
    pub spawn_modifier_ref: BlockRef,
}
