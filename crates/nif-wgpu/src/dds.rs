//! Minimal DDS reader.
//!
//! Handles DXT1, DXT3, DXT5 and uncompressed layouts, decoding every mipmap level to RGBA8.

use crate::texture::Decoded;

const MAGIC: &[u8; 4] = b"DDS ";
const HEADER_END: usize = 128;
const DX10_HEADER_END: usize = 148;

const FLAG_MIPMAPCOUNT: u32 = 0x2_0000;
const FLAG_FOURCC: u32 = 0x4;
const FLAG_RGB: u32 = 0x40;
const FLAG_ALPHAPIXELS: u32 = 0x1;

fn u32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

/// A channel mask turned into the shift and maximum needed to rescale it to 8 bits.
struct Channel {
    shift: u32,
    max: u32,
}

impl Channel {
    fn new(mask: u32) -> Option<Self> {
        (mask != 0).then(|| Self {
            shift: mask.trailing_zeros(),
            max: mask >> mask.trailing_zeros(),
        })
    }

    fn sample(&self, pixel: u32, mask: u32) -> u8 {
        let value = (pixel & mask) >> self.shift;
        if self.max == 0 {
            return 0;
        }
        ((value * 255 + self.max / 2) / self.max) as u8
    }
}

/// Decode a DDS to RGBA8, largest level first. A file with no chain gives one level.
pub fn decode(bytes: &[u8]) -> Option<Decoded> {
    if bytes.get(..4)? != MAGIC {
        return None;
    }
    let header_flags = u32_at(bytes, 8)?;
    let height = u32_at(bytes, 12)?;
    let width = u32_at(bytes, 16)?;
    let mip_count = u32_at(bytes, 28)?;
    let pixel_flags = u32_at(bytes, 80)?;
    let four_cc = bytes.get(84..88)?;
    let bit_count = u32_at(bytes, 88)?;
    let masks = [
        u32_at(bytes, 92)?,
        u32_at(bytes, 96)?,
        u32_at(bytes, 100)?,
        u32_at(bytes, 104)?,
    ];

    if width == 0 || height == 0 || width > 16384 || height > 16384 {
        return None;
    }
    // the count is only meaningful where the writer said it set it, and a file claiming more
    // levels than the size allows is reading its own header wrong
    let levels = if header_flags & FLAG_MIPMAPCOUNT != 0 {
        mip_count.clamp(1, width.max(height).ilog2() + 1)
    } else {
        1
    };

    let (layout, payload) = if pixel_flags & FLAG_FOURCC != 0 {
        let (format, payload) = match four_cc {
            b"DXT1" => (texpresso::Format::Bc1, bytes.get(HEADER_END..)?),
            b"DXT3" => (texpresso::Format::Bc2, bytes.get(HEADER_END..)?),
            b"DXT5" => (texpresso::Format::Bc3, bytes.get(HEADER_END..)?),
            b"DX10" => {
                // the DX10 header adds 20 bytes before the payload and gives a numeric format
                let format = match u32_at(bytes, HEADER_END)? {
                    70..=72 => texpresso::Format::Bc1,
                    73..=75 => texpresso::Format::Bc2,
                    76..=78 => texpresso::Format::Bc3,
                    _ => return None,
                };
                (format, bytes.get(DX10_HEADER_END..)?)
            }
            _ => return None,
        };
        (Layout::Block(format), payload)
    } else {
        if pixel_flags & FLAG_RGB == 0 {
            return None;
        }
        let bytes_per_pixel = match bit_count {
            16 | 24 | 32 => (bit_count / 8) as usize,
            _ => return None,
        };
        let channels = Channels {
            has_alpha: pixel_flags & FLAG_ALPHAPIXELS != 0,
            masks,
            channels: [
                Channel::new(masks[0]),
                Channel::new(masks[1]),
                Channel::new(masks[2]),
                Channel::new(masks[3]),
            ],
            bytes_per_pixel,
        };
        (Layout::Loose(channels), bytes.get(HEADER_END..)?)
    };

    // the levels follow one another with no padding between them, so each one starts where the
    // last ended and a file that stops early ends the chain there
    let mut decoded = Decoded {
        width,
        height,
        levels: Vec::with_capacity(levels as usize),
    };
    let mut at = 0usize;
    for level in 0..levels {
        let (width, height) = decoded.size(level);
        let stored = layout.stored(width, height)?;
        let Some(src) = payload.get(at..at.checked_add(stored)?) else {
            break;
        };
        decoded.levels.push(layout.decode(width, height, src)?);
        at += stored;
    }

    (!decoded.levels.is_empty()).then_some(decoded)
}

/// How the texels of one level are stored: in compressed blocks, or loose behind channel masks.
enum Layout {
    Block(texpresso::Format),
    Loose(Channels),
}

struct Channels {
    has_alpha: bool,
    masks: [u32; 4],
    channels: [Option<Channel>; 4],
    bytes_per_pixel: usize,
}

