use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

pub mod blocks;
pub mod error;
pub mod header;
mod parse_utils;

#[derive(Debug, PartialEq, BinRead)]
pub struct Nif {
    pub header: header::Header,
    #[br(args(
        (&header).block_types.iter().map(|b| b.value.clone()).collect(),
        (&header).block_type_index.clone(),
    ))]
    #[br(parse_with = parse_utils::parse_blocks)]
    pub blocks: Vec<blocks::Block>,
}

impl Nif {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
