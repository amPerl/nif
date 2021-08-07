use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysVolumeEmitter {
    pub base: NiPSysEmitter,
    pub emitter_object_ref: i32,
}

impl NiPSysVolumeEmitter {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
