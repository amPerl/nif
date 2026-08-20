use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiBoolInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: u8,
    pub data_ref: BlockRef,
}

impl NiBoolInterpolator {
    pub fn pose_value(&self) -> Option<bool> {
        match self.value {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}
