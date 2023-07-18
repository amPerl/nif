use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAlphaController {
    pub base: NiFloatInterpController,
}

impl NiAlphaController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
