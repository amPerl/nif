use super::NiInterpController;
use crate::common::BlockRef;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiMultiTargetTransformController {
    pub base: NiInterpController,
    pub num_extra_targets: u16,
    #[br(count = num_extra_targets)]
    pub extra_target_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiMultiTargetTransformController {
    type Target = NiInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
