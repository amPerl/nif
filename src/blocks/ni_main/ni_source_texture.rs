use crate::common::BlockRef;

use super::ni_object_net::NiObjectNET;
use super::ni_string::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSourceTexture {
    pub base: NiObjectNET,
    #[br(map = |x: u8| x > 0)]
    pub use_external: bool,
    pub file_name: NiString,
    #[br(if(use_external))]
    pub unknown_link_ref: Option<i32>,
    #[br(if(!use_external))]
    pub pixel_data_ref: BlockRef,
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
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead)]
pub enum MipMapFormat {
    #[br(magic = 0u32)]
    No,
    #[br(magic = 1u32)]
    Yes,
    #[br(magic = 2u32)]
    Default,
    Unknown(u32),
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
    Unknown(u32),
}

impl std::ops::Deref for NiSourceTexture {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
