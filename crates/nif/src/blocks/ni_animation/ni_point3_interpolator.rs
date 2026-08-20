use binrw::{BinRead, BinWrite};

use super::NiKeyBasedInterpolator;
use crate::common::{BlockRef, Vector3};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPoint3Interpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: Vector3,
    pub data_ref: BlockRef,
}
