use binread::BinRead;

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[cfg(feature = "glam")]
impl Into<glam::Vec2> for &Vector2 {
    fn into(self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}

#[cfg(feature = "glam")]
impl Into<glam::Vec2> for Vector2 {
    fn into(self) -> glam::Vec2 {
        (&self).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(feature = "glam")]
impl Into<glam::Vec3> for &Vector3 {
    fn into(self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, self.z)
    }
}

#[cfg(feature = "glam")]
impl Into<glam::Vec3> for Vector3 {
    fn into(self) -> glam::Vec3 {
        (&self).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct TexCoord {
    pub u: f32,
    pub v: f32,
}

#[cfg(feature = "glam")]
impl Into<glam::Vec2> for &TexCoord {
    fn into(self) -> glam::Vec2 {
        glam::Vec2::new(self.u, self.v)
    }
}

#[cfg(feature = "glam")]
impl Into<glam::Vec2> for TexCoord {
    fn into(self) -> glam::Vec2 {
        (&self).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}
