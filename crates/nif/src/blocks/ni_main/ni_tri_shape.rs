use super::NiGeometry;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiTriShape {
    pub base: NiGeometry,
}

impl std::ops::Deref for NiTriShape {
    type Target = NiGeometry;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
