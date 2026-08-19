
use super::NiGeometryData;
use crate::common::{Quaternion, Vector3};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiParticlesData {
    pub base: NiGeometryData,

    #[br(temp)]
    #[bw(calc = u8::from(radii.is_some()))]
    has_radii: u8,
    #[br(if(has_radii != 0), count = base.vertex_count())]
    pub radii: Option<Vec<f32>>,

    pub num_active: u16,

    #[br(temp)]
    #[bw(calc = u8::from(sizes.is_some()))]
    has_sizes: u8,
    #[br(if(has_sizes != 0), count = base.vertex_count())]
    pub sizes: Option<Vec<f32>>,

    #[br(temp)]
    #[bw(calc = u8::from(rotations.is_some()))]
    has_rotations: u8,
    #[br(if(has_rotations != 0), count = base.vertex_count())]
    pub rotations: Option<Vec<Quaternion>>,

    #[br(temp)]
    #[bw(calc = u8::from(rotation_angles.is_some()))]
    has_rotation_angles: u8,
    #[br(if(has_rotation_angles != 0), count = base.vertex_count())]
    pub rotation_angles: Option<Vec<f32>>,

    #[br(temp)]
    #[bw(calc = u8::from(rotation_axes.is_some()))]
    has_rotation_axes: u8,
    #[br(if(has_rotation_axes != 0), count = base.vertex_count())]
    pub rotation_axes: Option<Vec<Vector3>>,
}

impl std::ops::Deref for NiParticlesData {
    type Target = NiGeometryData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
