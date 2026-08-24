use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;
use crate::common::Vector3;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysRotationModifier {
    pub base: NiPSysModifier,
    pub initial_rotation_speed: f32,
    pub initial_rotation_speed_variation: f32,
    pub initial_rotation_angle: f32,
    pub initial_rotation_angle_variation: f32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub random_rot_speed_sign: bool,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub random_initial_axis: bool,
    pub initial_axis: Vector3,
}

impl std::ops::Deref for NiPSysRotationModifier {
    type Target = NiPSysModifier;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
