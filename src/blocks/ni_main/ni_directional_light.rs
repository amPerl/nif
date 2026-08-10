use binrw::BinRead;

use super::NiLight;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiDirectionalLight {
    pub base: NiLight,
}

impl std::ops::Deref for NiDirectionalLight {
    type Target = NiLight;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
