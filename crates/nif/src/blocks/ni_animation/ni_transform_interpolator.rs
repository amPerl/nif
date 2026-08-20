use binrw::{BinRead, BinWrite};

use super::NiKeyBasedInterpolator;
use crate::common::{BlockRef, NiQuatTransform};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTransformInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub transform: NiQuatTransform,
    pub data_ref: BlockRef,
}
