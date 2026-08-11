use binrw::BinRead;

use crate::{
    blocks::NiString,
    common::{BlockRef, NiQuatTransform},
};

use super::NiInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiLookAtInterpolator {
    pub base: NiInterpolator,
    pub flags: u16,
    pub look_at: BlockRef,
    pub look_at_name: NiString,
    pub transform: NiQuatTransform,
    pub interpolator_translation: BlockRef,
    pub interpolator_roll: BlockRef,
    pub interpolator_scale: BlockRef,
}

impl NiLookAtInterpolator {
    pub fn flip(&self) -> bool {
        self.flags & 0x0001 != 0
    }
    pub fn look_y_axis(&self) -> bool {
        self.flags & 0x0002 != 0
    }
    pub fn look_z_axis(&self) -> bool {
        self.flags & 0x0004 != 0
    }
}
