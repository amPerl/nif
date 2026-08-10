use super::ni_string::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiIntegerExtraData {
    pub name: NiString,
    pub value: u32,
}
