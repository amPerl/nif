use super::NiGeometry;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriShape {
    pub base: NiGeometry,
}

impl std::ops::Deref for NiTriShape {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
