use super::ni_object_net::NiObjectNET;
use crate::common::{BlockRef, Matrix22, TexCoord};

use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiTexturingProperty {
    pub base: NiObjectNET,
    pub apply_mode: ApplyMode,
    pub texture_count: u32,

    #[br(temp)]
    #[bw(calc = u8::from(base_texture.is_some()))]
    has_base_texture: u8,
    #[br(if(has_base_texture != 0))]
    pub base_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(dark_texture.is_some()))]
    has_dark_texture: u8,
    #[br(if(has_dark_texture != 0))]
    pub dark_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(detail_texture.is_some()))]
    has_detail_texture: u8,
    #[br(if(has_detail_texture != 0))]
    pub detail_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(gloss_texture.is_some()))]
    has_gloss_texture: u8,
    #[br(if(has_gloss_texture != 0))]
    pub gloss_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(glow_texture.is_some()))]
    has_glow_texture: u8,
    #[br(if(has_glow_texture != 0))]
    pub glow_texture: Option<Box<TexDesc>>,

    #[br(if(texture_count > 5))]
    pub bump_map: Option<BumpMap>,

    #[br(temp, if(texture_count > 6))]
    #[bw(if(*texture_count > 6), calc = u8::from(decal0_texture.is_some()))]
    has_decal0_texture: u8,
    #[br(if(has_decal0_texture != 0))]
    pub decal0_texture: Option<Box<TexDesc>>,

    #[br(temp, if(texture_count > 7))]
    #[bw(if(*texture_count > 7), calc = u8::from(decal1_texture.is_some()))]
    has_decal1_texture: u8,
    #[br(if(has_decal1_texture != 0))]
    pub decal1_texture: Option<Box<TexDesc>>,

    #[br(temp, if(texture_count > 8))]
    #[bw(if(*texture_count > 8), calc = u8::from(decal2_texture.is_some()))]
    has_decal2_texture: u8,
    #[br(if(has_decal2_texture != 0))]
    pub decal2_texture: Option<Box<TexDesc>>,

    #[br(temp, if(texture_count > 9))]
    #[bw(if(*texture_count > 9), calc = u8::from(decal3_texture.is_some()))]
    has_decal3_texture: u8,
    #[br(if(has_decal3_texture != 0))]
    pub decal3_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = shader_textures.len() as u32)]
    num_shader_textures: u32,
    #[br(count=num_shader_textures)]
    pub shader_textures: Vec<ShaderTexDesc>,
}
#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct BumpMapData {
    pub texture: Box<TexDesc>,
    pub luma_scale: f32,
    pub luma_offset: f32,
    pub matrix: Matrix22,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum BumpMap {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Present(BumpMapData),
    Invalid {
        flag: u8,
        data: BumpMapData,
    },
}

impl BumpMap {
    pub fn get(&self) -> Option<&BumpMapData> {
        match self {
            BumpMap::None => Option::None,
            BumpMap::Present(d) => Some(d),
            BumpMap::Invalid { data, .. } => Some(data),
        }
    }
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct TexDesc {
    pub source_ref: BlockRef,
    pub clamp_mode: TexClampMode,
    pub filter_mode: TexFilterMode,
    pub uv_set: u32,
    pub transform: TexTransform,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct TextureTransform {
    pub translation: TexCoord,
    pub tiling: TexCoord,
    pub w_rotation: f32,
    pub transform_type: u32,
    pub center_offset: TexCoord,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum TexTransform {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Present(TextureTransform),
    Invalid {
        flag: u8,
        transform: TextureTransform,
    },
}

impl TexTransform {
    pub fn get(&self) -> Option<&TextureTransform> {
        match self {
            TexTransform::None => Option::None,
            TexTransform::Present(t) => Some(t),
            TexTransform::Invalid { transform, .. } => Some(transform),
        }
    }
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct ShaderMap {
    pub map: TexDesc,
    pub map_id: u32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum ShaderTexDesc {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Map(ShaderMap),
    Invalid {
        flag: u8,
        map: ShaderMap,
    },
}

impl ShaderTexDesc {
    pub fn get(&self) -> Option<&ShaderMap> {
        match self {
            ShaderTexDesc::None => Option::None,
            ShaderTexDesc::Map(m) => Some(m),
            ShaderTexDesc::Invalid { map, .. } => Some(map),
        }
    }
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
