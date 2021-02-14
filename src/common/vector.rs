use binread::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
