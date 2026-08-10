use binrw::BinRead;

use crate::common::{KeyGroup, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPosData {
    pub data: KeyGroup<Vector3>,
}
