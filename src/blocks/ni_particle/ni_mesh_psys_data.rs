use super::NiPSysData;
use crate::common::BlockRef;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiMeshPSysData {
    pub base: NiPSysData,
    pub default_pool_size: u32,
    #[br(map = |x: u8| x > 0)]
    pub fill_pools_on_load: bool,
    pub num_generations: u32,
    #[br(count = num_generations)]
    pub generations: Vec<u32>,
    pub particle_meshes_ref: BlockRef,
}

impl std::ops::Deref for NiMeshPSysData {
    type Target = NiPSysData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
