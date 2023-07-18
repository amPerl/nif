use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiSourceTexture;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSourceCubeMap {
    pub base: NiSourceTexture,
}

impl NiSourceCubeMap {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
