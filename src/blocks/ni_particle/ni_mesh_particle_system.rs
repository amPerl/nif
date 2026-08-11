use super::NiParticleSystem;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiMeshParticleSystem {
    pub base: NiParticleSystem,
}

impl std::ops::Deref for NiMeshParticleSystem {
    type Target = NiParticleSystem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
