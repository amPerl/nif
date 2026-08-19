use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiDitherProperty {
    pub base: NiObjectNET,
    pub flags: DitherFlags,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum DitherFlags {
    #[brw(magic = 0u16)]
    Disabled,
    #[brw(magic = 1u16)]
    Enabled,
}

impl std::ops::Deref for NiDitherProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
