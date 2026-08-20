use binrw::{BinRead, BinWrite};

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiBoolData {
    pub data: KeyGroup<u8>,
}
