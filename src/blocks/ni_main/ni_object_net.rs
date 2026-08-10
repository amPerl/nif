use crate::common::BlockRef;

use super::ni_string::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiObjectNET {
    pub name: NiString,
    pub num_extra_data_refs: u32,
    #[br(count = num_extra_data_refs)]
    pub extra_data_refs: Vec<BlockRef>,
    pub controller_ref: BlockRef,
}
