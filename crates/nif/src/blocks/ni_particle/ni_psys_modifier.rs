use binrw::{BinRead, BinWrite};

use crate::{blocks::NiString, common::BlockRef};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysModifier {
    pub name: NiString,
    pub order: u32, // NiPSysModifierOrder
    pub target_ref: BlockRef,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub active: bool,
}
