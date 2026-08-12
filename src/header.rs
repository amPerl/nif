use super::blocks::NiString;
use super::error::NifError;
use super::parse_utils;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt, BinWrite,
};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[brw(magic = b"Gamebryo File Format, Version ")]
#[br(assert(version == 0x14000004, NifError::NotImplemented("Version not implemented")))]
#[br(assert(version_from_str == version, NifError::InvalidValueError))]
pub struct Header {
    #[br(parse_with = parse_utils::parse_version)]
    #[bw(write_with = parse_utils::write_version)]
    pub version_from_str: u32,
    pub version: u32,
    #[br(assert(endian_type == EndianType::LittleEndian, NifError::NotImplemented("big-endian files")))]
    pub endian_type: EndianType,
    pub user_version: u32,
    #[br(temp)]
    #[bw(calc = block_type_index.len() as u32)]
    num_blocks: u32,
    #[br(temp)]
    #[bw(calc = block_types.len() as u16)]
    num_block_types: u16,
    #[br(count = num_block_types)]
    pub block_types: Vec<NiString>,
    #[br(count = num_blocks)]
    pub block_type_index: Vec<u16>,
    pub unknown: u32,
}

impl Header {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, NifError> {
        Ok(reader.read_le()?)
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum EndianType {
    #[brw(magic = 1u8)]
    LittleEndian,
    #[brw(magic = 0u8)]
    BigEndian,
}
