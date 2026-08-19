use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
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

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum StencilAction {
    #[brw(magic = 0u32)]
    Keep,
    #[brw(magic = 1u32)]
    Zero,
    #[brw(magic = 2u32)]
    Replace,
    #[brw(magic = 3u32)]
    Increment,
    #[brw(magic = 4u32)]
    Decrement,
    #[brw(magic = 5u32)]
    Invert,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum StencilDrawMode {
    #[brw(magic = 0u32)]
    CcwOrBoth,
    #[brw(magic = 1u32)]
    Ccw,
    #[brw(magic = 2u32)]
    Cw,
    #[brw(magic = 3u32)]
    Both,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum StencilTestFunc {
    #[brw(magic = 0u32)]
    Never,
    #[brw(magic = 1u32)]
    Less,
    #[brw(magic = 2u32)]
    Equal,
    #[brw(magic = 3u32)]
    LessEqual,
    #[brw(magic = 4u32)]
    Greater,
    #[brw(magic = 5u32)]
    NotEqual,
    #[brw(magic = 6u32)]
    GreaterEqual,
    #[brw(magic = 7u32)]
    Always,
}

impl std::ops::Deref for NiStencilProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
