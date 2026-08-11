use crate::blocks::NiTimeController;
use crate::common::BlockRef;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NodeSet {
    pub num_nodes: u32,
    #[br(count = num_nodes)]
    pub node_refs: Vec<BlockRef>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct SkinInfo {
    pub shape_ref: BlockRef,
    pub skin_instance_ref: BlockRef,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct SkinInfoSet {
    pub num_skin_info: u32,
    #[br(count = num_skin_info)]
    pub skin_info: Vec<SkinInfo>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBoneLODController {
    pub base: NiTimeController,
    pub lod: u32,
    pub num_lods: u32,
    pub num_node_groups: u32,
    #[br(count = num_lods)]
    pub node_groups: Vec<NodeSet>,
    pub num_shape_groups: u32,
    #[br(count = num_shape_groups)]
    pub shape_groups_1: Vec<SkinInfoSet>,
    pub num_shape_groups_2: u32,
    #[br(count = num_shape_groups_2)]
    pub shape_groups_2: Vec<BlockRef>,
}

impl std::ops::Deref for NiBoneLODController {
    type Target = NiTimeController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
