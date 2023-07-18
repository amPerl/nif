use super::ni_object_net::NiObjectNET;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiStencilProperty {
    pub base: NiObjectNET,
    pub stencil_enabled: u8,
    pub stencil_function: StencilTestFunc,
    pub stencil_ref: u32,
    pub stencil_mask: u32,
    pub fail_action: StencilAction,
    pub zfail_action: StencilAction,
    pub pass_action: StencilAction,
    pub draw_mode: StencilDrawMode,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum StencilAction {
    #[br(magic = 0u32)]
    Keep,
    #[br(magic = 1u32)]
    Zero,
    #[br(magic = 2u32)]
    Replace,
    #[br(magic = 3u32)]
    Increment,
    #[br(magic = 4u32)]
    Decrement,
    #[br(magic = 5u32)]
    Invert,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum StencilDrawMode {
    #[br(magic = 0u32)]
    CcwOrBoth,
    #[br(magic = 1u32)]
    Ccw,
    #[br(magic = 2u32)]
    Cw,
    #[br(magic = 3u32)]
    Both,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum StencilTestFunc {
    #[br(magic = 0u32)]
    Never,
    #[br(magic = 1u32)]
    Less,
    #[br(magic = 2u32)]
    Equal,
    #[br(magic = 3u32)]
    LessEqual,
    #[br(magic = 4u32)]
    Greater,
    #[br(magic = 5u32)]
    NotEqual,
    #[br(magic = 6u32)]
    GreaterEqual,
    #[br(magic = 7u32)]
    Always,
}

impl NiStencilProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiStencilProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
