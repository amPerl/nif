use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiGeometryData;
use crate::common::{Quaternion, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiParticlesData {
    pub base: NiGeometryData,

    #[br(map = |x: u8| x > 0)]
    pub has_radii: bool,
    #[br(if(has_radii), count = base.num_vertices)]
    pub radii: Option<Vec<f32>>,

    pub num_active: u16,

    #[br(map = |x: u8| x > 0)]
    pub has_sizes: bool,
    #[br(if(has_sizes), count = base.num_vertices)]
    pub sizes: Option<Vec<f32>>,

    #[br(map = |x: u8| x > 0)]
    pub has_rotations: bool,
    #[br(if(has_rotations), count = base.num_vertices)]
    pub rotations: Option<Vec<Quaternion>>,

    #[br(map = |x: u8| x > 0)]
    pub has_rotation_angles: bool,
    #[br(if(has_rotation_angles), count = base.num_vertices)]
    pub rotation_angles: Option<Vec<f32>>,

    #[br(map = |x: u8| x > 0)]
    pub has_rotation_axes: bool,
    #[br(if(has_rotation_axes), count = base.num_vertices)]
    pub rotation_axes: Option<Vec<Vector3>>,
}

impl NiParticlesData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiParticlesData {
    type Target = NiGeometryData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
