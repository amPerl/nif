use super::ni_switch_node::NiSwitchNode;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiLODNode {
    pub base: NiSwitchNode,
    pub lod_level_data_ref: i32,
}

impl NiLODNode {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
