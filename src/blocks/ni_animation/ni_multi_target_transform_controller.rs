use super::NiInterpController;
use crate::common::BlockRef;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiMultiTargetTransformController {
    pub base: NiInterpController,
    #[br(temp)]
    #[bw(calc = extra_target_refs.len() as u16)]
    num_extra_targets: u16,
    #[br(count = num_extra_targets)]
    pub extra_target_refs: Vec<BlockRef>,
}

impl std::ops::Deref for NiMultiTargetTransformController {
    type Target = NiInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
