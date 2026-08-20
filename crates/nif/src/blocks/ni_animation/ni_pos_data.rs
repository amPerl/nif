use binrw::{BinRead, BinWrite};

use crate::common::{KeyGroup, Vector3};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPosData {
    pub data: KeyGroup<Vector3>,
}
