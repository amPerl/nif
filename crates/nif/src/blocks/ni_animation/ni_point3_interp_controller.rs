use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPoint3InterpController {
    pub base: NiSingleInterpController,
}

impl NiPoint3InterpController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
