use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiObjectNET;
use crate::common::Color3;
#[derive(Debug, PartialEq, BinRead)]
pub struct NiMaterialProperty {
    pub base: NiObjectNET,
    pub color_ambient: Color3,
    pub color_diffuse: Color3,
    pub color_specular: Color3,
    pub color_emissive: Color3,
    pub glossiness: f32,
    pub alpha: f32,
}

impl NiMaterialProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
