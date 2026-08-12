use super::ni_string::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiIntegersExtraData {
    pub name: NiString,
    pub num_integers: u32,
    #[br(count = num_integers)]
    pub data: Vec<u32>,
}
