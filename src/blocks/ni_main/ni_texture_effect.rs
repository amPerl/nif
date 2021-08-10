use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::{Matrix33, Vector3};

use super::{NiDynamicEffect, TexClampMode, TexFilterMode};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTextureEffect {
    pub base: NiDynamicEffect,
    pub model_projection_matrix: Matrix33,
    pub model_projection_translation: Vector3,
    pub texture_filtering: TexFilterMode,
    pub texture_clamping: TexClampMode,
    pub texture_type: u32,               // EffectType (new: TextureType)
    pub coordinate_generation_type: u32, // CoordGenType
    pub source_texture_ref: i32,
    pub enable_plane: u8,
    pub plane: NiPlane,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPlane {
    pub normal: Vector3,
    pub constant: f32,
}

impl NiTextureEffect {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
