use super::ni_object_net::NiObjectNET;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAlphaProperty {
    pub base: NiObjectNET,
    pub flags: u16,
    pub threshold: u8,
}

impl NiAlphaProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
