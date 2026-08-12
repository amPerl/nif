use super::NiPSysModifier;
use crate::common::BlockRef;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysMeshUpdateModifier {
    pub base: NiPSysModifier,
    pub num_meshes: u32,
    #[br(count = num_meshes)]
    pub mesh_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiPSysMeshUpdateModifier {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
