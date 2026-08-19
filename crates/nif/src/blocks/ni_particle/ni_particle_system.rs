
use crate::{blocks::NiParticles, common::BlockRef};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiParticleSystem {
    pub base: NiParticles,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub world_space: bool,
    #[br(temp)]
    #[bw(calc = modifiers_refs.len() as u32)]
    num_modifiers: u32,
    #[br(count = num_modifiers)]
    pub modifiers_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiParticleSystem {
    type Target = NiParticles;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
