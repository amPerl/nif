use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
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

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PixelFormatComponent {
    pub kind: u32,       // PixelComponent
    pub convention: u32, // PixelRepresentation
    pub bits_per_channel: u8,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub is_signed: bool,
}

/// Describes the pixel format used by the NiPixelData object to store a texture.
#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum PixelFormat {
    #[brw(magic = 0u32)]
    FmtRgb, // 24-bit RGB. 8 bits per red, blue, and green component.
    #[brw(magic = 1u32)]
    FmtRgba, // 32-bit RGB with alpha. 8 bits per red, blue, green, and alpha component.
    #[brw(magic = 2u32)]
    FmtPalette, // 8-bit palette index.
    #[brw(magic = 3u32)]
    FmtPaletteAlpha, // 8-bit palette index with alpha.
    #[brw(magic = 4u32)]
    FmtDXT1, // DXT1 compressed texture.
    #[brw(magic = 5u32)]
    FmtDXT3, // DXT3 compressed texture.
    #[brw(magic = 6u32)]
    FmtDXT5, // DXT5 compressed texture.
    #[brw(magic = 7u32)]
    FmtRgb24NonInt, // (Deprecated) 24-bit noninterleaved texture, an old PS2 format.
    #[brw(magic = 8u32)]
    FmtBump, // Uncompressed dU/dV gradient bump map.
    #[brw(magic = 9u32)]
    FmtBumpLuma, // Uncompressed dU/dV gradient bump map with luma channel representing shininess.
    #[brw(magic = 10u32)]
    FmtRenderSpec, // Generic descriptor for any renderer-specific format not described by other formats.
    #[brw(magic = 11u32)]
    Fmt1Ch, // Generic descriptor for formats with 1 component.
    #[brw(magic = 12u32)]
    Fmt2Ch, // Generic descriptor for formats with 2 components.
    #[brw(magic = 13u32)]
    Fmt3Ch, // Generic descriptor for formats with 3 components.
    #[brw(magic = 14u32)]
    Fmt4Ch, // Generic descriptor for formats with 4 components.
    #[brw(magic = 15u32)]
    FmtDepthStencil, // Indicates the NiPixelFormat is meant to be used on a depth/stencil surface.
    Unknown(u32),
}
