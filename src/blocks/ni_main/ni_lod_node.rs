use crate::common::BlockRef;

use super::ni_switch_node::NiSwitchNode;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiLODNode {
    pub base: NiSwitchNode,
    pub lod_level_data_ref: BlockRef,
}

impl std::ops::Deref for NiLODNode {
    type Target = NiSwitchNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
