use binrw::{BinRead, BinWrite};

use super::NiTriBasedGeomData;
use crate::common::Triangle;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiTriShapeData {
    pub base: NiTriBasedGeomData,

    pub num_triangle_points: u32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_triangles: bool,
    #[br(if(has_triangles))]
    #[br(count=base.num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    pub num_match_groups: u16,
    #[br(count=num_match_groups)]
    pub match_groups: Vec<MatchGroup>,
}
#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct MatchGroup {
    pub num_vertices: u16,
    #[br(count=num_vertices)]
    pub vertex_indices: Vec<u16>,
}

impl std::ops::Deref for NiTriShapeData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
