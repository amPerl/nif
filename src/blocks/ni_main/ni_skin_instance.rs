use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiSkinInstance {
    pub data_ref: BlockRef,
    pub skin_partition: i32,
    pub skeleton_root: i32,
    pub num_bones: u32,
    #[br(count = num_bones)]
    pub bone_refs: Vec<BlockRef>,
}
