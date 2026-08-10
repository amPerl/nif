use binrw::BinRead;

use super::NiLight;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAmbientLight {
    pub base: NiLight,
}

impl std::ops::Deref for NiAmbientLight {
    type Target = NiLight;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
