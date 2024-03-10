use super::ni_node::NiNode;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBillboardNode {
    pub base: NiNode,
    pub billboard_mode: BillboardMode,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum BillboardMode {
    #[br(magic = 0u16)]
    AlwaysFaceCamera,
    #[br(magic = 1u16)]
    RotateAboutUp,
    #[br(magic = 2u16)]
    RigidFaceCamera,
    #[br(magic = 3u16)]
    AlwaysFaceCenter,
    #[br(magic = 4u16)]
    RigidFaceCenter,
    #[br(magic = 5u16)]
    BSRotateAboutUp,
    #[br(magic = 9u16)]
    RotateAboutUp2,
}

impl NiBillboardNode {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiBillboardNode {
    type Target = NiNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
