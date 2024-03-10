use super::Vector3;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead, Clone)]
pub struct Matrix33 {
    pub column_major: [f32; 9],
}

impl From<&Matrix33> for glam::Mat3 {
    fn from(val: &Matrix33) -> Self {
        glam::Mat3::from_cols_array(&val.column_major).transpose()
    }
}

impl From<Matrix33> for glam::Mat3 {
    fn from(val: Matrix33) -> Self {
        (&val).into()
    }
}

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransform {
    pub rotation: Matrix33,
    pub translation: Vector3,
    pub scale: f32,
}
