use super::NiPSysModifier;
use crate::common::BlockRef;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiPSysMeshUpdateModifier {
    pub base: NiPSysModifier,
    #[br(temp)]
    #[bw(calc = mesh_refs.len() as u32)]
    num_meshes: u32,
    #[br(count = num_meshes)]
    pub mesh_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiPSysMeshUpdateModifier {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
