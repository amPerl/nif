use super::NiPSysModifier;
use crate::common::{BlockRef, Vector3};
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysDragModifier {
    pub base: NiPSysModifier,
    pub drag_object_ref: BlockRef,
    pub drag_axis: Vector3,
    pub percentage: f32,
    pub range: f32,
    pub range_falloff: f32,
}

impl std::ops::Deref for NiPSysDragModifier {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
