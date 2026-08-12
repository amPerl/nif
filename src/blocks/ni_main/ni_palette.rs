use crate::common::ByteColor4;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPalette {
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_alpha: bool,
    pub num_entries: u32,
    #[br(count = num_entries)]
    pub palette: Vec<ByteColor4>,
}
