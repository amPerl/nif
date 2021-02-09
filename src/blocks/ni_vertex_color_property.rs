use super::ni_object::NiObject;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiVertexColorProperty {
    pub base: NiObject,
    pub flags: u16,
    pub vertex_mode: VertMode,
    pub lighting_mode: LightMode,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum VertMode {
    #[br(magic = 0u32)]
    SourceIgnore,
    #[br(magic = 1u32)]
    SourceEmissive,
    #[br(magic = 2u32)]
    SourceAmbientDiffuse,
    Unknown,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum LightMode {
    #[br(magic = 0u32)]
    Emissive,
    #[br(magic = 1u32)]
    EmissiveAmbientDiffuse,
    Unknown,
}

impl NiVertexColorProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
