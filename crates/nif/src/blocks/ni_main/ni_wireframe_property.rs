use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiWireframeProperty {
    pub base: NiObjectNET,
    pub flags: u16,
}

impl NiWireframeProperty {
    pub fn is_enabled(&self) -> bool {
        self.flags != 0
    }
}

impl std::ops::Deref for NiWireframeProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
