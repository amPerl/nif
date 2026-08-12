use binrw::{BinRead, BinWrite};

use super::NiBound;
use crate::common::NiTransform;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiSkinData {
    pub skin_transform: NiTransform,
    pub num_bones: u32,
    pub has_vertex_weights: u8,
    #[br(args { count: num_bones as _, inner: (has_vertex_weights,) })]
    pub bone_list: Vec<BoneData>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(has_vertex_weights: u8))]
pub struct BoneData {
    pub skin_transform: NiTransform,
    pub bounding_sphere: NiBound,
    pub num_vertices: u16,
    #[br(if(has_vertex_weights != 0), count = num_vertices)]
    pub vertex_weights: Option<Vec<BoneVertData>>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct BoneVertData {
    pub index: u16,
    pub weight: f32,
}
