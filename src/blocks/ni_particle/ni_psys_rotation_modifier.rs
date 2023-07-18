use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiPSysModifier;
use crate::common::Vector3;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysRotationModifier {
    pub base: NiPSysModifier,
    pub initial_rotation_speed: f32,
    pub initial_rotation_speed_variation: f32,
    pub initial_rotation_angle: f32,
    pub initial_rotation_angle_variation: f32,
    #[br(map = |x: u8| x > 0)]
    pub random_rot_speed_sign: bool,
    #[br(map = |x: u8| x > 0)]
    pub random_initial_axis: bool,
    pub initial_axis: Vector3,
}

impl NiPSysRotationModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
