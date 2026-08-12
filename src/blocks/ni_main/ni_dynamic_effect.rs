use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiAvObject;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiDynamicEffect {
    pub base: NiAvObject,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
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
