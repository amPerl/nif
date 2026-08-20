use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTextureTransformController {
    pub base: NiFloatInterpController,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub shader_map: bool,
    pub texture_slot: u32, // TexType
    pub operation: u32,    // TexTransform (new: TransformMember)
}
