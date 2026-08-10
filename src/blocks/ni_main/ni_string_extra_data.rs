use super::ni_string::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiStringExtraData {
    pub name: NiString,
    pub value: NiString,
}
