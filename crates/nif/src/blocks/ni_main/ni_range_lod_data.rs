use crate::common::Vector3;
use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiRangeLODData {
    pub center: Vector3,
    #[br(temp)]
    #[bw(calc = lod_levels.len() as u32)]
    num_lod_levels: u32,
    #[br(count=num_lod_levels)]
    pub lod_levels: Vec<LODRange>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct LODRange {
    pub near: f32,
    pub far: f32,
}
