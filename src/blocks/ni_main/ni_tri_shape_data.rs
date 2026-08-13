
use super::NiTriBasedGeomData;
use crate::common::Triangle;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiTriShapeData {
    #[bw(args(triangles.as_ref().map_or(base.num_triangles, |t| t.len() as u16)))]
    pub base: NiTriBasedGeomData,

    pub num_triangle_points: u32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_triangles: bool,
    #[br(if(has_triangles))]
    #[br(count=base.num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    #[br(temp)]
    #[bw(calc = match_groups.len() as u16)]
    num_match_groups: u16,
    #[br(count=num_match_groups)]
    pub match_groups: Vec<MatchGroup>,
}
#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct MatchGroup {
    #[br(temp)]
    #[bw(calc = vertex_indices.len() as u16)]
    num_vertices: u16,
    #[br(count=num_vertices)]
    pub vertex_indices: Vec<u16>,
}

impl std::ops::Deref for NiTriShapeData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
