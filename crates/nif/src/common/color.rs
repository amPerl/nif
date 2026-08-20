use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone, Copy)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for Color3 {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone, Copy)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color4 {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone, Copy, Default)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ByteColor4 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
