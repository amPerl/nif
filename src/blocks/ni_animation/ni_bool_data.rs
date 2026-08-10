use binrw::BinRead;

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBoolData {
    pub data: KeyGroup<u8>,
}
