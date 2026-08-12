use binrw::{BinRead, BinWrite};

use crate::common::Triangle;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiSkinPartition {
    #[br(temp)]
    #[bw(calc = partitions.len() as u32)]
    num_partitions: u32,
    #[br(count = num_partitions)]
    pub partitions: Vec<SkinPartition>,
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct SkinPartition {
    pub num_vertices: u16,
    pub num_triangles: u16,
    #[br(temp)]
    #[bw(calc = bones.len() as u16)]
    num_bones: u16,
    pub num_strips: u16,
    pub num_weights_per_vertex: u16,

    #[br(count = num_bones)]
    pub bones: Vec<u16>,

    pub has_vertex_map: u8,
    #[br(if(has_vertex_map > 0), count = num_vertices)]
    pub vertex_map: Option<Vec<u16>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_vertex_weights: bool,
    #[br(if(has_vertex_weights), args { count: num_vertices as _, inner: (num_weights_per_vertex,) })]
    pub vertex_weights: Option<Vec<VertexWeights>>,

    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_faces: bool,
    #[br(if(has_faces && num_strips != 0), count = strip_lengths.iter().map(|l| *l as usize).sum::<usize>())]
    pub strips: Option<Vec<u16>>,
    #[br(if(has_faces && num_strips == 0), count = num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_bone_indices: bool,
    #[br(if(has_bone_indices), count = num_vertices as usize * num_weights_per_vertex as usize)]
    pub bone_indices: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(num_weights_per_vertex: u16))]
pub struct VertexWeights {
    #[br(count = num_weights_per_vertex)]
    pub weights: Vec<f32>,
}
