use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiZBufferProperty {
    pub base: NiObjectNET,
    pub flags: u16,
    pub function: ZCompareMode,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum ZCompareMode {
    #[brw(magic = 0u32)]
    ZCompAlways,
    #[brw(magic = 1u32)]
    ZCompLess,
    #[brw(magic = 2u32)]
    ZCompEqual,
    #[brw(magic = 3u32)]
    ZCompLessEqual,
    #[brw(magic = 4u32)]
    ZCompGreater,
    #[brw(magic = 5u32)]
    ZCompNotEqual,
    #[brw(magic = 6u32)]
    ZCompGreaterEqual,
    #[brw(magic = 7u32)]
    ZCompNever,
    Unknown(u32),
}

impl std::ops::Deref for NiZBufferProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl NiZBufferProperty {
    pub fn depth_test(&self) -> bool {
        self.flags & 0x0001 != 0
    }
    pub fn depth_write(&self) -> bool {
        self.flags & 0x0002 != 0
    }
}
