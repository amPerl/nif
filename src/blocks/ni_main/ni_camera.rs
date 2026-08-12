use crate::common::BlockRef;

use super::NiAvObject;

use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiCamera {
    pub base: NiAvObject,
    pub flags: u16,
    pub frustum_left: f32,
    pub frustum_right: f32,
    pub frustum_top: f32,
    pub frustum_bottom: f32,
    pub frustum_near: f32,
    pub frustum_far: f32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub use_orthographic_projection: bool,
    pub viewport_left: f32,
    pub viewport_right: f32,
    pub viewport_top: f32,
    pub viewport_bottom: f32,
    pub lod_adjust: f32,
    pub scene: BlockRef,
    pub num_screen_polygons: u32,
    pub num_screen_textures: u32,
}

impl std::ops::Deref for NiCamera {
    type Target = NiAvObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
