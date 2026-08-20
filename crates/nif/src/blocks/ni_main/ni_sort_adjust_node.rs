use super::ni_node::NiNode;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiSortAdjustNode {
    pub base: NiNode,
    pub sorting_mode: u32,
}

impl std::ops::Deref for NiSortAdjustNode {
    type Target = NiNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
