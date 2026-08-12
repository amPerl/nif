use crate::common::Vector3;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiRangeLODData {
    pub center: Vector3,
    pub num_lod_levels: u32,
    #[br(count=num_lod_levels)]
    pub lod_levels: Vec<LODRange>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct LODRange {
    pub near: f32,
    pub far: f32,
}
