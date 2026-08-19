use super::ni_string::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiIntegerExtraData {
    pub name: NiString,
    pub value: u32,
}
