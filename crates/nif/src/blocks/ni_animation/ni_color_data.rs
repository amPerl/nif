use binrw::{BinRead, BinWrite};

use crate::common::{Color4, KeyGroup};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiColorData {
    pub data: KeyGroup<Color4>,
}
