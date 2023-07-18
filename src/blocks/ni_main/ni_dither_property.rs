use super::ni_object_net::NiObjectNET;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

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

impl NiDitherProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiDitherProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
