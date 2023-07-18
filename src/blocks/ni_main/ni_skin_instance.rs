use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSkinInstance {
    pub data_ref: BlockRef,
    pub skin_partition: i32,
    pub skeleton_root: i32,
    pub num_bones: u32,
    #[br(count = num_bones)]
    pub bone_refs: Vec<BlockRef>,
}

impl NiSkinInstance {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
