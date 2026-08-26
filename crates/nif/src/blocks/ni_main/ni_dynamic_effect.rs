
use crate::common::BlockRef;

use super::NiAvObject;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiDynamicEffect {
    pub base: NiAvObject,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub switch_state: bool,
    #[br(temp)]
    #[bw(calc = unaffected_node_refs.len() as u32)]
    num_unaffected_nodes: u32,
    #[br(count = num_unaffected_nodes)]
    pub unaffected_node_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiDynamicEffect {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
