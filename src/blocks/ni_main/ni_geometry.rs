use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiAvObject;
use crate::NifError;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiGeometry {
    pub base: NiAvObject,
    pub data_ref: i32,
    pub skin_instance_ref: i32,
    pub material_data: MaterialData,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct MaterialData {
    #[br(assert(!has_shader, NifError::NotImplemented("MaterialData with shader not implemented")))]
    #[br(map = |x: u8| x > 0)]
    pub has_shader: bool,
}

impl NiGeometry {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
