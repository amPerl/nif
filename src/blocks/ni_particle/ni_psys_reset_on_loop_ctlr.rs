use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::blocks::NiTimeController;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysResetOnLoopCtlr {
    pub base: NiTimeController,
}

impl NiPSysResetOnLoopCtlr {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
