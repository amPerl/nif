use super::ni_string::NiString;
use crate::common::Color4;

use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiColorExtraData {
    pub name: NiString,
    pub data: Color4,
}
