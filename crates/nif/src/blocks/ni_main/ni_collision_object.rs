use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiCollisionObject {
    pub target_ref: BlockRef,
}
