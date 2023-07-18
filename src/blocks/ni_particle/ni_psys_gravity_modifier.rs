use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

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

impl NiPSysGravityModifier {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
