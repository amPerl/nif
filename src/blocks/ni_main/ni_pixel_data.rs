use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

use super::NiPixelFormat;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPixelData {
    pub base: NiPixelFormat,
    pub palette_ref: BlockRef,
    pub num_mipmaps: u32,
    pub bytes_per_pixel: u32,
    #[br(count = num_mipmaps)]
    pub mipmaps: Vec<MipMap>,
    pub num_pixels: u32,
    pub num_faces: u32,
    #[br(args { count: num_faces as _, inner: (num_pixels,) })]
    pub pixel_data: Vec<PixelData>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(import(num_pixels: u32))]
pub struct PixelData {
    #[br(count = num_pixels)]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct MipMap {
    pub width: u32,
    pub height: u32,
    pub offset: u32,
}

impl NiPixelData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
