use binrw::BinRead;

use crate::{blocks::NiParticlesData, common::Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysData {
    pub base: NiParticlesData,

    #[br(count = base.base.num_vertices)]
    pub particle_info: Vec<NiParticleInfo>,

    #[br(map = |x: u8| x > 0)]
    pub has_rotation_speeds: bool,
    #[br(if(has_rotation_speeds), count = base.base.num_vertices)]
    pub rotation_speeds: Option<Vec<f32>>,

    pub num_added_particles: u16,
    pub added_particles_base: u16,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct NiParticleInfo {
    pub velocity: Vector3,
    pub age: f32,
    pub life_span: f32,
    pub last_update: f32,
    pub spawn_generation: u16,
    pub code: u16,
}
