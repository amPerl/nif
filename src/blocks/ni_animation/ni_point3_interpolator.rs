use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiKeyBasedInterpolator;
use crate::common::{BlockRef, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPoint3Interpolator {
    pub base: NiKeyBasedInterpolator,
    pub value: Vector3,
    pub data_ref: BlockRef,
}

impl NiPoint3Interpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
