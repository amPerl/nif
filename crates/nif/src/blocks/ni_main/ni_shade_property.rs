use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiShadeProperty {
    pub base: NiObjectNET,
    pub flags: ShadeFlags,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum ShadeFlags {
    #[brw(magic = 0u16)]
    Hard,
    #[brw(magic = 1u16)]
    Smooth,
}

impl std::ops::Deref for NiShadeProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
