use crate::common::BlockRef;

use super::ni_interp_controller::NiInterpController;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSingleInterpController {
    pub base: NiInterpController,
    pub interpolator_ref: BlockRef,
}

impl NiSingleInterpController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
