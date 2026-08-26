use binrw::{BinRead, BinWrite};

use crate::{
    blocks::NiString,
    common::{BlockRef, NiQuatTransform},
};

use super::NiInterpolator;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
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

/// Which of the object's own axes is aimed at the target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookAxis {
    X,
    Y,
    Z,
}

impl NiLookAtInterpolator {
    pub fn flip(&self) -> bool {
        self.flags & 0x0001 != 0
    }

    /// The axis is a two bit field rather than a bit each, so reading the bits apart leaves no
    /// way to tell the x case from a value nothing set.
    pub fn axis(&self) -> LookAxis {
        match (self.flags & 0x0006) >> 1 {
            1 => LookAxis::Y,
            2 => LookAxis::Z,
            _ => LookAxis::X,
        }
    }
}
