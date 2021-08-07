use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysModifierCtlr;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysEmitterCtlr {
    pub base: NiPSysModifierCtlr,
    pub visibility_interpolator_ref: i32,
}

impl NiPSysEmitterCtlr {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
