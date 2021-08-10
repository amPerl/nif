use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: f32, // Pose value if lacking NiFloatData
    pub data_ref: i32,
}

impl NiFloatInterpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
