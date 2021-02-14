use binread::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
