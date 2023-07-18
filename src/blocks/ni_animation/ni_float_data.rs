use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatData {
    pub data: KeyGroup<f32>,
}

impl NiFloatData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
