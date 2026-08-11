use crate::common::ByteColor4;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPalette {
    #[br(map = |x: u8| x > 0)]
    pub has_alpha: bool,
    pub num_entries: u32,
    #[br(count = num_entries)]
    pub palette: Vec<ByteColor4>,
}
