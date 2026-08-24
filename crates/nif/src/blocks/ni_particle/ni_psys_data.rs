use binrw::{BinRead, BinWrite};

use crate::{blocks::NiParticlesData, common::Vector3};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysData {
    pub base: NiParticlesData,

    #[br(count = base.base.vertex_count())]
    pub particle_info: Vec<NiParticleInfo>,

    #[br(temp)]
    #[bw(calc = u8::from(rotation_speeds.is_some()))]
    has_rotation_speeds: u8,
    #[br(if(has_rotation_speeds != 0), count = base.base.vertex_count())]
    pub rotation_speeds: Option<Vec<f32>>,

    pub num_added_particles: u16,
    pub added_particles_base: u16,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiParticleInfo {
    pub velocity: Vector3,
    pub age: f32,
    pub life_span: f32,
    pub last_update: f32,
    pub spawn_generation: u16,
    pub code: u16,
}

impl std::ops::Deref for NiPSysData {
    type Target = NiParticlesData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
