use super::ni_string::NiString;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiObjectNET {
    pub name: NiString,
    pub num_extra_data_refs: u32,
    #[br(count = num_extra_data_refs)]
    pub extra_data_refs: Vec<i32>,
    pub controller_ref: i32,
}

impl NiObjectNET {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
