use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysPositionModifier {
    pub base: NiPSysModifier,
}

impl NiPSysPositionModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
