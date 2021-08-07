use binread::BinRead;

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct TexCoord {
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct NiQuatTransform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: f32,
}
