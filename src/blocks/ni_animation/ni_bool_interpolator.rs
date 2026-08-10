use binrw::BinRead;

use crate::common::BlockRef;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBoolInterpolator {
    pub base: NiKeyBasedInterpolator,
    #[br(map = |x: u8| x > 0)]
    pub value: bool, // Pose value if lacking NiBoolData
    pub data_ref: BlockRef,
}
