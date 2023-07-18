use super::ni_object_net::NiObjectNET;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

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

impl NiShadeProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiShadeProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
