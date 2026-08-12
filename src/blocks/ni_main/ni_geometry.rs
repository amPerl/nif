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

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct MaterialData {
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_shader: bool,
    #[br(if(has_shader))]
    pub shader_name: Option<NiString>,
    #[br(if(has_shader))]
    pub shader_extra_data_ref: Option<BlockRef>,
}

impl std::ops::Deref for NiGeometry {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
