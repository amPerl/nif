use binrw::{BinRead, BinWrite};

use super::NiSourceTexture;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiSourceCubeMap {
    pub base: NiSourceTexture,
}

impl std::ops::Deref for NiSourceCubeMap {
    type Target = NiSourceTexture;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
