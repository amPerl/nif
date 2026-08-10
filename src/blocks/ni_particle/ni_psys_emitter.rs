use binrw::BinRead;

use super::NiPSysModifier;
use crate::common::Color4;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysEmitter {
    pub base: NiPSysModifier,
    pub speed: f32,
    pub speed_variation: f32,
    pub declination: f32,
    pub declination_variation: f32,
    pub planar_angle: f32,
    pub planar_angle_variation: f32,
    pub initial_color: Color4,
    pub initial_radius: f32,
    pub radius_variation: f32,
    pub life_span: f32,
    pub life_span_variation: f32,
}
