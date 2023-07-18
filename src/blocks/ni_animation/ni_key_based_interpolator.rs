use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiKeyBasedInterpolator {
    pub base: NiInterpolator,
}

impl NiKeyBasedInterpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
