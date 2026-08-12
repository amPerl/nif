use crate::common::BlockRef;

use super::ni_av_object::NiAvObject;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiNode {
    pub base: NiAvObject,
    #[br(temp)]
    #[bw(calc = child_refs.len() as u32)]
    num_child_refs: u32,
    #[br(count = num_child_refs)]
    pub child_refs: Vec<BlockRef>,
    #[br(temp)]
    #[bw(calc = effect_refs.len() as u32)]
    num_effect_refs: u32,
    #[br(count = num_effect_refs)]
    pub effect_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiNode {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
