use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<&Vector2> for glam::Vec2 {
    fn from(val: &Vector2) -> Self {
        glam::Vec2::new(val.x, val.y)
    }
}

impl From<Vector2> for glam::Vec2 {
    fn from(val: Vector2) -> Self {
        (&val).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&Vector3> for glam::Vec3 {
    fn from(val: &Vector3) -> Self {
        glam::Vec3::new(val.x, val.y, val.z)
    }
}

impl From<Vector3> for glam::Vec3 {
    fn from(val: Vector3) -> Self {
        (&val).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct TexCoord {
    pub u: f32,
    pub v: f32,
}

impl From<&TexCoord> for glam::Vec2 {
    fn from(val: &TexCoord) -> Self {
        glam::Vec2::new(val.u, val.v)
    }
}

impl From<TexCoord> for glam::Vec2 {
    fn from(val: TexCoord) -> Self {
        (&val).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}
