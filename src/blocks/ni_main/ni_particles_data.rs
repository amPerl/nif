use binrw::{BinRead, BinWrite};

use super::NiGeometryData;
use crate::common::{Quaternion, Vector3};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiParticlesData {
    pub base: NiGeometryData,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_radii: bool,
    #[br(if(has_radii), count = base.num_vertices)]
    pub radii: Option<Vec<f32>>,

    pub num_active: u16,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_sizes: bool,
    #[br(if(has_sizes), count = base.num_vertices)]
    pub sizes: Option<Vec<f32>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_rotations: bool,
    #[br(if(has_rotations), count = base.num_vertices)]
    pub rotations: Option<Vec<Quaternion>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_rotation_angles: bool,
    #[br(if(has_rotation_angles), count = base.num_vertices)]
    pub rotation_angles: Option<Vec<f32>>,

    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_rotation_axes: bool,
    #[br(if(has_rotation_axes), count = base.num_vertices)]
    pub rotation_axes: Option<Vec<Vector3>>,
}

impl std::ops::Deref for NiParticlesData {
    type Target = NiGeometryData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
