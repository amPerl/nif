use super::ni_object_net::NiObjectNET;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiShadeProperty {
    pub base: NiObjectNET,
    pub flags: ShadeFlags,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum ShadeFlags {
    #[br(magic = 0u16)]
    Hard,
    #[br(magic = 1u16)]
    Smooth,
}

impl std::ops::Deref for NiShadeProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
