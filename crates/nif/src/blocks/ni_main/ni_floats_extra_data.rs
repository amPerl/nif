use super::ni_string::NiString;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatsExtraData {
    pub name: NiString,
    pub num_floats: u32,
    #[br(count = num_floats)]
    pub data: Vec<f32>,
}

impl NiFloatsExtraData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
