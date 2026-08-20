
use super::NiGeometryData;

#[binrw::binrw]
#[bw(import(triangle_count: u16))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTriBasedGeomData {
    pub base: NiGeometryData,
    #[bw(map = |_: &u16| triangle_count)]
    pub num_triangles: u16,
}

impl std::ops::Deref for NiTriBasedGeomData {
    type Target = NiGeometryData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
