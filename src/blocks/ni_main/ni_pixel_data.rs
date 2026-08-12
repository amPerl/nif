use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPixelFormat;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiPixelData {
    pub base: NiPixelFormat,
    pub palette_ref: BlockRef,
    #[br(temp)]
    #[bw(calc = mipmaps.len() as u32)]
    num_mipmaps: u32,
    pub bytes_per_pixel: u32,
    #[br(count = num_mipmaps)]
    pub mipmaps: Vec<MipMap>,
    pub num_pixels: u32,
    #[br(temp)]
    #[bw(calc = pixel_data.len() as u32)]
    num_faces: u32,
    #[br(args { count: num_faces as _, inner: (num_pixels,) })]
    pub pixel_data: Vec<PixelData>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(num_pixels: u32))]
pub struct PixelData {
    #[br(count = num_pixels)]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct MipMap {
    pub width: u32,
    pub height: u32,
    pub offset: u32,
}
