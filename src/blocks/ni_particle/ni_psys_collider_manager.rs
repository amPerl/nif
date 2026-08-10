use binrw::BinRead;

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysColliderManager {
    pub base: NiPSysModifier,
    pub collider_ref: BlockRef,
}
