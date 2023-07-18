use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysBoxEmitter {
    pub base: NiPSysVolumeEmitter,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl NiPSysBoxEmitter {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
