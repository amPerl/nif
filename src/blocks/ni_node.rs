use super::ni_av_object::NiAvObject;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiNode {
    pub base: NiAvObject,
    pub num_child_refs: u32,
    #[br(count = num_child_refs)]
    pub child_refs: Vec<i32>,
    pub num_effect_refs: u32,
    #[br(count = num_effect_refs)]
    pub effect_refs: Vec<i32>,
}

impl NiNode {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
