use nif::blocks::{NiPalette, NiPixelData, PixelFormat};

/// Decode embedded pixel data to RGBA8. wgpu cannot be assumed to support BC formats,
/// so compressed textures are decompressed on the CPU.
pub fn decode_texture(
    pixels: &NiPixelData,
    palette: Option<&NiPalette>,
) -> Option<(u32, u32, Vec<u8>)> {
    let mip = pixels.mipmaps.first()?;
    let face = pixels.pixel_data.first()?;
    let (width, height) = (mip.width.max(1), mip.height.max(1));
    let count = (width as usize) * (height as usize);
    let data = face.data.get(mip.offset as usize..)?;

    let mut rgba = vec![255u8; count * 4];
    match pixels.base.pixel_format {
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
            let format = match pixels.base.pixel_format {
                PixelFormat::FmtDXT1 => texpresso::Format::Bc1,
                PixelFormat::FmtDXT3 => texpresso::Format::Bc2,
                _ => texpresso::Format::Bc3,
            };
            let blocks = ((width as usize).div_ceil(4)) * ((height as usize).div_ceil(4));
            let src = data.get(..blocks * format.block_size())?;
            format.decompress(src, width as usize, height as usize, &mut rgba);
        }
        _ => return None,
    }
    Some((width, height, rgba))
}
