use binrw::BinRead;

use super::NiSourceTexture;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSourceCubeMap {
    pub base: NiSourceTexture,
}

impl std::ops::Deref for NiSourceCubeMap {
    type Target = NiSourceTexture;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
