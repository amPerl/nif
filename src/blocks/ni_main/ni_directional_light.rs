use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiLight;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiDirectionalLight {
    pub base: NiLight,
}

impl NiDirectionalLight {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
