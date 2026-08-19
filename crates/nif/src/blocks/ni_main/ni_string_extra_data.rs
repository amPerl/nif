use super::ni_string::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiStringExtraData {
    pub name: NiString,
    pub value: NiString,
}
