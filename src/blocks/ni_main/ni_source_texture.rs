use super::ni_object_net::NiObjectNET;
use super::ni_string::NiString;
use crate::error::NifError;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
#[br(assert(!direct_render, NifError::NotImplemented("Direct Render")))]
pub struct NiSourceTexture {
    pub base: NiObjectNET,
    pub use_external: u8,
    pub file_name: NiString,
    #[br(if(use_external == 1))]
    pub unknown_link_ref: Option<i32>,
    #[br(if(use_external == 0))]
    pub pixel_data_ref: Option<i32>,
    pub pixel_layout: PixelLayout,
    pub mipmap_format: MipMapFormat,
    pub alpha_format: AlphaFormat,
    pub is_static: u8,
    #[br(map = |x: u8| x > 0)]
    pub direct_render: bool,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum PixelLayout {
    #[br(magic = 0u32)]
    Palettised,
    #[br(magic = 1u32)]
    HighColor16,
    #[br(magic = 2u32)]
    HighColor32,
    #[br(magic = 3u32)]
    Compressed,
    #[br(magic = 4u32)]
    BumpMap,
    #[br(magic = 5u32)]
    Palettised4,
    #[br(magic = 6u32)]
    Default,
    Unknown,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum MipMapFormat {
    #[br(magic = 0u32)]
    No,
    #[br(magic = 1u32)]
    Yes,
    #[br(magic = 2u32)]
    Default,
    Unknown,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum AlphaFormat {
    #[br(magic = 0u32)]
    None,
    #[br(magic = 1u32)]
    Binary,
    #[br(magic = 2u32)]
    Smooth,
    #[br(magic = 3u32)]
    Default,
    Unknown,
}

impl NiSourceTexture {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiSourceTexture {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
