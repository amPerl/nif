use binrw::BinRead;

use super::NiSourceTexture;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSourceCubeMap {
    pub base: NiSourceTexture,
}

