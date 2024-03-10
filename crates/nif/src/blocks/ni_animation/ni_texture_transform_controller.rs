use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTextureTransformController {
    pub base: NiFloatInterpController,
    #[br(map = |x: u8| x > 0)]
    pub shader_map: bool,
    pub texture_slot: u32, // TexType
    pub operation: u32,    // TexTransform (new: TransformMember)
}

impl NiTextureTransformController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
