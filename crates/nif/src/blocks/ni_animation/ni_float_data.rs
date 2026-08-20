use binrw::{BinRead, BinWrite};

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiFloatData {
    pub data: KeyGroup<f32>,
}
