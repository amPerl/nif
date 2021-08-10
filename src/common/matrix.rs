use binread::BinRead;

use super::Vector3;

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Matrix33 {
    pub column_major: [f32; 9],
}

#[cfg(feature = "glam")]
impl Into<glam::Mat3> for &Matrix33 {
    fn into(self) -> glam::Mat3 {
        let result = glam::Mat3::from_cols_array(&self.column_major);
        result.transpose()
    }
}

#[cfg(feature = "glam")]
impl Into<glam::Mat3> for Matrix33 {
    fn into(self) -> glam::Mat3 {
        (&self).into()
    }
}

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransform {
    pub rotation: Matrix33,
    pub translation: Vector3,
    pub scale: f32,
}
