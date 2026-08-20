use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::{NiAvObject, NiString};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiGeometry {
    pub base: NiAvObject,
    pub data_ref: BlockRef,
    pub skin_instance_ref: BlockRef,
    pub material_data: MaterialData,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ShaderInfo {
    pub name: NiString,
    pub extra_data_ref: BlockRef,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum MaterialData {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Shader(ShaderInfo),
    Invalid {
        flag: u8,
        shader: ShaderInfo,
    },
}

impl MaterialData {
    pub fn shader(&self) -> Option<&ShaderInfo> {
        match self {
            MaterialData::None => Option::None,
            MaterialData::Shader(shader) => Some(shader),
            MaterialData::Invalid { shader, .. } => Some(shader),
        }
    }
}

impl std::ops::Deref for NiGeometry {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
