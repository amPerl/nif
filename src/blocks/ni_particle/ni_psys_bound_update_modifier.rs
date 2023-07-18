use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysBoundUpdateModifier {
    pub base: NiPSysModifier,
    pub update_skip: u16,
}

impl NiPSysBoundUpdateModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
