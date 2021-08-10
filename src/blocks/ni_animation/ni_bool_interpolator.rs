use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBoolInterpolator {
    pub base: NiKeyBasedInterpolator,
    #[br(map = |x: u8| x > 0)]
    pub value: bool, // Pose value if lacking NiBoolData
    pub data_ref: i32,
}

impl NiBoolInterpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
