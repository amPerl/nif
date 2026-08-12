use super::ni_object_net::NiObjectNET;
use crate::common::{BlockRef, Matrix22, TexCoord};

use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiTexturingProperty {
    pub base: NiObjectNET,
    pub apply_mode: ApplyMode,
    pub texture_count: u32,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_base_texture: bool,
    #[br(if(has_base_texture))]
    pub base_texture: Option<Box<TexDesc>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_dark_texture: bool,
    #[br(if(has_dark_texture))]
    pub dark_texture: Option<Box<TexDesc>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_detail_texture: bool,
    #[br(if(has_detail_texture))]
    pub detail_texture: Option<Box<TexDesc>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_gloss_texture: bool,
    #[br(if(has_gloss_texture))]
    pub gloss_texture: Option<Box<TexDesc>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_glow_texture: bool,
    #[br(if(has_glow_texture))]
    pub glow_texture: Option<Box<TexDesc>>,

    #[br(if(texture_count > 5), map = |x: Option<u8>| x.is_some_and(|x| x > 0))]
    #[bw(if(*texture_count > 5), map = |x: &bool| u8::from(*x))]
    pub has_bump_map_texture: bool,
    #[br(if(has_bump_map_texture))]
    pub bump_map_texture: Option<Box<TexDesc>>,
    #[br(if(has_bump_map_texture))]
    pub bump_map_luma_scale: Option<f32>,
    #[br(if(has_bump_map_texture))]
    pub bump_map_luma_offset: Option<f32>,
    #[br(if(has_bump_map_texture))]
    pub bump_map_matrix: Option<Matrix22>,

    #[br(if(texture_count > 6), map = |x: Option<u8>| x.is_some_and(|x| x > 0))]
    #[bw(if(*texture_count > 6), map = |x: &bool| u8::from(*x))]
    pub has_decal0_texture: bool,
    #[br(if(has_decal0_texture))]
    pub decal0_texture: Option<Box<TexDesc>>,

    #[br(if(texture_count > 7), map = |x: Option<u8>| x.is_some_and(|x| x > 0))]
    #[bw(if(*texture_count > 7), map = |x: &bool| u8::from(*x))]
    pub has_decal1_texture: bool,
    #[br(if(has_decal1_texture))]
    pub decal1_texture: Option<Box<TexDesc>>,

    #[br(if(texture_count > 8), map = |x: Option<u8>| x.is_some_and(|x| x > 0))]
    #[bw(if(*texture_count > 8), map = |x: &bool| u8::from(*x))]
    pub has_decal2_texture: bool,
    #[br(if(has_decal2_texture))]
    pub decal2_texture: Option<Box<TexDesc>>,

    #[br(if(texture_count > 9), map = |x: Option<u8>| x.is_some_and(|x| x > 0))]
    #[bw(if(*texture_count > 9), map = |x: &bool| u8::from(*x))]
    pub has_decal3_texture: bool,
    #[br(if(has_decal3_texture))]
    pub decal3_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = shader_textures.len() as u32)]
    num_shader_textures: u32,
    #[br(count=num_shader_textures)]
    pub shader_textures: Vec<ShaderTexDesc>,
}
#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct TexDesc {
    pub source_ref: BlockRef,
    pub clamp_mode: TexClampMode,
    pub filter_mode: TexFilterMode,
    pub uv_set: u32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
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

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct ShaderTexDesc {
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_map: bool,
    #[br(if(has_map))]
    pub map: Option<TexDesc>,
    #[br(if(has_map))]
    pub map_id: Option<u32>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum ApplyMode {
    #[brw(magic = 0u32)]
    Replace,
    #[brw(magic = 1u32)]
    Decal,
    #[brw(magic = 2u32)]
    Modulate,
    #[brw(magic = 3u32)]
    Hilight,
    #[brw(magic = 4u32)]
    Hilight2,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum TexClampMode {
    #[brw(magic = 0u32)]
    ClampSClampT,
    #[brw(magic = 1u32)]
    ClampSWrapT,
    #[brw(magic = 2u32)]
    WrapSClampT,
    #[brw(magic = 3u32)]
    WrapSWrapT,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum TexFilterMode {
    #[brw(magic = 0u32)]
    Nearest,
    #[brw(magic = 1u32)]
    Bilerp,
    #[brw(magic = 2u32)]
    Trilerp,
    #[brw(magic = 3u32)]
    NearestMipNearest,
    #[brw(magic = 4u32)]
    NearestMipLerp,
    #[brw(magic = 5u32)]
    BilerpMipNearest,
    Unknown(u32),
}

impl std::ops::Deref for NiTexturingProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
