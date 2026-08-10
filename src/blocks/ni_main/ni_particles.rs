use binrw::BinRead;

use super::NiGeometry;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiParticles {
    pub base: NiGeometry,
}

impl std::ops::Deref for NiParticles {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
