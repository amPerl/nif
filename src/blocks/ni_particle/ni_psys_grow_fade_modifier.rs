use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysGrowFadeModifier {
    pub base: NiPSysModifier,
    pub grow_time: f32,
    pub grow_generation: u16,
    pub fade_time: f32,
    pub fade_generation: u16,
}

impl NiPSysGrowFadeModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
