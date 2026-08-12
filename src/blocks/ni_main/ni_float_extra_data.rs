use super::ni_string::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiFloatExtraData {
    pub name: NiString,
    pub value: f32,
}
