
use super::NiTriBasedGeomData;
use crate::common::Triangle;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiTriShapeData {
    #[bw(args(triangles.as_ref().map_or(base.num_triangles, |t| t.len() as u16)))]
    pub base: NiTriBasedGeomData,

    #[br(temp)]
    #[bw(calc = triangles.as_ref().map_or(0, |t| (t.len() * 3) as u32))]
    num_triangle_points: u32,
    #[br(temp)]
    #[bw(calc = u8::from(triangles.is_some()))]
    has_triangles: u8,
    #[br(if(has_triangles != 0))]
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
