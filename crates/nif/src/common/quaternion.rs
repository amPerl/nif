use super::Vector3;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone, Copy)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiQuatTransform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: f32,
}

#[cfg(feature = "glam")]
impl From<&Quaternion> for glam::Quat {
    fn from(val: &Quaternion) -> Self {
        glam::Quat::from_xyzw(val.x, val.y, val.z, val.w)
    }
}

#[cfg(feature = "glam")]
impl From<Quaternion> for glam::Quat {
    fn from(val: Quaternion) -> Self {
        (&val).into()
    }
}

#[cfg(feature = "glam")]
impl From<glam::Quat> for Quaternion {
    fn from(val: glam::Quat) -> Self {
        Quaternion {
            w: val.w,
            x: val.x,
            y: val.y,
            z: val.z,
        }
    }
}
