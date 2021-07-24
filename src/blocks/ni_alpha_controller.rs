use super::ni_string::NiString;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAlphaController {
    pub name: NiString,
    pub value: NiString,
}

impl NiAlphaController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
