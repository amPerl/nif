use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysAgeDeathModifier {
    pub base: NiPSysModifier,
    #[br(map = |x: u8| x > 0)]
    pub spawn_on_death: bool,
    pub spawn_modifier_ref: i32,
}

impl NiPSysAgeDeathModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
