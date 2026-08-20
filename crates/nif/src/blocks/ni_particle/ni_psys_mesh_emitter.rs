use binrw::{BinRead, BinWrite};

use super::NiPSysEmitter;
use crate::common::{BlockRef, Vector3};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysMeshEmitter {
    pub base: NiPSysEmitter,
    #[br(temp)]
    #[bw(calc = emitter_mesh_refs.len() as u32)]
    num_emitter_meshes: u32,
    #[br(count = num_emitter_meshes)]
    pub emitter_mesh_refs: Vec<BlockRef>,
    pub initial_velocity_type: VelocityType,
    pub emission_type: EmitFrom,
    pub emission_axis: Vector3,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum VelocityType {
    #[brw(magic = 0u32)]
    UseNormals,
    #[brw(magic = 1u32)]
    UseRandom,
    #[brw(magic = 2u32)]
    UseDirection,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum EmitFrom {
    #[brw(magic = 0u32)]
    Vertices,
    #[brw(magic = 1u32)]
    FaceCenter,
    #[brw(magic = 2u32)]
    EdgeCenter,
    #[brw(magic = 3u32)]
    FaceSurface,
    #[brw(magic = 4u32)]
    EdgeSurface,
    Unknown(u32),
}
