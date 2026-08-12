use super::NiParticleSystem;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiMeshParticleSystem {
    pub base: NiParticleSystem,
}

impl std::ops::Deref for NiMeshParticleSystem {
    type Target = NiParticleSystem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
