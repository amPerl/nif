use super::NiGeometry;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriStrips {
    pub base: NiGeometry,
}

impl NiTriStrips {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
