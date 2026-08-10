use binrw::BinRead;

use crate::common::BlockRef;

use super::NiAvObject;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiDynamicEffect {
    pub base: NiAvObject,
    #[br(map = |x: u8| x > 0)]
    pub switch_state: bool,
    pub num_affected_nodes: u32,
    #[br(count = num_affected_nodes)]
    pub affected_node_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiDynamicEffect {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
