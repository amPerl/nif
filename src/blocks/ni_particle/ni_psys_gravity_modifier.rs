use binrw::BinRead;

use super::NiPSysModifier;
use crate::common::{BlockRef, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysGravityModifier {
    pub base: NiPSysModifier,
    pub gravity_object_ref: BlockRef,
    pub gravity_axis: Vector3,
    pub decay: f32,
    pub strength: f32,
    pub force_type: u32, // ForceType
    pub turbulence: f32,
    pub turbulence_scale: f32,
}
