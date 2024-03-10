use crate::common::BlockRef;

use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFlipController {
    pub base: NiFloatInterpController,
    pub texture_slot: u32, // TexType
    pub num_sources: u32,
    #[br(count = num_sources)]
    pub source_refs: Vec<BlockRef>,
}

impl NiFlipController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
