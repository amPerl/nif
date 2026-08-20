use super::ni_node::NiNode;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiBillboardNode {
    pub base: NiNode,
    pub billboard_mode: BillboardMode,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum BillboardMode {
    #[brw(magic = 0u16)]
    AlwaysFaceCamera,
    #[brw(magic = 1u16)]
    RotateAboutUp,
    #[brw(magic = 2u16)]
    RigidFaceCamera,
    #[brw(magic = 3u16)]
    AlwaysFaceCenter,
    #[brw(magic = 4u16)]
    RigidFaceCenter,
    #[brw(magic = 5u16)]
    BSRotateAboutUp,
    #[brw(magic = 9u16)]
    RotateAboutUp2,
}

impl std::ops::Deref for NiBillboardNode {
    type Target = NiNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
