use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;
use crate::common::{BlockRef, Vector3};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysGravityModifier {
    pub base: NiPSysModifier,
    pub gravity_object_ref: BlockRef,
    pub gravity_axis: Vector3,
    pub decay: f32,
    pub strength: f32,
    pub force_type: ForceType,
    pub turbulence: f32,
    pub turbulence_scale: f32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum ForceType {
    #[brw(magic = 0u32)]
    Planar,
    #[brw(magic = 1u32)]
    Spherical,
    Unknown(u32),
}
