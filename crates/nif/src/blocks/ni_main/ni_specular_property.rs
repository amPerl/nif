use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiSpecularProperty {
    pub base: NiObjectNET,
    pub flags: u16,
}

impl NiSpecularProperty {
    pub fn is_enabled(&self) -> bool {
        self.flags != 0
    }
}

impl std::ops::Deref for NiSpecularProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
