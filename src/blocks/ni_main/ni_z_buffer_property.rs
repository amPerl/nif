use super::ni_object_net::NiObjectNET;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiZBufferProperty {
    pub base: NiObjectNET,
    pub flags: u16,
    pub function: ZCompareMode,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum ZCompareMode {
    #[br(magic = 0u32)]
    ZCompAlways,
    #[br(magic = 1u32)]
    ZCompLess,
    #[br(magic = 2u32)]
    ZCompEqual,
    #[br(magic = 3u32)]
    ZCompLessEqual,
    #[br(magic = 4u32)]
    ZCompGreater,
    #[br(magic = 5u32)]
    ZCompNotEqual,
    #[br(magic = 6u32)]
    ZCompGreaterEqual,
    #[br(magic = 7u32)]
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
