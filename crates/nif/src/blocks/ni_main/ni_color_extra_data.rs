use super::ni_string::NiString;
use crate::common::Color4;

use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiColorExtraData {
    pub name: NiString,
    pub data: Color4,
}
