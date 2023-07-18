use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiTriBasedGeomData;
use crate::common::Triangle;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriShapeData {
    pub base: NiTriBasedGeomData,

    pub num_triangle_points: u32,
    #[br(map = |x: u8| x > 0)]
    pub has_triangles: bool,
    #[br(if(has_triangles))]
    #[br(count=base.num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    pub num_match_groups: u16,
    #[br(count=num_match_groups)]
    pub match_groups: Vec<MatchGroup>,
}
#[derive(Debug, PartialEq, BinRead)]
pub struct MatchGroup {
    pub num_vertices: u16,
    #[br(count=num_vertices)]
    pub vertex_indices: Vec<u16>,
}

impl NiTriShapeData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiTriShapeData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
