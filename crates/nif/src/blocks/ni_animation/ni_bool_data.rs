use binrw::{BinRead, BinWrite};

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiBoolData {
    pub data: KeyGroup<u8>,
}
