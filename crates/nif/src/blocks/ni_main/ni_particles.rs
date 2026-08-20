use binrw::{BinRead, BinWrite};

use super::NiGeometry;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiParticles {
    pub base: NiGeometry,
}

impl std::ops::Deref for NiParticles {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
