use super::{ni_av_object::NiAvObject, ni_string::NiString};
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriShape {
    pub base: NiAvObject,
    pub data_ref: i32,
    pub skin_instance_ref: i32,
    #[br(map = |x: u8| x > 0)]
    pub has_shader: bool,
    #[br(if(has_shader))]
    pub material_data: Option<MaterialDataShader>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct MaterialDataShader {
    pub name: NiString,
    pub extra_data_ref: i32,
}

impl NiTriShape {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
