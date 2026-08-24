use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysColliderManager {
    pub base: NiPSysModifier,
    pub collider_ref: BlockRef,
}

impl std::ops::Deref for NiPSysColliderManager {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
