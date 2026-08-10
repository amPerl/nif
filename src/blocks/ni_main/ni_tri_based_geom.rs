use binrw::BinRead;

use super::NiGeometry;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriBasedGeom {
    pub base: NiGeometry,
}

impl std::ops::Deref for NiTriBasedGeom {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
