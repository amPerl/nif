use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiGeometry;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriBasedGeom {
    pub base: NiGeometry,
}

impl NiTriBasedGeom {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
