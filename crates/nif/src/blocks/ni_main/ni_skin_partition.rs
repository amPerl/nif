use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::Triangle;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSkinPartition {
    pub num_partitions: u32,
    #[br(count = num_partitions)]
    pub partitions: Vec<SkinPartition>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct SkinPartition {
    pub num_vertices: u16,
    pub num_triangles: u16,
    pub num_bones: u16,
    pub num_strips: u16,
    pub num_weights_per_vertex: u16,

    #[br(count = num_bones)]
    pub bones: Vec<u16>,

    pub has_vertex_map: u8,
    #[br(if(has_vertex_map > 0), count = num_vertices)]
    pub vertex_map: Option<Vec<u16>>,

    #[br(map = |x: u8| x > 0)]
    pub has_vertex_weights: bool,
    #[br(if(has_vertex_weights), args { count: num_vertices as _, inner: (num_weights_per_vertex,) })]
    pub vertex_weights: Option<Vec<VertexWeights>>,

    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(map = |x: u8| x > 0)]
    pub has_faces: bool,
    #[br(if(has_faces && num_strips != 0), count = num_strips * strip_lengths.iter().sum::<u16>())]
    pub strips: Option<Vec<u16>>,
    #[br(if(has_faces && num_strips == 0), count = num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    #[br(map = |x: u8| x > 0)]
    pub has_bone_indices: bool,
    #[br(if(has_bone_indices), count = num_vertices * num_weights_per_vertex)]
    pub bone_indices: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(import(num_weights_per_vertex: u16))]
pub struct VertexWeights {
    #[br(count = num_weights_per_vertex)]
    pub weights: Vec<f32>,
}

impl NiSkinPartition {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
