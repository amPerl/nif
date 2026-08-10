use binrw::BinRead;

use super::NiGeometryData;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriBasedGeomData {
    pub base: NiGeometryData,
    pub num_triangles: u16,
}

impl std::ops::Deref for NiTriBasedGeomData {
    type Target = NiGeometryData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
