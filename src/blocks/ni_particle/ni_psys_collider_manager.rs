use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysColliderManager {
    pub base: NiPSysModifier,
    pub collider_ref: BlockRef,
}

impl NiPSysColliderManager {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
