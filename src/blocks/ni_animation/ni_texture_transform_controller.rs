use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTextureTransformController {
    pub base: NiFloatInterpController,
    #[br(map = |x: u8| x > 0)]
    pub shader_map: bool,
    pub texture_slot: u32, // TexType
    pub operation: u32,    // TexTransform (new: TransformMember)
}
