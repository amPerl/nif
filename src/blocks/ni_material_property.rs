use super::ni_object::NiObject;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiMaterialProperty {
    pub base: NiObject,
    pub color_ambient: (f32, f32, f32),
    pub color_diffuse: (f32, f32, f32),
    pub color_specular: (f32, f32, f32),
    pub color_emissive: (f32, f32, f32),
    pub glossiness: f32,
    pub alpha: f32,
}

impl NiMaterialProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
