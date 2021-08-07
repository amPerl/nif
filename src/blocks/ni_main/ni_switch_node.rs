use super::ni_node::NiNode;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSwitchNode {
    pub base: NiNode,
    pub switch_node_flags: NiSwitchFlags,
    pub index: u32,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum NiSwitchFlags {
    #[br(magic = 0u16)]
    UpdateOnlyActiveChild,
    #[br(magic = 1u16)]
    UpdateControllers,
}

impl NiSwitchNode {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
