use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysCylinderEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
    pub height: f32,
}

impl NiPSysCylinderEmitter {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
