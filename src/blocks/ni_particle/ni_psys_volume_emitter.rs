use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

use super::NiPSysEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysVolumeEmitter {
    pub base: NiPSysEmitter,
    pub emitter_object_ref: BlockRef,
}

impl NiPSysVolumeEmitter {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
