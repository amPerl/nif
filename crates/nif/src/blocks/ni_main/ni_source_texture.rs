use crate::common::BlockRef;

use super::ni_object_net::NiObjectNET;
use super::ni_string::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiSourceTexture {
    pub base: NiObjectNET,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub use_external: bool,
    pub file_name: NiString,
    #[br(if(use_external))]
    pub unknown_link_ref: Option<i32>,
    #[br(if(!use_external))]
    #[bw(if(use_external == 0))]
    pub pixel_data_ref: BlockRef,
    pub pixel_layout: PixelLayout,
    pub mipmap_format: MipMapFormat,
    pub alpha_format: AlphaFormat,
    pub is_static: u8,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub direct_render: bool,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum PixelLayout {
    #[brw(magic = 0u32)]
    Palettised,
    #[brw(magic = 1u32)]
    HighColor16,
    #[brw(magic = 2u32)]
    HighColor32,
    #[brw(magic = 3u32)]
    Compressed,
    #[brw(magic = 4u32)]
    BumpMap,
    #[brw(magic = 5u32)]
    Palettised4,
    #[brw(magic = 6u32)]
    Default,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum MipMapFormat {
    #[brw(magic = 0u32)]
    No,
    #[brw(magic = 1u32)]
    Yes,
    #[brw(magic = 2u32)]
    Default,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum AlphaFormat {
    #[brw(magic = 0u32)]
    None,
    #[brw(magic = 1u32)]
    Binary,
    #[brw(magic = 2u32)]
    Smooth,
    #[brw(magic = 3u32)]
    Default,
    Unknown(u32),
}

impl std::ops::Deref for NiSourceTexture {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
