use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::{blocks::NiString, common::BlockRef};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifier {
    pub name: NiString,
    pub order: u32, // NiPSysModifierOrder
    pub target_ref: BlockRef,
    #[br(map = |x: u8| x > 0)]
    pub active: bool,
}

impl NiPSysModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
