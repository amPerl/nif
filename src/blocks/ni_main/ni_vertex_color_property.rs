use super::ni_object_net::NiObjectNET;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiVertexColorProperty {
    pub base: NiObjectNET,
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
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead)]
pub enum LightMode {
    #[br(magic = 0u32)]
    Emissive,
    #[br(magic = 1u32)]
    EmissiveAmbientDiffuse,
    Unknown(u32),
}

impl std::ops::Deref for NiVertexColorProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
