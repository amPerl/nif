use binrw::BinRead;

use super::NiKeyBasedInterpolator;
use crate::common::{BlockRef, NiQuatTransform};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransformInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub transform: NiQuatTransform,
    pub data_ref: BlockRef,
}
