use binrw::{BinRead, BinWrite};

use super::NiLight;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiAmbientLight {
    pub base: NiLight,
}

impl std::ops::Deref for NiAmbientLight {
    type Target = NiLight;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
