use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSkinInstance {
    pub data_ref: i32,
    pub skin_partition: i32,
    pub skeleton_root: i32,
    pub num_bones: u32,
    #[br(count = num_bones)]
    pub bone_refs: Vec<i32>,
}

impl NiSkinInstance {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
