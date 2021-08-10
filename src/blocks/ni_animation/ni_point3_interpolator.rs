use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::Vector3;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPoint3Interpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: Vector3,
    pub data_ref: i32,
}

impl NiPoint3Interpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
