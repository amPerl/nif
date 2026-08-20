use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiCollisionObject {
    pub target_ref: BlockRef,
}
