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
    #[bw(map = |x: &u16| vertex_map.as_ref().map(|v| v.len() as u16)
        .or_else(|| vertex_weights.as_ref().map(|v| v.len() as u16)).unwrap_or(*x))]
    pub num_vertices: u16,
    #[bw(map = |x: &u16| triangles.as_ref().map_or(*x, |t| t.len() as u16))]
    pub num_triangles: u16,
    #[br(temp)]
    #[bw(calc = bones.len() as u16)]
    num_bones: u16,
    #[bw(map = |_: &u16| strip_lengths.len() as u16)]
    pub num_strips: u16,
    #[bw(map = |x: &u16| vertex_weights.as_ref().and_then(|v| v.first())
        .map_or(*x, |w| w.weights.len() as u16))]
    pub num_weights_per_vertex: u16,

    #[br(count = num_bones)]
    pub bones: Vec<u16>,

    #[br(temp)]
    #[bw(calc = u8::from(vertex_map.is_some()))]
    has_vertex_map: u8,
    #[br(if(has_vertex_map > 0), count = num_vertices)]
    pub vertex_map: Option<Vec<u16>>,

    #[br(temp)]
    #[bw(calc = u8::from(vertex_weights.is_some()))]
    has_vertex_weights: u8,
    #[br(if(has_vertex_weights != 0), args { count: num_vertices as _, inner: (num_weights_per_vertex,) })]
    pub vertex_weights: Option<Vec<VertexWeights>>,

    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(temp)]
    #[bw(calc = u8::from(strips.is_some() || triangles.is_some() ))]
    has_faces: u8,
    #[br(if(has_faces != 0 && num_strips != 0), count = strip_lengths.iter().map(|l| *l as usize).sum::<usize>())]
    pub strips: Option<Vec<u16>>,
    #[br(if(has_faces != 0 && num_strips == 0), count = num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    #[br(temp)]
    #[bw(calc = u8::from(bone_indices.is_some()))]
    has_bone_indices: u8,
    #[br(if(has_bone_indices != 0), count = num_vertices as usize * num_weights_per_vertex as usize)]
    pub bone_indices: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(num_weights_per_vertex: u16))]
pub struct VertexWeights {
    #[br(count = num_weights_per_vertex)]
    pub weights: Vec<f32>,
}
