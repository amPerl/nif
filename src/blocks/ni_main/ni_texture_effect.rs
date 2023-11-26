use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::{NiDynamicEffect, TexClampMode, TexFilterMode};
use crate::common::{BlockRef, Matrix33, NiPlane, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTextureEffect {
    pub base: NiDynamicEffect,
    pub model_projection_matrix: Matrix33,
    pub model_projection_translation: Vector3,
    pub texture_filtering: TexFilterMode,
    pub texture_clamping: TexClampMode,
    pub texture_type: u32,               // EffectType (new: TextureType)
    pub coordinate_generation_type: u32, // CoordGenType
    pub source_texture_ref: BlockRef,
    pub enable_plane: u8,
    pub plane: NiPlane,
}

impl NiTextureEffect {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiTextureEffect {
    type Target = NiDynamicEffect;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
