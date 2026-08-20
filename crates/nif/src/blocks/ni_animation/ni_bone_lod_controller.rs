use crate::blocks::NiTimeController;
use crate::common::BlockRef;
use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NodeSet {
    #[br(temp)]
    #[bw(calc = node_refs.len() as u32)]
    num_nodes: u32,
    #[br(count = num_nodes)]
    pub node_refs: Vec<BlockRef>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SkinInfo {
    pub shape_ref: BlockRef,
    pub skin_instance_ref: BlockRef,
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SkinInfoSet {
    #[br(temp)]
    #[bw(calc = skin_info.len() as u32)]
    num_skin_info: u32,
    #[br(count = num_skin_info)]
    pub skin_info: Vec<SkinInfo>,
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiBoneLODController {
    pub base: NiTimeController,
    pub lod: u32,
    #[br(temp)]
    #[bw(calc = node_groups.len() as u32)]
    num_lods: u32,
    pub num_node_groups: u32,
    #[br(count = num_lods)]
    pub node_groups: Vec<NodeSet>,
    #[br(temp)]
    #[bw(calc = shape_groups_1.len() as u32)]
    num_shape_groups: u32,
    #[br(count = num_shape_groups)]
    pub shape_groups_1: Vec<SkinInfoSet>,
    #[br(temp)]
    #[bw(calc = shape_groups_2.len() as u32)]
    num_shape_groups_2: u32,
    #[br(count = num_shape_groups_2)]
    pub shape_groups_2: Vec<BlockRef>,
}

impl std::ops::Deref for NiBoneLODController {
    type Target = NiTimeController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
