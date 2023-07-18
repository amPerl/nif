use super::blocks::NiString;
use super::error::NifError;
use super::parse_utils;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
#[br(magic = b"Gamebryo File Format, Version ")]
#[br(assert(version == 0x14000004, NifError::NotImplemented("Version not implemented")))]
pub struct Header {
    #[br(parse_with = parse_utils::parse_version)]
    pub version_from_str: u32,
    pub version: u32,
    pub endian_type: EndianType,
    pub user_version: u32,
    pub num_blocks: u32,
    pub num_block_types: u16,
    #[br(count = num_block_types)]
    pub block_types: Vec<NiString>,
    #[br(count = num_blocks)]
    pub block_type_index: Vec<u16>,
    pub unknown: u32,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum EndianType {
    #[br(magic = 1u8)]
    LittleEndian,
    #[br(magic = 0u8)]
    BigEndian,
}

impl Header {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
