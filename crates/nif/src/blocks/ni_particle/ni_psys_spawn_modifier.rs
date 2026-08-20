use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysSpawnModifier {
    pub base: NiPSysModifier,
    pub num_spawn_generations: u16,
    pub percentage_spawned: f32,
    pub min_num_to_spawn: u16,
    pub max_num_to_spawn: u16,
    pub spawn_speed_variation: f32,
    pub spawn_dir_variation: f32,
    pub life_span: f32,
    pub life_span_variation: f32,
}
