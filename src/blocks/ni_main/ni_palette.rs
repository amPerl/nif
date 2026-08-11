use crate::common::ByteColor4;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPalette {
    #[br(map = |x: u8| x > 0)]
    pub has_alpha: bool,
    pub num_entries: u32,
    #[br(count = if num_entries == 16 { 16 } else { 256 })]
    pub palette: Vec<ByteColor4>,
}
