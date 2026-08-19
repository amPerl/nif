use crate::common::BlockRef;

use super::ni_interp_controller::NiInterpController;
use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiGeomMorpherController {
    pub base: NiInterpController,
    pub morpher_flags: GeomMorpherFlags,
    pub data_ref: BlockRef,
    pub always_update: u8,
    #[br(temp)]
    #[bw(calc = interpolator_refs.len() as u32)]
    num_interpolators: u32,
    #[br(count = num_interpolators)]
    pub interpolator_refs: Vec<BlockRef>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum GeomMorpherFlags {
    #[brw(magic = 0u16)]
    UpdateNormalsDisabled,
    #[brw(magic = 1u16)]
    UpdateNormalsEnabled,
    Unknown(u16),
}
