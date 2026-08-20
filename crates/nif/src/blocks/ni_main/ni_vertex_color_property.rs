use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiVertexColorProperty {
    pub base: NiObjectNET,
    pub flags: u16,
    pub vertex_mode: VertMode,
    pub lighting_mode: LightMode,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum VertMode {
    #[brw(magic = 0u32)]
    SourceIgnore,
    #[brw(magic = 1u32)]
    SourceEmissive,
    #[brw(magic = 2u32)]
    SourceAmbientDiffuse,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum LightMode {
    #[brw(magic = 0u32)]
    Emissive,
    #[brw(magic = 1u32)]
    EmissiveAmbientDiffuse,
    Unknown(u32),
}

impl std::ops::Deref for NiVertexColorProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
