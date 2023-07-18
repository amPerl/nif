use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::{Color4, KeyGroup};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiColorData {
    pub data: KeyGroup<Color4>,
}

impl NiColorData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
