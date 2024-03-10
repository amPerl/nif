use super::ni_object_net::NiObjectNET;
use crate::common::{BlockRef, TexCoord};

use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTexturingProperty {
    pub base: NiObjectNET,
    pub apply_mode: ApplyMode,
    pub texture_count: u32,

    #[br(map = |x: u8| x > 0)]
    pub has_base_texture: bool,
    #[br(if(has_base_texture))]
    pub base_texture: Option<TexDesc>,

    #[br(map = |x: u8| x > 0)]
    pub has_dark_texture: bool,
    #[br(if(has_dark_texture))]
    pub dark_texture: Option<TexDesc>,

    #[br(map = |x: u8| x > 0)]
    pub has_detail_texture: bool,
    #[br(if(has_detail_texture))]
    pub detail_texture: Option<TexDesc>,

    #[br(map = |x: u8| x > 0)]
    pub has_gloss_texture: bool,
    #[br(if(has_gloss_texture))]
    pub gloss_texture: Option<TexDesc>,

    #[br(map = |x: u8| x > 0)]
    pub has_glow_texture: bool,
    #[br(if(has_glow_texture))]
    pub glow_texture: Option<TexDesc>,

    #[br(map = |x: u8| x > 0)]
    pub has_bump_map_texture: bool,
    #[br(if(has_bump_map_texture))]
    pub bump_map_texture: Option<TexDesc>,

    #[br(map = |x: u8| x > 0)]
    pub has_decal0_texture: bool,
    #[br(if(has_decal0_texture))]
    pub decal0_texture: Option<TexDesc>,

    pub num_shader_textures: u32,
    #[br(count=num_shader_textures)]
    pub shader_textures: Vec<ShaderTexDesc>,
}
#[derive(Debug, PartialEq, BinRead)]
pub struct TexDesc {
    pub source_ref: BlockRef,
    pub clamp_mode: TexClampMode,
    pub filter_mode: TexFilterMode,
    pub uv_set: u32,
    #[br(map = |x: u8| x > 0)]
    pub has_texture_transform: bool,
    #[br(if(has_texture_transform))]
    pub translation: Option<TexCoord>,
    #[br(if(has_texture_transform))]
    pub tiling: Option<TexCoord>,
    #[br(if(has_texture_transform))]
    pub w_rotation: Option<f32>,
    #[br(if(has_texture_transform))]
    pub transform_type: Option<u32>,
    #[br(if(has_texture_transform))]
    pub center_offset: Option<TexCoord>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct ShaderTexDesc {
    #[br(map = |x: u8| x > 0)]
    pub has_map: bool,
    #[br(if(has_map))]
    pub map: Option<TexDesc>,
    pub map_id: u32,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum ApplyMode {
    #[br(magic = 0u32)]
    Replace,
    #[br(magic = 1u32)]
    Decal,
    #[br(magic = 2u32)]
    Modulate,
    #[br(magic = 3u32)]
    Hilight,
    #[br(magic = 4u32)]
    Hilight2,
    Unknown,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum TexClampMode {
    #[br(magic = 0u32)]
    ClampSClampT,
    #[br(magic = 1u32)]
    ClampSWrapT,
    #[br(magic = 2u32)]
    WrapSClampT,
    #[br(magic = 3u32)]
    WrapSWrapT,
    Unknown,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum TexFilterMode {
    #[br(magic = 0u32)]
    Nearest,
    #[br(magic = 1u32)]
    Bilerp,
    #[br(magic = 2u32)]
    Trilerp,
    #[br(magic = 3u32)]
    NearestMipNearest,
    #[br(magic = 4u32)]
    NearestMipLerp,
    #[br(magic = 5u32)]
    BilerpMipNearest,
    Unknown,
}

impl NiTexturingProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiTexturingProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
