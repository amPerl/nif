use crate::common::BlockRef;

use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiFlipController {
    pub base: NiFloatInterpController,
    pub texture_slot: u32, // TexType
    pub num_sources: u32,
    #[br(count = num_sources)]
    pub source_refs: Vec<BlockRef>,
}
