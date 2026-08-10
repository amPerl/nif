use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPixelFormat {
    pub pixel_format: PixelFormat,
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

/// Describes the pixel format used by the NiPixelData object to store a texture.
#[derive(Debug, PartialEq, BinRead)]
pub enum PixelFormat {
    #[br(magic = 0u32)]
    FmtRgb, // 24-bit RGB. 8 bits per red, blue, and green component.
    #[br(magic = 1u32)]
    FmtRgba, // 32-bit RGB with alpha. 8 bits per red, blue, green, and alpha component.
    #[br(magic = 2u32)]
    FmtPalette, // 8-bit palette index.
    #[br(magic = 3u32)]
    FmtPaletteAlpha, // 8-bit palette index with alpha.
    #[br(magic = 4u32)]
    FmtDXT1, // DXT1 compressed texture.
    #[br(magic = 5u32)]
    FmtDXT3, // DXT3 compressed texture.
    #[br(magic = 6u32)]
    FmtDXT5, // DXT5 compressed texture.
    #[br(magic = 7u32)]
    FmtRgb24NonInt, // (Deprecated) 24-bit noninterleaved texture, an old PS2 format.
    #[br(magic = 8u32)]
    FmtBump, // Uncompressed dU/dV gradient bump map.
    #[br(magic = 9u32)]
    FmtBumpLuma, // Uncompressed dU/dV gradient bump map with luma channel representing shininess.
    #[br(magic = 10u32)]
    FmtRenderSpec, // Generic descriptor for any renderer-specific format not described by other formats.
    #[br(magic = 11u32)]
    Fmt1Ch, // Generic descriptor for formats with 1 component.
    #[br(magic = 12u32)]
    Fmt2Ch, // Generic descriptor for formats with 2 components.
    #[br(magic = 13u32)]
    Fmt3Ch, // Generic descriptor for formats with 3 components.
    #[br(magic = 14u32)]
    Fmt4Ch, // Generic descriptor for formats with 4 components.
    #[br(magic = 15u32)]
    FmtDepthStencil, // Indicates the NiPixelFormat is meant to be used on a depth/stencil surface.
    Unknown,
}
