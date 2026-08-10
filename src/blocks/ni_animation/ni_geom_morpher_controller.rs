use crate::common::BlockRef;

use super::ni_interp_controller::NiInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiGeomMorpherController {
    pub base: NiInterpController,
    pub morpher_flags: GeomMorpherFlags,
    pub data_ref: BlockRef,
    pub always_update: u8,
    pub num_interpolators: u32,
    #[br(count = num_interpolators)]
    pub interpolator_refs: Vec<BlockRef>,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum GeomMorpherFlags {
    #[br(magic = 0u16)]
    UpdateNormalsDisabled,
    #[br(magic = 1u16)]
    UpdateNormalsEnabled,
}
