use binrw::BinRead;

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiCollisionObject {
    pub target_ref: BlockRef,
}
