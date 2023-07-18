use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBoolData {
    pub data: KeyGroup<u8>,
}

impl NiBoolData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
