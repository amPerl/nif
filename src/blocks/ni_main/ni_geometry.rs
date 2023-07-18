use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

use super::{NiAvObject, NiString};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiGeometry {
    pub base: NiAvObject,
    pub data_ref: BlockRef,
    pub skin_instance_ref: BlockRef,
    #[br(map = |x: u8| x > 0)]
    pub has_shader: bool,
    #[br(if(has_shader))]
    pub material_data: Option<MaterialDataShader>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct MaterialDataShader {
    pub name: NiString,
    pub extra_data_ref: BlockRef,
}

impl NiGeometry {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiGeometry {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
