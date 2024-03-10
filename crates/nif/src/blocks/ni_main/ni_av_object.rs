use super::ni_object_net::NiObjectNET;
use crate::common::{BlockRef, Matrix33, Vector3};

use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAvObject {
    pub base: NiObjectNET,
    pub flags: u16,
    pub translation: Vector3,
    pub rotation: Matrix33,
    pub scale: f32,
    pub num_property_refs: u32,
    #[br(count = num_property_refs)]
    pub property_refs: Vec<BlockRef>,
    pub collision_ref: BlockRef,
}

impl NiAvObject {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }

    pub fn transform(&self) -> glam::Mat4 {
        let translation_orig = &self.translation;
        let rotation_orig = &self.rotation;

        let translation: glam::Mat4 = glam::Mat4::from_translation(translation_orig.into());
        let rotation = glam::Mat4::from_mat3(rotation_orig.into());
        let scale: glam::Mat4 =
            glam::Mat4::from_scale(glam::Vec3::new(self.scale, self.scale, self.scale));

        translation * rotation * scale
    }
}

impl std::ops::Deref for NiAvObject {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
