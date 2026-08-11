use super::ni_object_net::NiObjectNET;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
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
