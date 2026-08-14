use binrw::{BinRead, BinWrite};

use super::NiBound;
use crate::common::NiTransform;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiSkinData {
    pub skin_transform: NiTransform,
    #[br(temp)]
    #[bw(calc = bone_list.len() as u32)]
    num_bones: u32,
    #[bw(map = |x: &u8| bone_list.first().map_or(*x, |b| u8::from(b.vertex_weights.is_some())))]
    pub has_vertex_weights: u8,
    #[br(args { count: num_bones as _, inner: (has_vertex_weights,) })]
    pub bone_list: Vec<BoneData>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(has_vertex_weights: u8))]
pub struct BoneData {
    pub skin_transform: NiTransform,
    pub bounding_sphere: NiBound,
    #[bw(map = |x: &u16| vertex_weights.as_ref().map_or(*x, |v| v.len() as u16))]
    pub num_vertices: u16,
    #[br(if(has_vertex_weights != 0), count = num_vertices)]
    pub vertex_weights: Option<Vec<BoneVertData>>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct BoneVertData {
    pub index: u16,
    pub weight: f32,
}
