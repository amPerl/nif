use nif::blocks::{NiPalette, NiPixelData, PixelFormat};

/// A decoded texture and the smaller copies stored alongside it, largest first. Every level is
/// RGBA8, and each one is half the size of the one before, rounding down and stopping at one
/// texel, which is the rule the device sizes its own levels by.
pub struct Decoded {
    pub width: u32,
    pub height: u32,
    pub levels: Vec<Vec<u8>>,
}

impl Decoded {
    /// A texture with no smaller copies. Sampling one of these still reads the one level
    /// however far away it is drawn.
    pub fn flat(width: u32, height: u32, rgba: Vec<u8>) -> Self {
        Self {
            width,
            height,
            levels: vec![rgba],
        }
    }

    /// The size of a level, by the same halving the device uses.
    pub fn size(&self, level: u32) -> (u32, u32) {
        ((self.width >> level).max(1), (self.height >> level).max(1))
    }

    /// The largest level, which is the whole texture where there is only one.
    pub fn top(&self) -> &[u8] {
        &self.levels[0]
    }
}

/// Decode embedded pixel data to RGBA8. wgpu cannot be assumed to support BC formats,
/// so compressed textures are decompressed on the CPU.
pub fn decode_texture(pixels: &NiPixelData, palette: Option<&NiPalette>) -> Option<Decoded> {
    decode_face(pixels, palette, 0)
}

/// One face of a texture. A plain texture has the one; a cube map has six, in the order the
/// device wants them, and every one shares the mipmap table and the format.
pub fn decode_face(
    pixels: &NiPixelData,
    palette: Option<&NiPalette>,
    face: usize,
) -> Option<Decoded> {
    let top = pixels.mipmaps.first()?;
    let face = pixels.pixel_data.get(face)?;
    let mut decoded = Decoded {
        width: top.width.max(1),
        height: top.height.max(1),
        levels: Vec::with_capacity(pixels.mipmaps.len()),
    };

    for (at, mip) in pixels.mipmaps.iter().enumerate() {
        // a level whose size is not the halving of the one before it cannot be handed to the
        // device, so the chain ends there rather than the texture being refused
        if (mip.width.max(1), mip.height.max(1)) != decoded.size(at as u32) {
            break;
        }
        let Some(data) = face.data.get(mip.offset as usize..) else {
            break;
        };
        let Some(level) = decode_level(
            &pixels.base.pixel_format,
            palette,
            mip.width.max(1),
            mip.height.max(1),
            data,
        ) else {
            break;
        };
        decoded.levels.push(level);
    }

    (!decoded.levels.is_empty()).then_some(decoded)
}

/// One level, from the offset it starts at to RGBA8. `None` where the format is one this does
/// not read or the data runs out partway through.
fn decode_level(
    format: &PixelFormat,
    palette: Option<&NiPalette>,
    width: u32,
    height: u32,
    data: &[u8],
) -> Option<Vec<u8>> {
    let count = (width as usize) * (height as usize);
    let mut rgba = vec![255u8; count * 4];
    match format {
        PixelFormat::FmtRgba => {
            let src = data.get(..count * 4)?;
            rgba.copy_from_slice(src);
        }
        PixelFormat::FmtRgb => {
            let src = data.get(..count * 3)?;
            for (i, chunk) in src.chunks_exact(3).enumerate() {
                rgba[i * 4..i * 4 + 3].copy_from_slice(chunk);
            }
        }
        PixelFormat::FmtPalette | PixelFormat::FmtPaletteAlpha => {
            let entries = &palette?.palette;
            let src = data.get(..count)?;
            for (i, index) in src.iter().enumerate() {
                let entry = entries.get(*index as usize)?;
                rgba[i * 4..i * 4 + 4].copy_from_slice(&[entry.r, entry.g, entry.b, entry.a]);
            }
        }
        PixelFormat::FmtDXT1 | PixelFormat::FmtDXT3 | PixelFormat::FmtDXT5 => {
            let compression = match format {
                PixelFormat::FmtDXT1 => texpresso::Format::Bc1,
                PixelFormat::FmtDXT3 => texpresso::Format::Bc2,
                _ => texpresso::Format::Bc3,
            };
            // a level smaller than a block still occupies a whole one
            let blocks = ((width as usize).div_ceil(4)) * ((height as usize).div_ceil(4));
            let src = data.get(..blocks * compression.block_size())?;
            compression.decompress(src, width as usize, height as usize, &mut rgba);
        }
        _ => return None,
    }
    Some(rgba)
}
