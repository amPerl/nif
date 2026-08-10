use super::ni_string::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatExtraData {
    pub name: NiString,
    pub value: f32,
}
