use binrw::BinRead;

use crate::common::BlockRef;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: f32, // Pose value if lacking NiFloatData
    pub data_ref: BlockRef,
}
