use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::{blocks::NiParticlesData, common::Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysData {
    pub base: NiParticlesData,

    #[br(count = base.base.num_vertices)]
    pub particle_info: Vec<(Vector3, f32, f32, f32, u32)>,

    #[br(map = |x: u8| x > 0)]
    pub has_unknown_floats: bool,
    #[br(if(has_unknown_floats), count = base.base.num_vertices)]
    pub unknown_floats: Option<Vec<f32>>,

    pub unknown_short_1: u16,
    pub unknown_short_2: u16,
}

impl NiPSysData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
