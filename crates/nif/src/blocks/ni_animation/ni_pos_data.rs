use binrw::{BinRead, BinWrite};

use crate::common::{KeyGroup, Vector3};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPosData {
    pub data: KeyGroup<Vector3>,
}
