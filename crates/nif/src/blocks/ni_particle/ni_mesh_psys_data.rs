use super::NiPSysData;
use crate::common::BlockRef;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiMeshPSysData {
    pub base: NiPSysData,
    pub default_pool_size: u32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub fill_pools_on_load: bool,
    #[br(temp)]
    #[bw(calc = generations.len() as u32)]
    num_generations: u32,
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
