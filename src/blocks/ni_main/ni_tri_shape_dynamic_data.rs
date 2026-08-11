use super::NiTriShapeData;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriShapeDynamicData {
    pub base: NiTriShapeData,
    pub active_vertices: u16,
    pub active_triangles: u16,
}

impl std::ops::Deref for NiTriShapeDynamicData {
    type Target = NiTriShapeData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
