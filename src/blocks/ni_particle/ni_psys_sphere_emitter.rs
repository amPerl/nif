use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysSphereEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
}

impl NiPSysSphereEmitter {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
