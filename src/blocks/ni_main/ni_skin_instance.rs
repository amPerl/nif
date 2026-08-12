
use crate::common::BlockRef;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiSkinInstance {
    pub data_ref: BlockRef,
    pub skin_partition: i32,
    pub skeleton_root: i32,
    #[br(temp)]
    #[bw(calc = bone_refs.len() as u32)]
    num_bones: u32,
    #[br(count = num_bones)]
    pub bone_refs: Vec<BlockRef>,
}
