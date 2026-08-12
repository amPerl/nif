use crate::common::ByteColor4;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiPalette {
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_alpha: bool,
    #[br(temp)]
    #[bw(calc = palette.len() as u32)]
    num_entries: u32,
    #[br(count = num_entries)]
    pub palette: Vec<ByteColor4>,
}
