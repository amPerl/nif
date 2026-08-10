use super::NiGeometry;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriStrips {
    pub base: NiGeometry,
}

impl std::ops::Deref for NiTriStrips {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
