use crate::common::BlockRef;

use super::ni_float_interp_controller::NiFloatInterpController;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiFlipController {
    pub base: NiFloatInterpController,
    pub texture_slot: u32, // TexType
    #[br(temp)]
    #[bw(calc = source_refs.len() as u32)]
    num_sources: u32,
    #[br(count = num_sources)]
    pub source_refs: Vec<BlockRef>,
}