impl Layout {
    /// The bytes one level of this size occupies in the file.
    fn stored(&self, width: u32, height: u32) -> Option<usize> {
        let count = (width as usize).checked_mul(height as usize)?;
        match self {
            Layout::Block(format) => {
                let blocks = (width as usize).div_ceil(4) * (height as usize).div_ceil(4);
                blocks.checked_mul(format.block_size())
            }
            Layout::Loose(loose) => count.checked_mul(loose.bytes_per_pixel),
        }
    }

    fn decode(&self, width: u32, height: u32, src: &[u8]) -> Option<Vec<u8>> {
        let count = (width as usize).checked_mul(height as usize)?;
        let mut rgba = vec![255u8; count.checked_mul(4)?];
        match self {
            Layout::Block(format) => {
                format.decompress(src, width as usize, height as usize, &mut rgba);
            }
            Layout::Loose(loose) => {
                for (i, chunk) in src.chunks_exact(loose.bytes_per_pixel).enumerate() {
                    let mut pixel = 0u32;
                    for (byte, shift) in chunk.iter().zip((0..).step_by(8)) {
                        pixel |= u32::from(*byte) << shift;
                    }
                    let out = rgba.get_mut(i * 4..i * 4 + 4)?;
                    for (slot, (channel, mask)) in
                        loose.channels.iter().zip(loose.masks).enumerate()
                    {
                        let Some(channel) = channel else { continue };
                        if slot == 3 && !loose.has_alpha {
                            continue;
                        }
                        out[slot] = channel.sample(pixel, mask);
                    }
                }
            }
        }
        Some(rgba)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal uncompressed A8R8G8B8 header.
    fn argb8888(width: u32, height: u32, pixels: &[[u8; 4]]) -> Vec<u8> {
        let mut bytes = vec![0u8; HEADER_END];
        bytes[..4].copy_from_slice(MAGIC);
        bytes[12..16].copy_from_slice(&height.to_le_bytes());
        bytes[16..20].copy_from_slice(&width.to_le_bytes());
        bytes[80..84].copy_from_slice(&(FLAG_RGB | FLAG_ALPHAPIXELS).to_le_bytes());
        bytes[88..92].copy_from_slice(&32u32.to_le_bytes());
        bytes[92..96].copy_from_slice(&0x00ff_0000u32.to_le_bytes());
        bytes[96..100].copy_from_slice(&0x0000_ff00u32.to_le_bytes());
        bytes[100..104].copy_from_slice(&0x0000_00ffu32.to_le_bytes());
        bytes[104..108].copy_from_slice(&0xff00_0000u32.to_le_bytes());
        // stored little endian as B, G, R, A
        for p in pixels {
            bytes.extend_from_slice(&[p[2], p[1], p[0], p[3]]);
        }
        bytes
    }

    #[test]
    fn reads_uncompressed_argb() {
        let file = argb8888(2, 1, &[[10, 20, 30, 40], [200, 150, 100, 255]]);
        let decoded = decode(&file).expect("decodes");
        assert_eq!((decoded.width, decoded.height), (2, 1));
        assert_eq!(decoded.levels.len(), 1);
        assert_eq!(decoded.top(), &[10, 20, 30, 40, 200, 150, 100, 255]);
    }

    #[test]
    fn rejects_a_file_that_is_not_a_dds() {
        assert!(decode(b"not a dds file at all").is_none());
    }

    #[test]
    fn rejects_a_truncated_payload() {
        let mut file = argb8888(4, 4, &[[1, 2, 3, 4]]);
        file.truncate(HEADER_END + 8);
        assert!(decode(&file).is_none());
    }

    /// Two levels one after the other, with the count flagged the way a writer that means it does.
    #[test]
    fn reads_a_chain_level_by_level() {
        let mut file = argb8888(2, 2, &[[1, 1, 1, 255]; 4]);
        file[8..12].copy_from_slice(&(FLAG_MIPMAPCOUNT).to_le_bytes());
        file[28..32].copy_from_slice(&2u32.to_le_bytes());
        file.extend_from_slice(&[9, 9, 9, 255]);

        let decoded = decode(&file).expect("decodes");
        assert_eq!(decoded.levels.len(), 2);
        assert_eq!(decoded.size(1), (1, 1));
        assert_eq!(decoded.levels[1], &[9, 9, 9, 255]);
    }

    /// A count the file cannot back up ends the chain rather than failing the whole read.
    #[test]
    fn a_chain_that_stops_early_keeps_what_is_there() {
        let mut file = argb8888(2, 2, &[[1, 1, 1, 255]; 4]);
        file[8..12].copy_from_slice(&(FLAG_MIPMAPCOUNT).to_le_bytes());
        file[28..32].copy_from_slice(&2u32.to_le_bytes());

        let decoded = decode(&file).expect("decodes");
        assert_eq!(decoded.levels.len(), 1);
    }

    #[test]
    fn a_five_six_five_mask_scales_to_full_range() {
        let channel = Channel::new(0xf800).expect("non zero");
        assert_eq!(channel.sample(0xf800, 0xf800), 255);
        assert_eq!(channel.sample(0x0000, 0xf800), 0);
    }
}
