use super::ni_string::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiFloatsExtraData {
    pub name: NiString,
    pub num_floats: u32,
    #[br(count = num_floats)]
    pub data: Vec<f32>,
}
