use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

use super::NiPSysModifierCtlr;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysEmitterCtlr {
    pub base: NiPSysModifierCtlr,
    pub visibility_interpolator_ref: BlockRef,
}

impl NiPSysEmitterCtlr {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
