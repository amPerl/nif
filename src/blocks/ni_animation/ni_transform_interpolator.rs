use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiKeyBasedInterpolator;
use crate::common::{BlockRef, NiQuatTransform};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransformInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub transform: NiQuatTransform,
    pub data_ref: BlockRef,
}

impl NiTransformInterpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
