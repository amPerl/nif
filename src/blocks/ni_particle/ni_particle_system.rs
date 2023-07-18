use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::{blocks::NiParticles, common::BlockRef};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiParticleSystem {
    pub base: NiParticles,
    #[br(map = |x: u8| x > 0)]
    pub world_space: bool,
    pub num_modifiers: u32,
    #[br(count = num_modifiers)]
    pub modifiers_refs: Vec<BlockRef>,
}

impl NiParticleSystem {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiParticleSystem {
    type Target = NiParticles;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
