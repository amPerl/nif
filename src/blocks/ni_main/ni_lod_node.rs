use crate::common::BlockRef;

use super::ni_switch_node::NiSwitchNode;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiLODNode {
    pub base: NiSwitchNode,
    pub lod_level_data_ref: BlockRef,
}

impl NiLODNode {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiLODNode {
    type Target = NiSwitchNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
