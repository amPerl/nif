use crate::common::Color4;

use super::ni_string::NiString;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiColorExtraData {
    pub name: NiString,
    pub data: Color4,
}

impl NiColorExtraData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
