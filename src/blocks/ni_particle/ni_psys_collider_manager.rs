use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysColliderManager {
    pub base: NiPSysModifier,
    pub collider_ref: BlockRef,
}
