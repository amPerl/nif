use super::ni_object_net::NiObjectNET;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiDitherProperty {
    pub base: NiObjectNET,
    pub flags: DitherFlags,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum DitherFlags {
    #[br(magic = 0u16)]
    Disabled,
    #[br(magic = 1u16)]
    Enabled,
}

impl std::ops::Deref for NiDitherProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
