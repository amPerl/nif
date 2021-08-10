use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiAvObject;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiDynamicEffect {
    pub base: NiAvObject,
    #[br(map = |x: u8| x > 0)]
    pub switch_state: bool,
    pub num_affected_nodes: u32,
    #[br(count = num_affected_nodes)]
    pub affected_node_refs: Vec<i32>,
}

impl NiDynamicEffect {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
