use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::NiQuatTransform;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransformInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub transform: NiQuatTransform,
    pub data_ref: i32,
}

impl NiTransformInterpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
