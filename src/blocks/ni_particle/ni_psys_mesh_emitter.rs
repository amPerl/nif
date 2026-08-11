use binrw::BinRead;

use super::NiPSysEmitter;
use crate::common::{BlockRef, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysMeshEmitter {
    pub base: NiPSysEmitter,
    pub num_emitter_meshes: u32,
    #[br(count = num_emitter_meshes)]
    pub emitter_mesh_refs: Vec<BlockRef>,
    pub initial_velocity_type: VelocityType,
    pub emission_type: EmitFrom,
    pub emission_axis: Vector3,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum VelocityType {
    #[br(magic = 0u32)]
    UseNormals,
    #[br(magic = 1u32)]
    UseRandom,
    #[br(magic = 2u32)]
    UseDirection,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead)]
pub enum EmitFrom {
    #[br(magic = 0u32)]
    Vertices,
    #[br(magic = 1u32)]
    FaceCenter,
    #[br(magic = 2u32)]
    EdgeCenter,
    #[br(magic = 3u32)]
    FaceSurface,
    #[br(magic = 4u32)]
    EdgeSurface,
    Unknown(u32),
}
