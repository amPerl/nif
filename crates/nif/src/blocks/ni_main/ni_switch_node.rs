use super::ni_node::NiNode;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiSwitchNode {
    pub base: NiNode,
    pub switch_node_flags: NiSwitchFlags,
    pub index: u32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum NiSwitchFlags {
    #[brw(magic = 0u16)]
    UpdateOnlyActiveChild,
    #[brw(magic = 1u16)]
    UpdateControllers,
    Unknown(u16),
}

impl std::ops::Deref for NiSwitchNode {
    type Target = NiNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
