use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTimeController {
    pub next_controller_ref: BlockRef,
    pub flags: u16, // TimeControllerFlags, bitfield
    pub frequency: f32,
    pub phase: f32,
    pub start_time: f32,
    pub end_time: f32,
    pub target_ref: BlockRef,
}

impl NiTimeController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
