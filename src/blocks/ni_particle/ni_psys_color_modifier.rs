use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysColorModifier {
    pub base: NiPSysModifier,
    pub data_ref: BlockRef,
}

impl NiPSysColorModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
