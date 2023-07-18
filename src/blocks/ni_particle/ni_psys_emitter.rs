use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

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

impl NiPSysEmitter {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
