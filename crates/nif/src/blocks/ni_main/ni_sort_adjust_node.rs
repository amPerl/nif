use super::ni_node::NiNode;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiSortAdjustNode {
    pub base: NiNode,
    pub sorting_mode: u32,
}

impl NiSortAdjustNode {
    /// Leave the subtree to decide for itself, which is what the engine defaults to.
    pub const SORTING_INHERIT: u32 = 0;
    /// Draw the subtree where the traversal reaches it rather than back to front.
    pub const SORTING_OFF: u32 = 1;

    /// Whether this node takes its subtree out of the back to front pass. A mode the engine
    /// does not know is read as `SORTING_INHERIT`, which is what it does on load.
    pub fn suppresses_sorting(&self) -> bool {
        self.sorting_mode == Self::SORTING_OFF
    }
}

impl std::ops::Deref for NiSortAdjustNode {
    type Target = NiNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
