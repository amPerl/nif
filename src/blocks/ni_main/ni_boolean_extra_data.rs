use super::ni_string::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBooleanExtraData {
    pub name: NiString,
    #[br(map = |x: u8| x > 0)]
    pub value: bool,
}
