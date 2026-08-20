use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiPSysEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysVolumeEmitter {
    pub base: NiPSysEmitter,
    pub emitter_object_ref: BlockRef,
}
