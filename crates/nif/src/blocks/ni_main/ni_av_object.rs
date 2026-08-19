use super::ni_object_net::NiObjectNET;
use crate::common::{BlockRef, Matrix33, Vector3};


#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiAvObject {
    pub base: NiObjectNET,
    pub flags: u16,
    pub translation: Vector3,
    pub rotation: Matrix33,
    pub scale: f32,
    #[br(temp)]
    #[bw(calc = property_refs.len() as u32)]
    num_property_refs: u32,
    #[br(count = num_property_refs)]
    pub property_refs: Vec<BlockRef>,
    pub collision_ref: BlockRef,
}

impl std::ops::Deref for NiAvObject {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl NiAvObject {
    pub fn is_hidden(&self) -> bool {
        self.flags & 0x0001 != 0
    }
}
