use super::blocks::NiString;
use super::error::NifError;
use super::parse_utils;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt, BinWrite,
};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[bw(import(derived_types: Vec<NiString>, derived_index: Vec<u16>))]
#[brw(magic = b"Gamebryo File Format, Version ")]
#[br(assert(version == 0x14000004, NifError::NotImplemented("Version not implemented")))]
#[br(assert(version_from_str == version, NifError::InvalidValueError))]
pub struct Header {
    #[br(parse_with = parse_utils::parse_version)]
    #[bw(map = |_: &u32| parse_utils::version_bytes(*version))]
    version_from_str: u32,
    pub version: u32,
    #[br(assert(endian_type == EndianType::LittleEndian, NifError::NotImplemented("big-endian files")))]
    pub endian_type: EndianType,
    pub user_version: u32,
    #[br(temp)]
    #[bw(calc = derived_index.len() as u32)]
    num_blocks: u32,
    #[br(temp)]
    #[bw(calc = derived_types.len() as u16)]
    num_block_types: u16,
    #[br(count = num_block_types)]
    #[bw(map = |_: &Vec<NiString>| derived_types.clone())]
    block_types: Vec<NiString>,
    #[br(count = num_blocks)]
    #[bw(map = |_: &Vec<u16>| derived_index.clone())]
    block_type_index: Vec<u16>,
    pub unknown: u32,
}

impl Header {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, NifError> {
        Ok(reader.read_le()?)
    }

    pub fn block_types(&self) -> &[NiString] {
        &self.block_types
    }

    pub fn block_type_index(&self) -> &[u16] {
        &self.block_type_index
    }

    pub fn block_type_name(&self, block: usize) -> Option<&NiString> {
        let index = *self.block_type_index.get(block)?;
        self.block_types.get(usize::from(index))
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum EndianType {
    #[brw(magic = 1u8)]
    LittleEndian,
    #[brw(magic = 0u8)]
    BigEndian,
}
