use super::ni_object_net::NiObjectNET;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiAlphaProperty {
    pub base: NiObjectNET,
    pub flags: u16,
    pub threshold: u8,
}

impl NiAlphaProperty {
    pub fn alpha_blend(&self) -> bool {
        self.flags & 0x0001 != 0
    }
    pub fn source_blend_mode(&self) -> AlphaFunction {
        match (self.flags & 0x001E) >> 1 {
            0 => AlphaFunction::One,
            1 => AlphaFunction::Zero,
            2 => AlphaFunction::SrcColor,
            3 => AlphaFunction::InvSrcColor,
            4 => AlphaFunction::DestColor,
            5 => AlphaFunction::InvDestColor,
            6 => AlphaFunction::SrcAlpha,
            7 => AlphaFunction::InvSrcAlpha,
            8 => AlphaFunction::DestAlpha,
            9 => AlphaFunction::InvDestAlpha,
            _ => AlphaFunction::SrcAlphaSaturate,
        }
    }
    pub fn destination_blend_mode(&self) -> AlphaFunction {
        match (self.flags & 0x01E0) >> 5 {
            0 => AlphaFunction::One,
            1 => AlphaFunction::Zero,
            2 => AlphaFunction::SrcColor,
            3 => AlphaFunction::InvSrcColor,
            4 => AlphaFunction::DestColor,
            5 => AlphaFunction::InvDestColor,
            6 => AlphaFunction::SrcAlpha,
            7 => AlphaFunction::InvSrcAlpha,
            8 => AlphaFunction::DestAlpha,
            9 => AlphaFunction::InvDestAlpha,
            _ => AlphaFunction::SrcAlphaSaturate,
        }
    }
    pub fn alpha_test(&self) -> bool {
        self.flags & 0x0200 != 0
    }
    pub fn test_func(&self) -> TestFunction {
        match (self.flags & 0x1C00) >> 10 {
            0 => TestFunction::TestAlways,
            1 => TestFunction::TestLess,
            2 => TestFunction::TestEqual,
            3 => TestFunction::TestLessEqual,
            4 => TestFunction::TestGreater,
            5 => TestFunction::TestNotEqual,
            6 => TestFunction::TestGreaterEqual,
            _ => TestFunction::TestNever,
        }
    }
    pub fn no_sorter(&self) -> bool {
        self.flags & 0x2000 != 0
    }
    pub fn clone_unique(&self) -> bool {
        self.flags & 0x4000 != 0
    }
    pub fn editor_alpha_threshold(&self) -> bool {
        self.flags & 0x8000 != 0
    }
}

impl std::ops::Deref for NiAlphaProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum TestFunction {
    #[brw(magic = 0u32)]
    TestAlways, // Always true. Buffer is ignored.
    #[brw(magic = 1u32)]
    TestLess, // VRef ‹ VBuf
    #[brw(magic = 2u32)]
    TestEqual, // VRef = VBuf
    #[brw(magic = 3u32)]
    TestLessEqual, // VRef ≤ VBuf
    #[brw(magic = 4u32)]
    TestGreater, // VRef › VBuf
    #[brw(magic = 5u32)]
    TestNotEqual, // VRef ≠ VBuf
    #[brw(magic = 6u32)]
    TestGreaterEqual, // VRef ≥ VBuf
    #[brw(magic = 7u32)]
    TestNever, // Always false. Ref value is ignored.
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub enum AlphaFunction {
    #[brw(magic = 0u32)]
    One,
    #[brw(magic = 1u32)]
    Zero,
    #[brw(magic = 2u32)]
    SrcColor,
    #[brw(magic = 3u32)]
    InvSrcColor,
    #[brw(magic = 4u32)]
    DestColor,
    #[brw(magic = 5u32)]
    InvDestColor,
    #[brw(magic = 6u32)]
    SrcAlpha,
    #[brw(magic = 7u32)]
    InvSrcAlpha,
    #[brw(magic = 8u32)]
    DestAlpha,
    #[brw(magic = 9u32)]
    InvDestAlpha,
    #[brw(magic = 10u32)]
    SrcAlphaSaturate,
}
