use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysAgeDeathModifier {
    pub base: NiPSysModifier,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub spawn_on_death: bool,
    pub spawn_modifier_ref: BlockRef,
}

impl std::ops::Deref for NiPSysAgeDeathModifier {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
