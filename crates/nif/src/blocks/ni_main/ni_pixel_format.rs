use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPixelFormat {
    pub pixel_format: u32, // PixelFormat
    pub bits_per_pixel: u8,
    pub renderer_hint: i32,
    pub extra_data: u32,
    pub flags: u8,
    pub tiling: u32,
    #[br(count = 4)]
    pub channels: Vec<PixelFormatComponent>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct PixelFormatComponent {
    pub kind: u32,       // PixelComponent
    pub convention: u32, // PixelRepresentation
    pub bits_per_channel: u8,
    #[br(map = |x: u8| x > 0)]
    pub is_signed: bool,
}

impl NiPixelFormat {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
