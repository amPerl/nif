use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};
pub use error::NifError;

pub mod blocks;
pub mod collectors;
pub mod common;
pub mod error;
pub mod header;

pub use glam;

mod parse_utils;

#[derive(Debug, PartialEq, BinRead)]
pub struct Nif {
    pub header: header::Header,
    #[br(args(
        header.block_types.iter().map(|b| b.value.clone()).collect(),
        header.block_type_index.clone(),
    ))]
    #[br(parse_with = parse_utils::parse_blocks)]
    pub blocks: Vec<blocks::Block>,
}

impl Nif {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, NifError> {
        Ok(reader.read_le()?)
    }
}
