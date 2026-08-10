use crate::common::BlockRef;

use super::ni_av_object::NiAvObject;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiNode {
    pub base: NiAvObject,
    pub num_child_refs: u32,
    #[br(count = num_child_refs)]
    pub child_refs: Vec<BlockRef>,
    pub num_effect_refs: u32,
    #[br(count = num_effect_refs)]
    pub effect_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiNode {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
