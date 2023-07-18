use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysUpdateCtlr {
    pub base: NiTimeController,
}

impl NiPSysUpdateCtlr {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
