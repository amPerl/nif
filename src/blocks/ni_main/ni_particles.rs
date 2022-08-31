use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiGeometry;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiParticles {
    pub base: NiGeometry,
}

impl NiParticles {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiParticles {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
