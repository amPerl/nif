use binrw::BinRead;

use super::NiKeyBasedInterpolator;
use crate::common::{BlockRef, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPoint3Interpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: Vector3,
    pub data_ref: BlockRef,
}
