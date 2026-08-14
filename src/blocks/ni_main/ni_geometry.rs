use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::{NiAvObject, NiString};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiGeometry {
    pub base: NiAvObject,
    pub data_ref: BlockRef,
    pub skin_instance_ref: BlockRef,
    pub material_data: MaterialData,
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct MaterialData {
    #[br(temp)]
    #[bw(calc = u8::from(shader_name.is_some()))]
    has_shader: u8,
    #[br(if(has_shader != 0))]
    pub shader_name: Option<NiString>,
    #[br(if(has_shader != 0))]
    pub shader_extra_data_ref: Option<BlockRef>,
}

impl std::ops::Deref for NiGeometry {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
