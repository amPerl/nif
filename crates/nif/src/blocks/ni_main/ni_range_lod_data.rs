use crate::common::Vector3;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiRangeLODData {
    pub center: Vector3,
    pub num_lod_levels: u32,
    #[br(count=num_lod_levels)]
    pub lod_levels: Vec<LODRange>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct LODRange {
    pub near: f32,
    pub far: f32,
}

impl NiRangeLODData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
