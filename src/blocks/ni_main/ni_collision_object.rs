use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiCollisionObject {
    pub target_ref: BlockRef,
}

impl NiCollisionObject {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
