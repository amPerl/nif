use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::blocks::{NiSingleInterpController, NiString};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifierCtlr {
    pub base: NiSingleInterpController,
    pub modifier_name: NiString,
}

impl NiPSysModifierCtlr {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
