use binrw::BinRead;

use crate::common::BlockRef;

use super::NiPSysEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysVolumeEmitter {
    pub base: NiPSysEmitter,
    pub emitter_object_ref: BlockRef,
}
